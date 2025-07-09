use commons::cache::build_json_subscription_from_cache;

pub fn subscription_message(source: &str) -> String {
       
    let begin:&str = r#"{
        "type":"smd",
        "level":1,
        "entries":[
        "OF","BI"
        ],
        "products":"#;

    // Gen subscription
    let subscription:String = build_json_subscription_from_cache();    

    let end:&str = r#",
    "depth":1
    }"#;

    println!("{}",format!("{}{}{}", begin, subscription, end));
    format!("{}{}{}", begin, subscription, end)
}