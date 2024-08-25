use crate::credentials::Credentials;
use bon::builder;

#[cfg(feature = "async")]
pub mod api_async;
#[cfg(feature = "blocking")]
pub mod api_blocking;

// If neither the `async` nor `blocking` features are enabled, then the `auth` module is not needed.
#[cfg(any(feature = "async", feature = "blocking"))]
mod auth;
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

#[builder]
pub struct R3Client {
    credentials: Credentials,
}
