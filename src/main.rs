use search_libraries::crates::search_crates;
use search_libraries::npm::search_npm;
use search_libraries::to_json;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let query: Option<String> = Some("tokio".to_string());
    let result = search_crates(query, None, None).await;
    match result {
        Ok(data) => {
            to_json::write_json_to_file(&data, "crates.json")?;
            println!("Data written to crates.json");
        },
        Err(e) => eprintln!("Error: {}", e),
    }


    let query: Option<String> = Some("express".to_string());
    let result = search_npm(query, None).await;
     match result {
         Ok(data) => {
                to_json::write_json_to_file(&data, "npm.json")?;
                println!("Data written to npm.json");
         }
         Err(error) => eprintln!("Error: {}", error),
     }
    Ok(())
}
