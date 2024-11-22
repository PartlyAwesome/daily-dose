use chrono::{Duration, NaiveDateTime, NaiveTime, Utc};
use clap::Parser;
use clokwerk::{AsyncScheduler, Job, TimeUnits};
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

//struct Handler;

struct Data {} // User data
type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

/// Displays the account's discord age
//#[poise::command(slash_command, prefix_command)]
//async fn age(ctx: Context<'_>, #[description = "user"] user: Option<User>) -> Result<(), Error> {
//    let u = user.as_ref().unwrap_or_else(|| ctx.author());
//    let resp = format!("{}'s account was created at {}", u.name, u.created_at());
//    ctx.say(resp).await?;
//    Ok(())
//}

/// Deploys a daily dose!
#[poise::command(slash_command, prefix_command)]
async fn daily_dose(ctx: Context<'_>) -> Result<(), Error> {
    let video_filename = "dailydose.mp4";
    //let embed = CreateEmbed::new()
    //    .title("DAILY DOSE")
    //    //.attachment(video_filename)
    //    .footer(CreateEmbedFooter::new("dailydose.mp4"))
    //    .timestamp(Timestamp::now());
    let builder = CreateReply::default()
        //.embed(embed)
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
    let mut scheduler = AsyncScheduler::new();
    let token = ctx.http().token().to_string();
    let channel_id = ctx.channel_id().get();
    let next_time = generate_time_between(
        Utc::now().naive_local() + Duration::seconds(5),
        Utc::now().naive_local() + Duration::seconds(15),
    );
    println!("{next_time:?}");
    scheduler
        .every(1.day())
        .at_time(next_time)
        .run(move || {
            println!("running?");
            post_in_channel(token.to_owned(), channel_id)
        })
        .once();
    let png_filename = "kill.png";
    let builder = CreateReply::default()
        .attachment(CreateAttachment::path("./".to_string() + png_filename).await?);
    ctx.send(builder).await?;
    tokio::spawn(async move {
        loop {
            println!("spawn?");
            scheduler.run_pending().await;
            tokio::time::sleep(
                Duration::milliseconds(100)
                    .to_std()
                    .expect("It shouldn't break"),
            )
            .await;
        }
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

fn generate_time_between(start: NaiveDateTime, end: NaiveDateTime) -> NaiveTime {
    let seconds_in_range = (end - start).num_seconds();
    let random_seconds: i64 = rand::thread_rng().gen_range(0..seconds_in_range);
    let next_time = start + Duration::seconds(random_seconds);
    next_time.time()
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
