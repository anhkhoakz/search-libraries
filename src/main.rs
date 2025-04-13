use reqwest::Client;
use serde::Serialize;
use serde_json::Value;
use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::io::Write;

/// A client for making API requests.
pub struct ApiClient {
    base_url: String,
    params: HashMap<String, String>,
    user_agent: String,
}

impl ApiClient {
    /// Creates a new `ApiClient` with the specified base URL and user agent.
    ///
    /// # Arguments
    ///
    /// * `base_url` - The base URL for the API.
    /// * `user_agent` - The user agent string to use for requests.
    pub fn new(base_url: &str, user_agent: &str) -> Self {
        Self {
            base_url: base_url.to_string(),
            params: HashMap::new(),
            user_agent: user_agent.to_string(),
        }
    }

    /// Sets a query parameter for the API request.
    ///
    /// # Arguments
    ///
    /// * `key` - The name of the query parameter.
    /// * `value` - The value of the query parameter.
    ///
    /// # Returns
    ///
    /// Returns the updated `ApiClient` instance.
    pub fn set_param(mut self, key: &str, value: &str) -> Self {
        self.params.insert(key.to_string(), value.to_string());
        self
    }

    /// Sends a GET request to the specified endpoint.
    ///
    /// # Arguments
    ///
    /// * `endpoint` - The API endpoint to send the request to.
    ///
    /// # Returns
    ///
    /// Returns the JSON response as a `serde_json::Value` or an error.
    pub async fn get(&self, endpoint: &str) -> Result<Value, Box<dyn Error>> {
        let url = format!("{}{}", self.base_url, endpoint);
        let response = Client::new()
            .get(&url)
            .query(&self.params)
            .header("User-Agent", &self.user_agent)
            .send()
            .await?;
        Ok(response.json().await?)
    }
}

/// Searches for crates on crates.io.
///
/// # Arguments
///
/// * `query` - The search query (optional).
/// * `page` - The page number to retrieve (optional).
/// * `per_page` - The number of results per page (optional).
///
/// # Returns
///
/// Returns the search results as a `serde_json::Value` or an error.
pub async fn search_crates(
    query: Option<&str>,
    page: Option<i32>,
    per_page: Option<i32>,
) -> Result<Value, Box<dyn Error>> {
    ApiClient::new(
        "https://crates.io/api/v1/",
        "my_crawler (help@my_crawler.com)",
    )
    .set_param("page", &page.unwrap_or(1).to_string())
    .set_param("per_page", &per_page.unwrap_or(10).to_string())
    .set_param("q", query.unwrap_or(""))
    .get("crates")
    .await
}

/// Searches for packages on npm.
///
/// # Arguments
///
/// * `query` - The search query (optional).
/// * `size` - The number of results to retrieve (optional).
///
/// # Returns
///
/// Returns the search results as a `serde_json::Value` or an error.
pub async fn search_npm(query: Option<&str>, size: Option<u32>) -> Result<Value, Box<dyn Error>> {
    ApiClient::new(
        "https://api.npms.io/v2/search/",
        "my_crawler (help@my_crawler.com)",
    )
    .set_param("q", query.unwrap_or(""))
    .set_param("size", &size.unwrap_or(25).to_string())
    .get("")
    .await
}

/// Searches for packages on jsDelivr.
///
/// # Arguments
///
/// * `query` - The search query (optional).
/// * `page` - The page number to retrieve (optional).
///
/// # Returns
///
/// Returns the search results as a `serde_json::Value` or an error.
pub async fn search_jsdelivr(
    query: Option<&str>,
    page: Option<i32>,
) -> Result<Value, Box<dyn Error>> {
    ApiClient::new(
        "https://ofcncog2cu-dsn.algolia.net/1/indexes/npm-search/query",
        "my_crawler (help@my_crawler.com)",
    )
    .set_param("query", query.unwrap_or(""))
    .set_param("page", &page.unwrap_or(0).to_string())
    .set_param("hitsPerPage", "25")
    .set_param(
        "attributesToRetrieve",
        "name,version,description,homepage",
    )
    .set_param("attributesToHighlight", "")
    .set_param(
        "x-algolia-agent",
        "Algolia for JavaScript (3.35.1); Browser (lite)",
    )
    .set_param("x-algolia-application-id", "OFCNCOG2CU")
    .set_param("x-algolia-api-key", "f54e21fa3a2a0160595bb058179bfb1e")
    .get("")
    .await
}

/// Searches for images on Docker Hub.
/// * # Arguments
/// * `query` - The search query (optional).
/// * `page` - The page number to retrieve (optional).
/// * # Returns
/// The search results as a `serde_json::Value` or an error.
pub async fn search_docker(
    query: Option<&str>,
    page: Option<i32>,
) -> Result<Value, Box<dyn Error>> {
    ApiClient::new(
        "https://index.docker.io/v1/search",
        "my_crawler (help@my_crawler.com)",
    )
    .set_param("q", query.unwrap_or(""))
    .set_param("page", &page.unwrap_or(1).to_string())
    .get("")
    .await
}


/// Writes JSON data to a file.
///
/// # Arguments
///
/// * `data` - The data to write, which must implement `Serialize`.
/// * `file_name` - The name of the file to write to.
///
/// # Returns
///
/// Returns `Ok(())` if the operation succeeds, or an error.
pub fn write_json_to_file<T: Serialize>(data: &T, file_name: &str) -> Result<(), Box<dyn Error>> {
    let json_string = serde_json::to_string_pretty(data)?;
    File::create(file_name)?.write_all(json_string.as_bytes())?;
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 3 {
        eprintln!("Usage: {} <source> <query>", args[0]);
        return Ok(());
    }

    let source = &args[1];
    let query = &args[2];

    let result = match source.as_str() {
        "npm" => search_npm(Some(query), None).await,
        "docker" => search_docker(Some(query), None).await,
        "jsdelivr" => search_jsdelivr(Some(query), None).await,
        "crates" => search_crates(Some(query), None, None).await,
        _ => {
            eprintln!("Unsupported source: {}. Supported sources are 'npm' or 'docker'", source);
            return Ok(());
        }
    };

    match result {
        Ok(data) => {
            println!("{}", serde_json::to_string_pretty(&data)?);
        }
        Err(error) => {
            let error_response = serde_json::json!({
                "items": [
                    {
                        "title": "Error",
                        "subtitle": error.to_string()
                    }
                ]
            });
            println!("{}", serde_json::to_string_pretty(&error_response)?);
        }
    }
    Ok(())
}