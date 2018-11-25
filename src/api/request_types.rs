//! Request types for the FrontPage and Users Api

use crate::{json_def::*, query::*, request::response::*};

make_request! {
    ProjectsRequest = JsonPagedResponse<Project> with AlbumIdQuery;
    ProfileRequest = Profile;
    FollowersRequest = JsonPagedResponse<Follower>;
    FollowingsRequest = JsonPagedResponse<Follower>;
    SubmissionsRequest = JsonPagedResponse<Submission>;
    LikesRequest = JsonPagedResponse<Like>;
    TopRowItemsRequest = Vec<TopRowItem> with LimitQuery;
    CampaignInfoRequest = Campaign with SizeQuery, TakeOverQuery;
    JobsRequest = Vec<Job> with FeaturedQuery, LimitQuery;
}
