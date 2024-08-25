use chrono::Local;
use graphql_client::GraphQLQuery;

/// Define [`DateTime`] as a [`chrono::DateTime<Local>`], because it is not a built-in type in GraphQL.
type DateTime = chrono::DateTime<Local>;
type Any = serde_json::Value;
type Object = serde_json::Map<String, Any>;

// region Scripting
#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/graphql/schema.json",
    query_path = "src/graphql/GetFiles.graphql",
    response_derives = "Debug"
)]
pub struct GetFiles;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/graphql/schema.json",
    query_path = "src/graphql/DeleteFile.graphql",
    response_derives = "Debug"
)]
pub struct DeleteFile;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/graphql/schema.json",
    query_path = "src/graphql/DeleteFileVersion.graphql",
    response_derives = "Debug"
)]
pub struct DeleteFileVersion;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/graphql/schema.json",
    query_path = "src/graphql/StartJob.graphql",
    response_derives = "Debug"
)]
pub struct StartJob;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/graphql/schema.json",
    query_path = "src/graphql/CancelJob.graphql",
    response_derives = "Debug"
)]
pub struct CancelJob;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/graphql/schema.json",
    query_path = "src/graphql/GetJobs.graphql",
    response_derives = "Debug"
)]
pub struct GetJobs;
// endregion

// region Devices and Services

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/graphql/schema.json",
    query_path = "src/graphql/GetApplicationTypes.graphql",
    response_derives = "Debug"
)]
pub struct GetApplicationTypes;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/graphql/schema.json",
    query_path = "src/graphql/GetDevices.graphql",
    response_derives = "Debug"
)]
pub struct GetDevices;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/graphql/schema.json",
    query_path = "src/graphql/GetDevicesCSV.graphql",
    response_derives = "Debug"
)]
pub struct GetDevicesCSV;
// endregion
