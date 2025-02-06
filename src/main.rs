use ::tokio;
mod reddit_bot;

#[tokio::main]
async fn main() {
    // let client = reddit_bot::do_client()
    let sub = reddit_bot::read_sub("openuniversity".into()).await;
}
