use crate::repository::Repository;

pub struct RepositoryHandler { //remove later repository as state, get it as dependency
    pub repositories: Vec<Repository>,
}

impl RepositoryHandler {
    pub fn update_all_repos(&self) {
        for repo in self.repositories.iter() {
            repo.update();
        }
    }

    pub fn set_all_repos_to_master_branch(&self) {
        for repo in self.repositories.iter() {
            repo.set_to_master_branch();
        }
    }

    pub fn pull_latest_master_on_all_repos(&self) {
        for repo in self.repositories.iter() {
            repo.pull_latest_master();
        }
    }
}