use async_trait::async_trait;
use tokio::sync::mpsc;
use crate::browser::BrowserCommand;

#[async_trait]
pub trait CommandSource: Send + Sync {
    async fn run(&mut self, browser_tx: mpsc::Sender<BrowserCommand>);
}