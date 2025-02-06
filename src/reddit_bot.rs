use color_eyre::Result;
use roux::{Reddit, Subreddit};

pub async fn do_client() -> Result<roux::Me, roux::util::RouxError> {
    let client = Reddit::new()
        .username("OUHelperBot")
        .password("")
        .login()
        .await;
    client
}

pub async fn read_sub(sub: String) {
    let sub = Subreddit::new(&sub);
    let hot = sub.hot(25, None).await;
    dbg!(hot.unwrap());
}
