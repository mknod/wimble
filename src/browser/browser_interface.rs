// src/browser_interface.rs
use async_trait::async_trait;
use std::collections::HashMap;
use crate::config::BrowserConfig;

#[async_trait]
pub trait BrowserInterface: Send + Sync {
    async fn goto(&self, url: &str) -> Result<(), String>;
    async fn fetch_named_elements(&self, config: &BrowserConfig) -> Result<HashMap<String, String>, String>;
}

