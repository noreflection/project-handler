use crate::repository::Repository;
use crate::repository_handler::RepositoryHandler;

mod config;
mod repository;
mod scenario;
mod repository_handler;
mod dotnet_handler;
mod npm_handler;

fn main() {
    // 1. get all reps collection from config
    let repository_handler = RepositoryHandler { //to impl: putt them all in cargo.toml
        repositories: [
            repository::Repository  {
                name: "ticketing_system_api",
                url: "https://github.com/datasets/currency-codes",
                path: "/portal/identity-service",
                branch: "master",
            },
            repository::Repository {
                name: "ticketing_system_client",
                url: "https://github.com/datasets/currency-codes",
                path: "/portal/ticketing-system-api/",
                branch: "master",
            },
            repository::Repository {
                name: "ticketing_system_client",
                url: "https://github.com/datasets/currency-codes",
                path: "/portal/ticketing-system-api/",
                branch: "master",
            }
        ]
    };

    repository_handler.update_all_repos();
    repository_handler.set_all_repos_to_master_branch();
    
    let config = config::test_toml();//
    let temp = config.repositories.github;
    
    //2. provide them in update_all_repos as argument in chain
    
    //3. update_all_repos souuld return updated repos
    
    // 4. updated_repos collection should be provided 
    // in chain to  set_all_repos_to_master_branch as argument
    
    //5. maybe abstract it into scenario

    //to impl: when run different projects in diff repos simultaneously 
    //add aoutput to different tmux windows
    println!("data has been processed")
}
