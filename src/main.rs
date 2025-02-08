use clap::Parser;
use tokio::signal;

mod reddit_bot;

#[derive(Parser, Debug)]
struct Opts {}

#[tokio::main]
async fn main() {
    // let client = reddit_bot::do_client()
    let sub = reddit_bot::read_sub("openuniversity".into()).await;
    println!("Application running. Press Ctrl+C to exit.");
    signal::ctrl_c().await.unwrap();
    println!("Shutting down gracefully...");
}
