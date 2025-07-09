mod byma_rofex_ws;
mod byma_rofex_fix;
mod emulator;
pub mod dispatcher;

use crate::config::Config;

use std::sync::Arc;
use concurrent_queue::ConcurrentQueue;

pub struct Environment {
    market: String,
    protocol: String,
    url: String,
    user: String,
    pwd: String,
    account: String,
    token: Option<String>,
}

impl Environment {
    pub fn build(config: Config) -> Self {
        let token = if config.market == "BYMA-ROFEX" && config.protocol == "WS" {
            Some(byma_rofex_ws::get_token(&config.user, &config.pwd))
        } else {
            None
        };

        Environment {
            market: config.market.to_string(),
            protocol: config.protocol.to_string(),
            url: config.url.to_string(),
            user: config.user.to_string(),
            pwd: config.pwd.to_string(),
            account: config.account.to_string(),
            token,
        }
    }

    pub fn stream_data(&self, market_queue: Arc<ConcurrentQueue<String>>) {
        match (self.market.as_str(), self.protocol.as_str()) {
            ("BYMA-ROFEX", "WS") => {
                let token = self.token.as_ref().expect("token missing");
                byma_rofex_ws::start_stream(market_queue, token, &self.url);
            }
            //("BYMA-ROFEX", "FIX") => {
            //    byma_rofex_fix::start_stream(&self.user, &self.pwd);
            //}
            ("BYMA-Emulator", _) => {
                emulator::start_stream(market_queue, &self.user);
            }
            other => {
                panic!("Unsupported environment: {:?}", other);
            }
        }
    }
}

pub fn run(market_queue: Arc<ConcurrentQueue<String>>, config: Config) {
    let env = Environment::build(config);
    env.stream_data(market_queue);
}