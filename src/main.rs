use wimble::streambot::Bot;
use wimble::browser::Browser;
use wimble::config::load_config;
use wimble::command_source::CommandSource;
//use stdin_source::StdinSource;
use wimble::stdin_source::StdinSource;


#[tokio::main]
async fn main() {
    println!("Starting Wimble");

    let config = load_config().expect("Failed to load config");

    let (browser, browser_tx) = Browser::new(&config.browser).await.expect("Failed to initialize browser");

    let browser_clone = browser.clone();
    let browser_task = tokio::spawn(async move {
        browser_clone.keep_alive().await;
    });

    let mut sources: Vec<Box<dyn CommandSource>> = vec![
        Box::new(Bot::new(&config.streambot, browser_tx.clone())),
        Box::new(StdinSource {
            command_symbol: "!".into(), // or match your Twitch bot's symbol
        }),
    ];

    sources.push(Box::new(StdinSource {
        command_symbol: "!".into(), // or whatever matches Twitch
    }));

    let mut tasks = vec![browser_task];

    for mut source in sources {
        let tx = browser_tx.clone();
        tasks.push(tokio::spawn(async move {
            source.run(tx).await;
        }));
    }
    browser.goto(&config.browser.start_url).await.expect("Failed to load page");
    let _ = futures::future::join_all(tasks).await;

}
