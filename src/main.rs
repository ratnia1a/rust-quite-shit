use std::{env, sync::Arc};
use twilight_gateway::{Event, Intents, Shard, ShardId};
use twilight_http::Client as HttpClient;
use twilight_cache_inmemory::InMemoryCache;
use futures::StreamExt;
use tracing::{info, error};


#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    let token = env::var("DISCORD_BOT_TOKEN").expect("Expected a token in the environment");
    let http = HttpClient::new(token.clone());
    let cache = Arc::new(InMemoryCache::new());
    let intents = Intents::GUILD_MESSAGES | Intents::DIRECT_MESSAGES;
    let mut shard = Shard::new(ShardId::from(0u64), token, intents);
    shard.start().await.expect("Failed to start shard");
    while let Some(event) = shard.next().await {
        match event {
            Ok(Event::Ready { .. }) => {
                info!("Bot is ready!");
            }
            Ok(Event::MessageCreate(message)) => {
                info!("Received message: {}", message.content);
                // simple ping-pong command
                if message.content == "!ping" {
                    if let Err(e) = http.create_message(message.channel_id).content("Pong!").await {
                        error!("Failed to send message: {:?}", e);
                    }
                }
            }
            Err(e) => {
                error!("Error receiving event: {:?}", e);
            }
            _ => {}
        }
    }
}