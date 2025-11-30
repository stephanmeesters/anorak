use once_cell::sync::Lazy;
use std::env;

pub struct Config {
    pub port: u16,
    pub jackett_url : String,
    pub jackett_apikey: String,
    pub transmission_url: String,
    #[allow(dead_code)]
    pub transmission_username: Option<String>,
    #[allow(dead_code)]
    pub transmission_password: Option<String>,
}

pub static CONFIG: Lazy<Config> = Lazy::new(|| {
    Config { 
        port: 9341,
        jackett_url: env::var("JACKETT_URL").expect("Define the JACKETT_URL environment variable"),
        jackett_apikey: env::var("JACKETT_APIKEY").expect("Define the JACKETT_APIKEY environment variable"),
        transmission_url: env::var("TRANSMISSION_URL").expect("Define the TRANSMISSION_URL environment variable"),
        transmission_username: env::var("TRANSMISSION_USERNAME").ok(),
        transmission_password: env::var("TRANSMISSION_PASSWORD").ok(),
    }
});
