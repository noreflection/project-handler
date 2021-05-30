use crate::repository::Repository;

pub struct DotnetHandler { //remove late repository as state, get it as dependency
    pub repositories: [Repository; 3],
}

impl DotnetHandler {
    #[allow(unused)]
    pub fn update_all_repos(&self) {
        for repo in self.repositories.iter() {
            repo.update();
        }
    }
}