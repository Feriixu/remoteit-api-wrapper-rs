//! Contains items related to loading credentials from disk.
//!
//! Please see [`Credentials`] for more.

use crate::credentials::Credentials;
use base64::engine::general_purpose::STANDARD as BASE64_STANDARD;
use base64::Engine;
use bon::bon;
use std::collections::HashMap;
use std::path::PathBuf;

/// Errors that can occur during the loading of credentials from disk.
#[allow(missing_docs)]
#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("The user's home directory could not be found. Please refer to the `dirs` crate for more information.")]
    HomeDirNotFound,
    #[error("The credentials file could not be loaded: {0}")]
    CouldNotReadCredentials(#[from] std::io::Error),
    #[error("The credentials file could not be parsed: {0}")]
    CredentialsParse(#[from] config::ConfigError),
}

/// A struct representing the remote.it credentials file.
///
/// The credentials file can have multiple profiles, each with its own access key ID and secret access key.
///
/// The secret access keys of the profiles within this struct are base64 encoded.
/// At this point they are unverified, which is why the inner [`HashMap`] is private.
/// The secret key of the profile you want will be verified, when the profile is retrieved using one of:
/// - [`CredentialProfiles::take_profile`]
/// - [`CredentialProfiles::profile`]
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct CredentialProfiles {
    #[serde(flatten)]
    pub(crate) profiles: HashMap<String, Credentials>,
}

impl CredentialProfiles {
    /// See [`CredentialProfiles::take_profile`] if you need an owned value.
    ///
    /// # Returns
    /// - [`None`] if the profile with the given name does not exist.
    /// - [`Some`] containing a reference to the [`Credentials`] with the given name, if the profile exists.
    ///
    /// # Errors
    /// - [`base64::DecodeError`] if the secret access key of the profile with the given name is not base64 encoded.
    pub fn profile(&self, profile_name: &str) -> Result<Option<&Credentials>, base64::DecodeError> {
        let profile = self.profiles.get(profile_name);
        if profile.is_none() {
            return Ok(None);
        }

        let _decode_result = BASE64_STANDARD.decode(&profile.unwrap().r3_secret_access_key)?;

        Ok(profile)
    }

    /// Takes the profile with the given name out of the inner [`HashMap`], validated the secret access key and returns it.
    /// You can only take a profile once, after that it is removed from the inner [`HashMap`].
    ///
    /// # Returns
    /// - [`None`] if the profile with the given name does not exist.
    /// - [`Some`] containing the [`Credentials`] with the given name, if the profile exists.
    ///
    /// # Errors
    /// - [`base64::DecodeError`] if the secret access key of the profile with the given name is not base64 encoded.
    pub fn take_profile(
        &mut self,
        profile_name: &str,
    ) -> Result<Option<Credentials>, base64::DecodeError> {
        let profile = self.profiles.remove(profile_name);

        let Some(profile) = profile else {
            return Ok(None);
        };

        let _decode_result = BASE64_STANDARD.decode(&profile.r3_secret_access_key)?;

        Ok(Some(profile))
    }

    /// # Returns
    /// The number of profiles in the inner [`HashMap`].
    #[must_use]
    pub fn len(&self) -> usize {
        self.profiles.len()
    }

    /// # Returns
    /// - [`true`] if there are no profiles in the inner [`HashMap`].
    /// - [`false`] if there is at least one profile in the inner [`HashMap`].
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.profiles.is_empty()
    }
}

/// Impl block for credentials_loader related functions.
#[bon]
impl Credentials {
    /// Attempts to load the remote.it credentials from the user's home directory.
    /// The default location is `~/.remoteit/credentials`.
    ///
    /// # Errors
    /// * [`Error::HomeDirNotFound`], when the [`dirs`] create cannot find the user's home directory.
    /// * [`Error::CouldNotReadCredentials`], when the credentials file could not be parsed by the [`config`] crate.
    ///
    /// # Example
    /// You can load credentials from the default path (`~/.remoteit/credentials` on Unix-like), or provide a custom path.
    /// ```
    /// # use remoteit_api::credentials::Credentials;
    /// let credentials_file = Credentials::load_from_disk()
    ///     .custom_credentials_path("path/to/file") // Optional
    ///     .call();
    /// ```
    /// You can also pass a PathBuf, or anything that implements [`Into<PathBuf>`]
    /// ```
    /// # use std::path::PathBuf;
    /// # use remoteit_api::credentials::Credentials;
    /// let credentials_file = Credentials::load_from_disk()
    ///     .custom_credentials_path(PathBuf::from("path/to/file")) // Optional
    ///     .call();
    /// ```
    #[builder]
    pub fn load_from_disk(
        custom_credentials_path: Option<PathBuf>,
    ) -> Result<CredentialProfiles, Error> {
        let credentials_path = custom_credentials_path.unwrap_or(
            dirs::home_dir()
                .ok_or(Error::HomeDirNotFound)?
                .join(".remoteit")
                .join("credentials"),
        );

        let profiles: CredentialProfiles = config::Config::builder()
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
    use crate::credentials::Credentials;
    use std::io::Write;

    #[test]
    fn test_load_from_disk_empty() {
        let file = tempfile::NamedTempFile::new().unwrap();

        let credentials = Credentials::load_from_disk()
            .custom_credentials_path(file.path().to_path_buf())
            .call()
            .unwrap();

        assert!(credentials.is_empty());
    }

    #[test]
    fn test_load_from_disk_one() {
        let credentials = r"
            [default]
            R3_ACCESS_KEY_ID=foo
            R3_SECRET_ACCESS_KEY=YmFy
        ";

        let mut file = tempfile::NamedTempFile::new().unwrap();
        file.write_all(credentials.as_bytes()).unwrap();

        let credentials = Credentials::load_from_disk()
            .custom_credentials_path(file.path().to_path_buf())
            .call()
            .unwrap();

        assert_eq!(credentials.len(), 1);
        let credentials = credentials.profile("default").unwrap().unwrap();
        assert_eq!(credentials.r3_access_key_id, "foo");
        assert_eq!(credentials.r3_secret_access_key, "YmFy");
    }

    #[test]
    fn test_load_from_disk_two() {
        let credentials = r"
            [default]
            R3_ACCESS_KEY_ID=foo
            R3_SECRET_ACCESS_KEY=YmFy

            [other]
            R3_ACCESS_KEY_ID=baz
            R3_SECRET_ACCESS_KEY=YmFy
        ";

        let mut file = tempfile::NamedTempFile::new().unwrap();
        file.write_all(credentials.as_bytes()).unwrap();

        let credentials = Credentials::load_from_disk()
            .custom_credentials_path(file.path().to_path_buf())
            .call()
            .unwrap();

        assert_eq!(credentials.len(), 2);
        let profile = credentials.profile("default").unwrap().unwrap();
        assert_eq!(profile.r3_access_key_id, "foo");
        assert_eq!(profile.r3_secret_access_key, "YmFy");
        let profile = credentials.profile("other").unwrap().unwrap();
        assert_eq!(profile.r3_access_key_id, "baz");
        assert_eq!(profile.r3_secret_access_key, "YmFy");
    }
}
