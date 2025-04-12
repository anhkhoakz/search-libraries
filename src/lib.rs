pub mod crates;
pub mod npm;
pub mod to_json;

pub use reqwest;
pub use serde_json::Value;
pub use std::error::Error;
pub use std::collections::HashMap;


/// A simple API client for making HTTP requests
pub struct ApiClient {
    base_url: String,
    params: HashMap<String, String>,
    user_agent: String,
}

/// A builder for the `ApiClient`
pub struct ApiClientBuilder {
    base_url: String,
    params: HashMap<String, String>,
    user_agent: String,
}

/// A trait for building API clients
impl ApiClientBuilder {
    /// Creates a new `ApiClientBuilder`
    pub fn new(base_url: &str, user_agent: &str) -> Self {
        ApiClientBuilder {
            base_url: base_url.to_string(),
            params: HashMap::new(),
            user_agent: user_agent.to_string(),
        }
    }

    /// Sets a parameter for the API request
    pub fn set_param(mut self, key: &str, value: &str) -> Self {
        self.params.insert(key.to_string(), value.to_string());
        self
    }

    /// Sets multiple parameters for the API request
    pub fn build(self) -> ApiClient {
        ApiClient {
            base_url: self.base_url,
            params: self.params,
            user_agent: self.user_agent,
        }
    }
}

/// A trait for making API requests
impl ApiClient {
    /// Makes a GET request to the API
    pub async fn get(&self, endpoint: &str) -> Result<Value, Box<dyn Error>> {
        let client = reqwest::Client::new();
        let url = format!("{}{}", self.base_url, endpoint);
        let response = client
            .get(&url)
            .query(&self.params)
            .header("User-Agent", &self.user_agent)
            .send()
            .await?;

        let json: Value = response.json().await?;
        Ok(json)
    }
}
