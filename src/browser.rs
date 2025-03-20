use crate::config::BrowserConfig;
use thirtyfour::{prelude::*, ChromeCapabilities, common::keys};
use tokio::sync::mpsc;
use tokio::task;
use std::sync::Arc;

pub struct Browser {
    driver: WebDriver,
}

impl Browser {
    /// Initializes a new Chrome WebDriver instance and starts a background task to listen for commands.
    pub async fn new(config: &BrowserConfig) -> WebDriverResult<(Self, mpsc::Sender<keys::Key>)> {
        let mut caps = ChromeCapabilities::new();
        // caps.add_arg("--headless").unwrap();
        // caps.add_arg("--disable-gpu").unwrap();
        
        let driver = WebDriver::new("http://localhost:9515", caps).await?;
        let browser = Self { driver };

        let (tx, mut rx) = mpsc::channel(10); // Channel to receive keypress commands

        let driver_clone = browser.driver.clone();
        task::spawn(async move {
            while let Some(key) = rx.recv().await {
                let element = driver_clone.find(By::Tag("body")).await;
                if let Ok(el) = element {
                    if let Err(e) = el.send_keys(key).await {
                        eprintln!("Failed to send key: {:?}", e);
                    }
                }
            }
        });

        Ok((browser, tx)) // Return the Browser instance and Sender
    }

    /// Opens a URL
    pub async fn goto(&self, url: &str) -> WebDriverResult<()> {
        self.driver.goto(url).await?;
        Ok(())
    }

    /// Closes the WebDriver session
    pub async fn close(self) -> WebDriverResult<()> {
        self.driver.quit().await?;
        Ok(())
    }

    /// Keeps the browser alive
    pub async fn keep_alive(&self) {
        loop {
            tokio::time::sleep(std::time::Duration::from_secs(60)).await;
        }
    }
}
