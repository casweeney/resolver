use std::fs;
use git2::Repository;
use clap::{Args, Parser, Subcommand};
use std::error::Error;

pub mod utils;
use utils::helpers::*;


pub const GIT_DIAMOND_HARDHAT_JS_URL: &str = "https://github.com/mudgen/diamond-3-hardhat.git";
pub const GIT_DIAMOND_HARDHAT_TS_URL: &str = "https://github.com/Timidan/diamond-3-hardhat-typechain.git";
pub const GIT_DIAMOND_FOUNDRY_URL: &str = "https://github.com/FydeTreasury/Diamond-Foundry.git";
pub const GIT_NEST_JS_URL: &str = "https://github.com/nestjs/typescript-starter.git";

#[derive(Debug, Parser)]
#[clap(author, version, about)]
pub struct ClapperArgs {
    #[clap(subcommand)]
    pub entity_type: EntityType,
}

#[derive(Debug, Subcommand)]
pub enum EntityType {
    /// Clones repositories
    Get(GetCommand),
    Scaffold(ScaffoldCommand),
}

#[derive(Debug, Args)]
pub struct GetCommand{
    #[clap(subcommand)]
    pub command: GetSubCommand,
}

#[derive(Debug, Args)]
pub struct ScaffoldCommand{
    #[clap(subcommand)]
    pub command: ScaffoldSubCommand,
}



#[derive(Debug, Subcommand)]
pub enum GetSubCommand {
    Dhjs(GetDir),
    Dhts(GetDir),
    Dfd(GetDir),
    Nestjs(GetDir)
}

#[derive(Debug, Subcommand)]
pub enum ScaffoldSubCommand {
    ReactJS(GetDir),
    ReactTS(GetDir),
    Hardhat(GetDir),
    NestJs(GetDir),
    Laravel(GetDir),
    NextJs(GetDir),
}

#[derive(Debug, Args)]
pub struct GetDir {
    /// Specifies the name of the project directory to initialize
    pub dir_name: String
}

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
    NestJs,
    Laravel,
    NextJs,
}

pub struct Config {
    pub action: Action,
    pub item: Item,
    pub project_name: String
}

impl Config {
    pub fn resolve(args: ClapperArgs) -> Result<(), Box<dyn Error>> {
        match args.entity_type {
            EntityType::Get(get_command) => {
                match get_command.command {
                    GetSubCommand::Dhjs(dir) => Repository::clone("GIT_DIAMOND_HARDHAT_JS_URL", &dir.dir_name)?,
                    GetSubCommand::Dhts(dir) => Repository::clone("GIT_DIAMOND_HARDHAT_TS_URL", &dir.dir_name)?,
                    GetSubCommand::Dfd(dir) => Repository::clone("GIT_DIAMOND_FOUNDRY_URL", &dir.dir_name)?,
                    GetSubCommand::Nestjs(dir) => Repository::clone("GIT_NEST_JS_URL", &dir.dir_name)?,
                };

                // let git_dir = format!("{}/.git", config.project_name);

                // if fs::remove_dir_all(&git_dir).is_err() {
                //     println!("Warning: Failed to remove .git directory");
                // }
            },
            EntityType::Scaffold(scaffold_command) => {
                match scaffold_command.command {
                    ScaffoldSubCommand::ReactJS(dir) => {
                        if is_npm_installed() {
                            match create_react_app(dir.dir_name.clone()) {
                                Ok(_) => println!("Successfully created the React project!"),
                                Err(e) => eprintln!("Failed to create the React project: {}", e),
                            }
                        } else {
                            println!("You don't have npm installed")
                        }
                    },
                    ScaffoldSubCommand::ReactTS(dir) => {
                        if is_npm_installed() {
                            match create_react_app_with_typescript(dir.dir_name.clone()) {
                                Ok(_) => println!("Successfully created the TypeScript React project!"),
                                Err(e) => eprintln!("Failed to create the TypeScript React project: {}", e),
                            }
                        }
                    },
                    ScaffoldSubCommand::Hardhat(dir) => {
                        if is_npm_installed() {
                            match create_hardhat_project(dir.dir_name.clone()) {
                                Ok(_) => println!("Successfully created the Hardhat project!"),
                                Err(e) => eprintln!("Failed to create the Hardhat project: {}", e),
                            }
                        }
                    },
                    ScaffoldSubCommand::NestJs(dir) => {
                        if is_npm_installed() {
                            match create_nestjs_app(dir.dir_name.clone()) {
                                Ok(_) => println!("Successfully created the Nestjs project!"),
                                Err(e) => eprintln!("Failed to create the Nestjs project: {}", e),
                            }
                        }
                    },
                    ScaffoldSubCommand::Laravel(dir) => {
                        if is_php_installed() && is_laravel_installed() {
                            match create_laravel_project(dir.dir_name.clone()) {
                                Ok(_) => println!("Successfully created the Laravel project!"),
                                Err(e) => eprintln!("Failed to create the Laravel project: {}", e),
                            }
                        }
                    },
                    ScaffoldSubCommand::NextJs(dir) => {
                        if is_npm_installed() {
                            match create_next_app(dir.dir_name.clone()) {
                                Ok(_) => println!("Successfully created the Next application!"),
                                Err(e) => eprintln!("Failed to create the Next application: {}", e),
                            }
                        }
                    }
                }
            }
        }

        Ok(())
    }
}