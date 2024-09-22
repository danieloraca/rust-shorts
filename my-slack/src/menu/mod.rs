use dialoguer::{theme::ColorfulTheme, Select};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::env;
use std::error::Error;

#[derive(Serialize)]
struct SlackMessage {
    channel: String,
    text: String,
}

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
    let slack_token = env::var("SLACK_BOT_TOKEN")?;
    let client = Client::new();

    let response = client
        .get("https://slack.com/api/conversations.list")
        .bearer_auth(slack_token)
        .send()
        .await?;

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

        let serialized = serde_json::to_string_pretty(&channel_names).unwrap();
        Ok(serialized)
    } else {
        Ok("Failed to retrieve channels.".to_string())
    }
}

pub async fn get_slack_users() -> Result<String, Box<dyn Error>> {
    let slack_token = env::var("SLACK_BOT_TOKEN")?;
    let client = Client::new();

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

        let serialized = serde_json::to_string_pretty(&user_names).unwrap();
        Ok(user_names)
    } else {
        Ok("Failed to retrieve users.".to_string())
    }
}

pub async fn send_message(user_id: String, message_text: String) -> Result<String, Box<dyn Error>> {
    let slack_token = env::var("SLACK_BOT_TOKEN")?;

    let client = Client::new();

    let message = SlackMessage {
        channel: user_id.to_string(),
        text: message_text.to_string(),
    };

    let response = client
        .post("https://slack.com/api/chat.postMessage")
        .bearer_auth(slack_token)
        .json(&message)
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
                // ask for user id in console

                let user_id: String = dialoguer::Input::new()
                    .with_prompt("Enter User ID")
                    .interact()
                    .unwrap();
                println!("User ID: {}", user_id);

                let message_text: String = dialoguer::Input::new()
                    .with_prompt("Enter Message")
                    .interact()
                    .unwrap();
                println!("Message: {}", message_text);

                match send_message(user_id, message_text).await {
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
