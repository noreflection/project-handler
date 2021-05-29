use crate::handler::Handler;
use crate::repository::Repository;

mod repository;
mod scenario;
mod handler;

fn main() {
    // let ticketing_system_api_repository = repository::Repository {
    //     name: "ticketing_system_api",
    //     url: "https://github.com/datasets/currency-codes",
    //     path: "/portal/identity-service",
    //     branch: "master",
    // };
    // ticketing_system_api_repository.check();
    // 
    // let ticketing_system_client = repository::Repository {
    //     name: "ticketing_system_client",
    //     url: "https://github.com/datasets/currency-codes",
    //     path: "/portal/ticketing-system-api/",
    //     branch: "master",
    // };
    // ticketing_system_client.check();
    // 
    // let ticketing_system_client = repository::Repository {
    //     name: "ticketing_system_client",
    //     url: "https://github.com/datasets/currency-codes",
    //     path: "/portal/ticketing-system-api/",
    //     branch: "master",
    // };
    // ticketing_system_client.check();

    let handler = Handler {
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
    
    handler.check_all_repos();

    print!("data has been processed")
}
