pub enum Command {
    Scaffold,
    Get
}

pub enum ScaffoldItem {
    Hardhat,
    React,
    Next,
    Nest,
    Laravel
}

pub enum GetItem {
    HardhatJavascript, // hhjs
    HardhatTypescript, // hhts
    Foundry // fd
}

pub struct Config {
    pub command: String,
    pub item: String,
}

impl Config {
    pub fn new(args: Vec<String>) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Incomplete arguments");
        }

        let command = args[1].clone();
        let item = args[2].clone();

        if command != String::from("scaffold") && command != String::from("get") {
            return Err("Invalid command");
        }

        Ok(Config {
            command,
            item
        })
    }
}