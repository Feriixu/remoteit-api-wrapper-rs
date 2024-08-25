use remoteit_api::credentials::Credentials;
use remoteit_api::R3Client;

fn main() {
    // You can build the credentials manually like this.
    let _credentials = Credentials::builder()
        .r3_access_key_id("access_key_id")
        .r3_secret_access_key("secret_access_key")
        .build();

    // If the `credentials_loader` feature is enabled, the `load_from_disk` method is available.
    // By default (and as per remote.it spec), the credentials are loaded from `~/.remoteit/credentials`.
    let _credentials = Credentials::load_from_disk().call().unwrap();

    // Or you can load the credentials from a custom file path, if you prefer.
    let mut credentials = Credentials::load_from_disk()
        .custom_credentials_path("../../.env.remoteit") // This can be a &str, or a PathBuf, or anything that implements Into<PathBuf>.
        .call()
        .unwrap();

    // Choosing a profile from the loaded credentials file.
    // Pro Tip: Use `remove` to avoid having to clone.
    let default_profile = credentials
        .take_profile("default")
        .expect("Error while validating Credentials")
        .expect("No default profile found");

    // Once you have the credentials, you can build the client.
    let _client = R3Client::builder()
        .credentials(default_profile)
        .build();
}
