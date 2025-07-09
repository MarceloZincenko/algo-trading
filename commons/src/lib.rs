pub mod cache;
use concurrent_queue::ConcurrentQueue;
use std::sync::Arc;
use std::env;
use dotenv::dotenv;
use once_cell::sync::Lazy;

/* REDIS URL */
// All the config of strategies goes in readis
pub static REDIS_URL: Lazy<String> = Lazy::new(|| {
    dotenv().ok(); 
    env::var("CACHE_URL").expect("CACHE_URL must be set")
});

/* QUEUES */
pub static QUEUE_SIZE: usize = 1000;
pub type MarketMessage = String;

// Shared Market queue
pub static MARKET_QUEUE: Lazy<Arc<ConcurrentQueue<MarketMessage>>> =
    Lazy::new(|| Arc::new(ConcurrentQueue::bounded(QUEUE_SIZE)));

// Shared listener queues to be read by each strategy
pub static STRATEGY_QUEUES: Lazy<Vec<Arc<ConcurrentQueue<MarketMessage>>>> = Lazy::new(|| {
    vec![
        Arc::new(ConcurrentQueue::bounded(QUEUE_SIZE)), // Paste messages in Dynamodb
        Arc::new(ConcurrentQueue::bounded(QUEUE_SIZE)), // Settle arbitrage ARS
        Arc::new(ConcurrentQueue::bounded(QUEUE_SIZE)), // Settle arbitrage USM
    ]
});

/* TRACKING TICKERS FROM BYMA AND ROFEX */
// Traking tickers: All the tickers that we are going to use in the diferent strategies
pub static TRAKING_TICKERS_KEY: Lazy<&str> = Lazy::new(|| {
    "traking_byma_tickers" // typo: "traking" -> "tracking"
});
pub static TRAKING_TICKERS: Lazy<Vec<&str>> = Lazy::new(|| {
    vec!["GGAL", "GGALD", "YPFD", "AL30", "GD30"]
});

// Cauciones
pub static TRAKING_CAUCIONES_KEY: Lazy<&str> = Lazy::new(|| {
    "traking_cauciones"
});
pub static TRAKING_CAUCIONES: Lazy<Vec<&str>> = Lazy::new(|| {
    vec!["PESOS - 6D", "PESOS - 8D"]
});

/*
MERV - XMEV - AL41 - CI  
MERV - XMEV - GGALD - 24hs  
MERV - XMEV - GGALD - CI  
MERV - XMEV - GD41 - CI  
MERV - XMEV - COME - CI  
MERV - XMEV - KO - CI  
MERV - XMEV - SPY - CI  
MERV - XMEV - AL30D - 24hs  
MERV - XMEV - GD30 - 24hs  
MERV - XMEV - XD30 - 24hs  
MERV - XMEV - GGAL - 24hs  
MERV - XMEV - DIA - CI  
MERV - XMEV - GD41 - 24hs  
MERV - XMEV - AAPL - CI  
MERV - XMEV - GGALC - 24hs  
MERV - XMEV - AL29D - CI  
MERV - XMEV - AL30 - CI  
MERV - XMEV - BBD - 24hs  
MERV - XMEV - PESOS - 8D  
MERV - XMEV - PESOS - 6D  
MERV - XMEV - AL41D - CI  
MERV - XMEV - AL35 - 24hs  
MERV - XMEV - AL29 - CI  
MERV - XMEV - GGAL - CI  
MERV - XMEV - AAPLD - CI  
MERV - XMEV - AL30 - 24hs  
MERV - XMEV - AL35 - CI  
MERV - XMEV - COME - 24hs  
MERV - XMEV - BBD - CI  
MERV - XMEV - AL35D - CI  
MERV - XMEV - KO - 24hs  
MERV - XMEV - AL30D - CI  
MERV - XMEV - DIAD - CI  
MERV - XMEV - GD30 - CI  
MERV - XMEV - YPFD - CI  
MERV - XMEV - SPYD - CI  
MERV - XMEV - YPFD - 24hs  
MERV - XMEV - GGALC - CI 
*/

/* SETTLE ARBITRAGE STRATEGY SET UP */

// Settle arbitrage ARS strategy tickers
pub static SETTLE_ARBITRAGE_TICKERS_KEY: Lazy<&str> = Lazy::new(|| {
    // Could be loaded from env, file, etc.
    "settle_arbitrage_tickers"
});

pub static SETTLE_ARBITRAGE_TICKERS: Lazy<Vec<&str>> = Lazy::new(|| {
    // Could be loaded from env, file, etc.
    vec!["GGAL-ARS", "GGALD-USM"]
});


/* STEPS TO ADD A STRATEGY:

1- Create a listener

*/