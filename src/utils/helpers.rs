use std::fs;
use std::io::Error as OutputError;
use std::process::{Command, Output};
use std::error::Error;

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

pub fn is_composer_installed() -> bool {
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

pub fn is_forge_installed() -> bool {
    let output = Command::new("forge")
        .arg("--version")
        .output();

    check_output(output)
}

pub fn is_python_installed() -> bool {
    //TODO: Check for python3 for macos
    let output = Command::new("python3")
        .arg("--version")
        .output();

    check_output(output)
}

pub fn is_pip_installed() -> bool {
    let output = Command::new("pip3")
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

pub fn is_nargo_installed() -> bool {
    let output = Command::new("noirup")
        .arg("--version")
        .output();

    check_output(output)
}

pub fn get_os() -> String {
    let os_family = std::env::consts::OS;

    os_family.to_string()
}

fn check_output(output: Result<Output, OutputError>) -> bool {
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
pub fn create_react_app(project_name: String) -> Result<(), Box<dyn Error>> {
    if !is_npm_installed() {
        return  Err("You don't have npm installed".into());
    } else {
        Command::new("npx")
            .args(["create-react-app", project_name.as_str()])
            .spawn()?
            .wait()?;

        Ok(())
    }
}

pub fn create_react_app_with_typescript(project_name: String) -> Result<(), Box<dyn Error>> {
    if !is_npm_installed() {
        return  Err("You don't have npm installed".into());
    } else {
        Command::new("npx")
            .args(["create-react-app", project_name.as_str(), "--template", "typescript"])
            .spawn()?
            .wait()?;

        Ok(())
    }
}

pub fn create_hardhat_project(project_name: String) -> Result<(), Box<dyn Error>> {
    if !is_npm_installed() {
        return  Err("You don't have npm installed".into());
    } else {
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
}

pub fn create_nestjs_app(project_name: String) -> Result<(), Box<dyn Error>> {
    if !is_nestjs_installed() {
        if !is_npm_installed() {
            return  Err("You don't have npm installed".into());
        }

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

pub fn create_laravel_project(project_name: String) -> Result<(), Box<dyn Error>> {
    if !is_php_installed() && !is_composer_installed() {
        return  Err("You don't have PHP or Composer installed".into());
    } else {
        println!("Creating Laravel project: {}", project_name);

        Command::new("composer")
            .args(["create-project", "laravel/laravel", project_name.as_str()])
            .spawn()?
            .wait()?;

        Ok(())
    }
}

pub fn create_next_app(project_name: String) -> Result<(), Box<dyn Error>> {
    if !is_npm_installed() {
        return  Err("You don't have npm installed".into());
    } else {
        Command::new("npx")
            .args(["create-next-app@latest", project_name.as_str()])
            .spawn()?
            .wait()?;

        Ok(())
    }
}

pub fn create_new_foundry_project(project_name: String) -> Result<(), Box<dyn Error>> {
    if !is_forge_installed() {
        return  Err("You don't have Forge installed".into());
    } else {
        Command::new("forge")
            .args(["init", project_name.as_str()])
            .spawn()?
            .wait()?;

        Ok(())
    }
}

pub fn create_django_project(project_name: String) -> Result<(), Box<dyn Error>> {
    if !is_python_installed() && !is_pip_installed() {
        return  Err("You don't have Python installed".into());
    } else {
        Command::new("django-admin")
            .args(["startproject", project_name.as_str()])
            .spawn()?
            .wait()?;

        Ok(())
    }
}

pub fn create_vue_project(project_name: String) -> Result<(), Box<dyn Error>> {
    if !is_npm_installed() {
        return  Err("You don't have npm installed".into());
    } else {
        Command::new("npm")
            .args(["create", "vue@latest", project_name.as_str()])
            .spawn()?
            .wait()?;

        Ok(())
    }
}

pub fn create_vite_project(project_name: String) -> Result<(), Box<dyn Error>> {
    if !is_npm_installed() {
        return  Err("You don't have npm installed".into());
    } else {
        Command::new("npm")
            .args(["create", "vite@latest", project_name.as_str()])
            .spawn()?
            .wait()?;

        Ok(())
    }
}

// -------------------
// Installer functions
// -------------------

pub fn install_brew() -> Result<(), Box<dyn Error>> {
    if is_brew_installed() {
        return  Err("Brew is already installed!".into());
    } else {
        println!("Installing Homebrew...");

        let script_url = "https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh";
        let command = format!("curl -fsSL {} | /bin/bash", script_url);

        Command::new("sh")
            .arg("-c")
            .arg(command)
            .output()?;

        Ok(())
    }
}

pub fn install_choco() -> Result<(), Box<dyn Error>> {
    if is_choco_installed() {
        return  Err("Chocolatey is already installed!".into());
    } else {
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
}

pub fn install_node() -> Result<(), Box<dyn Error>> {
    if is_node_installed() {
        return  Err("Node is already installed!".into());
    } else {
        // let os_family = std::env::consts::OS;
        let os_family = get_os();

        match os_family.as_str() {
            "linux" => install_node_linux(),
            "windows" => install_node_windows(),
            "macos" => install_node_macos(),
            _ => panic!("Unsupported OS")
        }
    }
}

pub fn install_node_linux() -> Result<(), Box<dyn Error>> {
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

pub fn install_node_macos() -> Result<(), Box<dyn Error>> {
    println!("Installing Node.js on macOS...");

    if is_brew_installed() {
        Command::new("brew")
            .args(["install", "node"])
            .status()?;
    }

    Ok(())
}

pub fn install_node_windows() -> Result<(), Box<dyn Error>> {
    println!("Installing Node.js on Windows...");

    if is_choco_installed() {
        Command::new("choco")
            .args(["install", "nodejs", "-y"])
            .status()?;
    }

    Ok(())
}

pub fn install_scarb() -> Result<(), Box<dyn Error>> {
    if is_scarb_installed() {
        return  Err("Scarb is already installed!".into());
    } else {
        let install_cmd = "curl --proto '=https' --tlsv1.2 -sSf https://docs.swmansion.com/scarb/install.sh | sh";

        Command::new("sh")
            .arg("-c")
            .arg(install_cmd)
            .output()?;

        Ok(())
    }
}

pub fn install_starkli() -> Result<(), Box<dyn Error>> {
    if is_starkli_installed() {
        return  Err("Starkli is already installed!".into());
    } else {
        Command::new("sh")
            .arg("-c")
            .arg("curl https://get.starkli.sh | sh")
            .output()?;

        Command::new("starkliup")
            .output()?;

        Ok(())
    }
}

pub fn install_composer() {}

pub fn install_forge() -> Result<(), Box<dyn Error>> {
    if is_forge_installed() {
        return  Err("Forge is already installed!".into());
    } else {
        Command::new("sh")
            .arg("-c")
            .arg("curl -L https://foundry.paradigm.xyz | bash")
            .output()?;

        Command::new("foundryup")
            .output()?;

        Ok(())
    }
}


pub fn install_nargo() -> Result<(), Box<dyn Error>> {
    if is_nargo_installed() {
        return  Err("nargo is already installed!".into());
    } else {
        Command::new("sh")
            .arg("-c")
            .arg("curl -L https://raw.githubusercontent.com/noir-lang/noirup/main/install | bash")
            .output()?;

        Command::new("sh")
            .arg("-c")
            .arg("noirup")
            .output()?;

        Ok(())
    }
}
