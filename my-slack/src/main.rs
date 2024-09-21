use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::env;

#[derive(Serialize)]
struct SlackMessage {
    channel: String,
    text: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Retrieve the Slack Bot Token from environment variables
    let slack_token = env::var("SLACK_BOT_TOKEN")?;

    // Replace with the actual channel ID you want to send a message to
    let channel_id = "U01SW5S799S"; // Example channel ID

    // Initialize the HTTP client
    let client = Client::new();

    // Define the message payload
    let message = SlackMessage {
        channel: channel_id.to_string(),
        text: "Hello, channel from Rust2!".to_string(),
    };

    // Send the POST request to Slack's chat.postMessage API
    let response = client
        .post("https://slack.com/api/chat.postMessage")
        .bearer_auth(slack_token)
        .json(&message)
        .send()
        .await?;

    // Check the response from Slack
    let response_text = response.text().await?;
    println!("Send Message Response: {}", response_text);

    Ok(())
}
