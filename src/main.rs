use wimble::streambot::Bot;
use wimble::browser::Browser;
use wimble::config::load_config;
use tokio::task;

#[tokio::main]
async fn main() {
    println!("Starting Wimble");
    let config = load_config().expect("Failed to load config");

    // Initialize browser
    let (browser, browser_tx) = Browser::new(&config.browser).await.expect("Failed to initialize browser");

    // Open the start URL
    browser.goto(&config.browser.start_url).await.expect("Failed to load page");

    // Start a task to keep the browser alive
    let browser_task = task::spawn(async move {
        browser.keep_alive().await;
    });

    // Initialize and start bot
    let mut bot = Bot::new(&config.streambot, browser_tx);
    let bot_task = task::spawn(async move {
        bot.run().await;
    });

    // Run both tasks in parallel
    let _ = tokio::join!(browser_task, bot_task);
}