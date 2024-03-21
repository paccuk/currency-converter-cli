use std::env;

pub trait ConfigApi {
    fn get_key(&self) -> String;
    fn get_url(&self) -> String;
    fn load_env() -> Result<Config, String>;
}
pub struct Config {
    api_key: String,
    api_url: String,
}

impl ConfigApi for Config {
    fn load_env() -> Result<Config, String> {
        dotenv::dotenv().ok();

        let api_key = match env::var("API_KEY") {
            Ok(val) => val,
            Err(_) => return Err("API_KEY is not set".to_string()),
        };

        let api_url = match env::var("API_URL") {
            Ok(val) => val,
            Err(_) => return Err("API_URL is not set".to_string()),
        };

        Ok(Config {
            api_key: api_key,
            api_url: api_url,
        })
    }

    fn get_key(&self) -> String {
        self.api_key.clone()
    }

    fn get_url(&self) -> String {
        self.api_url.clone()
    }
}
