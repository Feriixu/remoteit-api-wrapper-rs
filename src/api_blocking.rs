use crate::auth::{build_auth_header, get_date};
use crate::operations::{
    cancel_job, delete_file, delete_file_version, get_application_types, get_devices, get_files,
    get_jobs, start_job, CancelJob, DeleteFile, DeleteFileVersion, GetApplicationTypes, GetDevices,
    GetFiles, GetJobs, StartJob,
};
use crate::{R3Client, BASE_URL, GRAPHQL_PATH};
use base64::prelude::BASE64_STANDARD;
use base64::Engine;
use bon::bon;
use graphql_client::{GraphQLQuery, QueryBody, Response};
use reqwest::blocking::Client;
use reqwest::Method;
use serde::{Deserialize, Serialize};
use std::error::Error;

#[bon]
impl R3Client {
    pub fn send_remoteit_graphql_request<V: Serialize, R: for<'a> Deserialize<'a>>(
        self,
        query_body: &QueryBody<V>,
    ) -> Result<Response<R>, Box<dyn Error>> {
        let date = get_date();
        let key = BASE64_STANDARD
            .decode(&self.credentials.r3_secret_access_key)
            .unwrap();
        let auth_header = build_auth_header(
            &self.credentials.r3_access_key_id,
            &key,
            "application/json",
            &Method::POST,
            GRAPHQL_PATH,
            &date,
        );
        let client = Client::new();
        let response = client
            .post(format!("{BASE_URL}{GRAPHQL_PATH}"))
            .header("Date", date)
            .header("Content-Type", "application/json")
            .header("Authorization", auth_header)
            .json(&query_body)
            .send()
            .unwrap();
        let response: Response<R> = response.json()?;
        Ok(response)
    }

    // region Scripting
    #[builder]
    pub fn get_files(self) -> Result<Response<get_files::ResponseData>, Box<dyn Error>> {
        let request_body = GetFiles::build_query(get_files::Variables {});
        self.send_remoteit_graphql_request(&request_body)
    }

    #[builder]
    pub fn delete_file(
        self,
        file_id: String,
    ) -> Result<Response<delete_file::ResponseData>, Box<dyn Error>> {
        let request_body = DeleteFile::build_query(delete_file::Variables { file_id });
        self.send_remoteit_graphql_request(&request_body)
    }

    #[builder]
    pub fn delete_file_version(
        self,
        file_version_id: String,
    ) -> Result<Response<delete_file_version::ResponseData>, Box<dyn Error>> {
        let request_body =
            DeleteFileVersion::build_query(delete_file_version::Variables { file_version_id });
        self.send_remoteit_graphql_request(&request_body)
    }

    #[builder]
    pub fn start_job(
        self,
        file_id: String,
        device_ids: Vec<String>,
        arguments: Option<Vec<start_job::ArgumentInput>>,
    ) -> Result<Response<start_job::ResponseData>, Box<dyn Error>> {
        let request_body = StartJob::build_query(start_job::Variables {
            file_id,
            device_ids,
            arguments,
        });
        self.send_remoteit_graphql_request(&request_body)
    }

    #[builder]
    pub fn cancel_job(
        self,
        job_id: String,
    ) -> Result<Response<cancel_job::ResponseData>, Box<dyn Error>> {
        let request_body = CancelJob::build_query(cancel_job::Variables { job_id });
        self.send_remoteit_graphql_request(&request_body)
    }

    #[builder]
    pub fn get_jobs(
        self,
        org_id: Option<String>,
        job_id_filter: Option<Vec<String>>,
        status_filter: Option<Vec<get_jobs::JobStatusEnum>>,
    ) -> Result<Response<get_jobs::ResponseData>, Box<dyn Error>> {
        let request_body = GetJobs::build_query(get_jobs::Variables {
            org_id,
            job_ids: job_id_filter,
            statuses: status_filter,
        });
        self.send_remoteit_graphql_request(&request_body)
    }

    // endregion

    // region Devices and Services

    #[builder]
    pub fn get_application_types(
        self,
    ) -> Result<Response<get_application_types::ResponseData>, Box<dyn Error>> {
        let request_body = GetApplicationTypes::build_query(get_application_types::Variables {});
        self.send_remoteit_graphql_request(&request_body)
    }

    #[builder]
    pub fn get_devices(
        self,
        org_id: Option<String>,
        limit: Option<i64>,
        offset: Option<i64>,
    ) -> Result<Response<get_devices::ResponseData>, Box<dyn Error>> {
        let request_body = GetDevices::build_query(get_devices::Variables {
            org_id,
            limit,
            offset,
        });
        self.send_remoteit_graphql_request(&request_body)
    }

    // endregion
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::credentials::Credentials;
    use std::path::PathBuf;

    fn get_credentials() -> Credentials {
        Credentials::load_from_disk()
            .custom_credentials_path(PathBuf::from(".env.remoteit"))
            .call()
            .unwrap()
            .take_profile("default")
            .unwrap()
            .unwrap()
    }

    fn get_client() -> R3Client {
        R3Client::builder().credentials(get_credentials()).build()
    }

    #[test]
    fn test_get_files() {
        let response = get_client().get_files().call().unwrap();
        assert!(response.data.is_some());
        assert!(response.errors.is_none());
    }

    #[test]
    fn test_get_jobs() {
        let response = get_client().get_jobs().call().unwrap();
        assert!(response.data.is_some());
        assert!(response.errors.is_none());
    }

    #[test]
    fn test_get_jobs_with_filters() {
        let response = get_client()
            .get_jobs()
            .job_id_filter(vec!["foobar".to_string()])
            .status_filter(vec![get_jobs::JobStatusEnum::SUCCESS])
            .call()
            .unwrap();
        assert!(response.data.is_some());
        assert!(response.errors.is_none());
    }

    #[test]
    fn test_get_application_types() {
        let response = get_client().get_application_types().call().unwrap();
        dbg!(&response);
        assert!(response.data.is_some());
        assert!(response.errors.is_none());
    }

    #[test]
    fn test_get_devices() {
        let response = get_client().get_devices().call().unwrap();
        assert!(response.data.is_some());
        assert!(response.errors.is_none());
    }
}
