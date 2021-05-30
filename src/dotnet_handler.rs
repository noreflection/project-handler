use crate::repository::Repository;

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
}