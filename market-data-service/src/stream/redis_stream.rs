use serde::{Deserialize, Serialize};
use redis::Connection;
use redis::Commands;

#[derive(Serialize, Deserialize, Debug)]
struct Price {
    price: f32,
    size: f32,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
enum Book {
    Vec(Vec<Price>),
    Null,
    String(String),
}

#[derive(Serialize, Deserialize, Debug)]
pub struct InstrumentId {
    marketId: String,
    pub symbol: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct MarketData {
    BI: Book,
    OF: Book,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Data {
    r#type: String,
    timestamp: i64,
    pub instrumentId: InstrumentId,
    marketData: MarketData,
}

fn get_settle_arbitrage_data(data: &Book) -> [f32;2] {

    // Checking if marketData.BI is empty
    if let Book::Vec(book_data) = &data {
        if !book_data.is_empty() {
            for price_data in book_data {
                return [price_data.size, price_data.price];
            }
        } 
    }

    return [0.0, 0.0];

}

fn create_code(symbol:&String) -> String {
    let parts:Vec<&str> = symbol.split('-').collect();
    if parts[parts.len()-1] == " CI" {
        return format!("{}{}", parts[2].replace(" ", ""), "0");
    } else if parts[parts.len()-1] == " 48hs" {
        return format!("{}{}", parts[2].replace(" ", ""), "2");    
    } else if parts[parts.len()-1] == " 24hs" {
        return format!("{}{}", parts[2].replace(" ", ""), "1");
    } else {
        return format!("{}{}", parts[2].replace(" ", ""), parts[3].replace(" ", ""));
    }
}

pub fn paste_in_streams(conn:&mut Connection, code: &String, bid: &[f32;2], offer: &[f32;2]) -> String {
    
    let s: String = conn.xadd(code, "*", &[
        ("bis", &bid[0]),
        ("bip", &bid[1]), 
        ("ofs", &offer[0]),
        ("ofp", &offer[1])
        ]).unwrap();
    
        return s;
}

fn paste_settle_arbitrage(conn:&mut Connection, data: &Data) {

    let bid:[f32;2]  = get_settle_arbitrage_data(&data.marketData.BI);
    let offer:[f32;2]  = get_settle_arbitrage_data(&data.marketData.OF);
    let code:String = create_code(&data.instrumentId.symbol);// ticker+settle

    paste_in_streams(conn, &code, &bid, &offer);

}

pub fn format_msg(source: &str, msg: &String) -> Data {
    return serde_json::from_str(&msg).unwrap();
}

pub fn stream_data<'a>(conn:&mut Connection, source: &str, strategy:&'a str, msg: &String) {

    let data: Data = format_msg(&source, &msg);

    if strategy == "SA" {
        paste_settle_arbitrage(conn, &data);
    }

}