//! ID Token (/token/auth_refresh) API definition.

use core::fmt;

use serde::Deserialize;

/// Response for get ID Token (/token/auth_refresh)
///
/// See: [JQuants API](https://jpx.gitbook.io/j-quants-en/api-reference/idtoken)
///
/// # Security
///
/// While the Debug trait is implemented, the id_token is masked due to security risks.
/// If you wish to display it, please do so at your own responsibility.
#[derive(Deserialize)]
pub struct IdTokenResponse {
    /// The ID token.
    #[serde(rename = "idToken")]
    pub id_token: String,
}

impl fmt::Debug for IdTokenResponse {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let masking = "********";

        f.debug_struct("IdTokenResponse")
            .field("id_token", &masking)
            .finish()
    }
}
