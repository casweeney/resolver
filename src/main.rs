use std::env;
use resolver::{Config, resolve};

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