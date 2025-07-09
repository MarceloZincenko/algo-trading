use super::Config;

pub fn config<'a>() -> Config<'a> {
    Config {
        market: "BYMA-ROFEX",
        protocol: "WS",
        url: "wss://api.remarkets.primary.com.ar",
        user: "marcelozin21430",
        pwd: "yjhbiM1$",
        account: "REM21430",
    }
}