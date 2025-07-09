mod config;
mod stream;

use commons::{MARKET_QUEUE, STRATEGY_QUEUES};

use std::{sync::Arc, thread, time::Duration};
use tungstenite::{connect, Message};
use url::Url;
use concurrent_queue::ConcurrentQueue;
use std::env;
use dotenv::dotenv;

fn main() {
    // Loads variables from .env into the environment    
    dotenv().ok(); 
    
    // Get the data source
    let market_data_source: String = env::var("MARKET_DATA_SOURCE").expect("Source not set");

    match config::get_config(&market_data_source) {
        Some(cfg) => {
            // Start streaming
            // Spawn WebSocket reader thread
            {
                let market_queue = Arc::clone(&MARKET_QUEUE);
                thread::spawn(move || stream::run(market_queue, cfg));
            }

            // Spawn dispatcher thread
            {
                let market_queue = Arc::clone(&MARKET_QUEUE);
                let strategy_queues: Vec<_> = STRATEGY_QUEUES.iter().map(Arc::clone).collect();
                thread::spawn(move || stream::dispatcher::run(market_queue, strategy_queues));
            }
            
            /*
            // Spawn listener threads
            for (i, q) in listener_queues.into_iter().enumerate() {
                thread::spawn(move || listener(i + 1, q));
            }
            */
            
            // Main thread just waits
            loop {
                thread::park();
            }
    
        }
        None => {
            eprintln!("Unknown config name: {}", market_data_source);
        }
    }

}