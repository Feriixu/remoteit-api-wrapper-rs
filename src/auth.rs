//! Contains functions related to request signing for the remote.it API.
//! They are used by this lib, but you can also use them to implement your own abstraction.


use base64::engine::general_purpose::STANDARD as BASE64_STANDARD;
use base64::Engine;
use bon::builder;
use chrono::Utc;
use reqwest::Method;
use ring::hmac;

/// You probably don't want to use this function directly, unless you are implementing your own abstraction over the remote.it API.
///
/// Signs the given `message` with the given `key` with the HMAC algorithm and base64-encodes the result.
///
/// # Returns
/// Base64 encoded HMAC signature.
pub fn create_signature(key: &[u8], message: &str) -> String {
    let signing_key = hmac::Key::new(hmac::HMAC_SHA256, key);
    let signature = hmac::sign(&signing_key, message.as_bytes());
    BASE64_STANDARD.encode(signature.as_ref())
}

/// You probably don't want to use this function directly, unless you are implementing your own abstraction over the remote.it API.
///
/// Create the value to use in the `Authorization` header for requests to the remote.it API.
///
/// This function is used by [`R3Client::send_remoteit_graphql_request`] and [`R3Client::send_remoteit_graphql_request_async`] to authorize requests to the remote.it API.
///
/// # Returns
/// A [`String`] which should be set as the value for the `Authorization` header for sending requests to the remote.it API.
///
/// # Example
/// ```
/// use reqwest::Method;
/// use remoteit_api::Credentials;
/// use remoteit_api::GRAPHQL_PATH;
/// let credentials = Credentials::load_from_disk().call().unwrap().take_profile("default").unwrap().unwrap();
/// let date = remoteit_api::auth::get_date();
/// let auth_header = remoteit_api::auth::build_auth_header()
///     .key_id(credentials.access_key_id())
///     .key(credentials.key())
///     .content_type("application/json")
///     .method(&Method::POST)
///     .path(GRAPHQL_PATH)
///     .date(&date)
///     .call();
/// ```
///
#[builder]
pub fn build_auth_header(
    key_id: &str,
    key: &[u8],
    content_type: &str,
    method: &Method,
    path: &str,
    date: &str,
) -> String {
    let signature_params =
        format!(
            "(request-target): {} {path}\nhost: api.remote.it\ndate: {date}\ncontent-type: {content_type}",
            method.to_string().to_lowercase()
        );
    #[cfg(debug_assertions)]
    dbg!(&signature_params);
    let signature = create_signature(key, &signature_params);
    format!(
        "Signature keyId=\"{key_id}\",algorithm=\"hmac-sha256\",headers=\"(request-target) host date content-type\",signature=\"{signature}\"")
}

/// You probably don't want to use this function directly, unless you are implementing your own abstraction for making requests to the remote.it API.
///
/// Creates a date string (now) to be used for signing requests to the remote.it API.
///
/// This function is used by [`R3Client::send_remoteit_graphql_request`] and [`R3Client::send_remoteit_graphql_request_async`].
///
/// # Returns
/// A date string (now) in the format required by the remote.it API.
pub fn get_date() -> String {
    Utc::now().format("%a, %d %b %Y %H:%M:%S GMT").to_string()
}
