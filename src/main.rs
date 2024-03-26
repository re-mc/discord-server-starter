use std::process::Command;

use serenity::async_trait;
use serenity::model::channel::Message;
use serenity::prelude::*;

use process_alive;

const TOKEN: &str = "[INSERT BOT TOKEN]";

static mut PID: u32 = 0;

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        if msg.content == "!start_server" {
            unsafe {
                let old_pid = process_alive::Pid::from(PID);
                if process_alive::state(old_pid).is_alive() {
                    let pid = Command::new("java")
                    .args([
                        "-jar", "purpur.jar", "--nogui"
                    ]).spawn().expect("Failed to Spawn Process").id();
                PID = pid;
                }
            }
        }
    }
}

#[tokio::main]
async fn main() {
    // Login with a bot token from the environment
    //let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");
    let token = TOKEN;
    // Set gateway intents, which decides what events the bot will be notified about
    let intents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT;

    // Create a new instance of the Client, logging in as a bot.
    let mut client =
        Client::builder(&token, intents).event_handler(Handler).await.expect("Err creating client");

    // Start listening for events by starting a single shard
    if let Err(why) = client.start().await {
        println!("Client error: {why:?}");
    }
}
