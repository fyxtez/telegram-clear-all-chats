use std::sync::Arc;

use crate::functions::{ConnectClientReturnType, connect_client, get_dialogs};
use dotenv::dotenv;
use futures::stream::{self, StreamExt};

mod config;
mod functions;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    let session_path = "session.session";

    let ConnectClientReturnType { client } = connect_client(session_path).await?;

    let client = Arc::new(client);

    let dialogs = get_dialogs(&client).await?;

    // Use bounded concurrency to mark dialogs as read.
    // `for_each_concurrent(10, ...)` allows up to 10 async requests to run in parallel,
    // which significantly improves throughput compared to a sequential loop,
    // while still applying backpressure so we don’t overwhelm the API or hit rate limits.
    // Provides predictable resource usage and safe parallelism.

    stream::iter(dialogs)
        .for_each_concurrent(10, |dialog| {
            let client = &client;
            async move {
                let _ = client.mark_as_read(&dialog.peer).await;
            }
        })
        .await;

    println!("All chats cleared.");

    Ok(())
}
