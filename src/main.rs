use ipfs_api::TryFromUri;
use ipfs_api::{IpfsApi, IpfsClient};
use std::io::Cursor;
#[tokio::main]
async fn main() {
    let endpoint = "http://127.0.0.1:5001".to_string();

    let client = IpfsClient::from_str(&endpoint).unwrap();

    let data = Cursor::new("Hello World");
    client.files_mkdir("/state", false).await;

    client
        .files_write("/state/output.file", true, true, data)
        .await
        .unwrap();
}
