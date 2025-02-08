use clap::Parser;
use color_eyre::eyre::Result;
use dotenv::dotenv;
use tokio::signal;

mod reddit_bot;

#[derive(Parser, Debug)]
struct Opts {
    #[arg(long, env = "REDDIT_USERNAME")]
    reddit_username: String,
    #[arg(long, env = "REDDIT_PASSWORD")]
    reddit_password: String,
    #[arg(long, env = "REDDIT_SUBREDDIT")]
    reddit_subreddit: String,
    #[arg(long, env = "REDDIT_CLIENT_ID")]
    reddit_client_id: String,
    #[arg(long, env = "REDDIT_CLIENT_SECRET")]
    reddit_client_secret: String,
    #[arg(long, env = "REDDIT_USER_AGENT")]
    reddit_user_agent: String,
}

impl Opts {
    pub fn from_env_and_args() -> Self {
        dotenv().ok();
        Self::parse()
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;
    let args = Opts::from_env_and_args();
    reddit_bot::read_sub(&args.reddit_subreddit).await;
    println!("Application running. Press Ctrl+C to exit.");
    signal::ctrl_c().await.unwrap();
    println!("Shutting down gracefully...");
    Ok(())
}
