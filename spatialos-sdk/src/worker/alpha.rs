use std::ffi::CString;

use futures::{Async, Future};

use spatialos_sdk_sys::worker::*;

use crate::worker::{
    internal::utils::cstr_to_string,
    parameters::ProtocolLoggingParameters,
};

pub struct Locator {
    pub(crate) internal: *mut Worker_Alpha_Locator,
}

impl Locator {
    pub fn create_development_player_identity_token(
        hostname: &str,
        port: u16,
        request: &mut PlayerIdentityTokenRequest,
    ) -> PlayerIdentityTokenFuture {
        unsafe {
            let cstr = CString::new(hostname).unwrap();
            let mut params = request.to_worker_sdk();
            PlayerIdentityTokenFuture::new(Worker_Alpha_CreateDevelopmentPlayerIdentityTokenAsync(
                cstr.as_ptr(),
                port,
                &mut params as *mut _,
            ))
        }
    }

    pub fn create_development_login_tokens(
        hostname: &str,
        port: u16,
        request: &mut LoginTokensRequest,
    ) -> LoginTokensFuture {
        unsafe {
            let cstr = CString::new(hostname).unwrap();
            let mut params = request.to_worker_sdk();
            LoginTokensFuture::new(Worker_Alpha_CreateDevelopmentLoginTokensAsync(
                cstr.as_ptr(),
                port,
                &mut params as *mut _,
            ))
        }
    }

    pub fn new(hostname: &str, port: u16, params: &LocatorParameters) -> Self {
        let hostname = CString::new(hostname).unwrap();
        let cparams = params.to_worker_sdk();

        unsafe {
            let ptr = Worker_Alpha_Locator_Create(hostname.as_ptr(), port, &cparams);

            Locator { internal: ptr }
        }
    }
}

pub struct LocatorParameters {
    player_identity: PlayerIdentityCredentials,
    use_insecure_connection: bool,
    logging: Option<ProtocolLoggingParameters>,
}

impl LocatorParameters {
    pub fn new(credentials: PlayerIdentityCredentials) -> Self {
        LocatorParameters {
            player_identity: credentials,
            use_insecure_connection: false,
            logging: None,
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

    fn to_worker_sdk(&self) -> Worker_Alpha_LocatorParameters {
        let credentials = self.player_identity.to_worker_sdk();

        Worker_Alpha_LocatorParameters {
            player_identity: credentials,
            use_insecure_connection: self.use_insecure_connection as u8,
            enable_logging: self.logging.is_some() as u8,
            logging: match self.logging {
                Some(ref params) => params.to_worker_sdk(),
                None => ProtocolLoggingParameters::default().to_worker_sdk(),
            },
        }
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

    fn to_worker_sdk(
        &self,
    ) -> Worker_Alpha_PlayerIdentityCredentials {
        Worker_Alpha_PlayerIdentityCredentials {
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
            dev_auth_token: CString::new(dev_auth_token.as_ref()).expect("`dev_auth_token` contained a null byte"),
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
        self.display_name = Some(CString::new(display_name.as_ref()).expect("`display_name` contained a null byte"));
        self
    }

    pub fn with_metadata<S: AsRef<str>>(mut self, metadata: S) -> Self {
        self.metadata = Some(CString::new(metadata.as_ref()).expect("`metadata` contained a null bytes"));
        self
    }

    pub fn with_insecure_connection(mut self) -> Self {
        self.use_insecure_connection = true;
        self
    }

    fn to_worker_sdk(
        &self,
    ) -> Worker_Alpha_PlayerIdentityTokenRequest {

        Worker_Alpha_PlayerIdentityTokenRequest {
            development_authentication_token_id: self.dev_auth_token.as_ptr(),
            player_id: self.player_id.as_ptr(),
            duration_seconds: match self.duration_seconds {
                Some(ref value) => value as *const _ as *mut u32, // TODO: Remove double cast when C SDK is fixed.
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
    internal: *mut Worker_Alpha_PlayerIdentityTokenResponseFuture,
    consumed: bool,
}

impl PlayerIdentityTokenFuture {
    fn new(ptr: *mut Worker_Alpha_PlayerIdentityTokenResponseFuture) -> Self {
        PlayerIdentityTokenFuture {
            internal: ptr,
            consumed: false,
        }
    }

    extern "C" fn callback_handler(
        user_data: *mut ::std::os::raw::c_void,
        response: *const Worker_Alpha_PlayerIdentityTokenResponse,
    ) {
        assert!(!response.is_null());
        unsafe {
            let response = *response;
            let data =
                &mut *(user_data as *mut Option<Result<PlayerIdentityTokenResponse, String>>);
            if !response.error.is_null() {
                let err = cstr_to_string(response.error);
                *data = Some(Err(err));
                return;
            }

            let response = PlayerIdentityTokenResponse::from_worker_sdk(&response);
            *data = Some(Ok(response));
        }
    }

    fn get_future(
        &self,
        timeout: Option<u32>,
    ) -> Option<Result<PlayerIdentityTokenResponse, String>> {
        assert!(!self.internal.is_null());

        let mut data: Option<Result<PlayerIdentityTokenResponse, String>> = None;
        unsafe {
            Worker_Alpha_PlayerIdentityTokenResponseFuture_Get(
                self.internal,
                timeout.map_or(::std::ptr::null(), |value| &value),
                &mut data as *mut _ as *mut ::std::os::raw::c_void,
                Some(PlayerIdentityTokenFuture::callback_handler),
            );
        }

        data
    }
}

impl Future for PlayerIdentityTokenFuture {
    type Item = PlayerIdentityTokenResponse;
    type Error = String;

    fn poll(&mut self) -> Result<Async<<Self as Future>::Item>, <Self as Future>::Error> {
        if self.consumed {
            return Err("PlayerIdentityTokenFuture has already been consumed".to_owned());
        }

        self.get_future(Some(0))
            .map_or(Ok(Async::NotReady), |result| {
                self.consumed = true;
                result.map(Async::Ready)
            })
    }

    fn wait(self) -> Result<<Self as Future>::Item, <Self as Future>::Error>
        where
            Self: Sized,
    {
        if self.consumed {
            return Err("PlayerIdentityTokenFuture has already been consumed".to_owned());
        }

        self.get_future(None).expect("Blocking call to Worker_Alpha_PlayerIdentityTokenResponseFuture_Get did not trigger callback")
    }
}

impl Drop for PlayerIdentityTokenFuture {
    fn drop(&mut self) {
        if !self.internal.is_null() {
            unsafe { Worker_Alpha_PlayerIdentityTokenResponseFuture_Destroy(self.internal) }
        }
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
            player_identity_token: CString::new(player_identity_token.as_ref()).expect("`player_identity_token` contained a null byte"),
            worker_type: CString::new(worker_type.as_ref()).expect("`worker_type` contained a null byte"),
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
    internal: *mut Worker_Alpha_LoginTokensResponseFuture,
    consumed: bool,
}

impl LoginTokensFuture {
    fn new(ptr: *mut Worker_Alpha_LoginTokensResponseFuture) -> Self {
        LoginTokensFuture {
            internal: ptr,
            consumed: false,
        }
    }

    extern "C" fn callback_handler(
        user_data: *mut ::std::os::raw::c_void,
        response: *const Worker_Alpha_LoginTokensResponse,
    ) {
        assert!(!response.is_null());
        unsafe {
            let response = *response;
            let data = &mut *(user_data as *mut Option<Result<LoginTokensResponse, String>>);
            if !response.error.is_null() {
                let err = cstr_to_string(response.error);
                *data = Some(Err(err));
                return;
            }

            let response = LoginTokensResponse::from_worker_sdk(&response);
            *data = Some(Ok(response));
        }
    }

    fn get_future(&self, timeout: Option<u32>) -> Option<Result<LoginTokensResponse, String>> {
        assert!(!self.internal.is_null());
        let mut data: Option<Result<LoginTokensResponse, String>> = None;
        unsafe {
            Worker_Alpha_LoginTokensResponseFuture_Get(
                self.internal,
                timeout.map_or(::std::ptr::null(), |value| &value),
                &mut data as *mut _ as *mut ::std::os::raw::c_void,
                Some(LoginTokensFuture::callback_handler),
            );

            data
        }
    }
}

impl Future for LoginTokensFuture {
    type Item = LoginTokensResponse;
    type Error = String;

    fn poll(&mut self) -> Result<Async<<Self as Future>::Item>, <Self as Future>::Error> {
        if self.consumed {
            return Err("LoginTokensFuture has already been consumed".to_owned());
        }

        self.get_future(Some(0))
            .map_or(Ok(Async::NotReady), |result| {
                self.consumed = true;
                result.map(Async::Ready)
            })
    }

    fn wait(self) -> Result<<Self as Future>::Item, <Self as Future>::Error>
        where
            Self: Sized,
    {
        if self.consumed {
            return Err("LoginTokensFuture has already been consumed".to_owned());
        }

        self.get_future(None)
            .expect("Blocking call to Worker_Alpha_LoginTokensFuture_Get did not trigger callback")
    }
}

impl Drop for LoginTokensFuture {
    fn drop(&mut self) {
        if !self.internal.is_null() {
            unsafe { Worker_Alpha_LoginTokensResponseFuture_Destroy(self.internal) }
        }
    }
}
