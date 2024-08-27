use std::path::PathBuf;
use bon::{bon, builder};

use crate::auth::{build_auth_header, get_date};

#[derive(Debug, Clone)]
#[builder]
pub struct FileUpload {
    pub file_name: String,
    pub file_path: PathBuf,
    pub executable: bool,
    pub short_desc: Option<String>,
    pub long_desc: Option<String>,
}

#[derive(serde::Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct UploadFileResponse {
    pub file_id: String,
    pub file_version_id: String,
    pub version: u32,
    pub name: String,
    pub executable: bool,
    pub owner_id: String,
    pub file_arguments: Vec<serde_json::Value>,
}

#[derive(serde::Deserialize, Clone, Debug)]
pub struct ErrorResponse {
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
        use crate::FILE_UPLOAD_PATH;
        use crate::BASE_URL;

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
            let response: ErrorResponse = response.json().map_err(|e| UploadFileError::ParseJson(e))?;
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
    pub async fn upload_file_async(&self, file_upload: FileUpload)
     ->  Result<UploadFileResponse, UploadFileError> {
        use crate::FILE_UPLOAD_PATH;
        use crate::BASE_URL;

        let client = reqwest::Client::new();

        let file_name = file_upload.file_path
            .file_name()
            .map(|val| val.to_string_lossy().to_string())
            .unwrap_or_default();

        let file = tokio::fs::File::open(&file_upload.file_name)
            .await?;

        let reader = reqwest::Body::wrap_stream(tokio_util::codec::FramedRead::new(file, tokio_util::codec::BytesCodec::new()));
        let mut form = reqwest::multipart::Form::new()
            .part(file_upload.file_name, reqwest::multipart::Part::stream(reader).file_name(file_name))
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
            let response: ErrorResponse = response.json().await.map_err(|e| UploadFileError::ParseJson(e))?;
            Err(UploadFileError::ApiError(response))
        }
    }
}