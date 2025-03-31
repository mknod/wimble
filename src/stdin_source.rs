use async_trait::async_trait;
use tokio::io::{self, AsyncBufReadExt, BufReader};
use tokio::sync::mpsc;

use crate::browser::BrowserCommand;
use crate::command_parser::parse_command;
use crate::command_source::CommandSource;
use crate::command_parser::handle_parsed_command;

/// Allows command input via stdin (e.g. terminal).
pub struct StdinSource {
    pub command_symbol: String,
}

#[async_trait]
impl CommandSource for StdinSource {
    async fn run(&mut self, browser_tx: mpsc::Sender<BrowserCommand>) {
        let stdin = io::stdin();
        let reader = BufReader::new(stdin);
        let mut lines = reader.lines();

        while let Ok(Some(line)) = lines.next_line().await {
            let symbol = self.command_symbol.clone();
            let tx = browser_tx.clone();
        
            let result = parse_command(&line, &symbol, &tx).await;
        
            handle_parsed_command(result, &tx, |msg| {
                println!("[STDIN RESPONSE] {}", msg);
            }).await;
        }
    }
}
