use bon::builder;
use crate::credentials::Credentials;

#[builder]
struct R3Client {
    credentials: Credentials,
}

#[cfg(feature = "blocking")]
pub mod api_blocking;
#[cfg(feature = "async")]
pub mod api_async;
mod auth;
pub mod credentials;
pub mod operations;

/// Base path for the remote.it API.
pub const BASE_URL: &str = "https://api.remote.it";

/// Path for the GraphQL API. Append this to [`BASE_URL`] to get the full URL.
pub const GRAPHQL_PATH: &str = "/graphql/v1";

/// Path for file uploads. Append this to [`BASE_URL`] to get the full URL.
pub const FILE_UPLOAD_PATH: &str = "/graphql/v1/file/upload";
