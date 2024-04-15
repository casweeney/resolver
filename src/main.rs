use std::env;
use resolver::Config;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = match Config::new(args) {
        Ok(cfg) => cfg,
        Err(e) => {
            eprintln!("Error initializing configuration: {}", e);
            return;
        }
    };

    println!("Command: {}, Argument: {}", config.command, config.item);
}