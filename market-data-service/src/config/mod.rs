pub mod primary_ws_dev;
pub mod primary_fix_dev;
pub mod primary_emulator;

pub struct Config<'a> {
    pub market: &'a str,
    pub protocol: &'a str,
    pub url: &'a str,
    pub user: &'a str,
    pub pwd: &'a str,
    pub account: &'a str,
}

// Factory function to get config by name
pub fn get_config<'a>(name: &str) -> Option<Config<'a>> {
    match name {
        "primary_ws_dev" => Some(primary_ws_dev::config()),
        "primary_fix_dev" => Some(primary_fix_dev::config()),
        "primary_emulator" => Some(primary_emulator::config()),
        _ => None,
    }
}