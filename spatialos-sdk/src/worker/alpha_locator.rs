use std::ffi::CString;

use futures::{Async, Future};

use spatialos_sdk_sys::worker::*;

use crate::worker:: {
    ConnectionStatusCode,
    parameters::ProtocolLoggingParameters ,
    internal::utils::cstr_to_string
};

pub struct AlphaLocator {}

impl AlphaLocator {
    pub fn create_development_player_identity_token(
        hostname: &str,
        port: u16,
        params: &mut PlayerIdentityTokenRequest,
    ) -> PlayerIdentityTokenFuture {
        unsafe {
            let cstr = CString::new(hostname).unwrap();
            let (mut params, data) = params.to_worker_sdk();
            PlayerIdentityTokenFuture::new(Worker_Alpha_CreateDevelopmentPlayerIdentityTokenAsync(cstr.as_ptr(), port, &mut params as *mut _))
        }
    }
}

pub struct AlphaLocatorParameters {
    pub player_identity: PlayerIdentityCredentials,
    pub use_insecure_connection: bool,
    pub logging: Option<ProtocolLoggingParameters>,
}

pub struct PlayerIdentityCredentials {
    pub player_identity_token: String,
    pub login_token: String,
}

pub struct PlayerIdentityTokenRequest {
    pub dev_auth_token: String,
    pub player_id: String,
    pub duration_seconds: Option<u32>,
    pub display_name: Option<String>,
    pub metadata: Option<String>,
    pub use_insecure_connection: bool,
}

impl PlayerIdentityTokenRequest {
    pub fn new<S: Into<String>, T: Into<String>>(dev_auth_token: S, player_id: T) -> Self {
        PlayerIdentityTokenRequest {
            dev_auth_token: dev_auth_token.into(),
            player_id: player_id.into(),
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

    pub fn with_display_name<S: Into<String>>(mut self, display_name: S) -> Self {
        self.display_name = Some(display_name.into());
        self
    }

    pub fn with_metadata<S: Into<String>>(mut self, metadata: S) -> Self {
        self.metadata = Some(metadata.into());
        self
    }

    pub fn with_insecure_connection(mut self) -> Self {
        self.use_insecure_connection = true;
        self
    }

    // TODO: Only reason this is `mut` is because that the duration_seconds is a non-const reference.
    fn to_worker_sdk(&mut self) -> (Worker_Alpha_PlayerIdentityTokenRequest, Vec<CString>) {
        unsafe {
            let mut underlying_data = vec![
                CString::new(self.dev_auth_token.as_str()).unwrap(),
                CString::new(self.player_id.as_str()).unwrap(),
            ];
            let mut metadata_index: usize = 2;

            if let Some(ref display_name) = self.display_name {
                underlying_data.push(CString::new(display_name.as_str()).unwrap());
                metadata_index += 1;
            };

            if let Some(ref metadata) = self.metadata {
                underlying_data.push(CString::new(metadata.as_str()).unwrap());
            }

            let request = Worker_Alpha_PlayerIdentityTokenRequest {
                development_authentication_token_id: underlying_data[0].as_ptr(),
                player_id: underlying_data[1].as_ptr(),
                duration_seconds: match self.duration_seconds {
                    Some(ref mut value) => value as *mut u32,
                    None => ::std::ptr::null_mut()
                },
                display_name: match self.display_name {
                    Some(_) => underlying_data[2].as_ptr(),
                    None => ::std::ptr::null(),
                },
                metadata: match self.metadata {
                    Some(_) => underlying_data[metadata_index].as_ptr(),
                    None => ::std::ptr::null(),
                },
                use_insecure_connection: self.use_insecure_connection as u8,
            };

            (request, underlying_data)
        }
    }
}

pub struct PlayerIdentityTokenResponse {
    pub player_identity_token: String,
}

impl PlayerIdentityTokenResponse {
    fn from_worker_sdk(response: &Worker_Alpha_PlayerIdentityTokenResponse) -> Self {
        PlayerIdentityTokenResponse {
            player_identity_token: cstr_to_string(response.player_identity_token)
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
            let data = &mut *(user_data as *mut Option<Result<PlayerIdentityTokenResponse, String>>);
            if !response.error.is_null() {
                let err = cstr_to_string(response.error);
                *data = Some(Err(err));
                return;
            }

            let response = PlayerIdentityTokenResponse::from_worker_sdk(&response);
            *data = Some(Ok(response));
        }
    }
}

impl Future for PlayerIdentityTokenFuture {
    type Item = PlayerIdentityTokenResponse;
    type Error = String;

    fn poll(&mut self) -> Result<Async<<Self as Future>::Item>, <Self as Future>::Error> {
        if self.consumed {
            return Err("PlayerIdentityTokenFuture has already been consumed".to_owned());
        }

        assert!(!self.internal.is_null());
        let mut data: Option<Result<PlayerIdentityTokenResponse, String>> = None;
        unsafe {
            Worker_Alpha_PlayerIdentityTokenResponseFuture_Get(
                self.internal,
                &0,
                &mut data as *mut _ as *mut ::std::os::raw::c_void,
                Some(PlayerIdentityTokenFuture::callback_handler),
            );
        }

        data.map_or(Ok(Async::NotReady), |result| {
            self.consumed = true;
            result.map(|resp| Async::Ready(resp))
        })
    }

    fn wait(self) -> Result<<Self as Future>::Item, <Self as Future>::Error>
    where
        Self: Sized,
    {
        if self.consumed {
            return Err("PlayerIdentityTokenFuture has already been consumed".to_owned());
        }

        assert!(!self.internal.is_null());
        let mut data: Option<Result<PlayerIdentityTokenResponse, String>> = None;
        unsafe {
            Worker_Alpha_PlayerIdentityTokenResponseFuture_Get(
                self.internal,
                ::std::ptr::null(),
                &mut data as *mut _ as *mut ::std::os::raw::c_void,
                Some(PlayerIdentityTokenFuture::callback_handler),
            );
        }

        data.expect("Blocking call to Worker_Alpha_PlayerIdentityTokenResponseFuture_Get did not trigger callback")
    }
}

impl Drop for PlayerIdentityTokenFuture {
    fn drop(&mut self) {
        if !self.internal.is_null() {
            unsafe { Worker_Alpha_PlayerIdentityTokenResponseFuture_Destroy(self.internal) }
        }
    }
}
