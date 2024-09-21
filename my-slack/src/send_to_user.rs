use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::env;

#[derive(Serialize)]
struct OpenConversation {
    users: String,
}

#[derive(Deserialize, Debug)]
struct OpenResponse {
    ok: bool,
    channel: Option<Channel>,
    error: Option<String>,
}

#[derive(Deserialize, Debug)]
struct Channel {
    id: String,
}

#[derive(Serialize)]
struct SlackMessage {
    channel: String,
    text: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let slack_token = env::var("SLACK_BOT_TOKEN")?;
    let user_id = "U01SW5S799S"; // Replace with DynamoDan's user ID

    let client = Client::new();

    // Step 1: Open a DM channel with the user
    let open_conversation = OpenConversation {
        users: user_id.to_string(),
    };

    let open_response = client
        .post("https://slack.com/api/conversations.open")
        .bearer_auth(&slack_token)
        .json(&open_conversation)
        .send()
        .await?;

    let open_response_text = open_response.text().await?;
    println!("Open Conversation Response: {}", open_response_text);

    // Parse the response to extract the channel ID
    let open_response: OpenResponse = serde_json::from_str(&open_response_text)?;

    if open_response.ok {
        if let Some(channel) = open_response.channel {
            // Step 2: Send a message to the opened DM channel
            let message = SlackMessage {
                channel: channel.id,  // Use the channel ID returned by conversations.open
                text: "Hello DynamoDan from Rust!".to_string(),
            };

            let response = client
                .post("https://slack.com/api/chat.postMessage")
                .bearer_auth(&slack_token)
                .json(&message)
                .send()
                .await?;

            let response_text = response.text().await?;
            println!("Send Message Response: {}", response_text);
        } else {
            println!("Failed to open conversation: Channel not found.");
        }
    } else {
        println!("Error opening conversation: {:?}", open_response.error);
    }

    Ok(())
}
