use super::Config;

pub fn config<'a>() -> Config<'a> {
    Config {
        market: "BYMA-Emulator",
        protocol: "",
        url: "",
        user: "",
        pwd: "",
        account: "",
    }
}