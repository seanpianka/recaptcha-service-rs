use async_trait::async_trait;

use crate::{Error, Service};

struct ServiceImpl {}

pub fn new_service() -> impl Service {
    ServiceImpl {}
}

#[async_trait]
impl Service for ServiceImpl {
    async fn verify(&self, token: String, agent_ip_address: Option<String>) -> Result<(), Error> {
        println!("dropped token=`{}`, agent_ip_address=`{:?}`", token, agent_ip_address);
        Ok(())
    }
}
