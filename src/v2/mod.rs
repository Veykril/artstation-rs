mod notifications;
mod messaging;

use self::notifications::Notifications;
use self::messaging::Messaging;
use ArtStation;

pub struct V2<'a> {
    art_client: &'a ArtStation,
}

impl<'a> V2<'a> {
    const API_BASE: &'static str = "/api/v2/";

    #[inline]
    pub(crate) fn new(art_client: &'a ArtStation) -> Self {
        V2 { art_client }
    }

    pub fn notifications(&self) -> Notifications {
        Notifications::new(self.art_client)
    }

    pub fn messaging(&self) -> Messaging {
        Messaging::new(self.art_client)
    }
}

use json_def::{v2::UnreadCount};
use request::ArtStationRequest;
use request::query::IncludeMarketPlaceQuery;

make_request! {
    UnreadCountRequest = UnreadCount with IncludeMarketPlaceQuery;
}
