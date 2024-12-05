mod strategy;

use alloy::transports::ws::WsConnect;
use alloy::{
    providers::{ProviderBuilder, RootProvider},
    pubsub::PubSubFrontend,
};
use anyhow::Result;
use std::sync::Arc;

use one_engine::{
    engine::Engine,
    collectors::block_collector::BlockCollector,
    types::{Actions, CollectorMap, Events},
};

use strategy::StrategyTest;

pub async fn create_default_wss_provider() -> Result<RootProvider<PubSubFrontend>, anyhow::Error> {
    let wss_rpc = std::env::var("WSS_RPC")?;
    let url: &str = wss_rpc.as_str();
    let client = ProviderBuilder::new().on_ws(WsConnect::new(url)).await?;
    Ok(client)
}

#[tokio::main(flavor = "multi_thread")]
async fn main() -> Result<()> {
    dotenv::from_filename(".env").ok();
    println!("{}", std::env::var("WSS_RPC").unwrap());

    // Setup provider
    let provider = create_default_wss_provider().await?;
    let provider = provider.boxed();
    let provider = Arc::new(provider);

    // Create engine
    let mut engine: Engine<Events, Actions> = Engine::default();

    // Setup block collector
    let block_collector = BlockCollector::new(provider.clone());
    let block_collector = CollectorMap::new(Box::new(block_collector), Events::NewBlock);
    engine.add_collector(Box::new(block_collector));

    // Setup mempool collector
    // TODO

    // Setup strategy
    let strategy = StrategyTest::new();
    engine.add_strategy(Box::new(strategy));

    // Setup executor
    // TODO

    // Start engine
    if let Ok(mut set) = engine.run().await {
        while let Some(res) = set.join_next().await {
            println!("res: {:?}", res);
        }
    }

    Ok(())
}
