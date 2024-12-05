use alloy::{primitives::U256, providers::Provider};
use anyhow::Result;
use async_trait::async_trait;
use std::sync::Arc;
use tokio_stream::StreamExt;

use crate::types::{Collector, CollectorStream};

/// A collector that listens for new blocks, and generates a stream of
/// [events](NewBlock) which contain the block number and hash.
pub struct BlockCollector<M> {
    provider: Arc<M>,
}

/// A new block event, containing the block number and hash.
#[derive(Debug, Clone)]
pub struct NewBlock {
    pub number: u64,
    pub gas_used: U256,
    pub gas_limit: U256,
    pub base_fee_per_gas: U256,
    pub timestamp: U256,
}

impl<M> BlockCollector<M> {
    pub fn new(provider: Arc<M>) -> Self {
        Self { provider }
    }
}

/// Implementation of the [Collector](Collector) trait for the [BlockCollector](BlockCollector).
/// This implementation uses the [PubsubClient](PubsubClient) to subscribe to new blocks.
#[async_trait]
impl<M> Collector<NewBlock> for BlockCollector<M>
where
    M: Provider,
    // M::Provider: PubsubClient,
    // M::Error: 'static,
{
    async fn get_event_stream(&self) -> Result<CollectorStream<'_, NewBlock>> {
        let stream = self.provider.subscribe_blocks().await?;
        let stream = stream.into_stream();

        let stream = stream.filter_map(|block| match block.header.number {
            Some(number) => Some(NewBlock {
                number: number,
                gas_limit: U256::from(block.header.gas_limit),
                gas_used: U256::from(block.header.gas_used),
                base_fee_per_gas: U256::from(block.header.base_fee_per_gas.unwrap_or_default()), // TODO check this default
                timestamp: U256::from(block.header.timestamp),
            }),
            None => None,
        });
        Ok(Box::pin(stream))
    }
}
