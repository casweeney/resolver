use std::fs;
use git2::Repository;
use std::error::Error;

pub mod utils;
use utils::helpers::*;
use utils::constants::*;
pub use utils::command_arguments::*;


pub fn resolve(args: ClapperArgs) -> Result<(), Box<dyn Error>> {
    match args.entity_type {
        EntityType::Get(get_command) => {
            match get_command.command {
                GetSubCommand::Dhjs(dir) => {
                    match Repository::clone(GIT_DIAMOND_HARDHAT_JS_URL, &dir.dir_name) {
                        Ok(_) => {
                            println!("Successfully cloned project!");
                            remove_git_dir(&dir.dir_name);
                        },
                        Err(e) => eprintln!("Failed to clone project: {}", e),
                    }
                },
                GetSubCommand::Dhts(dir) => {
                    match Repository::clone(GIT_DIAMOND_HARDHAT_TS_URL, &dir.dir_name) {
                        Ok(_) => {
                            println!("Successfully cloned project!");
                            remove_git_dir(&dir.dir_name);
                        },
                        Err(e) => eprintln!("Failed to clone project: {}", e),
                    }
                },
                GetSubCommand::Dfd(dir) => {
                    match Repository::clone(GIT_DIAMOND_FOUNDRY_URL, &dir.dir_name) {
                        Ok(_) => {
                            println!("Successfully cloned project!");
                            remove_git_dir(&dir.dir_name);
                        },
                        Err(e) => eprintln!("Failed to clone project: {}", e),
                    }
                },
                GetSubCommand::Nestjs(dir) => {
                    match Repository::clone(GIT_NEST_JS_URL, &dir.dir_name){
                        Ok(_) => {
                            println!("Successfully cloned project!");
                            remove_git_dir(&dir.dir_name);
                        },
                        Err(e) => eprintln!("Failed to clone project: {}", e),
                    }
                },
            };
        },
        EntityType::Scaffold(scaffold_command) => {
            match scaffold_command.command {
                ScaffoldSubCommand::Reactjs(dir) => {
                    match create_react_app(dir.dir_name.clone()) {
                        Ok(_) => println!("Successfully created the React project!"),
                        Err(e) => {
                            return  Err(e);
                        }
                    }
                },
                ScaffoldSubCommand::Reactts(dir) => {
                    match create_react_app_with_typescript(dir.dir_name.clone()) {
                        Ok(_) => println!("Successfully created the TypeScript React project!"),
                        Err(e) => {
                            return  Err(e);
                        }
                    }
                },
                ScaffoldSubCommand::Hardhat(dir) => {
                    match create_hardhat_project(dir.dir_name.clone()) {
                        Ok(_) => println!("Successfully created the Hardhat project!"),
                        Err(e) => {
                            return  Err(e);
                        }
                    }
                },
                ScaffoldSubCommand::Nestjs(dir) => {
                    match create_nestjs_app(dir.dir_name.clone()) {
                        Ok(_) => println!("Successfully created the Nestjs project!"),
                        Err(e) => {
                            return  Err(e);
                        }
                    }
                },
                ScaffoldSubCommand::Laravel(dir) => {
                    match create_laravel_project(dir.dir_name.clone()) {
                        Ok(_) => println!("Successfully created the Laravel project!"),
                        Err(e) => {
                            return  Err(e);
                        }
                    }
                },
                ScaffoldSubCommand::Nextjs(dir) => {
                    match create_next_app(dir.dir_name.clone()) {
                        Ok(_) => println!("Successfully created the Next application!"),
                        Err(e) => {
                            return  Err(e);
                        }
                    }
                }
            }
        },
        EntityType::Install(install_command) => {
            match install_command.command {
                InstallSubCommand::Brew => {
                    match install_brew() {
                        Ok(_) => println!("Homebrew installation successful!"),
                        Err(e) => {
                            return  Err(e);
                        }
                    }
                },
                InstallSubCommand::Choco => {
                    match install_choco() {
                        Ok(_) => println!("Chocolatey installation successful!"),
                        Err(e) => {
                            return  Err(e);
                        }
                    }
                },
                InstallSubCommand::Node => {
                    match install_node() {
                        Ok(_) => println!("Node.js installation successful!"),
                        Err(e) => {
                            return  Err(e);
                        }
                    }
                },
                InstallSubCommand::Scarb => {
                    match install_scarb() {
                        Ok(_) => println!("Scarb installation successful!"),
                        Err(e) => {
                            return  Err(e);
                        }
                    }
                }
            }
        }
    }

    Ok(())
}

fn remove_git_dir(dir_name: &String) {
    let git_dir = format!("{}/.git", dir_name);
    if fs::remove_dir_all(&git_dir).is_err() {
        println!("Warning: Failed to remove .git directory");
    }
}