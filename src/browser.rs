use std::sync::Mutex;
use std::mem::ManuallyDrop;
use lazy_static::lazy_static;
use thirtyfour::prelude::*;
use thirtyfour::common::keys::Key;
use std::error::Error;
use std::time::Duration;

use crate::{config, send_chat_message};

lazy_static! {
    static ref DRIVER: Mutex<Option<ManuallyDrop<WebDriver>>> = Mutex::new(None);
}
 pub async fn init_driver() -> Result<(), Box<dyn Error + Send + Sync>> {
     println!("init browser started");
     let caps = DesiredCapabilities::chrome();
    let driver = ManuallyDrop::new(WebDriver::new("http://localhost:9515", caps).await?);
    *DRIVER.lock().unwrap() = Some(driver);
    if DRIVER.lock().unwrap().is_none() {
        return Err("Failed to initialize WebDriver".into());
    }
    
    Ok(())
}

async fn wait_for_element(driver: &WebDriver, by: By, timeout: Duration) -> WebDriverResult<WebElement> {
    driver.query(by).wait(timeout, Duration::from_millis(500)).first().await
}


pub async fn browser() -> Result<(), Box<dyn Error + Send + Sync>> {
    println!("browser started");
    let driver = DRIVER.lock().unwrap();
    let driver = driver.as_ref().unwrap();
    let config: &toml::Value = config::get_config();
    let start_url = config["browser"]["start_url"].as_str().unwrap();
    println!("Browser starting at: {}", start_url);
    driver.goto(start_url).await?;
    // I dont like this, but I am leaving it to torture you. 
    let elem_body = wait_for_element(driver, By::Tag("body"), Duration::from_secs(10)).await?;
    elem_body.send_keys(Key::Space).await?;
    Ok(())
}


// Clean this up too, make it more CRUD like
// Add error handling

pub async fn browser_command(command:String ) -> Result<(), Box<dyn Error + Send + Sync>> {


    // Read [goto] section from config.toml and get all the keys
    let config: &toml::Value = config::get_config();
    let goto = config["goto"].as_table().unwrap();

    let keys = goto.keys();


    let driver = DRIVER.lock().unwrap();
    let driver = driver.as_ref().unwrap();

    let elem_body = driver.find(By::Tag("body")).await?;



      // If command is part of the keys, go to the corresponding value
      if keys.cloned().any(|x| x == command) {
        let url = goto[&command].as_str().unwrap();
        println!("Going to: {}", url);
        driver.goto(url).await?;
        let elem_body = wait_for_element(driver, By::Tag("body"), Duration::from_secs(10)).await?;
        elem_body.send_keys(Key::Space).await?;
        return Ok(());
    }



    
    match command.to_lowercase().as_str() {
        "channel_surf" => {
            let mut i = 0;
            loop {
                elem_body.send_keys(Key::Up).await?;
                // I dont like this.
                // Number of channels and time_to_wait can be set in config.toml instead of hard coded
                // Better yet have chat command to set these values
                // or both.

                tokio::time::sleep(Duration::from_secs(13)).await;
                i += 1;
                if i == 10 {
                    break;
                }
            }
        },
        "up" => elem_body.send_keys(Key::Up).await?,
        "down" => elem_body.send_keys(Key::Down).await?,
        "left" => elem_body.send_keys(Key::Left).await?,
        "right" => elem_body.send_keys(Key::Right).await?,
        "enter" => elem_body.send_keys(Key::Enter).await?,
        "space" => elem_body.send_keys(Key::Space).await?,
        "tab" => elem_body.send_keys(Key::Tab).await?,
        "backspace" => elem_body.send_keys(Key::Backspace).await?,
        "escape" => elem_body.send_keys(Key::Escape).await?,
        "add" => elem_body.send_keys(Key::Add).await?,
        "subtract" => elem_body.send_keys(Key::Subtract).await?,
        // If command is not recognized, attempt to send the key using send_keys
        "refresh" => driver.refresh().await?,
        "back" => driver.back().await?,
        "forward" => driver.forward().await?,
        "get_url" => {
            let current_url = driver.current_url().await?;
            // for send_chat_message to work, you need to have a twitch account and authenticate
            send_chat_message(current_url.to_string()).await;
            println!("Current URL: {}", current_url);
        },
        _ => elem_body.send_keys(command).await?,
    }



    // if there are any errors return them and let the user know



    Ok(())
}

