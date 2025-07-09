use std::{sync::Arc, thread, time::Duration};

#[path = "redis_stream.rs"] mod redis_stream;

use concurrent_queue::ConcurrentQueue;
use serde_json::Value;
use std::collections::HashSet;

/// Market message type
pub type MarketMessage = String;

fn extract_asset(symbol: &str) -> Option<&str> {
    symbol.split(" - ").nth(2)
}

/// Dispatcher routes market messages to strategy queues
pub fn run(
    market_queue: Arc<ConcurrentQueue<MarketMessage>>,
    strategy_queues: Vec<Arc<ConcurrentQueue<MarketMessage>>>,
) {
    let group_0: HashSet<&'static str> = ["GGAL", "AL30"].into_iter().collect();
    let group_1: HashSet<&'static str> = ["YPFD", "AL30D"].into_iter().collect();
    loop {
        if let Ok(msg) = Arc::clone(&market_queue).pop() {
            println!("Dispatcher read: {}", msg);
            // Convert the data in usable format
            let data = redis_stream::format_msg("BYMA", &msg);
            println!("Dispatcher converted msg: {:?}", data);
            let symbol = &data.instrumentId.symbol;
            
            if let Some(asset) = extract_asset(symbol) {
                if group_0.contains(asset) {
                    if let Some(q) = strategy_queues.get(0) {
                        println!("PASTE LISTENER 0: {:?}", data);
                        let _ = q.push(msg.clone());
                    }
                    if let Some(q) = strategy_queues.get(2) {
                        println!("PASTE LISTENER 3: {:?}", data);
                        let _ = q.push(msg.clone());
                    }
                } else if group_1.contains(asset) {
                    if let Some(q) = strategy_queues.get(1) {
                        println!("PASTE LISTENER 1: {:?}", data);
                        let _ = q.push(msg.clone());
                    }
                    if let Some(q) = strategy_queues.get(2) {
                        println!("PASTE LISTENER 3: {:?}", data);
                        let _ = q.push(msg.clone());
                    }
                } else {
                    if let Some(q) = strategy_queues.get(2) {
                        println!("PASTE LISTENER 3: {:?}", data);
                        let _ = q.push(msg.clone());
                    }
                }
            }

            
        } else {
            // Backoff when queue is empty
            thread::sleep(Duration::from_nanos(10));
        }
    }
}

/* WS ROFEX DATA FORM:
Depth 1:
{"type":"Md","timestamp":1752031660863,"instrumentId":{"marketId":"ROFX","symbol":"MERV - XMEV - GGAL - CI"},"marketData":{"OF":[{"price":6320.00,"size":53962}],"BI":[{"price":6150.00,"size":4953}]}}
Depth 5:
{"type":"Md","timestamp":1752031774979,"instrumentId":{"marketId":"ROFX","symbol":"MERV - XMEV - GGAL - CI"},"marketData":{"OF":[{"price":6320.00,"size":53962},{"price":6330.00,"size":49194},{"price":6340.00,"size":47995},{"price":6350.00,"size":64137},{"price":6360.00,"size":79015}],"BI":[{"price":6150.00,"size":4953},{"price":6140.00,"size":12122},{"price":6130.00,"size":8857},{"price":6120.00,"size":9848},{"price":6110.00,"size":12924}]}}
*/
