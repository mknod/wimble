use crate::config;
use crate::send_command;
use twitch_irc::ClientConfig;
use twitch_irc::SecureTCPTransport;
use twitch_irc::TwitchIRCClient;
use twitch_irc::message::ServerMessage;
use twitch_irc::login::StaticLoginCredentials;

/* Need to clean this up 
 Add tokens to config.toml and comments 
https://docs.rs/twitch-irc/latest/twitch_irc/ 


use async_trait::async_trait;
use twitch_irc::login::{RefreshingLoginCredentials, TokenStorage, UserAccessToken};
use twitch_irc::ClientConfig;

#[derive(Debug)]
struct CustomTokenStorage {
    // fields...
}

#[async_trait]
impl TokenStorage for CustomTokenStorage {
    type LoadError = std::io::Error; // or some other error
    type UpdateError = std::io::Error;

    async fn load_token(&mut self) -> Result<UserAccessToken, Self::LoadError> {
        // Load the currently stored token from the storage.
        Ok(UserAccessToken {
            access_token: todo!(),
            refresh_token: todo!(),
            created_at: todo!(),
            expires_at: todo!()
        })
    }

    async fn update_token(&mut self, token: &UserAccessToken) -> Result<(), Self::UpdateError> {
        // Called after the token was updated successfully, to save the new token.
        // After `update_token()` completes, the `load_token()` method should then return
        // that token for future invocations
        todo!()
    }
}

// these credentials can be generated for your app at https://dev.twitch.tv/console/apps
// the bot's username will be fetched based on your access token
let client_id = "rrbau1x7hl2ssz78nd2l32ns9jrx2w".to_owned();
let client_secret = "m6nuam2b2zgn2fw8actt8hwdummz1g".to_owned();
let storage = CustomTokenStorage { /* ... */ };

let credentials = RefreshingLoginCredentials::new(client_id, client_secret, storage);
// It is also possible to use the same credentials in other places
// such as API calls by cloning them.
let config = ClientConfig::new_simple(credentials);
// then create your client and use it
*/



// Also needs error handling and tests etc
pub async fn streambot() {
    println!("Starting streambot");
    let cconfig: &toml::Value = config::get_config();
    let command_symbol = cconfig["streambot"]["command_symbol"].as_str().unwrap();
    let channel = cconfig["streambot"]["channel"].as_str().unwrap();
    println!("Command symbol: {}", command_symbol);
    let config = ClientConfig::default();
    let (mut incoming_messages, client) =
        TwitchIRCClient::<SecureTCPTransport, StaticLoginCredentials>::new(config);
        let join_handle = async move {
        while let Some(message) = incoming_messages.recv().await {
            match message {
                ServerMessage::Privmsg(msg) => {
                    println!("(#{}) {}: {}", msg.channel_login, msg.sender.name, msg.message_text);
                    if msg.message_text.starts_with(command_symbol) {
                        let command = msg.message_text.trim_start_matches(command_symbol);
                        println!("Command: {}", command);
                        send_command(command.to_string()).await;
                    }
                },
                ServerMessage::Whisper(msg) => {
                    println!("(w) {}: {}", msg.sender.name, msg.message_text);
                },
                _ => {}
            }
        }
        };
      

            //browser::browser_command("Up".to_string()).await.unwrap();
            client.join(channel.to_owned()).unwrap();
    
            // keep the tokio executor alive.
            // If you return instead of waiting the background task will exit.
            join_handle.await;
    }

   pub async fn send_chat_message(message: String) {
    let cconfig: &toml::Value = config::get_config();
    let channel = cconfig["streambot"]["channel"].as_str().unwrap();
    let config = ClientConfig::default();
    let (_incoming_messages, client) =
        TwitchIRCClient::<SecureTCPTransport, StaticLoginCredentials>::new(config);
        client.join(channel.to_owned()).unwrap();
        client.say(channel.to_string(), message).await.unwrap();
    }


