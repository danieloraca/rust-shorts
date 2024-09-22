use dialoguer::{theme::ColorfulTheme, Select};

use crate::slack_talk;

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
                match slack_talk::get_slack_channels().await {
                    Ok(result) => println!("Result: {}", result),
                    Err(e) => println!("Error: {:?}", e),
                }
            }
            1 => {
                println!("Option 1!");
                match slack_talk::get_slack_users().await {
                    Ok(result) => println!("Result: {}", result),
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

                match slack_talk::send_message(user_id, message_text).await {
                    Ok(result) => println!("Result: {}", result),
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
