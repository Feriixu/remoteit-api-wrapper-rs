use remoteit_api::{Credentials, R3Client};

fn main() {
    // See the `load_credentials` example for alternative ways to load credentials.
    let mut credentials = Credentials::load_from_disk().call().unwrap();
    let profile = credentials.take_profile("default").unwrap().unwrap();

    // Create a new client with the loaded credentials.
    let client = R3Client::builder()
        .credentials(profile)
        .build();

    // Make a request to the remote.it API.
    // This call lists all files uploaded to the remote.it API.
    let response = client.get_files().call().unwrap();
    dbg!(&response);
}
