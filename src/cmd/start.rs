use dotenvy::dotenv;
use ezex_kms::config;
use std::sync::Arc;

use ezex_kms::api::grpc::server;

use std::{
    sync::atomic::{AtomicBool, Ordering},
    thread,
};
use tokio::task;

pub async fn start() {
    dotenv().ok();

    let config = config::Config::new();
    // TODO: Logger
    // logger::init_logger(&self.logger);

    let running = Arc::new(AtomicBool::new(true));
    let r = running.clone();
    ctrlc::set_handler(move || {
        r.store(false, Ordering::SeqCst);
    })
    .expect("Error setting Ctrl-C handler");

    let grpc_handle = task::spawn(async move {
        server::start_server(&config.grpc).await.unwrap();
    });

    log::info!("KMS started...");
    while running.load(Ordering::SeqCst) {
        thread::sleep(std::time::Duration::from_secs(1));
    }

    grpc_handle.abort();
}
