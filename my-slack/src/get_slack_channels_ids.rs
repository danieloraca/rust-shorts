use reqwest::Client;
use serde::Deserialize;
use std::env;

#[derive(Deserialize, Debug)]
struct SlackChannel {
    id: String,
    name: String,
}

#[derive(Deserialize, Debug)]
struct SlackChannelListResponse {
    ok: bool,
    channels: Vec<SlackChannel>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Retrieve the Slack Bot Token from environment variables
    let slack_token = env::var("SLACK_BOT_TOKEN")?;

    // Initialize the HTTP client
    let client = Client::new();

    // Send a GET request to Slack's conversations.list API to get all channels
    let response = client
        .get("https://slack.com/api/conversations.list")
        .bearer_auth(slack_token)
        .send()
        .await?;

    // Parse the response into the SlackChannelListResponse struct
    let channel_list: SlackChannelListResponse = response.json().await?;

    if channel_list.ok {
        let channels = channel_list.channels;
        for channel in channels {
            println!("Channel ID: {}, Name: {}", channel.id, channel.name);
        }
    } else {
        println!("Failed to retrieve channels.");
    }

    Ok(())
}
