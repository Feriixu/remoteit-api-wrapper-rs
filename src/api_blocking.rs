//! Enabled by the `blocking` feature. Contains blocking implementations of the pre-written queries.
//!
//! On the docs page of this module, you can only see the builder structs for the functions.
//!
//! Please see [`R3Client`] for the actual functions you can call.

use crate::auth::{build_auth_header, get_date};
use crate::operations::{cancel_job, delete_file, delete_file_version, get_application_types, get_devices, get_files, get_jobs, get_organization_self_membership, get_owned_organization, start_job, CancelJob, DeleteFile, DeleteFileVersion, GetApplicationTypes, GetDevices, GetFiles, GetJobs, GetOrganizationSelfMembership, GetOwnedOrganization, StartJob};
use crate::{R3Client, BASE_URL, GRAPHQL_PATH};
use bon::bon;
use graphql_client::{GraphQLQuery, QueryBody, Response};
use reqwest::blocking::Client;
use reqwest::Method;
use serde::{Deserialize, Serialize};
use std::error::Error;

/// Impl block for blocking API calls.
#[bon]
impl R3Client {
    /// Sends a signed GraphQL request to the remote.it API in a blocking way.
    ///
    /// You probably don't want to use this function directly, but rather use the other functions in this module like [`R3Client::get_files()`].
    ///
    /// # Errors
    /// - Any error that occurs during the request.
    /// - Any error that occurs during deserialization of the response.
    pub fn send_remoteit_graphql_request<V: Serialize, R: for<'a> Deserialize<'a>>(
        &self,
        query_body: &QueryBody<V>,
    ) -> Result<Response<R>, Box<dyn Error>> {
        let date = get_date();
        let auth_header = build_auth_header()
            .key_id(&self.credentials.r3_access_key_id)
            .key(&self.credentials.key)
            .content_type("application/json")
            .method(&Method::POST)
            .path(GRAPHQL_PATH)
            .date(&date)
            .call();
        let client = Client::new();
        let response = client
            .post(format!("{BASE_URL}{GRAPHQL_PATH}"))
            .header("Date", date)
            .header("Content-Type", "application/json")
            .header("Authorization", auth_header)
            .json(&query_body)
            .send()?;
        let response: Response<R> = response.json()?;
        Ok(response)
    }

    // region Scripting

    /// Get a list of files that were uploaded to remote.it.
    #[builder]
    pub fn get_files(&self) -> Result<Response<get_files::ResponseData>, Box<dyn Error>> {
        let request_body = GetFiles::build_query(get_files::Variables {});
        self.send_remoteit_graphql_request(&request_body)
    }

    /// Delete a file from remote.it. Deletes all versions of the file.
    #[builder]
    pub fn delete_file(
        &self,
        /// The ID of the file to delete.
        /// You can get this from the response of [`R3Client::get_files()`].
        file_id: String,
    ) -> Result<Response<delete_file::ResponseData>, Box<dyn Error>> {
        let request_body = DeleteFile::build_query(delete_file::Variables { file_id });
        self.send_remoteit_graphql_request(&request_body)
    }

    /// Delete a version of a file from remote.it. (Not the whole file)
    #[builder]
    pub fn delete_file_version(
        &self,
        /// The ID of the file version to delete.
        /// You can get this from the response of [`R3Client::get_files()`].
        file_version_id: String,
    ) -> Result<Response<delete_file_version::ResponseData>, Box<dyn Error>> {
        let request_body =
            DeleteFileVersion::build_query(delete_file_version::Variables { file_version_id });
        self.send_remoteit_graphql_request(&request_body)
    }

    /// Start scripting jobs on one or more devices.
    #[builder]
    pub fn start_job(
        &self,
        /// The ID of the script file to run.
        /// Note that this needs to be an executable file.
        /// Get a list of files using [`R3Client::get_files()`].
        file_id: String,
        /// The IDs of the devices to run the script on.
        /// Get a list of devices using [`R3Client::get_devices()`].
        device_ids: Vec<String>,
        /// Arguments to pass to the script.
        /// These are optional.
        /// For more information on script arguments please consult the remote.it API documentation.
        arguments: Option<Vec<start_job::ArgumentInput>>,
    ) -> Result<Response<start_job::ResponseData>, Box<dyn Error>> {
        let request_body = StartJob::build_query(start_job::Variables {
            file_id,
            device_ids,
            arguments,
        });
        self.send_remoteit_graphql_request(&request_body)
    }

    /// Cancel a job. See remote.it docs on more information on when jobs can be cancelled.
    #[builder]
    pub fn cancel_job(
        &self,
        /// The ID of the job to cancel.
        /// You get this after starting a job using [`R3Client::start_job()`].
        job_id: String,
    ) -> Result<Response<cancel_job::ResponseData>, Box<dyn Error>> {
        let request_body = CancelJob::build_query(cancel_job::Variables { job_id });
        self.send_remoteit_graphql_request(&request_body)
    }

    /// Get a list of jobs that were started on remote.it.
    #[builder]
    pub fn get_jobs(
        &self,
        /// Optional organization ID for org context.
        org_id: Option<String>,
        /// Optional limit how many results are returned. It is highly recommended to set a limit, because this query can take quite a while otherwise.
        limit: Option<i64>,
        /// Optional list of job IDs to filter by.
        job_id_filter: Option<Vec<String>>,
        /// Optional list of job statuses to filter by.
        status_filter: Option<Vec<get_jobs::JobStatusEnum>>,
    ) -> Result<Response<get_jobs::ResponseData>, Box<dyn Error>> {
        let request_body = GetJobs::build_query(get_jobs::Variables {
            org_id,
            limit,
            job_ids: job_id_filter,
            statuses: status_filter,
        });
        self.send_remoteit_graphql_request(&request_body)
    }

    // endregion
    // region Organizations
    /// Get data on your own organization, which belongs to the current user.
    /// This Organization may or may not exist. You can create and configure your organization through the remote.it Web UI.
    ///
    /// # Returns
    /// Data on your organization, if you have one.
    #[builder]
    pub fn get_owned_organization(
        &self,
    ) -> Result<Response<get_owned_organization::ResponseData>, Box<dyn Error>> {
        let request_body = GetOwnedOrganization::build_query(get_owned_organization::Variables {});
        self.send_remoteit_graphql_request(&request_body)
    }

    /// Get a list of organization memberships for the current user.
    ///
    /// # Returns
    /// A list of organizations that you are a member of.
    #[builder]
    pub fn get_organization_self_membership(&self) -> Result<Response<get_organization_self_membership::ResponseData>, Box<dyn Error>> {
        let request_body = GetOrganizationSelfMembership::build_query(get_organization_self_membership::Variables {});
        self.send_remoteit_graphql_request(&request_body)
    }
    // endregion
    // region Devices and Services

    /// Get a list of application types that are available on remote.it.
    #[builder]
    pub fn get_application_types(
        &self,
    ) -> Result<Response<get_application_types::ResponseData>, Box<dyn Error>> {
        let request_body = GetApplicationTypes::build_query(get_application_types::Variables {});
        self.send_remoteit_graphql_request(&request_body)
    }

    /// Get a list of devices.
    #[builder]
    pub fn get_devices(
        &self,
        /// Optional organization ID for org context.
        org_id: Option<String>,
        /// Optional limit for the number of devices to return.
        limit: Option<i64>,
        /// Optional offset for the devices. Useful for pagination.
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
        let response = get_client().get_jobs().limit(1).call().unwrap();
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
