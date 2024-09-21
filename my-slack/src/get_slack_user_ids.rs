use reqwest::Client;
use serde::Deserialize;
use std::env;

#[derive(Deserialize, Debug)]
struct SlackUser {
    id: String,
    name: String,
}

#[derive(Deserialize, Debug)]
struct SlackUserListResponse {
    ok: bool,
    members: Vec<SlackUser>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Retrieve the Slack Bot Token from environment variables
    let slack_token = env::var("SLACK_BOT_TOKEN")?;

    // Initialize the HTTP client
    let client = Client::new();

    // Send a GET request to Slack's users.list API
    let response = client
        .get("https://slack.com/api/users.list")
        .bearer_auth(slack_token)
        .send()
        .await?;

    let user_list: SlackUserListResponse = response.json().await?;

    let list = user_list.members;
    for user in list {
        println!("User ID: {}, Name: {}", user.id, user.name);
    }

    // // Look for the user DynamoDan in the list of users
    // if let Some(user) = user_list.members.into_iter().find(|u| u.last_name == "Oraca") {
    //     println!("DynamoDan's user ID: {}", user.id);
    // } else {
    //     println!("User DynamoDan not found.");
    // }

    Ok(())
}
