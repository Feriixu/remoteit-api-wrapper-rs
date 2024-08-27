//! Enabled by the `file_upload` feature. Contains structs and impl blocks related to uploading files to remote.it.
//!
//! On the docs page of this module, you can only see the builder structs for the functions.
//!
//! Please see [`R3Client`] for the actual functions you can call.

use bon::{bon, builder};
use std::path::PathBuf;

use crate::auth::{build_auth_header, get_date};

/// Struct to hold the details of a file to be uploaded to remote.it.
#[derive(Debug, Clone)]
#[builder]
pub struct FileUpload {
    /// The name of the file. This is what the file will be called in the remote.it system.
    pub file_name: String,
    /// The path to the file on the local filesystem.
    pub file_path: PathBuf,
    /// Whether the file is an executable script or an asset.
    pub executable: bool,
    /// A short description of the file.
    pub short_desc: Option<String>,
    /// A long description of the file.
    pub long_desc: Option<String>,
}

/// The positive response from the remote.it API when uploading a file.
#[derive(serde::Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct UploadFileResponse {
    /// The ID of the file. You can use this to reference the file in other API calls.
    pub file_id: String,
    /// The ID of the version of the file. You can use this to reference the version in other API calls.
    pub file_version_id: String,
    /// The version of the file. When you upload a file with the same name as one that already exists, the version will be incremented.
    pub version: u32,
    /// The name of the file.
    pub name: String,
    /// Whether the file is an executable script or an asset.
    pub executable: bool,
    /// The User ID of the owner of the file.
    pub owner_id: String,
    /// The available arguments for this file, if it is an executable script.
    /// See https://docs.remote.it/developer-tools/device-scripting#creating-scripts for more information.
    pub file_arguments: Vec<serde_json::Value>,
}

/// The negative response from the remote.it API when uploading a file.
#[derive(serde::Deserialize, Clone, Debug)]
pub struct ErrorResponse {
    /// The error message returned by the API.
    pub message: String,
}

#[derive(thiserror::Error, Debug)]
#[allow(missing_docs)]
pub enum UploadFileError {
    #[error("IO error while uploading file: {0}")]
    IO(#[from] std::io::Error),
    #[error("Failed to send upload file request: {0}")]
    Reqwest(#[from] reqwest::Error),
    #[error("Failed to parse response JSON: {0}")]
    ParseJson(reqwest::Error),
    #[error("The API returned an error: {0:?}")]
    ApiError(ErrorResponse),
}

#[cfg(feature = "blocking")]
#[bon]
impl crate::R3Client {
    /// Upload a file to remote.it.
    /// The file could be an executable script, or any other file to be used as a resource in scripts.
    ///
    /// Note: This is not GraphQL, it is a multipart form upload
    ///
    /// # Returns
    /// The response from the remote.it API. Contains the ID of the file and the version among other things. See [`UploadFileResponse`] for more details.
    ///
    /// # Errors
    /// - [`UploadFileError::IO`] if there is an error reading the file.
    /// - [`UploadFileError::Reqwest`] if there is an error sending the request.
    /// - [`UploadFileError::ApiError`] if the remote.it API returns an error response.
    /// - [`UploadFileError::ParseJson`] if there is an error parsing the response.
    #[builder]
    pub fn upload_file(
        &self,
        file_upload: FileUpload,
    ) -> Result<UploadFileResponse, UploadFileError> {
        use crate::BASE_URL;
        use crate::FILE_UPLOAD_PATH;

        let client = reqwest::blocking::Client::new();
        let mut form = reqwest::blocking::multipart::Form::new()
            .file(file_upload.file_name, file_upload.file_path)?
            .text("executable", file_upload.executable.to_string());

        if let Some(short_descr) = file_upload.short_desc {
            form = form.text("shortDesc", short_descr);
        }
        if let Some(long_descr) = file_upload.long_desc {
            form = form.text("longDesc", long_descr);
        }

        #[cfg(debug_assertions)]
        dbg!(&form);

        let content_type = format!("multipart/form-data; boundary={}", form.boundary());
        let date = get_date();
        let auth_header = build_auth_header()
            .key_id(&self.credentials.r3_access_key_id)
            .key(&self.credentials.key)
            .content_type(&content_type)
            .method(&reqwest::Method::POST)
            .path(FILE_UPLOAD_PATH)
            .date(&date)
            .call();

        let response = client
            .post(format!("{BASE_URL}{FILE_UPLOAD_PATH}"))
            .header("Date", date)
            .header("Authorization", auth_header)
            .header("Content-Type", content_type)
            .multipart(form)
            .send()?;

        if response.status().is_success() {
            let file_upload_response = response
                .json::<UploadFileResponse>()
                .map_err(|e| UploadFileError::ParseJson(e))?;
            Ok(file_upload_response)
        } else {
            let response: ErrorResponse =
                response.json().map_err(|e| UploadFileError::ParseJson(e))?;
            Err(UploadFileError::ApiError(response))
        }
    }
}

#[cfg(feature = "async")]
#[bon]
impl crate::R3Client {
    /// Upload a file to remote.it.
    /// The file could be an executable script, or any other file to be used as a resource in scripts.
    ///
    /// Note: This is not GraphQL, it is a multipart form upload
    ///
    /// # Returns
    /// The response from the remote.it API. Contains the ID of the file and the version among other things. See [`UploadFileResponse`] for more details.
    ///
    /// # Errors
    /// - [`UploadFileError::IO`] if there is an error reading the file.
    /// - [`UploadFileError::Reqwest`] if there is an error sending the request.
    /// - [`UploadFileError::ApiError`] if the remote.it API returns an error response.
    /// - [`UploadFileError::ParseJson`] if there is an error parsing the response.
    #[builder]
    pub async fn upload_file_async(
        &self,
        file_upload: FileUpload,
    ) -> Result<UploadFileResponse, UploadFileError> {
        use crate::BASE_URL;
        use crate::FILE_UPLOAD_PATH;

        let client = reqwest::Client::new();

        let file_name = file_upload
            .file_path
            .file_name()
            .map(|val| val.to_string_lossy().to_string())
            .unwrap_or_default();

        let file = tokio::fs::File::open(&file_upload.file_name).await?;

        let reader = reqwest::Body::wrap_stream(tokio_util::codec::FramedRead::new(
            file,
            tokio_util::codec::BytesCodec::new(),
        ));
        let mut form = reqwest::multipart::Form::new()
            .part(
                file_upload.file_name,
                reqwest::multipart::Part::stream(reader).file_name(file_name),
            )
            .text("executable", file_upload.executable.to_string());

        if let Some(short_descr) = file_upload.short_desc {
            form = form.text("shortDesc", short_descr);
        }
        if let Some(long_descr) = file_upload.long_desc {
            form = form.text("longDesc", long_descr);
        }

        #[cfg(debug_assertions)]
        dbg!(&form);

        let content_type = format!("multipart/form-data; boundary={}", form.boundary());
        let date = get_date();
        let auth_header = build_auth_header()
            .key_id(&self.credentials.r3_access_key_id)
            .key(&self.credentials.key)
            .content_type(&content_type)
            .method(&reqwest::Method::POST)
            .path(FILE_UPLOAD_PATH)
            .date(&date)
            .call();

        let response = client
            .post(format!("{BASE_URL}{FILE_UPLOAD_PATH}"))
            .header("Date", date)
            .header("Authorization", auth_header)
            .header("Content-Type", content_type)
            .multipart(form)
            .send()
            .await?;

        if response.status().is_success() {
            let file_upload_response = response
                .json::<UploadFileResponse>()
                .await
                .map_err(|e| UploadFileError::ParseJson(e))?;
            Ok(file_upload_response)
        } else {
            let response: ErrorResponse = response
                .json()
                .await
                .map_err(|e| UploadFileError::ParseJson(e))?;
            Err(UploadFileError::ApiError(response))
        }
    }
}
