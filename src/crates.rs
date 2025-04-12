use crate::ApiClientBuilder;

// https://crates.io/api/v1
/// Function to search for crates on crates.io
///
/// # Arguments
/// * `query` - An optional string to search for.
/// * `page` - An optional integer to specify the page number.
/// * `per_page` - An optional integer to specify the number of results per page.
pub async fn search_crates(
    query: Option<String>,
    page: Option<i32>,
    per_page: Option<i32>,
) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
    let user_agent = "my_crawler (help@my_crawler.com)";
    let mut client = ApiClientBuilder::new("https://crates.io/api/v1/", user_agent)
        .set_param("page", &page.unwrap_or(1).to_string())
        .set_param("per_page", &per_page.unwrap_or(10).to_string());

    if let Some(q) = query {
        client = client.set_param("q", &q);
    }

    client.build().get("crates").await
}
