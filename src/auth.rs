use base64::engine::general_purpose::STANDARD as BASE64_STANDARD;
use base64::Engine;
use chrono::Utc;
use reqwest::Method;
use ring::hmac;

pub(crate) fn create_signature(key: &[u8], message: &str) -> String {
    let signing_key = hmac::Key::new(hmac::HMAC_SHA256, key);
    let signature = hmac::sign(&signing_key, message.as_bytes());
    BASE64_STANDARD.encode(signature.as_ref())
}

pub(crate) fn build_auth_header(
    key_id: &str,
    key: &[u8],
    content_type: &str,
    method: &Method,
    url: &str,
    date: &str,
) -> String {
    let signature_params =
        format!(
            "(request-target): {} {url}\nhost: api.remote.it\ndate: {date}\ncontent-type: {content_type}",
            method.to_string().to_lowercase()
        );
    #[cfg(debug_assertions)]
    dbg!(&signature_params);
    let signature = create_signature(key, &signature_params);
    format!(
        "Signature keyId=\"{key_id}\",algorithm=\"hmac-sha256\",headers=\"(request-target) host date content-type\",signature=\"{signature}\"")
}

pub(crate) fn get_date() -> String {
    Utc::now().format("%a, %d %b %Y %H:%M:%S GMT").to_string()
}
