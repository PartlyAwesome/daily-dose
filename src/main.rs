use chrono_tz::Europe::London;
use chrono_tz::US::Pacific;
use clap::Parser;
use itertools::Itertools;
use poise::{serenity_prelude as serenity, CreateReply};
use rand::Rng;
use serde::{Deserialize, Serialize};
use serenity::all::CacheHttp;
use serenity::all::ChannelId;
use serenity::all::CreateAttachment;
use serenity::{all::GatewayIntents, Client};
use std::sync::Arc;
use tokio::time::{Duration, Instant};

static CONFIG_FILE: &str = "config.toml";

/// A bot that posts a video daily
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Your discord token
    #[arg(required = true, short, long, env = "DD_TOKEN")]
    token: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct Config {
    channels: Vec<u64>,
}

//struct Data {} // User data
type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Config, Error>;

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

#[poise::command(slash_command, prefix_command)]
async fn record_channel_to_file(ctx: Context<'_>) -> Result<(), Error> {
    let mut config: Config = read_from_file(CONFIG_FILE).expect("config TOML invalid");
    config.channels.push(ctx.channel_id().get());
    config.channels = config.channels.into_iter().unique().collect_vec();
    write_to_file(CONFIG_FILE, config).expect("unable to write config TOML to file");
    ctx.say("done").await?;
    Ok(())
}

fn read_from_file<T>(filename: &str) -> Result<T, toml::de::Error>
where
    for<'de> T: Deserialize<'de>,
{
    let serialised = std::fs::read_to_string(filename).unwrap();
    toml::from_str(&serialised)
}

fn write_to_file<T: Serialize>(filename: &str, object: T) -> std::io::Result<()> {
    let serialised = toml::to_string(&object).unwrap();
    std::fs::write(filename, serialised)
}

#[poise::command(slash_command, prefix_command)]
async fn initialise_file(ctx: Context<'_>) -> Result<(), Error> {
    //let channels_file = std::fs::read_to_string("channels.toml").unwrap();
    //let mut channels: Vec<String> = toml::from_str(&channels_file).expect("channels TOML invalid");
    //channels.push(ctx.channel_id().get().to_string());
    //channels = channels.into_iter().unique().collect_vec();
    let config = Config {
        channels: Vec::new(),
    };
    write_to_file(CONFIG_FILE, config).expect("unable to write config TOML to file");
    ctx.say("done").await?;
    Ok(())
}

//struct Handler;

//#[async_trait]
//impl EventHandler for Handler {
//    async fn cache_ready(&self, ctx: poise::serenity_prelude::Context, _guilds: Vec<GuildId>) {
//        //let token = ctx.http.token().to_string();
//        //let channel_id = 135489943710466049;
//        //Box::pin(async move {
//        let ctx = Arc::new(&ctx);
//        let ctx_clone = ctx.clone();
//
//        tokio::spawn(queue_post(&ctx_clone));
//        //})
//    }
//}

/// Start posting randomly
//#[poise::command(slash_command, prefix_command)]
//async fn random_post(ctx: Context<'_>) -> Result<(), Error> {
//    let token = ctx.http().token().to_string();
//    let channel_id = ctx.channel_id().get();
//    //let png_filename = "kill.png";
//    //let builder = CreateReply::default()
//    //    .attachment(CreateAttachment::path("./".to_string() + png_filename).await?);
//    //ctx.send(builder).await?;
//    //tokio::spawn(async move {
//    //    let next_time = gen_instant_between(
//    //        Instant::now() + Duration::from_secs(5),
//    //        Instant::now() + Duration::from_secs(15),
//    //    );
//    //    tokio::time::sleep_until(next_time).await;
//    //    post_in_channel(token.to_owned(), channel_id).await;
//    //});
//    tokio::spawn(queue_post(token, channel_id));
//    Ok(())
//}

//async fn queue_post(token: String, channel_id: u64) {
//    loop {
//        let next_time = gen_instant_between(
//            Instant::now() + Duration::from_secs(120),
//            Instant::now() + Duration::from_secs(480),
//        );
//        tokio::time::sleep_until(next_time).await;
//        post_in_channel(token.clone(), channel_id).await;
//    }
//}

async fn queue_post(ctx: serenity::Context, config: &Config) {
    let ctx = Arc::new(ctx);
    loop {
        let channels = config.channels.clone();
        //let ctx = ctx.clone();
        for channel in channels {
            let channel_ctx = Arc::clone(&ctx);
            println!("{channel:?}");
            tokio::spawn(async move {
                let next_time = gen_instant_between(
                    Instant::now() + Duration::from_secs(120),
                    Instant::now() + Duration::from_secs(480),
                );
                tokio::time::sleep_until(next_time).await;
                post_in_channel(&channel_ctx, channel).await;
            });
        }
        tokio::time::sleep_until(Instant::now() + Duration::from_secs(500)).await;
    }
}

async fn post_in_channel(ctx: &serenity::Context, channel_id: u64) {
    println!("starting post_in_channel");
    let _builder = CreateReply::default().content("test");
    let msg = ChannelId::new(channel_id).say(ctx.http(), "test").await;
    if let Err(why) = msg {
        println!("Err: {why:?}")
    }
}

fn gen_instant_between(start: Instant, end: Instant) -> Instant {
    let sec = (end - start).as_secs();
    let rand_sec = rand::thread_rng().gen_range(0..sec);
    Instant::now() + Duration::from_secs(rand_sec)
}

async fn event_handler(
    ctx: serenity::Context,
    event: &serenity::FullEvent,
    _framework: poise::FrameworkContext<'_, Config, Error>,
    data: &Config,
) -> Result<(), Error> {
    match event {
        serenity::FullEvent::CacheReady { guilds: _ } => {
            println!("cache ready!");
            println!("{data:#?}");
            let config = Arc::new(data);
            //let config_clone = config.clone();
            //let ctx = Arc::new(ctx);
            //async move {
            queue_post(ctx, &config).await;
            //};
        }
        _ => {}
    }
    Ok(())
}

#[tokio::main]
async fn main() {
    let args = Args::parse();

    let intents = GatewayIntents::non_privileged();

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: vec![
                kill_the_president(),
                daily_dose(),
                record_channel_to_file(),
                initialise_file(),
            ],
            event_handler: |ctx, event, framework, data| {
                Box::pin(event_handler(ctx.clone(), event, framework, data))
            },
            ..Default::default()
        })
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                let config: Config = read_from_file(CONFIG_FILE).expect("config TOML invalid");

                Ok(config)
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
