use wimble::streambot::Bot;
use wimble::browser::Browser;
use wimble::config::load_config;
use tokio::task;

#[tokio::main]
async fn main() {
    println!("Starting Wimble");
    // Load the configuration settings
    let config = load_config().expect("Failed to load config");

    // Initialize the browser with the loaded configuration
    let (browser, browser_tx) = Browser::new(&config.browser).await.expect("Failed to initialize browser");


    // Open the start URL specified in the configuration
    browser.goto(&config.browser.start_url).await.expect("Failed to load page");

    // Start a background task to keep the browser session alive
    let browser_task = task::spawn(async move {
        browser.keep_alive().await;
    });

    // Initialize the bot with the loaded configuration and the browser's Sender
//let mut bot = Bot::new(&config.streambot, browser_tx);
    let mut bot = Bot::new(&config.streambot, browser_tx);

    // Start a background task to run the bot
    let bot_task = task::spawn(async move {
        bot.run().await;
    });

    // Run both tasks in parallel
    let _ = tokio::join!(browser_task, bot_task);
}