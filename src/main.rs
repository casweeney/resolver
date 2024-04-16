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

    if let Err(e) = resolve(&config) {
        eprintln!("Error: resolver failed: {}", e);
    }

}