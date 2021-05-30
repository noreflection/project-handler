use std::path::Path;

use git2::{Commit, Error, Index, MergeOptions, ObjectType, Repository as Repo, ResetType};

pub struct Repository {
    pub name: &'static str,
    pub url: &'static str,
    pub path: &'static str,
    pub branch: &'static str, //extend: add kind:dotnet, npm
}

impl Repository {
    fn reset(&self, path: &Path) {
        let repo = match Repo::open(path) {
            Ok(repo) => repo,
            Err(e) => panic!("failed to open: {}", e),
        };
        repo.reset(
            &repo.revparse_single("HEAD").unwrap(),
            ResetType::Hard,
            None,
        )
            .unwrap();
    }

    fn clone(&self) {
        let _repo = match Repo::clone(self.url, self.path) {
            Ok(repo) => repo,
            Err(e) => panic!("failed to clone: {}", e),
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

    pub fn update(&self) { //add --force_rewrite_repos=true option(meaning it will rewrite git repos)
        let repo_path = Path::new(self.path);

        if !repo_path.exists() {
            self.clone();
        }

        if repo_path.exists() && repo_path.is_dir() {
            self.reset(repo_path);
            let _idx = match self.pull(repo_path) {
                Ok(idx) => idx,
                Err(e) => panic!("failed to pull: {}", e),
            };
        }
    }

    pub fn set_to_master_branch(&self){
        let repo = match Repo::open(self.path) {
            Ok(repo) => repo,
            Err(e) => panic!("failed to open: {}", e),
        };

        let refname = "master"; // or a tag (v0.1.1) or a commit (8e8128)
        let (object, reference) 
            = repo.revparse_ext(refname)
            .expect("Object not found");

        repo.checkout_tree(&object, None)
            .expect("failed to checkout");

        match reference {
            // gref is an actual reference like branches or tags
            Some(gref) => repo.set_head(gref.name().unwrap()),
            // this is a commit, not a reference
            None => repo.set_head_detached(object.id()),
        }
            .expect("failed to set HEAD");
    }

    //to impl: abstract these to scenarios
    //will take a repo and impl these 2 functions
    pub fn pull_latest_master(&self) {
        let repo_path = Path::new(self.path);

        if repo_path.exists() && repo_path.is_dir() {
            self.reset(repo_path);
            let _idx = match self.pull(repo_path) {
                Ok(idx) => idx,
                Err(e) => panic!("Failed to pull: {}", e),
            };
        }
    }
}

