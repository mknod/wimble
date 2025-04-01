use crate::config::StreambotConfig;
use crate::config::OutputMode;
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
use crate::command_parser::handle_parsed_command;
use crate::command_parser::CommandAction;

pub struct Bot {
    incoming_messages: mpsc::UnboundedReceiver<ServerMessage>,
    client: TwitchIRCClient<SecureTCPTransport, StaticLoginCredentials>,
    command_symbol: String,
    channel: String,
    browser_tx: mpsc::Sender<BrowserCommand>,
    output: OutputMode,
}

#[async_trait]
impl CommandSource for Bot {
    async fn run(&mut self, browser_tx: mpsc::Sender<BrowserCommand>) {
        self.browser_tx = browser_tx;
        self.run().await;
    }
}

impl Bot {
    pub fn new(config: &StreambotConfig, browser_tx: mpsc::Sender<BrowserCommand>) -> Self {
        let username = &config.username;
        let access_token = &config.access_token;
        let channel = config.channel.clone();
        let command_symbol = config.command_symbol.clone();
        let client_config = ClientConfig::new_simple(
            StaticLoginCredentials::new(username.to_string(), Some(access_token.to_string()))
        );
        let (incoming_messages, client) = TwitchIRCClient::new(client_config);

        client.join(channel.to_string()).expect("Failed to join channel");

        let output = config.output.clone();

        Self {
            incoming_messages,
            client,
            command_symbol,
            channel,
            browser_tx,
            output,
        }
    }

    pub async fn run(&mut self) {
        println!("Streambot is running...");

        while let Some(message) = self.incoming_messages.recv().await {
            if let ServerMessage::Privmsg(chat_message) = message {
                self.handle_chat_command(chat_message).await;
            }
        }
    }

    async fn send_response(&self, msg: &str) {
        match self.output {
            OutputMode::Chat => {
                let _ = self.client.say(self.channel.clone(), msg.to_string()).await;
            }
            OutputMode::Stdout => {
                println!("[Response] {}", msg);
            }
            OutputMode::Both => {
                println!("[Response] {}", msg);
                let _ = self.client.say(self.channel.clone(), msg.to_string()).await;
            }
        }
    }

    pub async fn handle_chat_command(&self, message: PrivmsgMessage) {
        let content = message.message_text.clone();
        let tx = self.browser_tx.clone();
        let symbol = &self.command_symbol;
    
        let result = parse_command(&content, &symbol, &tx).await;
    
        match result {
            CommandAction::SendToBrowser(cmd) => {
                let _ = tx.send(cmd).await;
            }
            CommandAction::WithResponse(cmd, msg) => {
                let _ = tx.send(cmd).await;
                self.send_response(&msg).await;
            }
            CommandAction::ResponseOnly(msg) => {
                self.send_response(&msg).await;
            }
            CommandAction::Noop => {}
        }
    }


}
