use crate::json_def::v2::Permissions;
use crate::request::query::UserId;

make_request! {
    PermissionsRequest = Permissions with UserId;
}
