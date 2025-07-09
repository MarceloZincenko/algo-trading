use super::Config;

pub fn config<'a>() -> Config<'a> {
    Config {
        market: "BYMA-ROFEX",
        protocol: "FIX",
        url: "fix.remarkets.primary.com.ar:9876",
        user: "marcelozin21430",
        pwd: "yjhbiM1$",
        account: "REM21430",
    }
}