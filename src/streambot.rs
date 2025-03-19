use twitch_irc::login::{StaticLoginCredentials, UserAccessToken};
use twitch_irc::{client, ClientConfig};
use twitch_irc::irc;
use twitch_irc::message::AsRawIRC;
use twitch_irc::{SecureTCPTransport, TwitchIRCClient};


 pub struct Streambot {
     channel: String,
     username: String,
     command_symbol: String,
     access_token: String,
 }

impl Streambot {
    pub fn new(channel: String, username: String, command_symbol: String, access_token: String) -> Self {
        Self {
            channel,
            username,
            command_symbol,
            access_token,
        }
    }

    pub async fn start_streambot(&self) {
        let config = ClientConfig::new_simple(
            StaticLoginCredentials::new(self.username.clone(), Some(self.access_token.clone()))
        );
        let (_, client) = 
                TwitchIRCClient::<SecureTCPTransport, StaticLoginCredentials>::new(config);
        let channel = self.channel.clone();
        client.say(channel.to_owned(), "streambot started".to_string()).await.unwrap();
    }
    
}