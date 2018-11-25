//! Request types for the V2 Api

use crate::json_def::v2::{Count, UnreadCount};
use crate::request::query::IncludeMarketplaceQuery;

make_request! {
    CountRequest = Count;
    UnreadCountRequest = UnreadCount with IncludeMarketplaceQuery;
}
