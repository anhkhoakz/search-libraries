use reqwest::Client;
use serde::Serialize;
use serde_json::Value;
use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::io::Write;

/// A client for making API requests.
pub struct ApiClient {
    search_url: String,
    params: HashMap<String, String>,
    user_agent: Option<String>,
}

impl ApiClient {
    /// Creates a new `ApiClient` with the specified search URL and optional user agent.
    pub fn new(search_url: &str, user_agent: Option<&str>) -> Self {
        Self {
            search_url: search_url.to_string(),
            params: HashMap::new(),
            user_agent: user_agent.map(|ua| ua.to_string()),
        }
    }

    /// Sets a query parameter for the API request.
    pub fn set_param(mut self, key: &str, value: &str) -> Self {
        self.params.insert(key.to_string(), value.to_string());
        self
    }

    /// Sends a GET request to the specified endpoint.
    pub async fn get(&self, endpoint: &str) -> Result<Value, Box<dyn Error>> {
        let url = format!("{}{}", self.search_url, endpoint);
        let client = Client::new();
        let mut request = client.get(&url).query(&self.params);

        if let Some(user_agent) = &self.user_agent {
            request = request.header("User-Agent", user_agent);
        }

        let response = request.send().await?;
        Ok(response.json().await?)
    }
}

/// Searches for crates on crates.io.
///
/// # Arguments
///
/// * `query` - The search query (optional).
///
/// # Returns
///
/// Returns the search results as a `serde_json::Value` or an error.
pub async fn search_crates(query: Option<&str>) -> Result<Value, Box<dyn Error>> {
    ApiClient::new(
        "https://crates.io/api/v1/",
        Some("my_crawler (help@my_crawler.com)"),
    )
    .set_param("page", "1")
    .set_param("per_page", "25")
    .set_param("q", query.unwrap_or(""))
    .get("crates")
    .await
}

/// Searches for packages on npm.
///
/// # Arguments
///
/// * `query` - The search query (optional).
///
/// # Returns
///
/// Returns the search results as a `serde_json::Value` or an error.
pub async fn search_npm(query: Option<&str>) -> Result<Value, Box<dyn Error>> {
    ApiClient::new("https://api.npms.io/v2/search/", None)
        .set_param("q", query.unwrap_or(""))
        .set_param("size", "25")
        .get("")
        .await
}

/// Searches for packages on jsDelivr with Alfred-style output.
///
/// # Arguments
///
/// * `query` - The search query (optional).
///
/// # Returns
///
/// Returns the search results as a `serde_json::Value` or an error.
pub async fn search_jsdelivr(query: Option<&str>) -> Result<Value, Box<dyn Error>> {
    let query = query.unwrap_or("");
    let attributes_to_retrieve = ["name", "version", "description", "homepage"];

    let payload = serde_json::json!({
        "params": format!(
            "query={}&page=0&hitsPerPage=25&attributesToHighlight=[]&attributesToRetrieve={}",
            query,
            serde_json::to_string(&attributes_to_retrieve)?
        )
    });

    let response = Client::new()
        .post("https://ofcncog2cu-dsn.algolia.net/1/indexes/npm-search/query")
        .header("x-algolia-agent", "Algolia for JavaScript (3.35.1); Browser (lite)")
        .header("x-algolia-application-id", "OFCNCOG2CU")
        .header("x-algolia-api-key", "f54e21fa3a2a0160595bb058179bfb1e")
        .json(&payload)
        .send()
        .await?;

    if response.status().is_success() {
        let hits = response.json::<Value>().await?.get("hits").cloned().unwrap_or_else(|| serde_json::json!([]));
        Ok(hits)
    } else {
        Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::Other,
            response.text().await?,
        )))
    }
}

/// Searches for images on Docker Hub.
/// * # Arguments
/// * `query` - The search query (optional).
/// * # Returns
/// The search results as a `serde_json::Value` or an error.
pub async fn search_docker(query: Option<&str>) -> Result<Value, Box<dyn Error>> {
    ApiClient::new("https://index.docker.io/v1/search", None)
        .set_param("q", query.unwrap_or(""))
        .set_param("page", "1")
        .get("")
        .await
}

/// Search for composer packages on Packagist.
/// *# Arguments
/// * `query` - The search query (optional).
/// * # Returns
/// The search results as a `serde_json::Value` or an error.

pub async fn search_composer(query: Option<&str>) -> Result<Value, Box<dyn Error>> {
    ApiClient::new("https://packagist.org/search.json", None)
        .set_param("q", query.unwrap_or(""))
        .set_param("per_page", "25")
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
        "npm" => search_npm(Some(query)).await,
        "docker" => search_docker(Some(query)).await,
        "jsdelivr" => search_jsdelivr(Some(query)).await,
        "crates" => search_crates(Some(query)).await,
        "composer" => search_composer(Some(query)).await,
        _ => {
            eprintln!("Unsupported source: {}. Supported sources are 'npm', 'docker', 'jsdelivr', 'crates', and 'composer'.", source);
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
