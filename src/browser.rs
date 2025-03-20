use crate::config::BrowserConfig;
use thirtyfour::{prelude::*, ChromeCapabilities, common::keys};
use tokio::sync::mpsc;
use tokio::task;
use std::sync::Arc;

pub struct Browser {
    driver: WebDriver, // WebDriver instance to control the browser
}

#[derive(Debug, Clone)]
pub enum BrowserCommand {
    PredefinedKey(keys::Key), // Predefined keypresses
    RawCharacter(String),     // Single character keypresses
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
    pub async fn new(config: &BrowserConfig) -> WebDriverResult<(Self, mpsc::Sender<BrowserCommand>)> {
        let mut caps = ChromeCapabilities::new();
        let driver = WebDriver::new("http://localhost:9515", caps).await?;
        let browser = Self { driver };

        let (tx, mut rx) = mpsc::channel(10); // Channel to receive keypress commands

        // I kind of hate this, but it's the only way I could think to get it to work. 
        // This is where the browser commands are actually executed, 
        // They are sent via browser_tx in the Bot struct.
        let driver_clone = browser.driver.clone();
        task::spawn(async move {
            while let Some(command) = rx.recv().await {
                let driver_clone = driver_clone.clone(); // Clone WebDriver for async move
                tokio::spawn(async move {
                    if let Ok(el) = driver_clone.find(By::Tag("body")).await {
                        match command {
                            // If the command is a predefined keypress, send the key
                            BrowserCommand::PredefinedKey(key) => {
                                // Send predefined keypress
                                if let Err(e) = el.send_keys(key).await {
                                    eprintln!("Failed to send predefined key: {:?}", e);
                                }
                            }
                            // If the command is a raw character, send the character
                            BrowserCommand::RawCharacter(text) => {
                                // Send raw character input
                                if let Err(e) = el.send_keys(text.as_str()).await {
                                    eprintln!("Failed to send raw character key: {:?}", e);
                                }
                            }
                        }
                    }
                });
            }
        });

        Ok((browser, tx)) // Return the Browser instance and Sender
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
