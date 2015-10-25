use models::owner;

pub struct Repo {
    gh_id: u64,
    name: String,
    language: String,
    has_issues: bool,
    owner: owner::Owner,
    url: String,
}

impl Repo {
    pub fn new() -> Repo {
        Repo {
            gh_id: 0,
            name: "default".into(),
            language: "default".into(),
            has_issues: false,
            owner: owner::OwnerBuilder::new().finalize(),
            url: "default".into(),
        }
    }
}

pub struct RepoBuilder {
    repo: Repo,
}

impl RepoBuilder {
    pub fn new() -> RepoBuilder {
        RepoBuilder {
            repo: Repo::new(),
        }
    }
}
