//! # Remote.it API Wrapper

// Enable all features for the documentation tests
#![cfg_attr(docsrs, feature(doc_cfg))]
#![cfg_attr(docsrs, doc(cfg(all())))]

use crate::credentials::Credentials;
use bon::builder;

#[cfg(feature = "async")]
pub mod api_async;
#[cfg(feature = "blocking")]
pub mod api_blocking;

// If neither the `async` nor `blocking` features are enabled, then the `auth` module is not needed.
#[cfg(any(feature = "async", feature = "blocking"))]
pub mod auth;
pub mod credentials;
#[cfg(feature = "credentials_loader")]
mod credentials_loader;
pub mod operations;

/// Base path for the remote.it API.
pub const BASE_URL: &str = "https://api.remote.it";

/// Path for the GraphQL API. Append this to [`BASE_URL`] to get the full URL.
pub const GRAPHQL_PATH: &str = "/graphql/v1";

/// Path for file uploads. Append this to [`BASE_URL`] to get the full URL.
pub const FILE_UPLOAD_PATH: &str = "/graphql/v1/file/upload";

/// A client for the remote.it API.
///
/// # Example
/// You can instantiate a new [`R3Client`] using the builder pattern:
/// ```
/// # use remoteit_api::R3Client;
/// # use remoteit_api::credentials::Credentials;
///
/// let credentials = Credentials::load_from_disk().call().unwrap().take_profile("default").unwrap().unwrap();
/// let client = R3Client::builder().credentials(credentials).build();
/// // Start making API calls
/// let devices = client.get_devices().call().unwrap();
/// ```
#[builder]
pub struct R3Client {
    credentials: Credentials,
}

impl R3Client {
    /// # Returns
    /// A reference to the credentials used by the client.
    #[must_use]
    pub fn credentials(&self) -> &Credentials {
        &self.credentials
    }
}
