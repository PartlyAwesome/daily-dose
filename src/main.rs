use clap::Parser;
use serenity::{
    all::{Context, EventHandler, GatewayIntents, Message, Ready},
    async_trait, Client,
};

/// A bot that posts a video daily
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Your discord token
    #[arg(required = true, short, long, env = "DD_TOKEN")]
    token: String,
}

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        if msg.content == "!ping" {
            if let Err(why) = msg.channel_id.say(&ctx.http, "Pong!").await {
                println!("Error sending message: {why:?}");
            }
        }
    }

    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected", ready.user.name);
    }
}

#[tokio::main]
async fn main() {
    let args = Args::parse();

    let intents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT;

    let mut client = Client::builder(&args.token, intents)
        .event_handler(Handler)
        .await
        .expect("Error creating client");

    if let Err(why) = client.start().await {
        println!("Client error: {why:?}");
    }
}
