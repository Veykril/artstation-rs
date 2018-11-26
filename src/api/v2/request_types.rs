//! Request types for the V2 Api

use crate::json_def::v2::{Count, Current, UnreadCount, UserFeeds};
use crate::request::{query::IncludeMarketplaceQuery, response::JsonPagedResponse};

make_request! {
    CountRequest = Count;
    CurrentRequest = Current;
    UnreadCountRequest = UnreadCount with IncludeMarketplaceQuery;
    UserFeedsRequest = JsonPagedResponse<UserFeeds>;
}
