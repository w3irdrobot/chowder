mod api;

use clap::Parser;

#[derive(Parser, Debug)]
#[clap(version, about)]
struct Args {
    /// Password for accessing the Bisq API
    #[clap(short = 'p', long = "api-password")]
    api_password: String,

    /// Host URL to Bisq API
    #[clap(long, default_value_t = String::from("http://localhost:9998"))]
    host: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let Args { api_password, host } = Args::parse();
    let bisq_api = api::Bisq::new(host, api_password).await?;

    println!("bisq version: {}", bisq_api.version().await?);

    Ok(())
}
