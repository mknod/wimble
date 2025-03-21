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

    // Clone `browser` so we can move it into the async task without losing ownership
    let browser_clone = browser.clone();

    // Open the start URL specified in the configuration    
    browser.goto(&config.browser.start_url).await.expect("Failed to load page");

    // Start a background task to keep the browser session alive
    let browser_task = task::spawn(async move {
        browser_clone.keep_alive().await; // Call the correct keep_alive() function
    });

    // Get elements using the original browser instance

    // Initialize the bot with the loaded configuration and the browser's Sender
    let mut bot = Bot::new(&config.streambot, browser_tx);

    // Start a background task to run the bot
    let bot_task = task::spawn(async move {
        bot.run().await;
    });

    // Run both tasks in parallel
    let _ = tokio::join!(browser_task, bot_task);
}
