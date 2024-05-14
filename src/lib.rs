use std::fs;
use git2::Repository;
use std::error::Error;
use colored::*;

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
                            println!("{}", "Successfully cloned diamond standard hardhat JS project!".bright_blue());
                            remove_git_dir(&dir.dir_name);
                        },
                        Err(e) => {
                            return Err(format!("Failed to clone project: {}", e).into())
                        }
                    }
                },
                GetSubCommand::Dhts(dir) => {
                    match Repository::clone(GIT_DIAMOND_HARDHAT_TS_URL, &dir.dir_name) {
                        Ok(_) => {
                            println!("{}", "Successfully cloned diamond standard hardhat TS project!".bright_blue());
                            remove_git_dir(&dir.dir_name);
                        },
                        Err(e) => {
                            return Err(format!("Failed to clone project: {}", e).into())
                        }
                    }
                },
                GetSubCommand::Dfd(dir) => {
                    match Repository::clone(GIT_DIAMOND_FOUNDRY_URL, &dir.dir_name) {
                        Ok(_) => {
                            println!("{}", "Successfully cloned diamond standard foundry project!".bright_blue());
                            remove_git_dir(&dir.dir_name);
                        },
                        Err(e) => {
                            return Err(format!("Failed to clone project: {}", e).into())
                        }
                    }
                },
                GetSubCommand::Nestjs(dir) => {
                    match Repository::clone(GIT_NEST_JS_URL, &dir.dir_name){
                        Ok(_) => {
                            println!("{}", "Successfully cloned a NestJS project!".bright_blue());
                            remove_git_dir(&dir.dir_name);
                        },
                        Err(e) => {
                            return Err(format!("Failed to clone project: {}", e).into())
                        }
                    }
                },
            };
        },
        EntityType::Scaffold(scaffold_command) => {
            match scaffold_command.command {
                ScaffoldSubCommand::Reactjs(dir) => {
                    match create_react_app(dir.dir_name.clone()) {
                        Ok(_) => println!("{}", "Successfully created the React project!".bright_blue()),
                        Err(e) => {
                            return  Err(e);
                        }
                    }
                },
                ScaffoldSubCommand::Reactts(dir) => {
                    match create_react_app_with_typescript(dir.dir_name.clone()) {
                        Ok(_) => println!("{}", "Successfully created the TypeScript React project!".bright_blue()),
                        Err(e) => {
                            return  Err(e);
                        }
                    }
                },
                ScaffoldSubCommand::Hardhat(dir) => {
                    match create_hardhat_project(dir.dir_name.clone()) {
                        Ok(_) => println!("{}", "Successfully created the Hardhat project!".bright_blue()),
                        Err(e) => {
                            return  Err(e);
                        }
                    }
                },
                ScaffoldSubCommand::Nestjs(dir) => {
                    match create_nestjs_app(dir.dir_name.clone()) {
                        Ok(_) => println!("{}", "Successfully created the Nestjs project!".bright_blue()),
                        Err(e) => {
                            return  Err(e);
                        }
                    }
                },
                ScaffoldSubCommand::Laravel(dir) => {
                    match create_laravel_project(dir.dir_name.clone()) {
                        Ok(_) => println!("{}", "Successfully created the Laravel project!".bright_blue()),
                        Err(e) => {
                            return  Err(e);
                        }
                    }
                },
                ScaffoldSubCommand::Nextjs(dir) => {
                    match create_next_app(dir.dir_name.clone()) {
                        Ok(_) => println!("{}", "Successfully created the Next application!".bright_blue()),
                        Err(e) => {
                            return  Err(e);
                        }
                    }
                },
                ScaffoldSubCommand::Foundry(dir) => {
                    match create_new_foundry_project(dir.dir_name.clone()) {
                        Ok(_) => println!("{}", "Successfully created the Foundry application!".bright_blue()),
                        Err(e) => {
                            return  Err(e);
                        }
                    }
                },
                ScaffoldSubCommand::Vue(dir) => {
                    match create_vue_project(dir.dir_name.clone()) {
                        Ok(_) => println!("{}", "Successfully created a Vue application!".bright_blue()),
                        Err(e) => {
                            return  Err(e);
                        }
                    }
                },
                ScaffoldSubCommand::Vite(dir) => {
                    match create_vite_project(dir.dir_name.clone()) {
                        Ok(_) => println!("{}", "Successfully created a Vite project!".bright_blue()),
                        Err(e) => {
                            return  Err(e);
                        }
                    }
                },
                ScaffoldSubCommand::Noir(dir) => {
                    match create_noir_project(dir.dir_name.clone()) {
                        Ok(_) => println!("{}", "Successfully created a Noir project!".bright_blue()),
                        Err(e) => {
                            return  Err(e);
                        }
                    }
                },
                ScaffoldSubCommand::Snforge(dir) => {
                    match create_starknet_foundry_project(dir.dir_name.clone()) {
                        Ok(_) => println!("{}", "Successfully created a starknet foundry project!".bright_blue()),
                        Err(e) => {
                            return Err(e);
                        }
                    }
                },
                ScaffoldSubCommand::RainbowKit(dir) => {
                    match create_rainbowkit_wagmi_next_app(dir.dir_name.clone()) {
                        Ok(_) => println!("{}", "Successfully created a Rainbowkit + Wagmi + Next.js app project!".bright_blue()),
                        Err(e) => {
                            return Err(e);
                        }
                    }
                },
                ScaffoldSubCommand::ReactNativeExpo(dir) => {
                    match create_expo_app(dir.dir_name.clone()) {
                        Ok(_) => println!("{}", "Successfully created a ReactNative Expo project!".bright_blue()),
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
                        Ok(_) => println!("{}", "Homebrew installation successful!".bright_blue()),
                        Err(e) => {
                            return  Err(e);
                        }
                    }
                },
                InstallSubCommand::Choco => {
                    match install_choco() {
                        Ok(_) => println!("{}", "Chocolatey installation successful!".bright_blue()),
                        Err(e) => {
                            return  Err(e);
                        }
                    }
                },
                InstallSubCommand::Node => {
                    match install_node() {
                        Ok(_) => println!("{}", "Node.js installation successful!".bright_blue()),
                        Err(e) => {
                            return  Err(e);
                        }
                    }
                },
                InstallSubCommand::Scarb => {
                    match install_scarb() {
                        Ok(_) => println!("{}", "Scarb installation successful!".bright_blue()),
                        Err(e) => {
                            return  Err(e);
                        }
                    }
                },
                InstallSubCommand::Forge => {
                    match install_forge() {
                        Ok(_) => println!("{}", "Forge installation successful!".bright_blue()),
                        Err(e) => {
                            return  Err(e);
                        }
                    }
                },
                InstallSubCommand::Starkli => {
                    match install_starkli() {
                        Ok(_) => println!("{}", "Starkli installation successful!".bright_blue()),
                        Err(e) => {
                            return  Err(e);
                        }
                    }
                },
                InstallSubCommand::Noir => {
                    match install_nargo() {
                        Ok(_) => println!("{}", "Noturup installation successful!".bright_blue()),
                        Err(e) => {
                            return  Err(e);
                        }
                    }
                },
                InstallSubCommand::Snfoundry(version) => {
                    match install_snforge(version.version_name.clone()) {
                        Ok(_) => println!("{}", "Starknet Foundry installation successful!".bright_blue()),
                        Err(e) => {
                            return Err(e)
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
