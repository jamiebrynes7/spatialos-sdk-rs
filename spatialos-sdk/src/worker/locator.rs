use std::ffi::{CStr, CString};

use futures::{Async, Future};

use spatialos_sdk_sys::worker::*;

use crate::worker::internal::utils::cstr_to_string;
use crate::worker::parameters::ProtocolLoggingParameters;

pub struct Locator {
    pub(crate) locator: *mut Worker_Locator,
}

impl Locator {
    pub fn new<T: Into<Vec<u8>>>(hostname: T, params: &LocatorParameters) -> Self {
        unsafe {
            let hostname = CString::new(hostname).unwrap();
            let worker_params = params.to_worker_sdk();
            let ptr = Worker_Locator_Create(hostname.as_ptr(), &worker_params);
            assert!(!ptr.is_null());
            Locator { locator: ptr }
        }
    }

    pub fn get_deployment_list_async(&self) -> DeploymentListFuture {
        unsafe {
            let future_ptr = Worker_Locator_GetDeploymentListAsync(self.locator);
            assert!(!future_ptr.is_null());
            DeploymentListFuture {
                internal: future_ptr,
                consumed: false,
            }
        }
    }
}

impl Drop for Locator {
    fn drop(&mut self) {
        if !self.locator.is_null() {
            unsafe { Worker_Locator_Destroy(self.locator) }
        }
    }
}

pub struct LocatorParameters {
    pub project_name: CString,
    pub credentials: LocatorCredentials,
    pub logging: ProtocolLoggingParameters,
    pub enable_logging: bool,
}

impl LocatorParameters {
    fn to_worker_sdk(&self) -> Worker_LocatorParameters {
        let credentials = self.credentials.to_worker_sdk();
        let (credentials_type, login_token, steam) = credentials;

        Worker_LocatorParameters {
            project_name: self.project_name.as_ptr(),
            credentials_type,
            login_token,
            steam,
            logging: self.logging.to_worker_sdk(),
            enable_logging: self.enable_logging as u8,
        }
    }

    pub fn new<T: AsRef<str>>(project_name: T, credentials: LocatorCredentials) -> Self {
        LocatorParameters {
            project_name: CString::new(project_name.as_ref())
                .expect("`project_name` contains a null byte"),
            credentials,
            logging: ProtocolLoggingParameters::default(),
            enable_logging: false,
        }
    }

    pub fn with_logging(mut self) -> Self {
        self.enable_logging = true;
        self
    }

    pub fn with_logging_parameters(mut self, params: ProtocolLoggingParameters) -> Self {
        self.logging = params;
        self.with_logging()
    }
}

pub enum LocatorCredentials {
    LoginToken(CString),
    Steam(SteamCredentials),
}

impl LocatorCredentials {
    pub fn login_token<S: AsRef<str>>(token: S) -> Self {
        LocatorCredentials::LoginToken(
            CString::new(token.as_ref()).expect("`token` contained null byte"),
        )
    }
}

impl LocatorCredentials {
    fn to_worker_sdk(&self) -> (u8, Worker_LoginTokenCredentials, Worker_SteamCredentials) {
        match self {
            LocatorCredentials::LoginToken(token) => (
                Worker_LocatorCredentialsTypes_WORKER_LOCATOR_LOGIN_TOKEN_CREDENTIALS as u8,
                Worker_LoginTokenCredentials {
                    token: token.as_ptr(),
                },
                Worker_SteamCredentials {
                    ticket: ::std::ptr::null(),
                    deployment_tag: ::std::ptr::null(),
                },
            ),
            LocatorCredentials::Steam(steam_credentials) => (
                Worker_LocatorCredentialsTypes_WORKER_LOCATOR_STEAM_CREDENTIALS as u8,
                Worker_LoginTokenCredentials {
                    token: ::std::ptr::null(),
                },
                Worker_SteamCredentials {
                    ticket: steam_credentials.ticket.as_ptr(),
                    deployment_tag: steam_credentials.deployment_tag.as_ptr(),
                },
            ),
        }
    }
}

pub struct SteamCredentials {
    pub ticket: CString,
    pub deployment_tag: CString,
}

impl SteamCredentials {
    pub fn new<S: AsRef<str>, T: AsRef<str>>(ticket: S, deployment_tag: T) -> Self {
        SteamCredentials {
            ticket: CString::new(ticket.as_ref()).expect("`ticket` contained null byte"),
            deployment_tag: CString::new(deployment_tag.as_ref())
                .expect("`deployment_tag` contained null byte"),
        }
    }
}

pub struct Deployment {
    pub deployment_name: String,
    pub assembly_name: String,
    pub description: String,
    pub users_connected: u32,
    pub users_capacity: u32,
}

impl Deployment {
    fn from_worker_sdk(deployment: &Worker_Deployment) -> Self {
        Deployment {
            deployment_name: cstr_to_string(deployment.deployment_name),
            assembly_name: cstr_to_string(deployment.assembly_name),
            description: cstr_to_string(deployment.description),
            users_connected: deployment.users_connected,
            users_capacity: deployment.users_capacity,
        }
    }
}

pub struct DeploymentListFuture {
    internal: *mut Worker_DeploymentListFuture,
    consumed: bool,
}

impl DeploymentListFuture {
    extern "C" fn callback_handler(
        user_data: *mut ::std::os::raw::c_void,
        deployment_list: *const Worker_DeploymentList,
    ) {
        assert!(!deployment_list.is_null());
        unsafe {
            let list = *deployment_list;
            let data = &mut *(user_data as *mut Option<Result<Vec<Deployment>, String>>);
            if !list.error.is_null() {
                let err = cstr_to_string(list.error);
                *data = Some(Err(err));
                return;
            }

            let deployments =
                ::std::slice::from_raw_parts(list.deployments, list.deployment_count as usize)
                    .iter()
                    .map(|deployment| Deployment::from_worker_sdk(deployment))
                    .collect::<Vec<Deployment>>();

            *data = Some(Ok(deployments));
        }
    }
}

impl Future for DeploymentListFuture {
    type Item = Vec<Deployment>;
    type Error = String;

    fn poll(&mut self) -> Result<Async<<Self as Future>::Item>, <Self as Future>::Error> {
        if self.consumed {
            return Err("DeploymentListFuture has already been consumed".to_owned());
        }

        assert!(!self.internal.is_null());
        let mut data: Option<Result<Vec<Deployment>, String>> = None;
        unsafe {
            Worker_DeploymentListFuture_Get(
                self.internal,
                &0,
                &mut data as *mut _ as *mut ::std::os::raw::c_void,
                Some(DeploymentListFuture::callback_handler),
            );
        }

        data.map_or(Ok(Async::NotReady), |result| {
            self.consumed = true;
            result.map(Async::Ready)
        })
    }

    fn wait(self) -> Result<<Self as Future>::Item, <Self as Future>::Error>
    where
        Self: Sized,
    {
        if self.consumed {
            return Err("DeploymentListFuture has already been consumed".to_owned());
        }

        assert!(!self.internal.is_null());
        let mut data: Option<Result<Vec<Deployment>, String>> = None;
        unsafe {
            Worker_DeploymentListFuture_Get(
                self.internal,
                ::std::ptr::null(),
                &mut data as *mut _ as *mut ::std::os::raw::c_void,
                Some(DeploymentListFuture::callback_handler),
            );
        }

        data.expect("Blocking call to Worker_DeploymentListFuture_Get did not trigger callback")
    }
}

impl Drop for DeploymentListFuture {
    fn drop(&mut self) {
        if !self.internal.is_null() {
            unsafe { Worker_DeploymentListFuture_Destroy(self.internal) }
        }
    }
}

pub type QueueStatusCallback = fn(&Result<u32, String>) -> bool;

pub(crate) extern "C" fn queue_status_callback_handler(
    user_data: *mut ::std::os::raw::c_void,
    queue_status: *const Worker_QueueStatus,
) -> u8 {
    unsafe {
        let status = *queue_status;
        let callback = *(user_data as *mut QueueStatusCallback);
        if status.error.is_null() {
            return callback(&Ok(status.position_in_queue)) as u8;
        }
        let str = CStr::from_ptr(status.error);
        callback(&Err(str.to_string_lossy().to_string())) as u8
    }
}
