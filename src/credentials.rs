//! This module is related to loading remote.it credentials from the user's home directory.
//! This is of course not the most secure way to store credentials, but it is the most convenient and recommended by remote.it.
//! If you store your credentials in a different way, you can pass them to the functions in this module directly instead of using this module to load them.

use bon::{bon, builder};
use std::collections::HashMap;
use std::path::PathBuf;

#[cfg(feature = "credentials_loader")]
#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("The user's home directory could not be found. Please refer to the `dirs` crate for more information.")]
    HomeDirNotFound,
    #[error("The credentials file could not be loaded: {0}")]
    CouldNotReadCredentials(#[from] std::io::Error),
    #[error("The credentials file could not be parsed: {0}")]
    CredentialsParse(#[from] config::ConfigError),
}

/// A remote.it credentials file contains zero or more profiles, each containing a set of credentials.
type CredentialsFile = HashMap<String, Credentials>;

#[builder]
#[derive(
    Debug, Clone, PartialOrd, PartialEq, Eq, Ord, Hash, serde::Deserialize, serde::Serialize,
)]
pub struct Credentials {
    pub(crate) r3_access_key_id: String,
    pub(crate) r3_secret_access_key: String,
}

#[cfg(feature = "credentials_loader")]
#[bon]
impl Credentials {
    /// Attempts to load the remote.it credentials from the user's home directory.
    /// The default location is `~/.remoteit/credentials`.
    ///
    /// # Errors
    /// * [`Error::HomeDirNotFound`], when the [`dirs`] create cannot find the user's home directory.
    /// * [`Error::CouldNotReadCredentials`], when the credentials file could not be parsed by the [`figment`] crate.
    #[cfg(feature = "credentials_loader")]
    #[builder]
    pub fn load_from_disk(
        custom_credentials_path: Option<PathBuf>,
    ) -> Result<CredentialsFile, Error> {
        let credentials_path = custom_credentials_path.unwrap_or(
            dirs::home_dir()
                .ok_or(Error::HomeDirNotFound)?
                .join(".remoteit")
                .join("credentials"),
        );

        let profiles: CredentialsFile = config::Config::builder()
            .add_source(config::File::new(
                credentials_path
                    .to_str()
                    .expect("It is highly unlikely, that there would be a "),
                config::FileFormat::Ini,
            ))
            .build()?
            .try_deserialize()?;

        Ok(profiles)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;

    #[test]
    fn test_credentials_builder() {
        let credentials = Credentials::builder()
            .r3_access_key_id("foo")
            .r3_secret_access_key("bar")
            .build();

        assert_eq!(credentials.r3_access_key_id, "foo");
        assert_eq!(credentials.r3_secret_access_key, "bar");
    }

    #[test]
    fn test_load_from_disk_empty() {
        let file = tempfile::NamedTempFile::new().unwrap();

        let credentials = Credentials::load_from_disk()
            .custom_credentials_path(file.path().to_path_buf())
            .call();

        assert!(credentials.is_ok());
        let credentials = credentials.unwrap();
        assert!(credentials.is_empty());
    }

    #[test]
    fn test_load_from_disk_one() {
        let credentials = r"
            [default]
            R3_ACCESS_KEY_ID=foo
            R3_SECRET_ACCESS_KEY=bar
        ";

        let mut file = tempfile::NamedTempFile::new().unwrap();
        file.write_all(credentials.as_bytes()).unwrap();

        let credentials = Credentials::load_from_disk()
            .custom_credentials_path(file.path().to_path_buf())
            .call();

        assert!(credentials.is_ok());
        let credentials = credentials.unwrap();
        assert_eq!(credentials.len(), 1);
        assert_eq!(credentials["default"].r3_access_key_id, "foo");
        assert_eq!(credentials["default"].r3_secret_access_key, "bar");
    }

    #[test]
    fn test_load_from_disk_two() {
        let credentials = r"
            [default]
            R3_ACCESS_KEY_ID=foo
            R3_SECRET_ACCESS_KEY=bar

            [other]
            R3_ACCESS_KEY_ID=baz
            R3_SECRET_ACCESS_KEY=qux
        ";

        let mut file = tempfile::NamedTempFile::new().unwrap();
        file.write_all(credentials.as_bytes()).unwrap();

        let credentials = Credentials::load_from_disk()
            .custom_credentials_path(file.path().to_path_buf())
            .call();

        assert!(credentials.is_ok());
        let credentials = credentials.unwrap();
        assert_eq!(credentials.len(), 2);
        assert_eq!(credentials["default"].r3_access_key_id, "foo");
        assert_eq!(credentials["default"].r3_secret_access_key, "bar");
        assert_eq!(credentials["other"].r3_access_key_id, "baz");
        assert_eq!(credentials["other"].r3_secret_access_key, "qux");
    }
}
