use models::payloads::*;

#[derive(Debug)]
pub enum EventType {
    Create,
    Fork,
    Other,
    CommitComment,
    Delete(Option<DeletePayload>),
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
    Push(Option<PushPayload>),
    Release,
    Repository,
    Status,
    TeamAdd,
    Watch(Option<WatchPayload>),
}

