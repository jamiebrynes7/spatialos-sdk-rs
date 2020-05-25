use std::ffi::CString;
use std::ptr;

use spatialos_sdk_sys::worker::*;

use crate::worker::{
    parameters::ProtocolLoggingParameters,
    parameters::LogsinkParameters,
    parameters::logsinks_to_worker_sdk,
    utils::cstr_to_string,
    worker_future::{WorkerFuture, WorkerSdkFuture},
};

pub struct Locator {
    pub(crate) locator: *mut Worker_Locator,
}

impl Locator {
    pub fn new<T: Into<Vec<u8>>>(hostname: T, port: u16, params: &LocatorParameters) -> Self {
        unsafe {
            let hostname = CString::new(hostname).unwrap();
            let worker_params = params.to_worker_sdk();
            let ptr = Worker_Locator_Create(hostname.as_ptr(), port, &worker_params);
            assert!(!ptr.is_null());
            Locator { locator: ptr }
        }
    }

    pub fn create_development_player_identity_token(
        hostname: &str,
        port: u16,
        request: PlayerIdentityTokenRequest,
    ) -> WorkerFuture<PlayerIdentityTokenFuture> {
        let cstr = CString::new(hostname).unwrap();
        WorkerFuture::new(PlayerIdentityTokenFuture::new(cstr, port, request))
    }

    pub fn create_development_login_tokens(
        hostname: &str,
        port: u16,
        request: LoginTokensRequest,
    ) -> WorkerFuture<LoginTokensFuture> {
        let cstr = CString::new(hostname).unwrap();
        WorkerFuture::new(LoginTokensFuture::new(cstr, port, request))
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
    pub credentials: PlayerIdentityCredentials,
    pub use_insecure_connection: bool,
    pub logging: Option<ProtocolLoggingParameters>,
    pub logsinks: Vec<LogsinkParameters>,
}

impl LocatorParameters {
    fn to_worker_sdk(&self) -> Worker_LocatorParameters {
        Worker_LocatorParameters {
            project_name: ::std::ptr::null(),
            credentials_type:
                Worker_LocatorCredentialsTypes_WORKER_LOCATOR_PLAYER_IDENTITY_CREDENTIALS as u8,
            login_token: Worker_LoginTokenCredentials::default(),
            steam: Worker_SteamCredentials::default(),
            player_identity: self.credentials.to_worker_sdk(),
            use_insecure_connection: self.use_insecure_connection as u8,
            logging: match self.logging {
                Some(ref params) => params.to_worker_sdk(),
                None => ProtocolLoggingParameters::default().to_worker_sdk(),
            },
            enable_logging: self.logging.is_some() as u8,
            logsink_count: self.logsinks.len() as u32,
            logsinks: logsinks_to_worker_sdk(&self.logsinks),
        }
    }

    pub fn new(credentials: PlayerIdentityCredentials) -> Self {
        LocatorParameters {
            credentials,
            use_insecure_connection: false,
            logging: None,
            logsinks: Default::default(),
        }
    }

    pub fn with_insecure_connection(mut self) -> Self {
        self.use_insecure_connection = true;
        self
    }

    pub fn with_logging(self) -> Self {
        self.with_logging_params(ProtocolLoggingParameters::default())
    }

    pub fn with_logging_params(mut self, params: ProtocolLoggingParameters) -> Self {
        self.logging = Some(params);
        self
    }
}

pub struct PlayerIdentityCredentials {
    player_identity_token: CString,
    login_token: CString,
}

impl PlayerIdentityCredentials {
    pub fn new<S: AsRef<str>, T: AsRef<str>>(pit: S, token: T) -> Self {
        PlayerIdentityCredentials {
            player_identity_token: CString::new(pit.as_ref()).expect("`pit` contained a null byte"),
            login_token: CString::new(token.as_ref()).expect("`token` contained a null byte"),
        }
    }

    fn to_worker_sdk(&self) -> Worker_PlayerIdentityCredentials {
        Worker_PlayerIdentityCredentials {
            player_identity_token: self.player_identity_token.as_ptr(),
            login_token: self.login_token.as_ptr(),
        }
    }
}

pub struct PlayerIdentityTokenRequest {
    pub dev_auth_token: CString,
    pub player_id: CString,
    pub duration_seconds: Option<u32>,
    pub display_name: Option<CString>,
    pub metadata: Option<CString>,
    pub use_insecure_connection: bool,
}

impl PlayerIdentityTokenRequest {
    pub fn new<S: AsRef<str>, T: AsRef<str>>(dev_auth_token: S, player_id: T) -> Self {
        PlayerIdentityTokenRequest {
            dev_auth_token: CString::new(dev_auth_token.as_ref())
                .expect("`dev_auth_token` contained a null byte"),
            player_id: CString::new(player_id.as_ref()).expect("`player_id` contained a null byte"),
            duration_seconds: None,
            display_name: None,
            metadata: None,
            use_insecure_connection: false,
        }
    }

    pub fn with_duration_secs(mut self, seconds: u32) -> Self {
        self.duration_seconds = Some(seconds);
        self
    }

    pub fn with_display_name<S: AsRef<str>>(mut self, display_name: S) -> Self {
        self.display_name = Some(
            CString::new(display_name.as_ref()).expect("`display_name` contained a null byte"),
        );
        self
    }

    pub fn with_metadata<S: AsRef<str>>(mut self, metadata: S) -> Self {
        self.metadata =
            Some(CString::new(metadata.as_ref()).expect("`metadata` contained a null bytes"));
        self
    }

    pub fn with_insecure_connection(mut self) -> Self {
        self.use_insecure_connection = true;
        self
    }

    fn to_worker_sdk(&self) -> Worker_Alpha_PlayerIdentityTokenRequest {
        Worker_Alpha_PlayerIdentityTokenRequest {
            development_authentication_token: self.dev_auth_token.as_ptr(),
            player_id: self.player_id.as_ptr(),
            duration_seconds: match self.duration_seconds {
                Some(ref value) => value as *const u32,
                None => ::std::ptr::null_mut(),
            },
            display_name: match self.display_name {
                Some(ref cstr) => cstr.as_ptr(),
                None => ::std::ptr::null(),
            },
            metadata: match self.metadata {
                Some(ref cstr) => cstr.as_ptr(),
                None => ::std::ptr::null(),
            },
            use_insecure_connection: self.use_insecure_connection as u8,
        }
    }
}

pub struct PlayerIdentityTokenResponse {
    pub player_identity_token: String,
}

impl PlayerIdentityTokenResponse {
    fn from_worker_sdk(response: &Worker_Alpha_PlayerIdentityTokenResponse) -> Self {
        PlayerIdentityTokenResponse {
            player_identity_token: cstr_to_string(response.player_identity_token),
        }
    }
}

pub struct PlayerIdentityTokenFuture {
    request: PlayerIdentityTokenRequest,
    hostname: CString,
    port: u16,
}

impl PlayerIdentityTokenFuture {
    fn new(hostname: CString, port: u16, request: PlayerIdentityTokenRequest) -> Self {
        PlayerIdentityTokenFuture {
            request,
            hostname,
            port,
        }
    }

    extern "C" fn callback_handler(
        user_data: *mut ::std::os::raw::c_void,
        response: *const Worker_Alpha_PlayerIdentityTokenResponse,
    ) {
        assert!(!response.is_null());
        unsafe {
            let response = *response;
            let data = &mut *(user_data as *mut Result<PlayerIdentityTokenResponse, String>);
            if Worker_ConnectionStatusCode::from(response.status.code)
                != Worker_ConnectionStatusCode_WORKER_CONNECTION_STATUS_CODE_SUCCESS
            {
                let err = cstr_to_string(response.status.detail);
                *data = Err(err);
                return;
            }

            let response = PlayerIdentityTokenResponse::from_worker_sdk(&response);
            *data = Ok(response);
        }
    }
}

impl WorkerSdkFuture for PlayerIdentityTokenFuture {
    type RawPointer = Worker_Alpha_PlayerIdentityTokenResponseFuture;
    type Output = Result<PlayerIdentityTokenResponse, String>;

    fn start(&self) -> *mut Self::RawPointer {
        unsafe {
            let mut params = self.request.to_worker_sdk();
            Worker_Alpha_CreateDevelopmentPlayerIdentityTokenAsync(
                self.hostname.as_ptr(),
                self.port,
                &mut params,
            )
        }
    }

    unsafe fn get(ptr: *mut Self::RawPointer) -> Self::Output {
        let mut data: Result<PlayerIdentityTokenResponse, String> =
            Err("Callback never called.".into());
        Worker_Alpha_PlayerIdentityTokenResponseFuture_Get(
            ptr,
            ptr::null(),
            &mut data as *mut _ as *mut ::std::os::raw::c_void,
            Some(PlayerIdentityTokenFuture::callback_handler),
        );

        data
    }

    unsafe fn destroy(ptr: *mut Self::RawPointer) {
        Worker_Alpha_PlayerIdentityTokenResponseFuture_Destroy(ptr)
    }
}

pub struct LoginTokensRequest {
    pub player_identity_token: CString,
    pub worker_type: CString,
    pub duration_seconds: Option<u32>,
    pub use_insecure_connection: bool,
}

impl LoginTokensRequest {
    pub fn new<S: AsRef<str>, T: AsRef<str>>(player_identity_token: S, worker_type: T) -> Self {
        LoginTokensRequest {
            player_identity_token: CString::new(player_identity_token.as_ref())
                .expect("`player_identity_token` contained a null byte"),
            worker_type: CString::new(worker_type.as_ref())
                .expect("`worker_type` contained a null byte"),
            duration_seconds: None,
            use_insecure_connection: false,
        }
    }

    pub fn with_duration_seconds(mut self, duration: u32) -> Self {
        self.duration_seconds = Some(duration);
        self
    }

    pub fn with_insecure_connection(mut self) -> Self {
        self.use_insecure_connection = true;
        self
    }

    fn to_worker_sdk(&self) -> Worker_Alpha_LoginTokensRequest {
        Worker_Alpha_LoginTokensRequest {
            player_identity_token: self.player_identity_token.as_ptr(),
            worker_type: self.worker_type.as_ptr(),
            duration_seconds: match self.duration_seconds {
                Some(ref value) => value as *const _ as *mut u32,
                None => ::std::ptr::null_mut(),
            },
            use_insecure_connection: self.use_insecure_connection as u8,
        }
    }
}

pub struct LoginTokensResponse {
    pub login_tokens: Vec<LoginTokenDetails>,
}

impl LoginTokensResponse {
    fn from_worker_sdk(response: &Worker_Alpha_LoginTokensResponse) -> Self {
        unsafe {
            let tokens = ::std::slice::from_raw_parts(
                response.login_tokens,
                response.login_token_count as usize,
            )
            .iter()
            .map(|token| LoginTokenDetails::from_worker_sdk(token))
            .collect::<Vec<LoginTokenDetails>>();

            LoginTokensResponse {
                login_tokens: tokens,
            }
        }
    }
}

pub struct LoginTokenDetails {
    pub deployment_id: String,
    pub deployment_name: String,
    pub tags: Vec<String>,
    pub login_token: String,
}

impl LoginTokenDetails {
    fn from_worker_sdk(token_details: &Worker_Alpha_LoginTokenDetails) -> Self {
        unsafe {
            let tags =
                ::std::slice::from_raw_parts(token_details.tags, token_details.tag_count as usize)
                    .iter()
                    .map(|tag| cstr_to_string(*tag))
                    .collect::<Vec<String>>();

            LoginTokenDetails {
                deployment_id: cstr_to_string(token_details.deployment_id),
                deployment_name: cstr_to_string(token_details.deployment_name),
                tags,
                login_token: cstr_to_string(token_details.login_token),
            }
        }
    }
}

pub struct LoginTokensFuture {
    request: LoginTokensRequest,
    hostname: CString,
    port: u16,
}

impl LoginTokensFuture {
    fn new(hostname: CString, port: u16, request: LoginTokensRequest) -> Self {
        LoginTokensFuture {
            request,
            hostname,
            port,
        }
    }

    extern "C" fn callback_handler(
        user_data: *mut ::std::os::raw::c_void,
        response: *const Worker_Alpha_LoginTokensResponse,
    ) {
        assert!(!response.is_null());
        unsafe {
            let response = *response;
            let data = &mut *(user_data as *mut Result<LoginTokensResponse, String>);
            if Worker_ConnectionStatusCode::from(response.status.code)
                != Worker_ConnectionStatusCode_WORKER_CONNECTION_STATUS_CODE_SUCCESS
            {
                let err = cstr_to_string(response.status.detail);
                *data = Err(err);
                return;
            }

            let response = LoginTokensResponse::from_worker_sdk(&response);
            *data = Ok(response);
        }
    }
}

impl WorkerSdkFuture for LoginTokensFuture {
    type RawPointer = Worker_Alpha_LoginTokensResponseFuture;
    type Output = Result<LoginTokensResponse, String>;

    fn start(&self) -> *mut Self::RawPointer {
        let mut params = self.request.to_worker_sdk();
        unsafe {
            Worker_Alpha_CreateDevelopmentLoginTokensAsync(
                self.hostname.as_ptr(),
                self.port,
                &mut params,
            )
        }
    }

    unsafe fn get(ptr: *mut Self::RawPointer) -> Self::Output {
        let mut data: Result<LoginTokensResponse, String> = Err("Callback never called.".into());
        Worker_Alpha_LoginTokensResponseFuture_Get(
            ptr,
            ptr::null(),
            &mut data as *mut _ as *mut ::std::os::raw::c_void,
            Some(LoginTokensFuture::callback_handler),
        );

        data
    }

    unsafe fn destroy(ptr: *mut Self::RawPointer) {
        Worker_Alpha_LoginTokensResponseFuture_Destroy(ptr)
    }
}
