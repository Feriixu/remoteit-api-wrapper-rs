use crate::auth::{build_auth_header, get_date};
use crate::operations::{cancel_job, delete_file, delete_file_version, get_application_types, get_devices, get_files, get_jobs, org_get_jobs, start_job, CancelJob, DeleteFile, DeleteFileVersion, GetApplicationTypes, GetDevices, GetFiles, GetJobs, OrgGetJobs, StartJob};
use crate::{R3Client, BASE_URL, GRAPHQL_PATH};
use base64::prelude::BASE64_STANDARD;
use base64::Engine;
use bon::{bon};
use graphql_client::{GraphQLQuery, QueryBody, Response};
use reqwest::Client;
use reqwest::Method;
use serde::{Deserialize, Serialize};
use std::error::Error;


#[bon]
impl R3Client {
    pub async fn send_remoteit_graphql_request_async<V: Serialize, R: for<'a> Deserialize<'a>>(
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
            .await?;
        let response: Response<R> = response.json().await?;
        Ok(response)
    }

    // region Scripting
    #[builder]
    pub async fn get_files_async(self) -> Result<Response<get_files::ResponseData>, Box<dyn Error>> {
        let request_body = GetFiles::build_query(get_files::Variables {});
        self.send_remoteit_graphql_request_async(&request_body).await
    }

    #[builder]
    pub async fn delete_file_async(
        self,
        file_id: String) -> Result<Response<delete_file::ResponseData>, Box<dyn Error>> {
        let request_body = DeleteFile::build_query(delete_file::Variables {
            file_id,
        });
        self.send_remoteit_graphql_request_async(&request_body).await
    }

    #[builder]
    pub async fn delete_file_version_async(
        self,
        file_version_id: String
    ) -> Result<Response<delete_file_version::ResponseData>, Box<dyn Error>> {
        let request_body = DeleteFileVersion::build_query(delete_file_version::Variables {
            file_version_id,
        });
        self.send_remoteit_graphql_request_async(&request_body).await
    }

    #[builder]
    pub async fn start_job_async(
        self,
        file_id: String,
        device_ids: Vec<String>,
        arguments: Option<Vec<start_job::ArgumentInput>>
    ) -> Result<Response<start_job::ResponseData>, Box<dyn Error>> {
        let request_body = StartJob::build_query(start_job::Variables {
            file_id,
            device_ids,
            arguments,
        });
        self.send_remoteit_graphql_request_async(&request_body).await
    }

    #[builder]
    pub async fn cancel_job_async(
        self,
        job_id: String
    ) -> Result<Response<cancel_job::ResponseData>, Box<dyn Error>> {
        let request_body = CancelJob::build_query(cancel_job::Variables {
            job_id,
        });
        self.send_remoteit_graphql_request_async(&request_body).await
    }

    #[builder]
    pub async fn get_jobs_async(
        self,
        job_id_filter: Option<Vec<String>>,
        status_filter: Option<Vec<get_jobs::JobStatusEnum>>,
    ) -> Result<Response<get_jobs::ResponseData>, Box<dyn Error>> {
        let request_body = GetJobs::build_query(get_jobs::Variables {
            job_ids: job_id_filter,
            statuses: status_filter,
        });
        self.send_remoteit_graphql_request_async(&request_body).await
    }

    #[builder]
    pub async fn org_get_jobs_async(
        self,
        org_id: String,
        job_id_filter: Option<Vec<String>>,
        status_filter: Option<Vec<org_get_jobs::JobStatusEnum>>,
    ) -> Result<Response<org_get_jobs::ResponseData>, Box<dyn Error>> {
        let request_body = OrgGetJobs::build_query(org_get_jobs::Variables {
            org_id: Some(org_id),
            job_ids: job_id_filter,
            statuses: status_filter,
        });
        self.send_remoteit_graphql_request_async(&request_body).await
    }
    // endregion

    // region Devices and Services

    #[builder]
    pub async fn get_application_types_async(self) -> Result<Response<get_application_types::ResponseData>, Box<dyn Error>> {
        let request_body = GetApplicationTypes::build_query(get_application_types::Variables {});
        self.send_remoteit_graphql_request_async(&request_body).await
    }

    #[builder]
    pub async fn get_devices_async(self,
                       limit: Option<i64>,
                       offset: Option<i64>,) -> Result<Response<get_devices::ResponseData>, Box<dyn Error>> {
        let request_body = GetDevices::build_query(get_devices::Variables {
            limit,
            offset,
        });
        self.send_remoteit_graphql_request_async(&request_body).await
    }

    // endregion
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;
    use crate::credentials::Credentials;

    fn get_credentials() -> Credentials {
        Credentials::load_from_disk()
            .custom_credentials_path(PathBuf::from(".env.remoteit"))
            .call()
            .unwrap()
            .remove("default")
            .unwrap()
    }

    fn get_client() -> R3Client {
        R3Client::builder().credentials(get_credentials()).build()
    }

    #[tokio::test]
    async fn test_get_files_async() {
        let response = get_client().get_files_async().call().await.unwrap();
        assert!(response.data.is_some());
        assert!(response.errors.is_none());
    }

    #[tokio::test]
    async fn test_get_jobs_async() {
        let response = get_client().get_jobs_async().call().await.unwrap();
        assert!(response.data.is_some());
        assert!(response.errors.is_none());
    }

    #[tokio::test]
    async fn test_get_jobs_with_filters_async() {
        let response = get_client()
            .get_jobs_async()
            .job_id_filter(vec!["foobar".to_string()])
            .status_filter(vec![get_jobs::JobStatusEnum::SUCCESS])
            .call()
            .await
            .unwrap();
        assert!(response.data.is_some());
        assert!(response.errors.is_none());
    }

    #[tokio::test]
    async fn test_get_application_types_async() {
        let response = get_client().get_application_types_async().call().await.unwrap();
        assert!(response.data.is_some());
        assert!(response.errors.is_none());
    }

    #[tokio::test]
    async fn test_get_devices_async() {
        let response = get_client().get_devices_async().call().await.unwrap();
        assert!(response.data.is_some());
        assert!(response.errors.is_none());
    }
}
