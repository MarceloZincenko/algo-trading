mod config;

use commons::cache::clean;

fn main() {
    // Flush all the cache
    match clean() {
        Ok(_) => println!("Flushall Redis cache"),
        Err(err) => eprintln!("Error when erasing Redis cache: {}", err),
    }
                
    // Paste all the securities we are going to follow
    config::paste_traking_tickers();

    // Paste all the tickers to apply settle arbitrage strategy
    config::paste_settle_arbitrage_tickers();
}
