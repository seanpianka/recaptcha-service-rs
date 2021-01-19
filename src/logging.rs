use async_trait::async_trait;
use slog::{info, Logger};

use crate::{Error, Service};

struct ServiceImpl {
    pub logger: Logger,
    pub next: Box<dyn Service + Send + Sync>,
}

pub fn new_service(logger: Logger, next: Box<dyn Service + Send + Sync>) -> impl Service {
    ServiceImpl { logger, next }
}

#[async_trait]
impl Service for ServiceImpl {
    async fn verify(&self, token: String, agent_ip_address: Option<String>) -> Result<(), Error> {
        let now = std::time::Instant::now();
        let result = self.next.verify(token.clone(), agent_ip_address.clone()).await;
        let err = match &result {
            Err(e) => Some(e.to_string()),
            _ => None,
        };
        info!(self.logger, "";
            "method" => "verify",
            "token" => token,
            "agent_ip_address" => agent_ip_address,
            "took" => now.elapsed().as_millis() as f64 / 1000.,
            "err" => err
        );
        result
    }
}

