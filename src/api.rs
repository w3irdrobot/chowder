mod api {
    tonic::include_proto!("io.bisq.protobuffer");
}

use api::get_version_client::GetVersionClient;
use api::GetVersionRequest;
use std::convert::From;
use std::time::Duration;
use thiserror::Error;
use tonic::metadata::AsciiMetadataValue;
use tonic::transport::{self, Channel};
use tonic::{Request, Status};

const DEFAULT_TIMEOUT: u64 = 30;

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

impl From<transport::Error> for ApiError {
    fn from(error: transport::Error) -> Self {
        ApiError::Connect(error.to_string())
    }
}

impl From<Status> for ApiError {
    fn from(status: Status) -> Self {
        ApiError::GrpcResponse(status.code().into(), status.message().to_string())
    }
}

pub struct Bisq {
    api_password: AsciiMetadataValue,
    conn: Channel,
}

impl Bisq {
    pub async fn new(api_endpoint: String, api_password: String) -> ApiResult<Self> {
        let conn = tonic::transport::Endpoint::new(api_endpoint)?.connect_lazy();
        let password = api_password
            .parse()
            .map_err(|_| ApiError::InvalidMetadataValue(api_password.clone()))?;

        Ok(Self {
            api_password: password,
            conn,
        })
    }

    pub async fn version(&self) -> ApiResult<String> {
        let mut client = GetVersionClient::new(self.conn.clone());
        let mut req = Request::new(GetVersionRequest {});
        req.set_timeout(Duration::from_secs(DEFAULT_TIMEOUT));

        let metadata = req.metadata_mut();
        metadata.insert("password", self.api_password.clone());

        let resp = client.get_version(req).await?.into_inner();

        Ok(resp.version)
    }
}
