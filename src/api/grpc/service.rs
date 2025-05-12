use tonic::{Request, Response, Status};

use super::kms::{
    GenerateAddressRequest, GenerateAddressResponse, VersionRequest, VersionResponse,
    kms_service_server::KmsService,
};

pub struct KMSServiceImpl {}

impl Default for KMSServiceImpl {
    fn default() -> Self {
        Self::new()
    }
}

impl KMSServiceImpl {
    pub fn new() -> Self {
        Self {}
    }
}

#[tonic::async_trait]
impl KmsService for KMSServiceImpl {
    async fn generate_address(
        &self,
        _request: Request<GenerateAddressRequest>,
    ) -> anyhow::Result<Response<GenerateAddressResponse>, Status> {
        let response = GenerateAddressResponse {
            address: "<<TODO>>".to_string(),
        };

        Ok(Response::new(response))
    }

    async fn version(
        &self,
        _request: Request<VersionRequest>,
    ) -> anyhow::Result<Response<VersionResponse>, Status> {
        let response = VersionResponse {
            version: env!("CARGO_PKG_VERSION").to_string(),
        };

        Ok(Response::new(response))
    }
}
