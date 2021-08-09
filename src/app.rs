use crate::git_handler::GitHandler;
use crate::dotnet_handler::DotnetHandler;
use crate::npm_handler::NpmHandler;
use crate::repository::Repository;
use crate::repository as repository;

//mod repository;

pub fn run() {
    let repositories = vec![  // 1. get all reps collection from config
                              repository::Repository {
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
    ];

    let git_handler = GitHandler { //to impl: putt them all in cargo.toml
        repositories
    };

    git_handler.update_all_repos();
    git_handler.set_all_repos_to_master_branch();
    git_handler.pull_latest_master_on_all_repos();

    let repositories = vec![  // 1. get all reps collection from config
                              repository::Repository {
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
    ];

    let mut dotnet_handler = DotnetHandler {
        repositories,
    };

    // let status = dotnet_handler.create_api_project_in_folder();
    // print!("status:{:?}", status);
    // 
    // let status = dotnet_handler.ef_database_update();
    // print!("status:{:?}", status);
    // 
    // let status = dotnet_handler.install_ef_tools();
    // print!("status:{:?}", status);

    let status = dotnet_handler.run_identity_prg_in_current_dir();
    print!("status:{:?}", status);

    let status = dotnet_handler.run_api_prg_in_current_dir();
    print!("status:{:?}", status);



    //let _config = config::test_toml();//
    //let temp = config.repositories.github;

    //2. provide them in update_all_repos as argument in chain

    //3. update_all_repos souuld return updated repos

    // 4. updated_repos collection should be provided 
    // in chain to  set_all_repos_to_master_branch as argument

    //5. maybe abstract it into scenario

    //to impl: when run different projects in diff repos simultaneously 
    //add aoutput to different tmux windows
    println!("data has been processed")
}