#![allow(missing_docs)]

use chrono::Local;
use graphql_client::GraphQLQuery;

/// Define [`DateTime`] as a [`chrono::DateTime<Local>`], because it is not a built-in type in GraphQL.
type DateTime = chrono::DateTime<Local>;
/// Define [`Any`] as a [`serde_json::Value`], because it is not a built-in type in GraphQL.
type Any = serde_json::Value;
/// Define [`Object`] as a [`serde_json::Map<String, Any>`], because it is not a built-in type in GraphQL.
type Object = serde_json::Map<String, Any>;

// region Scripting

/// Query, which retrieves a list of files, that were uploaded to remote.it.
#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/graphql/schema.json",
    query_path = "src/graphql/GetFiles.graphql",
    response_derives = "Debug"
)]
pub struct GetFiles;

/// Mutation, which deletes a file from remote.it. Deletes all versions of the file.
#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/graphql/schema.json",
    query_path = "src/graphql/DeleteFile.graphql",
    response_derives = "Debug"
)]
pub struct DeleteFile;

/// Mutation, which deletes a version of a file from remote.it. (Not the whole file)
#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/graphql/schema.json",
    query_path = "src/graphql/DeleteFileVersion.graphql",
    response_derives = "Debug"
)]
pub struct DeleteFileVersion;

/// Execution, to start scripting jobs on one or more devices.
#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/graphql/schema.json",
    query_path = "src/graphql/StartJob.graphql",
    response_derives = "Debug"
)]
pub struct StartJob;

/// Execution, to cancel a job. See remote.it docs on more information on when jobs can be cancelled.
#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/graphql/schema.json",
    query_path = "src/graphql/CancelJob.graphql",
    response_derives = "Debug"
)]
pub struct CancelJob;

/// Query, which retrieves a list of jobs, that were started on remote.it.
/// You can filter the jobs.
#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/graphql/schema.json",
    query_path = "src/graphql/GetJobs.graphql",
    response_derives = "Debug"
)]
pub struct GetJobs;
// endregion
// region Organizations
#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/graphql/schema.json",
    query_path = "src/graphql/GetOwnedOrganization.graphql",
    response_derives = "Debug"
)]
pub struct GetOwnedOrganization;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/graphql/schema.json",
    query_path = "src/graphql/GetOrganizationSelfMembership.graphql",
    response_derives = "Debug"
)]
pub struct GetOrganizationSelfMembership;
// endregion
// region Devices and Services

/// Query, which retrieves a list of services, that are available on remote.it.
#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/graphql/schema.json",
    query_path = "src/graphql/GetApplicationTypes.graphql",
    response_derives = "Debug"
)]
pub struct GetApplicationTypes;

/// Query, which retrieves a list of devices.
/// You can use this to get the IDs of devices, for example for starting scripting jobs.
#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/graphql/schema.json",
    query_path = "src/graphql/GetDevices.graphql",
    response_derives = "Debug"
)]
pub struct GetDevices;

/// Query, which retrieves a download link for a CSV file, that contains information about devices.
#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/graphql/schema.json",
    query_path = "src/graphql/GetDevicesCSV.graphql",
    response_derives = "Debug"
)]
pub struct GetDevicesCSV;
// endregion
