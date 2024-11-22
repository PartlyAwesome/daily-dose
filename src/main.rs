use chrono::{Duration, NaiveDateTime, NaiveTime, Utc};
use clap::Parser;
use poise::{serenity_prelude as serenity, CreateReply};
use rand::Rng;
use serenity::all::CreateAttachment;
use serenity::all::{ChannelId, Http};
use serenity::{all::GatewayIntents, Client};

/// A bot that posts a video daily
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Your discord token
    #[arg(required = true, short, long, env = "DD_TOKEN")]
    token: String,
}

struct Data {} // User data
type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

/// Deploys a daily dose!
#[poise::command(slash_command, prefix_command)]
async fn daily_dose(ctx: Context<'_>) -> Result<(), Error> {
    let video_filename = "dailydose.mp4";
    let builder = CreateReply::default()
        .attachment(CreateAttachment::path("./".to_string() + video_filename).await?);
    ctx.send(builder).await?;
    Ok(())
}

/// Kill the president
#[poise::command(slash_command, prefix_command)]
async fn kill_the_president(ctx: Context<'_>) -> Result<(), Error> {
    let png_filename = "kill.png";
    let builder = CreateReply::default()
        .attachment(CreateAttachment::path("./".to_string() + png_filename).await?);
    ctx.send(builder).await?;
    Ok(())
}

/// Start posting randomly
#[poise::command(slash_command, prefix_command)]
async fn random_post(ctx: Context<'_>) -> Result<(), Error> {
    let token = ctx.http().token().to_string();
    let channel_id = ctx.channel_id().get();
    let png_filename = "kill.png";
    let builder = CreateReply::default()
        .attachment(CreateAttachment::path("./".to_string() + png_filename).await?);
    ctx.send(builder).await?;
    tokio::spawn(async move {
        let next_time = gen_instant_between(
            tokio::time::Instant::now() + tokio::time::Duration::from_secs(5),
            tokio::time::Instant::now() + tokio::time::Duration::from_secs(15),
        );
        tokio::time::sleep_until(next_time).await;
        post_in_channel(token.to_owned(), channel_id).await;
    });
    Ok(())
}

async fn post_in_channel(token: String, channel_id: u64) {
    println!("starting post_in_channel");
    let _builder = CreateReply::default().content("test");
    let msg = ChannelId::new(channel_id)
        .say(Http::new(&token), "test")
        .await;
    if let Err(why) = msg {
        println!("Err: {why:?}")
    }
}

fn gen_instant_between(
    start: tokio::time::Instant,
    end: tokio::time::Instant,
) -> tokio::time::Instant {
    let sec: i64 = (end - start).as_secs().try_into().expect("Time overflow");
    let rand_sec: i64 = rand::thread_rng().gen_range(0..sec);
    tokio::time::Instant::now()
        + Duration::seconds(rand_sec)
            .to_std()
            .expect("It won't break")
}

#[tokio::main]
async fn main() {
    let args = Args::parse();

    let intents = GatewayIntents::non_privileged();

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: vec![kill_the_president(), daily_dose(), random_post()],
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
