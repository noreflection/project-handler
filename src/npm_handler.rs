use crate::repository::Repository;
use std::{process, env};
use std::process::ExitStatus;
use std::error::Error;
use std::path::Path;

#[cfg(windows)]
pub const NPM: &'static str = "npm.cmd";

#[cfg(not(windows))]
pub const NPM: &'static str = "npm";

pub struct NpmHandler {
    //remove later repository as state, get it as dependency
    pub repositories: Vec<Repository>,
    pub node_is_installed: bool,
}

impl NpmHandler {
    pub fn npm_install(&self) -> std::io::Result<ExitStatus> { //check Repository kind: should apply only on kind:dotnet
        let npm = Path::new("C:\\Program Files\\nodejs");
        assert!(env::set_current_dir(&npm).is_ok());
        
        let npm = process::Command::new(NPM)
            .arg("install")
            //.arg("-g")
            //.arg("puppeteer")
            .status();
        npm
    }

    pub fn check_for_node(&mut self) -> Result<(), Box<dyn Error>> {
        println!("Node Version: ");
        let node = process::Command::new("node")
            .arg("-v")
            .status()?;

        self.node_is_installed = node.success();
        Ok(())
    }

    pub fn install_puppeteer(&mut self) -> Result<(), Box<dyn Error>> {
        let npm = process::Command::new("npm")
            .arg("install")
            //.arg("puppeteer")
            .status()?;
        //self.puppeteer_is_installed = npm.success();

        Ok(())
    }
}