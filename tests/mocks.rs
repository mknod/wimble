// tests/mocks.rs
use mockall::{automock, predicate::*};
use async_trait::async_trait;
use std::collections::HashMap;
//use wimble::browser::browser::BrowserInterface;
use wimble::config::BrowserConfig;

#[automock]
#[async_trait]
pub trait MockableBrowser: Send + Sync {
    async fn goto(&self, url: &str) -> Result<(), String>;
    async fn fetch_named_elements(&self, config: &BrowserConfig) -> Result<HashMap<String, String>, String>;
}