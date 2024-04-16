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
    ReactJS
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
                match item.as_str() {
                    "reactjs" => Ok(Config {action: Action::Scaffold, item: Item::ReactJS, project_name}),
                    _ => {
                        return Err("Wrong item name");
                    }
                }
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
            let clone_url = match config.item {
                Item::HardhatJavascript => GIT_HARDHAT_JS_URL,
                Item::HardhatTypescript => GIT_HARDHAT_JS_URL,
                Item::Foundry => GIT_HARDHAT_JS_URL,
                _ => return Err(git2::Error::from_str("Unsupported project type"))
            };

            Repository::clone(clone_url, &config.project_name)?;

        },
        Action::Scaffold => {
            match config.item {
                Item::ReactJS => {
                    if is_npm_installed() {

                    } else {

                    }
                }
                _ => return Err(git2::Error::from_str("Unsupported project type"))
            }
        }
        _ => {
            println!("Unsupported action: Use only 'Get' or 'Scaffold' command");
            return Err(git2::Error::from_str("Unsupported action"));
        }
    }

    // Remove .git directory from clone project
    let git_dir = format!("{}/.git", config.project_name);
    if fs::remove_dir_all(&git_dir).is_err() {
        println!("Warning: Failed to remove .git directory");
    }

    println!("Success: Happy building !!!");

    Ok(())
}

fn is_npm_installed() -> bool {
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