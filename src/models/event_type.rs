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
    Gollum(Option<GollumPayload>),
    IssueComment(Option<IssueCommentPayload>),
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

impl ToString for EventType {
    fn to_string(&self) -> String {
        match *self {
            EventType::Create => "CreateEvent".into(),
            EventType::Fork => "ForkEvent".into(),
            EventType::Other => "Other".into(),
            EventType::CommitComment => "CommitCommentEvent".into(),
            EventType::Delete(..) => "DeleteEvent".into(),
            EventType::Deployment => "DeploymentEvent".into(),
            EventType::DeploymentStatus => "DeploymentStatusEvent".into(),
            EventType::Download => "DownloadEvent".into(),
            EventType::Follow => "FollowEvent".into(),
            EventType::ForkApply => "ForkApplyEvent".into(),
            EventType::Gist => "GistEvent".into(),
            EventType::Gollum(..) => "GollumEvent".into(),
            EventType::IssueComment(..) => "IssueCommentEvent".into(),
            EventType::Issues(..) => "IssueEvent".into(),
            EventType::Member => "MemberEvent".into(),
            EventType::Membership => "MembershipEvent".into(),
            EventType::PageBuild => "PageBuildEvent".into(),
            EventType::Public => "PublicEvent".into(),
            EventType::PullRequest => "PullRequestEvent".into(),
            EventType::PullRequestReviewComment => "PullRequestReviewCommentEvent".into(),
            EventType::Push(..) => "PushEvent".into(),
            EventType::Release => "ReleaseEvent".into(),
            EventType::Repository => "RepositoryEvent".into(),
            EventType::Status => "StatusEvent".into(),
            EventType::TeamAdd => "TeamAddEvent".into(),
            EventType::Watch(..) => "WatchEvent".into(),
        }
    }
}
