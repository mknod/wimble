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
    channel: String,
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
        let channel = config.channel.clone();
        let command_symbol = config.command_symbol.clone();

        // Create a new Twitch IRC client with the specified credentials
        let client_config = ClientConfig::new_simple(
            StaticLoginCredentials::new(username.to_string(), Some(access_token.to_string()))
        );
        let (incoming_messages, client) = TwitchIRCClient::new(client_config);
        
        // Join the specified Twitch channel
        client.join(channel.to_string()).expect("Failed to join channel");

        Self { incoming_messages, client, command_symbol, channel, browser_tx }
        
    }

    /// Runs the bot, listening for incoming messages and matching commands.
    pub async fn run(&mut self) {
        println!("Streambot is running...");

        // Listen for incoming Twitch messages
        while let Some(message) = self.incoming_messages.recv().await {
            if let ServerMessage::Privmsg(chat_message) = message {
                self.match_command( chat_message).await;
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
        let channel = &self.channel;
        if content.starts_with(&self.command_symbol) {
            
            let command = content[self.command_symbol.len()..].trim().to_string();

            let browser_command = match command.as_str() {

                "up" => Some(BrowserCommand::PredefinedKey(keys::Key::Up)),
                "down" => Some(BrowserCommand::PredefinedKey(keys::Key::Down)),
                "left" => Some(BrowserCommand::PredefinedKey(keys::Key::Left)),
                "right" => Some(BrowserCommand::PredefinedKey(keys::Key::Right)),
                "space" => Some(BrowserCommand::PredefinedKey(keys::Key::Space)),
                "enter" => Some(BrowserCommand::PredefinedKey(keys::Key::Enter)),
                "esc" => Some(BrowserCommand::PredefinedKey(keys::Key::Escape)),
                "delete" => Some(BrowserCommand::PredefinedKey(keys::Key::Delete)),
                "get_url" => {
                    let (url_sender, mut url_receiver) = mpsc::channel(1);
                    self.browser_tx.send(BrowserCommand::FetchUrl(url_sender)).await.unwrap();
                    if let Some(url) = url_receiver.recv().await {
                        println!("Fetched URL: {}", url);
                        self.client.say(channel.to_string(), url).await.expect("Failed to send message");
                    } else {
                        eprintln!("Failed to fetch URL");
                    }
                    None
                }
                _ => {
                    // Try dynamic element fetch command
                    let (sender, mut receiver) = mpsc::channel(1);
                        if let Err(e) = self.browser_tx.send(BrowserCommand::GetElementValue(command.clone(), sender)).await {
                            eprintln!("Failed to send GetElementValue: {}", e);
                            return;
                        }
                        if let Some(value) = receiver.recv().await {
                            let msg = format!("{}: {}", command, value);
                            self.client.say(channel.to_string(), msg).await.expect("Failed to send message");
                        }                                
                    
                    if command.len() == 1 {
                        Some(BrowserCommand::RawCharacter(command))
                    } else {
                        Some(BrowserCommand::Goto(command))
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

