mod api;
mod config;
mod currency_converter;
mod parser;

use config::{Config, ConfigApi};
use currency_converter::client_code;

fn main() {
    let config = match Config::load_env() {
        Ok(config) => config,
        Err(error) => {
            eprintln!("Failed to load environment variables: {}", error);
            std::process::exit(1);
        }
    };
    client_code(&config);
}
