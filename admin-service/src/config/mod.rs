use commons::TRAKING_CAUCIONES_KEY;
use commons::TRAKING_TICKERS_KEY;
use commons::SETTLE_ARBITRAGE_TICKERS_KEY;
use commons::TRAKING_CAUCIONES;
use commons::TRAKING_TICKERS;
use commons::SETTLE_ARBITRAGE_TICKERS;
use commons::cache::paste_str_vector_in_cache;

pub fn paste_traking_tickers() {
    match paste_str_vector_in_cache(*TRAKING_CAUCIONES_KEY, TRAKING_CAUCIONES.clone()) {
        Ok(_) => println!("Cauciones saved successfully."),
        Err(err) => eprintln!("Error saving cauciones: {}", err),
    }
    match paste_str_vector_in_cache(*TRAKING_TICKERS_KEY, TRAKING_TICKERS.clone()) {
        Ok(_) => println!("Tickers saved successfully."),
        Err(err) => eprintln!("Error saving tickers: {}", err),
    }

}

pub fn paste_settle_arbitrage_tickers() {
        match paste_str_vector_in_cache(*SETTLE_ARBITRAGE_TICKERS_KEY, SETTLE_ARBITRAGE_TICKERS.clone()) {
        Ok(_) => println!("Settle arbitrage tickers saved successfully."),
        Err(err) => eprintln!("Error saving tickers: {}", err),
    }
}