use std::default;

use ::serenity::all::{CreateEmbed, CreateEmbedFooter, Embed, EmbedVideo, Timestamp};
use clap::Parser;
use poise::{serenity_prelude as serenity, CreateReply};
use serenity::{all::GatewayIntents, Client, User};

/// A bot that posts a video daily
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Your discord token
    #[arg(required = true, short, long, env = "DD_TOKEN")]
    token: String,
}

//struct Handler;

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

/// Deploys a daily dose!
#[poise::command(slash_command, prefix_command)]
async fn daily_dose(ctx: Context<'_>) -> Result<(), Error> {
    let embed = CreateEmbed::new()
        .title("DAILY DOSE")
        .description("a daily dose")
        .fields(vec![
            ("First", "First body", true),
            ("Second", "Second body", true),
        ])
        .field("Third", "Third body", false)
        .footer(CreateEmbedFooter::new("footer"))
        .timestamp(Timestamp::now());
    let mut real_embed = Embed::default();
    real_embed.title = Some("DAILY DOSE".to_string());
    real_embed.kind = Some("video".to_string());
    let embed_video = EmbedVideo {
        url: "a".to_string(),
        proxy_url: None,
        height: None,
        width: None,
    };
    //let blah = EmbedVideo();
    //real_embed.video = Some();
    let builder = CreateReply::default().content("Test").embed(embed);
    ctx.send(builder).await?;
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
            commands: vec![age(), daily_dose()],
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
