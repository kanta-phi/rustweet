extern crate reqwest;
use reqwest::Client;
use serde::Deserialize;
use url::Url;

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
    retweeted_status: Box<Option<Tweet>>,
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

use anyhow::Result;

async fn get_tweets(max_id: Option<u64>) -> Result<Vec<Tweet>> {
    let mut params = vec![
        ("screen_name", SCREEN_NAME),
        ("count", "20"),
    ];
    let id;
    if let Some(max_id) = max_id {
        id = max_id.to_string();
        params.push(("max_id", &id));
    }

    let url = Url::parse_with_params("https://api.twitter.com/1.1/statuses/user_timeline.json", params)?;
    let token = std::env::var("TOKEN")?;
    let token = format!("Bearer {}", token);
    let client = Client::new().get(url).header("authorization", token);
    let tweets = client.send().await?.json::<Vec<Tweet>>().await?;
    Ok(tweets)
}

#[tokio::main]
async fn main() {
    let mut last_id = None;
    for n in 0..5 {
        match get_tweets(last_id).await {
            Ok(tweets) => {
                if tweets.len() == 0 {
                    break;
                }
                println!("=====================================");
                println!("page: {}", n+1);
                println!("=====================================");
                println!("{:#?}", tweets);
                last_id = Some(tweets.last().unwrap().id-1);
            },
            Err(e) => {
                eprintln!("{}", e);
                break;
            }
        }
    }
}
