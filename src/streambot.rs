use crate::config::StreambotConfig;
use crate::browser::BrowserCommand; 
use tokio::sync::mpsc;
use twitch_irc::login::StaticLoginCredentials;
use twitch_irc::ClientConfig;
use twitch_irc::TwitchIRCClient;
use twitch_irc::SecureTCPTransport;
use twitch_irc::message::PrivmsgMessage;
use twitch_irc::message::ServerMessage;
use thirtyfour::common::keys;

pub struct Bot {
    incoming_messages: mpsc::UnboundedReceiver<ServerMessage>, // Receiver for incoming Twitch messages
    client: TwitchIRCClient<SecureTCPTransport, StaticLoginCredentials>, // Twitch IRC client
    command_symbol: String, // Symbol used to identify commands
    browser_tx: mpsc::Sender<BrowserCommand>, // Sender for sending keypress commands to the browser
}

impl Bot {
    /// Creates a new Bot instance.
    /// 
    /// # Arguments
    /// 
    /// * `config` - A reference to the StreambotConfig containing configuration settings for the bot.
    /// * `browser_tx` - A Sender for sending keypress commands to the browser.
    /// 
    /// # Returns
    /// 
    /// A new Bot instance.
    pub fn new(config: &StreambotConfig, browser_tx: mpsc::Sender<BrowserCommand>) -> Self {

    //pub fn new(config: &StreambotConfig, browser_tx: mpsc::Sender<keys::Key>) -> Self {
        let username = &config.username;
        let access_token = &config.access_token;
        let channel = &config.channel;
        let command_symbol = config.command_symbol.clone();

        // Create a new Twitch IRC client with the specified credentials
        let client_config = ClientConfig::new_simple(
            StaticLoginCredentials::new(username.to_string(), Some(access_token.to_string()))
        );
        let (incoming_messages, client) = TwitchIRCClient::new(client_config);
        
        // Join the specified Twitch channel
        client.join(channel.to_string()).expect("Failed to join channel");

        Self { incoming_messages, client, command_symbol, browser_tx }
        
    }

    /// Runs the bot, listening for incoming messages and matching commands.
    pub async fn run(&mut self) {
        println!("Streambot is running...");

        // Listen for incoming Twitch messages
        while let Some(message) = self.incoming_messages.recv().await {
            if let ServerMessage::Privmsg(chat_message) = message {
                self.match_command(chat_message).await;
            }
        }
    }

    /// Matches and executes commands from chat messages.
    /// 
    /// # Arguments
    /// 
    /// * `chat_message` - The chat message containing the command.
    pub async fn match_command(&self, chat_message: PrivmsgMessage) {
        let content = chat_message.message_text.clone();

        if content.starts_with(&self.command_symbol) {
            let command = content[self.command_symbol.len()..].trim().to_string();

            let browser_command = match command.as_str() {
                "get_url" => {
                    // Use browser_tx to send a command to fetch URL instead
                    Some(BrowserCommand::FetchUrl)
                }
                "up" => Some(BrowserCommand::PredefinedKey(keys::Key::Up)),
                "down" => Some(BrowserCommand::PredefinedKey(keys::Key::Down)),
                "left" => Some(BrowserCommand::PredefinedKey(keys::Key::Left)),
                "right" => Some(BrowserCommand::PredefinedKey(keys::Key::Right)),
                "space" => Some(BrowserCommand::PredefinedKey(keys::Key::Space)),
                "enter" => Some(BrowserCommand::PredefinedKey(keys::Key::Enter)),
                "esc" => Some(BrowserCommand::PredefinedKey(keys::Key::Escape)),
                "delete" => Some(BrowserCommand::PredefinedKey(keys::Key::Delete)),
                _ => {
                    if command.len() == 1 {
                        Some(BrowserCommand::RawCharacter(command))
                    } else {
                        None
                    }
                }
            };

            if let Some(command) = browser_command {
                if let Err(e) = self.browser_tx.send(command).await {
                    eprintln!("Failed to send key command: {}", e);
                }
            }
        }
    }
}

