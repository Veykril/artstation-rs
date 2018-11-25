pub mod messaging;

mod cart;
mod notifications;

pub use self::cart::Cart;
pub use self::notifications::Notifications;

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
}

pub(crate) use self::private::UnreadCountRequest;
mod private {
    use crate::json_def::v2::UnreadCount;
    use crate::request::query::IncludeMarketplaceQuery;

    make_request! {
        UnreadCountRequest = UnreadCount with IncludeMarketplaceQuery;
    }
}
