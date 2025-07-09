use std::thread;
use std::time::Duration;
use commons::cache::connect_redis;

use std::sync::Arc;
use concurrent_queue::ConcurrentQueue;
use crate::config::Config;
#[path = "redis_stream.rs"] mod redis_stream;

fn run_settle_arbitrage() {

    let mut conn = connect_redis();

    // Structure
    // bid[size, price]
    // offer[size, price]
    redis_stream::paste_in_streams(&mut conn, &"PESOS1D".to_string(), &[10000.0, 75.0] , &[0.0, 0.0]);
    //println!("Pasted");
    thread::sleep(Duration::from_micros(1));
    redis_stream::paste_in_streams(&mut conn, &"DOLAR1D".to_string(), &[1000.0, 2.5] , &[0.0, 0.0]);
    //println!("Pasted");
    thread::sleep(Duration::from_micros(1));
    redis_stream::paste_in_streams(&mut conn, &"PESOS1D".to_string(), &[0.0, 0.0] , &[5000.0, 76.0]);
    //println!("Pasted");
    thread::sleep(Duration::from_micros(1));
    redis_stream::paste_in_streams(&mut conn, &"PESOS1D".to_string(), &[10000.0, 75.0] , &[5000.0, 76.0]);
    //println!("Pasted");
    thread::sleep(Duration::from_micros(1));
    redis_stream::paste_in_streams(&mut conn, &"PESOS1D".to_string(), &[20000.0, 75.5] , &[5000.0, 76.0]);
    //println!("Pasted");
    thread::sleep(Duration::from_micros(1));
    redis_stream::paste_in_streams(&mut conn, &"DOLAR1D".to_string(), &[1000.0, 2.5] , &[500.0, 2.6]);
    //println!("Pasted");
}

pub fn start_stream<'a>(market_queue: Arc<ConcurrentQueue<String>>, strategy:&'a str) {
    
    if strategy == "SA" {
        run_settle_arbitrage();
    }
}