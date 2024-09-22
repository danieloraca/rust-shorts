use reqwest::Client;
use std::env;
use std::error::Error;

use serde::{Deserialize, Serialize};

#[derive(Serialize)]
struct SlackMessage {
    channel: String,
    text: String,
}

#[derive(Serialize)]
struct User {
    id: String,
    name: String,
}

#[derive(Serialize)]
struct Users {
    users: Vec<User>,
}

#[derive(Serialize)]
struct Channel {
    id: String,
    name: String,
}

#[derive(Serialize)]
struct Channels {
    channels: Vec<Channel>,
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

    if channel_list.ok {
        let mut channel_names = Channels { channels: vec![] };
        let channels = channel_list.channels;
        for channel in channels {
            let channel = Channel {
                id: channel.id,
                name: channel.name,
            };

            channel_names.channels.push(channel);
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

    if user_list.ok {
        let mut users = Users { users: vec![] };
        for user in list {
            let user = User {
                id: user.id,
                name: user.name,
            };

            users.users.push(user);
        }

        let serialized = serde_json::to_string_pretty(&users).unwrap();
        Ok(serialized)
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
