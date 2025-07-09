use redis::{Client, Commands, Connection, RedisResult};
use crate::REDIS_URL; 
use crate::TRAKING_CAUCIONES_KEY;
use crate::TRAKING_TICKERS_KEY;

fn get_redis_connection() -> Connection {
    let client = Client::open(REDIS_URL.as_str()).expect("Failed to connect to Redis");
    client.get_connection().expect("Failed to get Redis connection")
}

pub fn connect_redis() -> Connection {
    get_redis_connection()
}

pub fn clean() -> RedisResult<()> {
    let mut conn = connect_redis();
    redis::cmd("FLUSHALL").query(&mut conn)?;
    Ok(())
}

pub fn paste_str_vector_in_cache(key: &str, items: Vec<&str>) -> RedisResult<()> {
    let mut conn = connect_redis();
    for item in &items {
        conn.rpush::<_, _, ()>(key, item)?;
    }
    Ok(())
}

pub fn read_str_vector_from_cache(key: &str) -> RedisResult<Vec<String>> {
    let mut conn = connect_redis();
    conn.lrange(key, 0, -1)
}

pub fn build_json_subscription_from_cache() -> String {
    let mut conn = connect_redis();

    // Get tickers and cauciones from Redis
    let cauciones: Vec<String> = match conn.lrange(*TRAKING_CAUCIONES_KEY, 0, -1) {
        Ok(val) => val,
        Err(e) => {
            eprintln!("Redis error: {}", e);
            vec![] // or return, or default fallback
        }
    };
    let tickers: Vec<String> = match conn.lrange(*TRAKING_TICKERS_KEY, 0, -1) {
        Ok(val) => val,
        Err(e) => {
            eprintln!("Redis error: {}", e);
            vec![] // or return, or default fallback
        }
    };

    let mut products = Vec::new();
    // Add tickers
    for ticker in tickers {
        products.push(format!(
            r#"{{"symbol":"MERV - XMEV - {} - CI", "marketId":"ROFX"}}"#,
            ticker
        ));
        products.push(format!(
            r#"{{"symbol":"MERV - XMEV - {} - 24hs", "marketId":"ROFX"}}"#,
            ticker
        ));
    }

    // Add cauciones
    for caucion in cauciones {
        products.push(format!(
            r#"{{"symbol":"MERV - XMEV - {}", "marketId":"ROFX"}}"#,
            caucion
        ));
    }

    format!("[{}]", products.join(","))
}