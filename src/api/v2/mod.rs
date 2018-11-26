pub mod messaging;
pub mod request_types;

mod activity_feed;
mod cart;
mod notifications;

pub use self::cart::Cart;
pub use self::notifications::Notifications;
pub use self::activity_feed::ActivityFeed;

use self::messaging::Messaging;
use crate::ArtStation;

/// This struct offers access to the api v2 endpoints. You get an instance by calling the [`v2`]
/// method of the ArtStation struct. All of the requests done from here onwards only work if the
/// client is logged in, otherwise the response will be an error status code.
///
/// [`v2`]: ../../struct.ArtStation.html#method.v2
pub struct V2<'a> {
    art_client: &'a ArtStation,
}

impl<'a> V2<'a> {
    const API_BASE: &'static str = "/api/v2/";

    #[inline]
    pub(crate) fn new(art_client: &'a ArtStation) -> Self {
        V2 { art_client }
    }

    pub fn cart(&self) -> Cart {
        Cart::new(self.art_client)
    }

    pub fn notifications(&self) -> Notifications {
        Notifications::new(self.art_client)
    }

    pub fn messaging(&self) -> Messaging {
        Messaging::new(self.art_client)
    }

    pub fn activiy_feed(&self) -> ActivityFeed {
        ActivityFeed::new(self.art_client)
    }
}
