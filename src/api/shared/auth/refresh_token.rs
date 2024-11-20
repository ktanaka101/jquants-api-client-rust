//! Refresh Token (/token/auth_user) API definition.

use std::fmt;

use serde::{Deserialize, Serialize};

/// Request for get Refresh Token (/token/auth_user)
///
/// See: [JQuants API](https://jpx.gitbook.io/j-quants-ja/api-reference/refreshtoken)
///
/// # Security
///
/// While the Debug trait is implemented, the `mailaddress` and `password` is masked due to security risks.
/// If you wish to display it, please do so at your own responsibility.
#[derive(Clone, Serialize)]
pub struct RefreshTokenRequest {
    /// The mail address.
    #[serde(rename = "mailaddress")]
    pub mail_address: String,
    /// The password.
    pub password: String,
}

impl fmt::Debug for RefreshTokenRequest {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let masking = "********";

        f.debug_struct("RefreshTokenRequest")
            .field("mail_address", &masking)
            .field("password", &masking)
            .finish()
    }
}

/// Response for get Refresh Token (/token/auth_user)
///
/// See: [JQuants API](https://jpx.gitbook.io/j-quants-ja/api-reference/refreshtoken)
///
/// # Security
///
/// While the Debug trait is implemented, the `refresh_token` is masked due to security risks.
/// If you wish to display it, please do so at your own responsibility.
#[derive(Deserialize)]
pub struct RefreshTokenResponse {
    /// The refresh token.
    #[serde(rename = "refreshToken")]
    pub refresh_token: String,
}

impl fmt::Debug for RefreshTokenResponse {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let masking = "********";

        f.debug_struct("RefreshTokenResponse")
            .field("refresh_token", &masking)
            .finish()
    }
}
