use halowabot::functionality::magic::Handler;
use serenity::Client;
use std::env;
use tokio::*;
#[tokio::main]
async fn main() {
    let token = env::var("TOKEN").expect("Expected a token in the environment");

    let mut client = Client::builder(&token)
        .event_handler(Handler)
        .await
        .expect("Err creating client");

    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}
