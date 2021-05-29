use crate::repository::Repository;

pub struct Handler {
    pub repositories: [Repository; 3],
}

impl Handler {
    pub fn check_all_repos(&self) {
        for n in self.repositories.iter() {
            n.check();
        }
    }
}