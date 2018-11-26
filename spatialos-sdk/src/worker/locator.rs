use spatialos_sdk_sys::worker::*;

use std::ffi::{CStr, CString};

use worker::parameters::ProtocolLoggingParameters;
use worker::internal::utils::cstr_to_string;

pub struct Locator {
    pub(crate) locator: *mut Worker_Locator,
}

impl Locator {
    pub fn new(hostname: &str, params: &LocatorParameters) -> Self {
        unsafe {
            let hostname = CString::new(hostname).unwrap();
            let (worker_params, underlying_data) = params.to_worker_sdk();
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
                future: future_ptr,
                was_consumed: false,
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
    pub project_name: String,
    pub credentials: LocatorCredentials,
    pub logging: ProtocolLoggingParameters,
    pub enable_logging: bool,
}

impl LocatorParameters {
    fn to_worker_sdk(&self) -> (Worker_LocatorParameters, Vec<CString>) {
        let project_name_cstr = CString::new(self.project_name.as_str()).unwrap();
        let (credentials_type, login_token_credentials, steam_credentials, mut underlying_data) =
            self.credentials.to_worker_sdk();
        underlying_data.push(project_name_cstr);
        (Worker_LocatorParameters {
            project_name: underlying_data[underlying_data.len() - 1].as_ptr(),
            credentials_type,
            login_token: login_token_credentials,
            steam: steam_credentials,
            logging: self.logging.to_worker_sdk(),
            enable_logging: self.enable_logging as u8,
        }, underlying_data)
    }
}

pub enum LocatorCredentials {
    LoginToken(String),
    Steam(SteamCredentials),
}

impl LocatorCredentials {
    fn to_worker_sdk(
        &self,
    ) -> (
        u8,
        Worker_LoginTokenCredentials,
        Worker_SteamCredentials,
        Vec<CString>,
    ) {
        match self {
            LocatorCredentials::LoginToken(token) => {
                let token_cstr = CString::new(token.as_str()).unwrap();
                (
                    Worker_LocatorCredentialsTypes_WORKER_LOCATOR_LOGIN_TOKEN_CREDENTIALS as u8,
                    Worker_LoginTokenCredentials {
                        token: token_cstr.as_ptr(),
                    },
                    Worker_SteamCredentials {
                        ticket: ::std::ptr::null(),
                        deployment_tag: ::std::ptr::null(),
                    },
                    vec![token_cstr],
                )
            }
            LocatorCredentials::Steam(steam_credentials) => {
                let ticket_cstr = CString::new(steam_credentials.ticket.as_str()).unwrap();
                let tag_cstr = CString::new(steam_credentials.deployment_tag.as_str()).unwrap();
                (
                    Worker_LocatorCredentialsTypes_WORKER_LOCATOR_STEAM_CREDENTIALS as u8,
                    Worker_LoginTokenCredentials {
                        token: ::std::ptr::null(),
                    },
                    Worker_SteamCredentials {
                        ticket: ticket_cstr.as_ptr(),
                        deployment_tag: tag_cstr.as_ptr(),
                    },
                    vec![ticket_cstr, tag_cstr],
                )
            }
        }
    }
}

pub struct SteamCredentials {
    pub ticket: String,
    pub deployment_tag: String,
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
    future: *mut Worker_DeploymentListFuture,
    was_consumed: bool,
}

impl DeploymentListFuture {
    pub fn get(&mut self) -> Result<Vec<Deployment>, String> {
        if self.was_consumed {
            return Err("DeploymentListFuture has already been consumed".to_owned());
        }

        assert!(!self.future.is_null());
        let mut data: Result<Vec<Deployment>, String> = Ok(vec![]);
        unsafe {
            Worker_DeploymentListFuture_Get(
                self.future,
                ::std::ptr::null(),
                (&mut data as *mut Result<Vec<Deployment>, String>) as *mut ::std::os::raw::c_void,
                Some(DeploymentListFuture::deployment_list_handler),
            );
        }
        self.was_consumed = true;
        data
    }

    extern "C" fn deployment_list_handler(
        user_data: *mut ::std::os::raw::c_void,
        deployment_list: *const Worker_DeploymentList,
    ) {
        assert!(!deployment_list.is_null());
        unsafe {
            let list = (*deployment_list);
            let mut data = &mut *(user_data as *mut Result<Vec<Deployment>, String>);
            let err_ptr = list.error;
            if !err_ptr.is_null() {
                let err = cstr_to_string(err_ptr);
                *data = Err(err);
                return;
            }

            let deployments =
                ::std::slice::from_raw_parts(list.deployments, list.deployment_count as usize)
                    .iter()
                    .map(|deployment| Deployment::from_worker_sdk(deployment))
                    .collect::<Vec<Deployment>>();

            *data = Ok(deployments);
        }
    }
}

impl Drop for DeploymentListFuture {
    fn drop(&mut self) {
        if !self.future.is_null() {
            unsafe { Worker_DeploymentListFuture_Destroy(self.future) }
        }
    }
}

pub type QueueStatusCallback = fn(Result<u32, String>) -> bool;

pub(crate) extern "C" fn queue_status_callback_handler(
    user_data: *mut ::std::os::raw::c_void,
    queue_status: *const Worker_QueueStatus
) -> u8 {
    unsafe {
        let status = *queue_status;
        let callback = *(user_data as *mut QueueStatusCallback);
        if status.error.is_null() {
            return callback(Ok(status.position_in_queue)) as u8;
        }
        let str = CStr::from_ptr(status.error);
        callback(Err(str.to_string_lossy().to_string())) as u8
    }
}