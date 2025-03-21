use crate::config::BrowserConfig;
use thirtyfour::{prelude::*, ChromeCapabilities, common::keys};
use std::collections::HashMap;
use tokio::sync::mpsc;
use tokio::task;
use std::sync::Arc;

#[derive(Clone)]
pub struct Browser {
    pub driver: WebDriver, // WebDriver instance to control the browser
}

#[derive(Debug, Clone)]
pub enum BrowserCommand {
    PredefinedKey(keys::Key), // Predefined keypresses
    RawCharacter(String),     // Single character keypresses
    FetchUrl,

}


impl Browser {
    fn clone(&self) -> Self {
        Self {
            driver: self.driver.clone(), // Clone the WebDriver instance
        }
    }

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
                            BrowserCommand::PredefinedKey(key) => {
                                // Send predefined keypress
                                if let Err(e) = el.send_keys(key).await {
                                    eprintln!("Failed to send predefined key: {:?}", e);
                                }
                            }
                            BrowserCommand::RawCharacter(text) => {
                                // Send raw character input
                                if let Err(e) = el.send_keys(text.as_str()).await {
                                    eprintln!("Failed to send raw character key: {:?}", e);
                                }
                            }
                            BrowserCommand::FetchUrl => {
                                if let Ok(url) = driver_clone.current_url().await {
                                    println!("Current URL: {}", url);
                                } else {
                                    eprintln!("Failed to fetch URL");
                                }
                            },
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
    pub async fn get_named_elements(&self, config: &BrowserConfig) -> WebDriverResult<HashMap<String, String>> {
        let mut results = HashMap::new();

        for (name, element_config) in &config.elements {
            if let Ok(element) = self.driver.find(By::XPath(&element_config.element)).await {
                let value = self.get_attribute_or_text(&element, &element_config.attribute).await.unwrap_or_default();
                results.insert(name.clone(), value);
            }
        }

        Ok(results)
    }

    /// Retrieves an element's attribute or text content.
    ///
    /// # Arguments
    /// - `element`: The `WebElement` to extract data from.
    /// - `attribute`: The requested attribute or `"text"` for inner text.
    ///
    /// # Returns
    /// - `String`: The extracted value or an empty string on failure.
    async fn get_attribute_or_text(&self, element: &WebElement, attribute: &str) -> WebDriverResult<String> {
        if attribute == "text" {
            return Ok(element.text().await?);
        }
        Ok(element.attr(attribute).await?.unwrap_or_default())
    }
    
    pub async fn fetch_named_elements(&self, config: &BrowserConfig) {
        match self.get_named_elements(config).await {
            Ok(elements) => {
                for (name, value) in &elements {
                    println!("{} -> {}", name, value);
                }
            }
            Err(e) => {
                eprintln!("Failed to get elements: {}", e);
            }
        }
    }
}
