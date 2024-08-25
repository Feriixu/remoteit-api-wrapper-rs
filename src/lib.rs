//! # Remote.it API Wrapper
//!
//! The most important structs are [`R3Client`] and [`Credentials`].
//!
//! To use this crate obtain your [`Credentials`] by loading them from disk using the provided methods,
//! or, if you want to store the credentials in another way, instantiate a [`Credentials`] struct directly.
//! See [`Credentials::load_from_disk`] and [`Credentials::builder`] for details and examples.
//!
//! Then instantiate an [`R3Client`] using [`R3Client::builder`] and start calling the API functions on it.
//!
//! # Features
//!
//! - Enable `blocking` to use the blocking versions of the API functions from the [`api_blocking`] module.
//! - Enable `async` to use the asynchronous versions of the API funcitons from the [`api_async`] module.
//! - Enable `credentials_loader` to use the [`Credentials::load_from_disk`] function.
//!     This is gated behind a feature, because it introduces additional dependencies.
//!

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
pub mod credentials_loader;
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
/// You can create a new [`R3Client`] using the builder pattern:
/// ```
/// # use remoteit_api::R3Client;
/// # use remoteit_api::credentials::Credentials;
/// let credentials = Credentials::load_from_disk()
///     .call()
///     .unwrap()
///     .take_profile("default")
///     .expect("Couldn't parse secret access key!")
///     .expect("Profile with given name does not exist!");
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
