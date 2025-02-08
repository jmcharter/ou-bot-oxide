use color_eyre::Result;
use roux::{Reddit, Subreddit};

pub async fn get_client(
    user_agent: &str,
    client_id: &str,
    client_secret: &str,
    username: &str,
    password: &str,
) -> Result<roux::Me, roux::util::RouxError> {
    Reddit::new(user_agent, client_id, client_secret)
        .username(username)
        .password(password)
        .login()
        .await
}

pub async fn read_sub(sub: &str) {
    let sub = Subreddit::new(sub);
    let hot = sub.hot(25, None).await;
    dbg!(hot.unwrap());
}
