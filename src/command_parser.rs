use crate::browser::BrowserCommand;
use thirtyfour::common::keys;
use tokio::sync::mpsc;

pub struct ParsedCommandResult {
    pub command: Option<BrowserCommand>,
    pub response: Option<String>,
}

pub enum CommandAction {
    SendToBrowser(BrowserCommand),
    WithResponse(BrowserCommand, String),
    ResponseOnly(String),
    Noop,
}


pub async fn parse_command(
    input: &str,
    symbol: &str,
    tx: &mpsc::Sender<BrowserCommand>, // used only where needed
) -> CommandAction {
    if !input.starts_with(symbol) {
        return CommandAction::Noop;
    }

    let command = input[symbol.len()..].trim().to_string();

    match command.as_str() {
        "up" => CommandAction::SendToBrowser(BrowserCommand::PredefinedKey(keys::Key::Up)),
        "down" => CommandAction::SendToBrowser(BrowserCommand::PredefinedKey(keys::Key::Down)),
        "space" => CommandAction::SendToBrowser(BrowserCommand::PredefinedKey(keys::Key::Space)),

        "get_url" => {
            let (url_sender, mut url_receiver) = mpsc::channel(1);
            if tx.send(BrowserCommand::FetchUrl(url_sender)).await.is_ok() {
                if let Some(url) = url_receiver.recv().await {
                    return CommandAction::ResponseOnly(url);
                }
            }
            CommandAction::ResponseOnly("Failed to fetch URL".into())
        }

        _ if command.starts_with("toggle_") => {
            let key = command.trim_start_matches("toggle_").to_string();
            CommandAction::SendToBrowser(BrowserCommand::ClickElement(key))
        }

        _ => {
            let (sender, mut receiver) = mpsc::channel(1);
            if tx.send(BrowserCommand::GetElementValue(command.clone(), sender)).await.is_ok() {
                if let Some(value) = receiver.recv().await {
                    return CommandAction::WithResponse(
                        BrowserCommand::Goto(command.clone()),
                        format!("{}: {}", command, value),
                    );
                }
            }

            if command.len() == 1 {
                CommandAction::SendToBrowser(BrowserCommand::RawCharacter(command))
            } else {
                CommandAction::SendToBrowser(BrowserCommand::Goto(command))
            }
        }
    }
}
