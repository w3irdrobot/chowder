mod api {
    tonic::include_proto!("io.bisq.protobuffer");
}

use api::get_version_client::GetVersionClient;
use api::GetVersionRequest;
use std::convert::From;
use std::time::Duration;
use thiserror::Error;
use tonic::metadata::errors::InvalidMetadataValue;
use tonic::{Request, Status};

pub type ApiResult<T> = Result<T, ApiError>;

#[derive(Debug, Error)]
pub enum ApiError {
    #[error("can't connect to Bisq API: {0}")]
    Connect(String),
    #[error("error response {0} from the API: {1}")]
    GrpcResponse(i32, String),
    #[error("invalid metadata value {0}")]
    InvalidMetadataValue(String),
}

impl From<tonic::transport::Error> for ApiError {
    fn from(error: tonic::transport::Error) -> Self {
        ApiError::Connect(error.to_string())
    }
}

impl From<Status> for ApiError {
    fn from(status: Status) -> Self {
        ApiError::GrpcResponse(status.code().into(), status.message().to_string())
    }
}

pub struct Bisq {
    api_password: String,
    api_endpoint: String,
}

impl Bisq {
    pub async fn new(api_password: String, api_endpoint: String) -> ApiResult<Self> {
        let mut client = GetVersionClient::connect(api_endpoint.clone()).await?;

        let mut req = Request::new(GetVersionRequest {});
        req.set_timeout(Duration::from_secs(5));

        let metadata = req.metadata_mut();
        let password = api_password
            .parse()
            .map_err(|_| ApiError::InvalidMetadataValue(api_password.clone()))?;
        metadata.insert("password", password);

        client.get_version(req).await?;

        Ok(Self {
            api_password,
            api_endpoint,
        })
    }
}
