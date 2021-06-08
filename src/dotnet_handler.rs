use crate::repository::Repository;
use std::{process, env};
use std::error::Error;
use std::path::Path;

pub struct DotnetHandler { //remove late repository as state, get it as dependency
    pub repositories: Vec<Repository>,
}

impl DotnetHandler {
    #[allow(unused)]
    pub fn dotnet_run_all_repos(&self) { //check Repository kind: should apply only on kind:dotnet
        for repo in self.repositories.iter() {
            repo.update();
        }
    }

    pub fn create_api_project_in_folder(&mut self) -> Result<(), Box<dyn Error>> {
        let root = Path::new("C:\\portal\\identity-service\\");
        assert!(env::set_current_dir(&root).is_ok());

        let node = process::Command::new("dotnet")
            .arg("new")
            .arg("api")
            .status()?;
        
        Ok(())
    }

    pub fn run_identity_prg_in_current_dir(&mut self) -> Result<(), Box<dyn Error>> {
        let prj_path = "C:\\portal\\identity-service\\";

        let root = Path::new(prj_path);
        assert!(env::set_current_dir(&root).is_ok());

        let node = process::Command::new("dotnet")
            .arg("run")
            .status()?;

        Ok(())
    }

    pub fn run_api_prg_in_current_dir(&mut self) -> Result<(), Box<dyn Error>> {
        let prj_path = "C:\\portal\\ticketing-system-api\\";

        let root = Path::new(prj_path);
        assert!(env::set_current_dir(&root).is_ok());

        let node = process::Command::new("dotnet")
            .arg("run")
            .status()?;

        Ok(())
    }

    pub fn ef_database_update(&mut self) -> Result<(), Box<dyn Error>> {
        let prj_path = "C:\\portal\\identity-service\\";
        let dotnet_path = "C:\\Program Files\\dotnet\\";
        
        let root = Path::new(prj_path);
        assert!(env::set_current_dir(&root).is_ok());
        //env::set_var("dotnet","C:\\Program Files\\dotnet\\");

        let node = process::Command::new("dotnet")
            .arg("ef")
            .arg("database")
            .arg("update")
            .status()?;

        Ok(())
    }
}