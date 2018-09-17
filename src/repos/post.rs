//! Access the Repos portion of the GitHub API
imports!();
use client::PostQueryBuilder;

new_type!(
    Sha
    Statuses
    Repo
    Repos
    Owner
    Issues
    Pulls
    Number
    RequestedReviewers
);

from!(
    @PostQueryBuilder
        -> Repos = "repos"
    @Repos
        => Owner
    @Owner
        => Repo
    @Repo
        -> Statuses = "statuses"
        -> Issues = "issues"
        -> Pulls = "pulls"
    @Pulls
        => Number
    @Number
        -> RequestedReviewers = "requested_reviewers"
    @Statuses
        => Sha

);

impl_macro!(
    @Repos
        |
        |=> owner ->  Owner = username_str
    @Owner
        |
        |=> repo -> Repo = repo_str
    @Repo
        |=> statuses -> Statuses
        |=> issues -> Issues
        |=> pulls -> Pulls
        |
    @Pulls
        |
        |=> number -> Number = number_str
    @Number
        |=> requested_reviewers -> RequestedReviewers
        |
    @Statuses
        |
        |=> sha -> Sha = sha_str
    @Issues
        |
);

exec!(Sha);
exec!(Issues);
exec!(RequestedReviewers);
