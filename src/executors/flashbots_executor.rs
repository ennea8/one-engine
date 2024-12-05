use anyhow::Result;
use async_trait::async_trait;

use alloy::rpc::types::mev::EthSendBundle;

use crate::types::Executor;

/// A Flashbots executor that sends transactions to the Flashbots relay.
pub struct FlashbotsExecutor;

impl FlashbotsExecutor {
    pub fn new() -> Self {
        Self {}
    }
}

pub type FlashbotsBundle = Vec<EthSendBundle>;

#[async_trait]
impl Executor<FlashbotsBundle> for FlashbotsExecutor {
    async fn execute(&self, action: FlashbotsBundle) -> Result<()> {
        info!("Executing Flashbots bundle, {:?}", action);

        Ok(())
    }
}
