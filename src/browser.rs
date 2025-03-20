use crate::config::BrowserConfig;
use thirtyfour::{prelude::*, ChromeCapabilities, common::keys};
use tokio::sync::mpsc;
use tokio::task;
use std::sync::Arc;

pub struct Browser {
    driver: WebDriver, // WebDriver instance to control the browser
}

impl Browser {
    /// Initializes a new Chrome WebDriver instance and starts a background task to listen for commands.
    /// 
    /// # Arguments
    /// 
    /// * `config` - A reference to the BrowserConfig containing configuration settings for the browser.
    /// 
    /// # Returns
    /// 
    /// A tuple containing the Browser instance and a Sender for sending keypress commands.
    pub async fn new(config: &BrowserConfig) -> WebDriverResult<(Self, mpsc::Sender<keys::Key>)> {
        let mut caps = ChromeCapabilities::new();
        // Uncomment the following lines to run the browser in headless mode
        // caps.add_arg("--headless").unwrap();
        // caps.add_arg("--disable-gpu").unwrap();
        
        // Create a new WebDriver instance with the specified capabilities
        let driver = WebDriver::new("http://localhost:9515", caps).await?;
        let browser = Self { driver };

        // Create a channel to receive keypress commands
        let (tx, mut rx) = mpsc::channel(10);

        // Clone the WebDriver instance for use in the background task
        let driver_clone = browser.driver.clone();
        task::spawn(async move {
            // Listen for keypress commands and send them to the browser
            while let Some(key) = rx.recv().await {
                let element = driver_clone.find(By::Tag("body")).await;
                if let Ok(el) = element {
                    if let Err(e) = el.send_keys(key).await {
                        eprintln!("Failed to send key: {:?}", e);
                    }
                }
            }
        });

        // Return the Browser instance and Sender
        Ok((browser, tx))
    }

    /// Opens a URL in the browser.
    /// 
    /// # Arguments
    /// 
    /// * `url` - The URL to open.
    /// 
    /// # Returns
    /// 
    /// A WebDriverResult indicating the success or failure of the operation.
    pub async fn goto(&self, url: &str) -> WebDriverResult<()> {
        self.driver.goto(url).await?;
        Ok(())
    }

    /// Closes the WebDriver session.
    /// 
    /// # Returns
    /// 
    /// A WebDriverResult indicating the success or failure of the operation.
    pub async fn close(self) -> WebDriverResult<()> {
        self.driver.quit().await?;
        Ok(())
    }

    /// Keeps the browser session alive by periodically sleeping.
    pub async fn keep_alive(&self) {
        loop {
            tokio::time::sleep(std::time::Duration::from_secs(60)).await;
        }
    }
}
