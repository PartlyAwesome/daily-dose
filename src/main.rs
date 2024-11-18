use clap::Parser;

/// A bot that posts a video daily
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Your discord token
    #[arg(required = true, short, long, env = "DD_TOKEN")]
    token: String,
}

#[tokio::main]
async fn main() {
    let _args = Args::parse();
    println!("we here");
}
