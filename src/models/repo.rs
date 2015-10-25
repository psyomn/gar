use models::owner;

pub struct Repo {
    gh_id: u64,
    name: String,
    language: String,
    has_issues: bool,
    owner: owner::Owner,
    url: String,
}
