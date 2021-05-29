use std::fs;
use std::fs::File;
use std::io::{stderr, stdout, Write};
use std::path::Path;

use git2::{Commit, Error, Index, MergeOptions, ObjectType, Repository as Repo, ResetType};

struct Repository {
    url: &'static str,
    path: &'static str,
    branch: &'static str,
}

impl Repository {
    fn reset(&self, path: &Path) {
        let repo = match Repo::open(path) {
            Ok(repo) => repo,
            Err(e) => panic!("Failed to open: {}", e),
        };
        repo.reset(
            &repo.revparse_single("HEAD").unwrap(),
            ResetType::Hard,
            None,
        )
            .unwrap();
    }

    fn clone(&self) {
        let repo = match Repo::clone(self.url, self.path) {
            Ok(repo) => repo,
            Err(e) => panic!("failed to init: {}", e),
        };
    }

    fn find_last_commit<'repo>(&self, repo: &'repo Repo) -> Result<Commit<'repo>, Error> {
        let obj = repo.head()?.resolve()?.peel(ObjectType::Commit)?;
        match obj.into_commit() {
            Ok(c) => Ok(c),
            _ => Err(Error::from_str("commit error")),
        }
    }

    fn pull(&self, path: &Path) -> Result<Index, Error> {
        let repo = Repo::open(path)?;

        repo.find_remote("origin")?
            .fetch(&[self.branch], None, None)?;

        let last_commit = self.find_last_commit(&repo)?;
        let reference = repo.find_reference("FETCH_HEAD")?;
        let fetched_commit = reference.peel_to_commit()?;
        let index =
            repo.merge_commits(&last_commit, &fetched_commit, Some(&MergeOptions::new()))?;

        return Ok(index);
    }

    pub fn check(&self) {
        let repo_path = Path::new(self.path);

        if !repo_path.exists() {
            self.clone();
            return;
        }

        if repo_path.exists() && repo_path.is_dir() {
            self.reset(repo_path);
            let idx = match self.pull(repo_path) {
                Ok(idx) => idx,
                Err(e) => panic!("Failed to pull: {}", e),
            };
        }
    }
}

fn main() {
    let currencies = Repository {
        url: "https://github.com/datasets/currency-codes",
        path: "./resources/currency-codes",
        branch: "master",
    };

    currencies.check();
}
