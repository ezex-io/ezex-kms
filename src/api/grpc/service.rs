use tonic::{Request, Response, Status};

use super::kms::{
    GetAddressRequest, GetAddressResponse, MakeWalletRequest, MakeWalletResponse, VersionRequest,
    VersionResponse, kms_service_server::KmsService,
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
    async fn make_wallet(
        &self,
        _request: Request<MakeWalletRequest>,
    ) -> anyhow::Result<Response<MakeWalletResponse>, Status> {
        let response = MakeWalletResponse {
            wallet_id: "<<TODO>>".to_string(),
        };

        Ok(Response::new(response))
    }

    async fn get_address(
        &self,
        _request: Request<GetAddressRequest>,
    ) -> anyhow::Result<Response<GetAddressResponse>, Status> {
        let response = GetAddressResponse {
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
