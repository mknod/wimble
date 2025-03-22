use crate::config::BrowserConfig;
use crate::config::ElementConfig;
use thirtyfour::{prelude::*, ChromeCapabilities, common::keys };
use std::collections::HashMap;
use tokio::sync::mpsc;
use tokio::task;
use tokio::time::{timeout, Duration};
use std::sync::Arc;



#[derive(Clone)]
pub struct Browser {
    pub driver: WebDriver, // WebDriver instance to control the browser
}

#[derive(Debug, Clone)]
pub enum BrowserCommand {
    PredefinedKey(keys::Key), // Predefined keypresses
    RawCharacter(String),     // Single character keypresses
    FetchUrl(mpsc::Sender<String>),
    Goto(String),             // New command to go to a specific URL
    GetElementValue(String, mpsc::Sender<String>),
    
}


impl Browser {
    /*fn clone(&self) -> Self {
        Self {
            driver: self.driver.clone(), // Clone the WebDriver instance
        }
    }
    */
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
        let mut caps: ChromeCapabilities = ChromeCapabilities::new();
        caps.add_arg("--auto-open-devtools-for-tabs")?; // Optional: opens DevTools
        caps.add_arg("--start-maximized")?;
        caps.add_arg("--disable-infobars")?; 
        caps.add_arg("--disable-extensions")?;
        caps.add_arg("--disable-popup-blocking")?;
        caps.add_arg("--no-sandbox")?;
        caps.add_arg("--disable-dev-shm-usage")?;
        caps.add_arg("--disable-gpu")?;
        caps.add_arg("--remote-debugging-port=9222")?; 
        caps.add_arg("--auto-open-devtools-for-tabs")?;

        
        let driver = WebDriver::new("http://localhost:9515", caps).await?;
        let browser = Self { driver };
        let (tx, mut rx) = mpsc::channel(10); // Channel to receive keypress commands

        // I kind of hate this, but it's the only way I could think to get it to work. 
        // This is where the browser commands are actually executed, 
        // They are sent via browser_tx in the Bot struct.
        let driver_clone = browser.driver.clone();
        let goto_config = config.goto.clone(); // Clone the goto configuration
        let elements_config = Arc::new(config.elements.clone());

        println!("Elements config: {:?}", elements_config);
        // ======================================================================================
        // task::spawn(async move {
        //     while let Some(command) = rx.recv().await {
        //         let driver_clone = driver_clone.clone(); // Clone WebDriver for async move
        //         let goto_config = goto_config.clone(); // Clone the goto configuration for async move
        //         let elements_config = Arc::clone(&elements_config); // ✅ cheap clone of the Arc

        //         tokio::spawn(async move {

        //             match command {

        //                 BrowserCommand::GetElementValue(key, sender) => {
        //                     println!("Handling GetElementValue command for key: {}", key);
        //                     if let Some(element_cfg) = elements_config.get(&key) {
        //                         let el_result = timeout(Duration::from_secs(5), driver_clone.find(By::XPath(&element_cfg.element))).await;
        //                         if let Ok(Ok(el)) = el_result {
        //                             let val = if element_cfg.attribute == "text" {
        //                                 el.text().await.unwrap_or_default()
        //                             } else {
        //                                 el.attr(&element_cfg.attribute).await.unwrap_or(None).unwrap_or_default()
        //                             };
        //                             let _ = sender.send(val).await;
        //                         } else {
        //                             let _ = sender.send("Element not found".to_string()).await;
        //                         }
        //                     } else {
        //                         println!("Configuration not found for key: {}", key);
        //                         let _ = sender.send("Key not found in config".to_string()).await;
        //                     }
        //                 }
                
        //                 BrowserCommand::PredefinedKey(key) => {
        //                     if let Ok(el) = driver_clone.find(By::Tag("body")).await {
        //                         if let Err(e) = el.send_keys(key).await {
        //                             eprintln!("Failed to send predefined key: {:?}", e);
        //                         }
        //                     }
        //                 }
                
        //                 BrowserCommand::RawCharacter(text) => {
        //                     if let Ok(el) = driver_clone.find(By::Tag("body")).await {
        //                         if let Err(e) = el.send_keys(text.as_str()).await {
        //                             eprintln!("Failed to send raw character key: {:?}", e);
        //                         }
        //                     }
        //                 }
                
        //                 BrowserCommand::FetchUrl(sender) => {
        //                     if let Ok(url) = driver_clone.current_url().await {
        //                         let url_string = url.to_string();
        //                         if let Err(e) = sender.send(url_string).await {
        //                             eprintln!("Failed to send URL: {:?}", e);
        //                         }
        //                     } else {
        //                         eprintln!("Failed to fetch URL");
        //                     }
        //                 }
                
        //                 BrowserCommand::Goto(key) => {
        //                     println!("Going to URL for key: {}", key);
        //                     if let Some(url) = goto_config.get(&key) {
        //                         if let Err(e) = driver_clone.goto(url).await {
        //                             eprintln!("Failed to go to URL {}: {:?}", url, e);
        //                         }
        //                     } else {
        //                         eprintln!("No URL found for key: {}", key);
        //                     }
        //                 }
                           
        //             }
        //         });
        //     }
        //});


        // ======================================================================================

        task::spawn(async move {
            while let Some(command) = rx.recv().await {
                let driver_clone = driver_clone.clone();
                let goto_config = goto_config.clone();
                let elements_config = Arc::clone(&elements_config); // ✅ Arc clone (cheap)
        
                tokio::spawn(async move {
                    match command {
                        BrowserCommand::GetElementValue(key, sender) => {

                            if let Some(element_cfg) = elements_config.get(&key) {
                                let val = Browser::fetch_element_value(&driver_clone, element_cfg).await.unwrap_or_else(|_| "Error".into());
                                let _ = sender.send(val).await;
                                println!("Looking up key: {}", key);
                                println!("Known keys: {:?}", elements_config.keys());
                            } else {
                                let _ = sender.send("Key not found in config".to_string()).await;
                            }
                        }

                        BrowserCommand::PredefinedKey(key) => {
                           if let Ok(el) = driver_clone.find(By::Tag("body")).await {
                               if let Err(e) = el.send_keys(key).await {
                                   eprintln!("Failed to send predefined key: {:?}", e);
                               }
                           }
                       }
            
                       BrowserCommand::RawCharacter(text) => {
                           if let Ok(el) = driver_clone.find(By::Tag("body")).await {
                               if let Err(e) = el.send_keys(text.as_str()).await {
                                   eprintln!("Failed to send raw character key: {:?}", e);
                               }
                           }
                       }
            
                       BrowserCommand::FetchUrl(sender) => {
                           if let Ok(url) = driver_clone.current_url().await {
                               let url_string = url.to_string();
                               if let Err(e) = sender.send(url_string).await {
                                   eprintln!("Failed to send URL: {:?}", e);
                               }
                           } else {
                               eprintln!("Failed to fetch URL");
                           }
                       }
            
                       BrowserCommand::Goto(key) => {
                           println!("Going to URL for key: {}", key);
                           if let Some(url) = goto_config.get(&key) {
                               if let Err(e) = driver_clone.goto(url).await {
                                   eprintln!("Failed to go to URL {}: {:?}", url, e);
                               }
                           } else {
                               eprintln!("No URL found for key: {}", key);
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


   pub async fn fetch_element_value(driver: &WebDriver, element_cfg: &ElementConfig) -> WebDriverResult<String> {
    // Check if iframe exists in config.toml
    if let Some(iframe_selector) = &element_cfg.iframe {
        //let iframe_format = format!("{}", iframe_selector.strip_prefix("#").unwrap());
        println!("[fetch_element_value] Looking for iframe: {}", iframe_selector);
        let iframen = iframe_selector.strip_prefix("#").unwrap();
        let iframe_locator = driver.find(By::Id(iframen)).await;
        match iframe_locator {
            Ok(iframe) => {
                if let Err(e) = iframe.enter_frame().await {
                    println!("[fetch_element_value] Failed to enter iframe: {:?}", e);
                    return Ok("Failed to enter iframe".into());
                } else {
                    println!("[fetch_element_value] Entered iframe");
                }
            }
            Err(e) => {
                println!("[fetch_element_value] Failed to find iframe: {:?}", e);
                return Ok("Failed to find iframe".into());
            }
        }
    }

    // Lookup the element
    let locator = if element_cfg.element.trim().starts_with('/') {
        By::XPath(&element_cfg.element)
    } else {
        By::Css(&element_cfg.element)
    };

    let el_result = timeout(Duration::from_secs(5), driver.find(locator)).await;

    match el_result {
        Ok(Ok(el)) => {
            println!("[fetch_element_value] Element found!");
            if element_cfg.attribute == "text" {
                el.text().await
            } else {
                Ok(el.attr(&element_cfg.attribute).await?.unwrap_or_default())
            }
        }
        Ok(Err(e)) => {
            println!("[fetch_element_value] find() error: {:?}", e);
            Ok("Element not found (error)".into())
        }
        Err(_) => {
            println!("[fetch_element_value] Timeout waiting for element");
            Ok("Element not found (timeout)".into())
            }
        }
    }
}

