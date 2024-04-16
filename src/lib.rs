use std::fs;
use git2::Repository;
use std::process::Command;
use std::io::Result as IOResult;

pub const GIT_DIAMOND_HARDHAT_JS_URL: &str = "https://github.com/mudgen/diamond-3-hardhat.git";
pub const GIT_DIAMOND_HARDHAT_TS_URL: &str = "https://github.com/Timidan/diamond-3-hardhat-typechain.git";
pub const GIT_DIAMOND_FOUNDRY_URL: &str = "https://github.com/FydeTreasury/Diamond-Foundry.git";
pub const GIT_NEST_JS_URL: &str = "https://github.com/nestjs/typescript-starter.git";

pub enum Action {
    Scaffold,
    Get
}

pub enum Item {
    DiamondHardhatJavascript, // dhjs
    DiamondHardhatTypescript, // dhts
    DiamondFoundry, // dfd
    ReactJS,
    ReactTS,
    Hardhat,
    NestJs
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
                    "dhjs" => Ok(Config {action: Action::Get, item: Item::DiamondHardhatJavascript, project_name}),
                    "dhts" => Ok(Config {action: Action::Get, item: Item::DiamondHardhatTypescript, project_name}),
                    "dfd" => Ok(Config {action: Action::Get, item: Item::DiamondFoundry, project_name}),
                    "nestjs" => Ok(Config {action: Action::Get, item: Item::NestJs, project_name}),
                    _ => {
                        return Err("Wrong item name");
                    }
                }
            },
            "scaffold" => {
                match item.as_str() {
                    "reactjs" => Ok(Config {action: Action::Scaffold, item: Item::ReactJS, project_name}),
                    "reactts" => Ok(Config {action: Action::Scaffold, item: Item::ReactTS, project_name}),
                    "hardhat" => Ok(Config {action: Action::Scaffold, item: Item::Hardhat, project_name}),
                    "nestjs" => Ok(Config {action: Action::Scaffold, item: Item::NestJs, project_name}),
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
                Item::DiamondHardhatJavascript => GIT_DIAMOND_HARDHAT_JS_URL,
                Item::DiamondHardhatTypescript => GIT_DIAMOND_HARDHAT_TS_URL,
                Item::DiamondFoundry => GIT_DIAMOND_FOUNDRY_URL,
                Item::NestJs => GIT_NEST_JS_URL,
                _ => return Err(git2::Error::from_str("Unsupported project type"))
            };

            Repository::clone(clone_url, &config.project_name)?;

            // Remove .git directory from clone project
            let git_dir = format!("{}/.git", config.project_name);

            if fs::remove_dir_all(&git_dir).is_err() {
                println!("Warning: Failed to remove .git directory");
            }

        },
        Action::Scaffold => {
            match config.item {
                Item::ReactJS => {
                    if is_npm_installed() {
                        match create_react_app(config.project_name.clone()) {
                            Ok(_) => println!("Successfully created the React project!"),
                            Err(e) => eprintln!("Failed to create the React project: {}", e),
                        }
                    } else {
                        println!("You don't have npm installed")
                    }
                },
                Item::ReactTS => {
                    if is_npm_installed() {
                        match create_react_app_with_typescript(config.project_name.clone()) {
                            Ok(_) => println!("Successfully created the TypeScript React project!"),
                            Err(e) => eprintln!("Failed to create the TypeScript React project: {}", e),
                        }
                    }
                },
                Item::Hardhat => {
                    if is_npm_installed() {
                        match create_hardhat_project(config.project_name.clone()) {
                            Ok(_) => println!("Successfully created the Hardhat project!"),
                            Err(e) => eprintln!("Failed to create the Hardhat project: {}", e),
                        }
                    }
                },
                Item::NestJs => {
                    if is_npm_installed() {
                        match create_nestjs_app(config.project_name.clone()) {
                            Ok(_) => println!("Successfully created the Nestjs project!"),
                            Err(e) => eprintln!("Failed to create the Nestjs project: {}", e),
                        }
                    }
                }
                _ => return Err(git2::Error::from_str("Unsupported project type"))
            }
        }
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
        _ => false
    }
}

fn is_nestjs_installed() -> bool {
    let output = Command::new("nest")
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
        _ => false
    }
}

fn create_react_app(project_name: String) -> IOResult<()> {
    Command::new("npx")
        .args(["create-react-app", project_name.as_str()])
        .spawn()?
        .wait()?;

    Ok(())
}

fn create_react_app_with_typescript(project_name: String) -> IOResult<()> {
    Command::new("npx")
        .args(["create-react-app", project_name.as_str(), "--template", "typescript"])
        .spawn()?
        .wait()?;

    Ok(())
}

fn create_hardhat_project(project_name: String) -> IOResult<()> {
    fs::create_dir_all(project_name.as_str())?;

    Command::new("npm")
        .args(["init", "--yes"])
        .current_dir(project_name.as_str())
        .spawn()?
        .wait()?;

    Command::new("npx")
        .args(["hardhat", "init"])
        .current_dir(project_name.as_str())
        .spawn()?
        .wait()?;

    Ok(())
}

fn create_nestjs_app(project_name: String) -> IOResult<()> {
    if !is_nestjs_installed() {
        Command::new("npm")
            .args(["i", "-g", "@nestjs/cli"])
            .spawn()?
            .wait()?;
    }

    Command::new("nest")
        .args(["new", project_name.as_str()])
        .spawn()?
        .wait()?;

    Ok(())
}