use crate::ApiClientBuilder;

/// Search for packages on npm
/// This function allows you to search for packages on npm using the npms.io API.
///
/// # Arguments
/// * `query` - An optional string to search for.
/// * `size` - An optional integer to specify the number of results to return.
pub async fn search_npm(
    query: Option<String>,
    size: Option<u32>
) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
    let user_agent = "my_crawler (help@my_crawler.com)";
    let mut client = ApiClientBuilder::new("https://api.npms.io/v2/search/", user_agent);

    if let Some(q) = query {
        client = client.set_param("q", &q);
    }

    let size_value = size.unwrap_or(25);
    client = client.set_param("size", &size_value.to_string());

    client.build().get("").await
}
