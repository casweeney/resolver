use std::fs;
use git2::Repository;
use std::process::Command;

pub const GIT_HARDHAT_JS_URL: &str = "https://github.com/mudgen/diamond-3-hardhat.git";
pub const GIT_HARDHAT_TS_URL: &str = "https://github.com/Timidan/diamond-3-hardhat-typechain.git";
pub const GIT_FOUNDRY_URL: &str = "https://github.com/FydeTreasury/Diamond-Foundry.git";

pub enum Action {
    Scaffold,
    Get
}

pub enum Item {
    HardhatJavascript, // hhjs
    HardhatTypescript, // hhts
    Foundry, // fd
}

pub struct Config {
    pub action: Action,
    pub item: Item,
    pub project_name: String
}

impl Config {
    pub fn init(args: Vec<String>) -> Result<Config, &'static str> {
        if args.len() < 4 {
            return Err("Incomplete arguments");
        }

        let action = args[1].clone();
        let item = args[2].clone();
        let project_name = args[3].clone();

        if action != String::from("scaffold") && action != String::from("get") {
            return Err("Invalid command");
        }

        if item.is_empty() {
            return Err("Invalid item");
        }

        if project_name.is_empty() {
            return Err("Input project name");
        }

        match action.as_str() {
            "get" => {
                match item.as_str() {
                    "hhjs" => Ok(Config {action: Action::Get, item: Item::HardhatJavascript, project_name}),
                    "hhts" => Ok(Config {action: Action::Get, item: Item::HardhatTypescript, project_name}),
                    "fd" => Ok(Config {action: Action::Get, item: Item::Foundry, project_name}),
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
    match config.action {
        Action::Get => {
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

pub fn is_npm_installed() -> bool {
    let output = Command::new("npm")
        .arg("--version")
        .output();


    match output {
        Ok(output) => {
            if output.status.success() {
                true
            } else {
                false
            }
        },
        Err(e) => {
            false
        }
    }
}