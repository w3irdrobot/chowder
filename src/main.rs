mod bisq {
    tonic::include_proto!("io.bisq.protobuffer");
}

use bisq::get_version_client::GetVersionClient;
use bisq::GetVersionRequest;
use std::time::Duration;
use tonic::Request;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let password = std::env::var("BISQ_PASSWORD")?;

    let mut client = GetVersionClient::connect("http://localhost:9998").await?;

    let mut req = Request::new(GetVersionRequest {});
    req.set_timeout(Duration::from_secs(5));

    let metadata = req.metadata_mut();
    metadata.insert("password", password.parse()?);

    let response = client.get_version(req).await?.into_inner();
    println!("bisq version: {}", response.version);

    Ok(())
}
