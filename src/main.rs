use std::env;
use resolver::{Config, resolve, is_npm_installed};

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = match Config::init(args) {
        Ok(cfg) => cfg,
        Err(e) => {
            eprintln!("Error initializing configuration: {}", e);
            return;
        }
    };

    resolve(&config);

}