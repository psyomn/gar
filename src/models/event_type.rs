use models::payloads::*;

#[derive(Debug)]
pub enum EventType {
    Create,
    Fork,
    Other,
    CommitComment,
    Delete,
    Deployment,
    DeploymentStatus,
    Download,
    Follow,
    ForkApply,
    Gist,
    Gollum,
    IssueComment,
    Issues(Option<IssuePayload>),
    Member,
    Membership,
    PageBuild,
    Public,
    PullRequest,
    PullRequestReviewComment,
    Push,
    Release,
    Repository,
    Status,
    TeamAdd,
    Watch,
}

