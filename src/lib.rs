use std::clone;
use std::fs;

use git2::Repository;

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
}

pub struct Config {
    pub command: Command,
    pub item: Item,
    pub project_name: String
}

impl Config {
    pub fn init(args: Vec<String>) -> Result<Config, &'static str> {
        if args.len() < 4 {
            return Err("Incomplete arguments");
        }

        let command = args[1].clone();
        let item = args[2].clone();
        let project_name = args[3].clone();

        if command != String::from("scaffold") && command != String::from("get") {
            return Err("Invalid command");
        }

        if item.is_empty() {
            return Err("Invalid item");
        }

        if project_name.is_empty() {
            return Err("Input project name");
        }

        match command.as_str() {
            "get" => {
                match item.as_str() {
                    "hhjs" => Ok(Config {command: Command::Get, item: Item::HardhatJavascript, project_name}),
                    "hhts" => Ok(Config {command: Command::Get, item: Item::HardhatTypescript, project_name}),
                    "fd" => Ok(Config {command: Command::Get, item: Item::Foundry, project_name}),
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
}

pub fn resolve(config: &Config) -> Result<(), git2::Error> {

    match config.command {
        Command::Get => {
            match config.item {
                Item::HardhatJavascript => {
                    Repository::clone(GIT_HARDHAT_JS_URL, config.project_name.clone())?;
                },
                Item::HardhatTypescript => {
                    Repository::clone(GIT_HARDHAT_JS_URL, config.project_name.clone())?;
                },
                Item::Foundry => {
                    Repository::clone(GIT_HARDHAT_JS_URL, config.project_name.clone())?;
                }
            }
        },
        _ => {
            println!("Use only get command");
        }
    }

    // Remove .git directory from clone project
    let git_dir = format!("{}/.git", config.project_name);
    fs::remove_dir_all(git_dir).unwrap();

    println!("Success: Happy building !!!");

    Ok(())
}