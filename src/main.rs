mod config;
mod browser;
mod streambot;

#[tokio::main]
async fn main() {
     // Should check that these are enabled in config.toml before starting them. 
  start_browser().await;
  start_streambot().await;
}


async fn start_browser() -> Option<()> {
     if browser::init_driver().await.is_err() {
          return None;
     }
     if browser::browser().await.is_err() {
          return None;
     }
     Some(())
     //let _ = browser::browser_command("Up".to_string());
}

async fn start_streambot() {
     streambot::streambot().await;
}

// If a command is recieved from the streambot, send it to the browser
async fn send_command(command: String) {
     let _ = browser::browser_command(command).await;
}

async fn send_chat_message(message: String) {
     let _ = streambot::send_chat_message(message).await;
}