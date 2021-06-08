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
        //let npm = Path::new("C:\\portal\\identity-service\\newprj");
        
        //let npm = Path::new("C:\\portal\\identity-service");
        assert!(env::set_current_dir(&npm).is_ok());

        env::set_var("npm", "C:\\Program Files\\nodejs");

        let status = process::Command::new(NPM)
            //.env("PATH", "C:\\Program Files\\nodejs")
            //.arg("-c")
            //.arg("-y")
            .arg(format!("-W {0}","C:\\portal\\identity-service\\newprj" ))
            .arg("install")
            //.current_dir("C:\\portal\\identity-service\\newprj")
            .status()
            .expect("failed to execute npm init -y");
        match status.code() {
            Some(code) => println!("exited with status code: {}", code),
            None => println!("process terminated by signal")
        }
    }
}