//! This module is related to loading remote.it credentials from the user's home directory.
//! This is of course not the most secure way to store credentials, but it is the most convenient and recommended by remote.it.
//! If you store your credentials in a different way, you can pass them to the functions in this module directly instead of using this module to load them.

use base64::prelude::BASE64_STANDARD;
use base64::Engine;
use bon::bon;

/// Credentials for the remote.it API.
/// Remote.it credentials consist of an access key ID and a base64 encoded secret access key.
///
/// # Example
/// You can directly create a new [`Credentials`] struct using the builder pattern:
/// ```
/// # use remoteit_api::credentials::Credentials;
/// let credentials = Credentials::builder()
///     .r3_access_key_id("foo")
///     .r3_secret_access_key("YmFy")
///     .build();
/// ```
/// If you enable the `credentials_loader` feature, you can also load the credentials from the default, or a custom file:
/// ```
/// # use remoteit_api::credentials::Credentials;
/// let creds_from_default_loc = Credentials::load_from_disk().call().unwrap();
/// let creds_from_custom_loc = Credentials::load_from_disk().custom_credentials_path("/path/to/credentials").call().unwrap();
/// ```
#[derive(
    Debug, Clone, PartialOrd, PartialEq, Eq, Ord, Hash, serde::Deserialize, serde::Serialize,
)]
pub struct Credentials {
    pub(crate) r3_access_key_id: String,
    pub(crate) r3_secret_access_key: String,
    pub(crate) key: Vec<u8>,
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
        let key = BASE64_STANDARD.decode(&r3_secret_access_key)?;
        Ok(Self {
            r3_access_key_id,
            r3_secret_access_key,
            key,
        })
    }

    /// # Returns
    /// The base64 decoded secret access key.
    #[must_use]
    pub fn key(&self) -> &[u8] {
        &self.key
    }

    /// # Returns
    /// A reference to the r3_access_key_id
    pub fn access_key_id(&self) -> &str {
        &self.r3_access_key_id
    }

    /// # Returns
    /// The base64 encoded r3_secret_access_key
    pub fn secret_access_key(&self) -> &str {
        &self.r3_secret_access_key
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
