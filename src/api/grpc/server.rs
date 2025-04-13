use super::config::Config;
use super::kms::kms_service_server::KmsServiceServer;
use crate::api::grpc::service::KMSServiceImpl;
use log::{error, info};
use tonic::transport::Server;

pub async fn start_server(config: &Config) -> anyhow::Result<(), Box<dyn std::error::Error>> {
    // defining address for our service
    let kms_service_impl = KMSServiceImpl::new();
    let address = config.address.parse().unwrap();
    info!("kms Server listening on {}", address);

    if let Err(e) = Server::builder()
        .add_service(KmsServiceServer::new(kms_service_impl))
        .serve(address)
        .await
    {
        error!("failed to read from socket; err = {:?}", e);
    };

    Ok(())
}
