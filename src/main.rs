extern crate reqwest;
use reqwest::Client;

// user name
const SCREEN_NAME: &str = "Kanta_phi";

#[tokio::main]
async fn main() {
    let url = format!("https://api.twitter.com/1.1/statuses/user_timeline.json?screen_name={}", SCREEN_NAME);
    let token = std::env::var("TOKEN").unwrap();
    let token = format!("Bearer {}", token);
    let client = Client::new().get(&url).header("authorization", token);
    let res = client.send().await.unwrap().text().await.unwrap();
    println!("{}", res);
}
