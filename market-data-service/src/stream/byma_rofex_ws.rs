use tungstenite::handshake::client::Request;
use tungstenite::connect;
use tungstenite::Message;
use url::Url;
use crate::Duration;
use std::sync::Arc;
use concurrent_queue::ConcurrentQueue;
use crate::config::Config;
use std::thread;

#[path = "redis_stream.rs"] mod redis_stream;
#[path = "market_subscription.rs"] mod market_subscription;

pub fn get_token<'a>(user:&'a str, password:&'a str) -> String {
    let client = reqwest::blocking::Client::new();
    
    // Post method to the API to get auth token
    let response = client
    .post("https://api.remarkets.primary.com.ar/auth/getToken")
    .header("X-Username", user)
    .header("X-Password", password)
    .send();

    // Take token from response
    let token = response.expect("No correct response from API")
    .headers()
    .get("X-Auth-Token")
    .expect("X-Auth-Token header not found")
    .to_str()
    .expect("Failed to convert header value to string")
    .to_string();
   
    return token
}

pub fn start_stream<'a>(market_queue: Arc<ConcurrentQueue<String>>, token:&String, url:&'a str) {
    
    // Source
    let source:&str = "WS_BYMA";

    //Parse the url
    let parsed_url = Url::parse(url).unwrap();
    
    // Prepare the request to connect
    let request = Request::builder()
        .method("GET")
        .header("Host", "api.remarkets.primary.com.ar")
        .header("Origin", "api.remarkets.primary.com.ar")
        .header("Connection", "Upgrade")
        .header("Upgrade", "websocket")
        .header("Sec-WebSocket-Version", "13")
        .header("Sec-WebSocket-Key", tungstenite::handshake::client::generate_key())
        .header("X-Auth-Token", token.as_str())
        .uri(parsed_url.as_str())
        .body(())
        .unwrap();

    loop {
        match connect(request.clone()) {
            Ok((mut socket, _)) => {
                println!("[WS] Connected.");

                // Subscribe once after successful connection
                let suscription: &str = &market_subscription::subscription_message(source);
                let _ = socket.write_message(Message::Text(suscription.into()));

                // Read messages
                while let Ok(msg) = socket.read_message() {
                    if msg.is_text() {
                        let payload = msg.into_text().unwrap();

                        // Push message to input queue (drop if full)
                        if market_queue.push(payload.clone()).is_err() {
                            eprintln!("[WS] Input queue full, dropped message");
                        } else {
                            println!("{}", payload);
                        }
                    }
                }

                eprintln!("[WS] Disconnected, reconnecting...");
            }
            Err(e) => {
                eprintln!("[WS] Connection error: {e}, retrying in 2s...");
            }
        }

        // Wait before retrying
        thread::sleep(Duration::from_secs(2));
    }
}
   