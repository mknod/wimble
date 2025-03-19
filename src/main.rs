// Import from your own library crate
use wimble::streambot::Streambot;
use wimble::config::Settings;

#[tokio::main]
async fn main() {
    let settings = Settings::new();
    println!("Settings: {:?}", settings);

    let streambot = Streambot::new(
        settings.streambot().channel().to_owned(),
        settings.streambot().username().to_owned(),
        settings.streambot().command_symbol().to_owned(),
        settings.streambot().access_token().to_owned(),
    );

    streambot.start_streambot().await;

}


// async fn start_browser() -> Option<()> {
//      if browser::init_driver().await.is_err() {
//           return None;
//      }
//      if browser::browser().await.is_err() {
//           return None;
//      }
//      Some(())
//      //let _ = browser::browser_command("Up".to_string());
// }

// async fn start_streambot() {
//      //streambot::streambot().await;
// }

// If a command is recieved from the streambot, send it to the browser
//async fn send_command(command: String) {
//     let _ = browser::browser_command(command).await;
//}

//async fn send_chat_message(message: String) {
//     let _ = streambot::send_chat_message(message).await;
//}