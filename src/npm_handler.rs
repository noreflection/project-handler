use crate::repository::Repository;
use std::{process, env};
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
    pub fn npm_init_with_defaults(&self) { //check Repository kind: should apply only on kind:dotnet
        let npm = Path::new("C:\\Program Files\\nodejs");
        //let npm = Path::new("C:\\portal\\identity-service");
        assert!(env::set_current_dir(&npm).is_ok());

        let status = process::Command::new(NPM)
            .arg("init")
            .arg("-y")
            .status()
            .expect("failed to execute npm init -y");
        match status.code() {
            Some(code) => println!("exited with status code: {}", code),
            None => println!("process terminated by signal")
        }
    }

    pub fn check_node_version(&mut self) -> Result<(), Box<dyn Error>> {
        let node = process::Command::new("node")
            .arg("-v")
            .status()?;

        self.node_is_installed = node.success();
        Ok(())
    }
}