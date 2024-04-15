use std::env;
use dotenvy::dotenv_override;

const EMPTY_STRING: &'static str = "";

/// Load envs from .env file
pub fn load() {
    dotenv_override().ok();
}

/// List all envs
pub fn ls() {
    for (key, value) in env::vars() {
        println!("{}: {}", key, value);
    }
}

/// Get env as string, returning "" if env not found
pub fn str(key: &str) -> String {
    match env::var(key) {
        Ok(val) => val,
        Err(_) => EMPTY_STRING.to_string(),
    }
}

/// Get env as u32 returning 0 if not found
pub fn u32(key: &str) -> u32 {
    match str(key).parse::<u32>() {
        Ok(val) => val,
        Err(_) => 0,
    }
}

pub fn i32(key: &str) -> i32 {
    match str(key).parse::<i32>() {
        Ok(val) => val,
        Err(_) => 0,
    }
}
