use anyhow::Result;
use async_trait::async_trait;
use one_engine::types::{Actions, Events, Strategy};

pub struct StrategyTest;

impl StrategyTest {
    pub fn new() -> Self {
        Self {}
    }
}

#[async_trait]
impl Strategy<Events, Actions> for StrategyTest {
    async fn sync_state(&mut self) -> Result<(), anyhow::Error> {
        println!("sync_state");
        Ok(())
    }

    async fn process_event(&mut self, event: Events) -> Option<Actions> {
        println!("on_event: {:?}", event);

        None
    }
}
