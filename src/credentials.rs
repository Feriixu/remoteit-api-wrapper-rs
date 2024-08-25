//! This module is related to loading remote.it credentials from the user's home directory.
//! This is of course not the most secure way to store credentials, but it is the most convenient and recommended by remote.it.
//! If you store your credentials in a different way, you can pass them to the functions in this module directly instead of using this module to load them.

use base64::prelude::BASE64_STANDARD;
use base64::Engine;
use bon::bon;

#[derive(
    Debug, Clone, PartialOrd, PartialEq, Eq, Ord, Hash, serde::Deserialize, serde::Serialize,
)]
pub struct Credentials {
    pub(crate) r3_access_key_id: String,
    pub(crate) r3_secret_access_key: String,
}

#[bon]
impl Credentials {
    /// Validated the given secret access key and creates a new [`Credentials`] struct.
    ///
    /// # Errors
    /// - [`base64::DecodeError`] if the secret access key is not base64 encoded.
    ///
    /// # Example
    /// ```
    /// # use remoteit_api::credentials::Credentials;
    /// let credentials = Credentials::builder()
    ///     .r3_access_key_id("foo")
    ///     .r3_secret_access_key("YmFy")
    ///     .build();
    /// ```
    #[builder]
    pub fn new(
        r3_access_key_id: String,
        r3_secret_access_key: String,
    ) -> Result<Self, base64::DecodeError> {
        let _decode_result = BASE64_STANDARD.decode(&r3_secret_access_key)?;
        Ok(Self {
            r3_access_key_id,
            r3_secret_access_key,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_credentials_builder() {
        let credentials = Credentials::builder()
            .r3_access_key_id("foo")
            .r3_secret_access_key("YmFy")
            .build()
            .unwrap();

        assert_eq!(credentials.r3_access_key_id, "foo");
        assert_eq!(credentials.r3_secret_access_key, "YmFy");
    }
}
