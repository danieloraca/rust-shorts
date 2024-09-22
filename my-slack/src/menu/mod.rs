use dialoguer::{theme::ColorfulTheme, Select};
use reqwest::Client;
use serde::Deserialize;
use std::env;
use std::error::Error;

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

pub async fn get_slack_channels() -> Result<String, Box<dyn Error>> {
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

    // println!("{:?}", response);

    // Parse the response into the SlackChannelListResponse struct
    let channel_list: SlackChannelListResponse = response.json().await?;

    let mut channel_names = String::new();
    if channel_list.ok {
        let channels = channel_list.channels;
        for channel in channels {
            channel_names.push_str(&format!(
                "Channel ID: {}, Name: {}\n",
                channel.id, channel.name
            ));
        }

        Ok(channel_names)
    } else {
        Ok("Failed to retrieve channels.".to_string())
    }
}

pub async fn get_slack_users() -> Result<String, Box<dyn Error>> {
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

    let mut user_names = String::new();
    if user_list.ok {
        for user in list {
            user_names.push_str(&format!("User ID: {}, Name: {}\n", user.id, user.name));
        }

        Ok(user_names)
    } else {
        Ok("Failed to retrieve users.".to_string())
    }
}

pub async fn send_message() -> Result<String, Box<dyn Error>> {
    let slack_token = env::var("SLACK_BOT_TOKEN")?;
    let slack_channel = env::var("SLACK_CHANNEL_ID")?;
    let slack_user = env::var("SLACK_USER_ID")?;

    let client = Client::new();

    let response = client
        .post("https://slack.com/api/chat.postMessage")
        .bearer_auth(slack_token)
        .form(&[
            ("channel", slack_channel),
            ("text", "Hello from Rust!".to_string()),
            ("user", slack_user),
        ])
        .send()
        .await?;

    let response_text = response.text().await?;

    Ok(response_text)
}

pub async fn show_menu() -> () {
    loop {
        let options = vec![
            "Get Slack Channels",
            "Get Slack Users",
            "Send Message",
            "Exit",
        ];

        let selection = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("Select an option:")
            .items(&options)
            .default(0) // Default to first option
            .interact()
            .unwrap();

        match selection {
            0 => {
                println!("Option 0!");
                match get_slack_channels().await {
                    Ok(result) => println!("Result: {:?}", result),
                    Err(e) => println!("Error: {:?}", e),
                }
            }
            1 => {
                println!("Option 1!");
                match get_slack_users().await {
                    Ok(result) => println!("Result: {:?}", result),
                    Err(e) => println!("Error: {:?}", e),
                }
            }
            2 => {
                println!("Option 2!");
                match send_message().await {
                    Ok(result) => println!("Result: {:?}", result),
                    Err(e) => println!("Error: {:?}", e),
                }
            }
            3 => {
                println!("Exiting...");
                break;
            }
            _ => unreachable!(),
        }
        println!();
    }
}
