mod api;
mod application;

use api::Bisq;
use application::Application;
use clap::Parser;
use std::time::Duration;

#[derive(Parser, Debug)]
#[clap(version, about)]
struct Args {
    /// Password for accessing the Bisq API
    #[clap(short = 'p', long = "api-password")]
    api_password: String,

    /// Password for unlocking the wallet
    #[clap(short = 'P', long = "wallet-password")]
    wallet_password: String,

    /// Host URL to Bisq API
    #[clap(long, default_value_t = String::from("http://localhost:9998"))]
    host: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let Args {
        api_password,
        host,
        wallet_password,
    } = Args::parse();
    let bisq_api = Bisq::new(host, api_password, wallet_password).await?;
    let mut app = Application::new(bisq_api)?;

    match app.run().await {
        Err(e) => println!("error starting the application: {}", e),
        _ => tokio::time::sleep(Duration::from_secs(10)).await,
    };

    app.close()?;

    Ok(())
}
