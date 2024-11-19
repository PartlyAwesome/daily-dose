use ::serenity::framework;
use clap::Parser;
use poise::serenity_prelude as serenity;
use serenity::{all::GatewayIntents, Client, User};

/// A bot that posts a video daily
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Your discord token
    #[arg(required = true, short, long, env = "DD_TOKEN")]
    token: String,
}

struct Handler;

struct Data {} // User data
type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

/// Displays the account's discord age
#[poise::command(slash_command, prefix_command)]
async fn age(ctx: Context<'_>, #[description = "user"] user: Option<User>) -> Result<(), Error> {
    let u = user.as_ref().unwrap_or_else(|| ctx.author());
    let resp = format!("{}'s account was created at {}", u.name, u.created_at());
    ctx.say(resp).await?;
    Ok(())
}

//#[async_trait]
//impl EventHandler for Handler {
//    async fn message(&self, ctx: Context, msg: Message) {
//        if msg.content == "!ping" {
//            if let Err(why) = msg.channel_id.say(&ctx.http, "Pong!").await {
//                println!("Error sending message: {why:?}");
//            }
//        }
//    }
//
//    async fn ready(&self, _: Context, ready: Ready) {
//        println!("{} is connected", ready.user.name);
//    }
//}

#[tokio::main]
async fn main() {
    let args = Args::parse();

    let intents = GatewayIntents::non_privileged();

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: vec![age()],
            ..Default::default()
        })
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                Ok(Data {})
            })
        })
        .build();

    let client = Client::builder(&args.token, intents)
        .framework(framework)
        .await;

    if let Err(why) = client.expect("huh").start().await {
        println!("Client error: {why:?}");
    }
}
