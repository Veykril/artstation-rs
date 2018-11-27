//! Request types for the FrontPage and Users Api

use crate::{json_def::*, query::*, request::response::*};

make_request! {
    CampaignInfoRequest = Campaign with SizeQuery, TakeOverQuery;
    FollowersRequest = JsonPagedResponse<Follower>;
    FollowingsRequest = JsonPagedResponse<Follower>;
    HoverCardRequest = HoverCard;
    JobsRequest = Vec<Job> with FeaturedQuery, LimitQuery, LatestQuery;
    LikesRequest = JsonPagedResponse<Like>;
    ProfileRequest = Profile;
    UserProjectsRequest = JsonPagedResponse<Project>
        with AlbumIdQuery, CategoryQuery, MarmorSetQuery, PanoQuery, SketchfabQuery, VideoQuery, SortingQuery;
    ProjectsRequest = JsonPagedResponse<Project> with RandomizeQuery;
    QuickRequest = Profile;
    RandomProjectsRequest = JsonPagedResponse<Project>;
    SubmissionsRequest = JsonPagedResponse<Submission>;
    TopRowItemsRequest = Vec<TopRowItem> with LimitQuery;
}
