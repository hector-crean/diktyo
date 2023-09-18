use reqwest::Error;
use serde::Deserialize;
use tokio;

#[derive(Deserialize, Debug)]
struct ApiResponse {
    // Define the fields based on the API response you expect
    example_field: String,
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let url = "https://api.example.com/endpoint"; // Replace with your API endpoint
    let response = reqwest::get(url).await?;

    let response_text = response.text().await?;
    println!("Response Text: {}", response_text);

    // Uncomment below lines if the API returns JSON
    // let api_response: ApiResponse = reqwest::get(url).await?.json().await?;
    // println!("API Response: {:?}", api_response);

    Ok(())
}
