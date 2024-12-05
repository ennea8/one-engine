use alloy::{providers::Provider, rpc::types::eth::Transaction};
use anyhow::Result;
use async_trait::async_trait;
use std::sync::Arc;

use crate::types::{Collector, CollectorStream};

/// A collector that listens for new transactions in the mempool, and generates a stream of
/// [events](Transaction) which contain the transaction.
pub struct MempoolCollector<M> {
    provider: Arc<M>,
}

impl<M> MempoolCollector<M> {
    pub fn new(provider: Arc<M>) -> Self {
        Self { provider }
    }
}

/// Implementation of the [Collector](Collector) trait for the [MempoolCollector](MempoolCollector).
/// This implementation uses the [PubsubClient](PubsubClient) to subscribe to new transactions.
#[async_trait]
impl<M> Collector<Transaction> for MempoolCollector<M>
where
    M: Provider,
    // M::Provider: PubsubClient,
    // M::Error: 'static,
{
    async fn get_event_stream(&self) -> Result<CollectorStream<'_, Transaction>> {
        let stream = self
            .provider
            .subscribe_full_pending_transactions()
            .await
            .map_err(|_| anyhow::anyhow!("Failed to create mempool stream"))?;

        let stream = stream.into_stream();

        Ok(Box::pin(stream))
    }
}
