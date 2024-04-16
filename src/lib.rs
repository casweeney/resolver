pub const GIT_HARDHAT_JS_URL: &str = "https://github.com/mudgen/diamond-3-hardhat.git";
pub const GIT_HARDHAT_TS_URL: &str = "https://github.com/Timidan/diamond-3-hardhat-typechain.git";
pub const GIT_FOUNDRY_URL: &str = "https://github.com/FydeTreasury/Diamond-Foundry.git";

pub enum Command {
    Scaffold,
    Get
}

pub enum Item {
    HardhatJavascript, // hhjs
    HardhatTypescript, // hhts
    Foundry, // fd
    Hardhat,
    React,
    Next,
    Nest,
    Laravel
}

pub struct Config {
    pub command: Command,
    pub item: Item,
}

impl Config {
    pub fn init(args: Vec<String>) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Incomplete arguments");
        }

        let command = args[1].clone();
        let item = args[2].clone();

        if command != String::from("scaffold") && command != String::from("get") {
            return Err("Invalid command");
        }

        if item.is_empty() {
            return Err("Invalid item name");
        }

        match command.as_str() {
            "get" => {
                match item.as_str() {
                    "hhjs" => Ok(Config {command: Command::Get, item: Item::HardhatJavascript}),
                    "hhts" => Ok(Config {command: Command::Get, item: Item::HardhatTypescript}),
                    "fd" => Ok(Config {command: Command::Get, item: Item::Foundry}),
                    _ => {
                        return Err("Wrong item name");
                    }
                }
            },
            "scaffold" => {
                return Err("Use only get command");
            }
            _ => {
                return Err("Use only get command");
            }
        }
    }

    pub fn resolve(config: &Config) {

    }
}