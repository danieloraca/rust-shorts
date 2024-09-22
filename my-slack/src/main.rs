mod menu;
mod slack_talk;

#[tokio::main]
async fn main() -> () {
    menu::show_menu().await;
}
