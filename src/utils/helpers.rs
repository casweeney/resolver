use std::fs;
use std::process::Command;
pub use std::io::Result as IOResult;

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
        _ => false
    }
}

pub fn is_nestjs_installed() -> bool {
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