use crate::config::StreambotConfig;
use crate::browser::BrowserCommand; 
use tokio::sync::mpsc;
use twitch_irc::login::StaticLoginCredentials;
use twitch_irc::ClientConfig;
use twitch_irc::TwitchIRCClient;
use twitch_irc::SecureTCPTransport;
use twitch_irc::message::PrivmsgMessage;
use twitch_irc::message::ServerMessage;
use crate::command_source::CommandSource;
use async_trait::async_trait;
use crate::command_parser::parse_command;
//use crate::command_parser::CommandAction;
use crate::command_parser::handle_parsed_command;



pub struct Bot {
    incoming_messages: mpsc::UnboundedReceiver<ServerMessage>, // Receiver for incoming Twitch messages
    // Warnings about client not being used are harmless
    client: TwitchIRCClient<SecureTCPTransport, StaticLoginCredentials>, // Twitch IRC client
    command_symbol: String, // Symbol used to identify commands
    channel: String,
    browser_tx: mpsc::Sender<BrowserCommand>, // Sender for sending keypress commands to the browser
}

#[async_trait]
impl CommandSource for Bot {
    async fn run(&mut self, browser_tx: mpsc::Sender<BrowserCommand>) {
        self.browser_tx = browser_tx;
        self.run().await;
    }
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
                self.handle_chat_command(chat_message).await;
            }
        }
    }

    pub async fn handle_chat_command(&self, message: PrivmsgMessage) {
        let content = message.message_text.clone();
        let tx = self.browser_tx.clone();
        let symbol = &self.command_symbol;
        let channel = self.channel.clone();
    
        let result = parse_command(&content, &symbol, &tx).await;
    
        handle_parsed_command(result, &tx, |msg| {
            println!("[Response] {}", msg);
            let _ = self.client.say(channel.clone(), msg);
        }).await;
    }

}