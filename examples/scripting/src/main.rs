use remoteit_api::api_blocking::R3Client;
use remoteit_api::credentials::Credentials;

fn main() {
    let credentials = Credentials::builder()
        .r3_access_key_id("access_key_id")
        .r3_secret_access_key("secret_acces_key")
        .build();
    let client = R3Client::builder()
        .credentials(credentials)
        .build();

    let response = client.get_files().call().unwrap();
    dbg!(&response);
}
