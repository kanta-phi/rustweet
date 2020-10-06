extern crate reqwest;
use reqwest::Client;
use serde::Deserialize;

// user name
const SCREEN_NAME: &str = "Kanta_phi";

#[derive(Deserialize, Debug)]
struct Tweet {
    id: u64,
    created_at: String,
    text: String,
    retweet_count: u32,
    favorite_count: u32,
    user: User,
    entities: Entity,
}

#[derive(Deserialize, Debug)]
struct User {
    id: u64,
    name: String,
    screen_name: String,
}

#[derive(Deserialize, Debug)]
struct Entity {
    hashtags: Vec<Hashtag>,
    user_mentions: Vec<UserMention>,
}

#[derive(Deserialize, Debug)]
struct Hashtag {
    text: String,
}

#[derive(Deserialize, Debug)]
struct UserMention {
    id: u64,
    name: String,
    screen_name: String,
}

#[tokio::main]
async fn main() {
    let url = format!("https://api.twitter.com/1.1/statuses/user_timeline.json?screen_name={}", SCREEN_NAME);
    let token = std::env::var("TOKEN").unwrap();
    let token = format!("Bearer {}", token);
    let client = Client::new().get(&url).header("authorization", token);
    let res = client.send().await.unwrap().json::<Vec<Tweet>>().await.unwrap();
    println!("{:#?}", res);
}
