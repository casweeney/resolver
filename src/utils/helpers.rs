use std::fs;
use std::io::Error;
use std::process::{Command, Output};
pub use std::io::Result as IOResult;
use std::env::consts::OS;

use crate::{LINUX, WINDOWS};
// -------------------
// Checker functions
// -------------------
pub fn is_node_installed() -> bool {
    let output = Command::new("node")
        .arg("--version")
        .output();

    check_output(output)
}

pub fn is_npm_installed() -> bool {
    let output = Command::new("npm")
        .arg("--version")
        .output();

    check_output(output)
}

pub fn is_nestjs_installed() -> bool {
    let output = Command::new("nest")
        .arg("--version")
        .output();

    check_output(output)
}

pub fn is_php_installed() -> bool {
    let output = Command::new("php")
        .arg("--version")
        .output();

    check_output(output)
}

pub fn is_laravel_installed() -> bool {
    let output = Command::new("composer")
        .arg("--version")
        .output();

    check_output(output)
}

pub fn is_scarb_installed() -> bool {
    let output = Command::new("scarb")
        .arg("--version")
        .output();

    check_output(output)
}

pub fn is_starkli_installed() -> bool {
    let output = Command::new("starkli")
        .arg("--version")
        .output();

    check_output(output)
}

pub fn is_brew_installed() -> bool {
    let output = Command::new("brew")
        .arg("--version")
        .output();

    check_output(output)
}

pub fn is_choco_installed() -> bool {
    let output = Command::new("choco")
        .arg("--version")
        .output();

    check_output(output)
}

pub fn get_os() -> String {
    OS.to_string()
}

fn check_output(output: Result<Output, Error>) -> bool {
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


// -------------------
// Scaffold functions
// -------------------
pub fn create_react_app(project_name: String) -> IOResult<()> {
    Command::new("npx")
        .args(["create-react-app", project_name.as_str()])
        .spawn()?
        .wait()?;

    Ok(())
}

pub fn create_react_app_with_typescript(project_name: String) -> IOResult<()> {
    Command::new("npx")
        .args(["create-react-app", project_name.as_str(), "--template", "typescript"])
        .spawn()?
        .wait()?;

    Ok(())
}

pub fn create_hardhat_project(project_name: String) -> IOResult<()> {
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

pub fn create_nestjs_app(project_name: String) -> IOResult<()> {
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

pub fn create_laravel_project(project_name: String) -> IOResult<()> {
    println!("Creating Laravel project: {}", project_name);

    Command::new("composer")
        .args(["create-project", "laravel/laravel", project_name.as_str()])
        .spawn()?
        .wait()?;

    Ok(())
}

pub fn create_next_app(project_name: String) -> IOResult<()> {
    Command::new("npx")
        .args(["create-next-app@latest", project_name.as_str()])
        .spawn()?
        .wait()?;

    Ok(())
}

// -------------------
// Installer functions
// -------------------

pub fn install_brew() -> IOResult<()> {
    if !is_brew_installed() {
        println!("Installing Homebrew...");

        let script_url = "https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh";
        let command = format!("curl -fsSL {} | /bin/bash", script_url);

        Command::new("sh")
            .arg("-c")
            .arg(command)
            .output()?;
    } else {
        println!("Brew is already installed!");
    }

    Ok(())
}

pub fn install_choco() -> IOResult<()> {
    println!("Installing Chocolatey");

    let powershell_script = r#"
        Set-ExecutionPolicy Bypass -Scope Process -Force;
        [System.Net.ServicePointManager]::SecurityProtocol = [System.Net.ServicePointManager]::SecurityProtocol -bor 3072;
        iex ((New-Object System.Net.WebClient).DownloadString('https://chocolatey.org/install.ps1'))
    "#;

    Command::new("powershell")
        .arg("-Command")
        .arg(powershell_script)
        .output()?;


    Ok(())
}

pub fn install_node() -> IOResult<()> {
    match get_os().as_str() {
        MACOS => {
            install_node_macos()
        },
        WINDOWS => {
            install_node_windows()
        },
        LINUX => {
            install_node_linux()
        },
        _ => panic!("Unsupported OS")
    }
}

pub fn install_node_linux() -> IOResult<()> {
    println!("Installing Node.js on Linux...");

    Command::new("sudo")
        .arg("apt-get")
        .args(["update"])
        .status()?;

    Command::new("sudo")
        .arg("apt-get")
        .args(["install", "-y", "nodejs"])
        .status()?;
    
    Ok(())
}

pub fn install_node_macos() -> IOResult<()> {
    println!("Installing Node.js on macOS...");

    if is_brew_installed() {
        Command::new("brew")
            .args(["install", "node"])
            .status()?;
    }

    Ok(())
}

pub fn install_node_windows() -> IOResult<()> {
    println!("Installing Node.js on Windows...");

    if is_choco_installed() {
        Command::new("choco")
            .args(["install", "nodejs", "-y"])
            .status()?;
    }

    Ok(())
}

pub fn install_scarb() -> IOResult<()> {
    let install_cmd = "curl --proto '=https' --tlsv1.2 -sSf https://docs.swmansion.com/scarb/install.sh | sh";

    Command::new("sh")
        .arg("-c")
        .arg(install_cmd)
        .output()?;

    Ok(())
}

pub fn install_starkli() {}

pub fn install_composer() {}