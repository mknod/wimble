use crate::config::StreambotConfig;
use tokio::sync::mpsc;
use twitch_irc::login::StaticLoginCredentials;
use twitch_irc::ClientConfig;
use twitch_irc::TwitchIRCClient;
use twitch_irc::SecureTCPTransport;
use twitch_irc::message::PrivmsgMessage;
use twitch_irc::message::ServerMessage;
use thirtyfour::common::keys;

pub struct Bot {
    incoming_messages: mpsc::UnboundedReceiver<ServerMessage>,
    client: TwitchIRCClient<SecureTCPTransport, StaticLoginCredentials>,
    command_symbol: String,
    browser_tx: mpsc::Sender<keys::Key>, // Channel to send key commands
}

impl Bot {
    pub fn new(config: &StreambotConfig, browser_tx: mpsc::Sender<keys::Key>) -> Self {
        let username = &config.username;
        let access_token = &config.access_token;
        let channel = &config.channel;
        let command_symbol = config.command_symbol.clone();

        let client_config = ClientConfig::new_simple(
            StaticLoginCredentials::new(username.to_string(), Some(access_token.to_string()))
        );
        let (incoming_messages, client) = TwitchIRCClient::new(client_config);
        
        client.join(channel.to_string()).expect("Failed to join channel");

        Self { incoming_messages, client, command_symbol, browser_tx }
    }

    pub async fn run(&mut self) {
        println!("Streambot is running...");

        while let Some(message) = self.incoming_messages.recv().await {
            if let ServerMessage::Privmsg(chat_message) = message {
                self.match_command(chat_message).await;
            }
        }
    }

    pub async fn match_command(&self, chat_message: PrivmsgMessage) {
        let content = chat_message.message_text.clone();

        if content.starts_with(&self.command_symbol) {
            let command = content[self.command_symbol.len()..].trim().to_string();
            
            let key = match command.as_str() {
                "up" => Some(keys::Key::Up),
                "down" => Some(keys::Key::Down),
                "left" => Some(keys::Key::Left),
                "right" => Some(keys::Key::Right),
                "space" => Some(keys::Key::Space),
                _ => None,
            };

            if let Some(key) = key {
                if let Err(e) = self.browser_tx.send(key).await {
                    eprintln!("Failed to send key command: {}", e);
                }
            }
        }
    }
}
