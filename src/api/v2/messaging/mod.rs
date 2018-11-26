pub mod request_types;

mod conversations;
pub use self::conversations::Conversations;
mod messages;
pub use self::messages::Messages;

use crate::ArtStation;

/// This struct offers access to the v2 messaging endpoints. You get an instance by calling the [`messaging`]
/// method of the [`V2`] struct.
///
/// [`V2`]: ../struct.V2.html
/// [`messaging`]: ../struct.V2.html#method.messaging
pub struct Messaging<'a> {
    art_client: &'a ArtStation,
}

impl<'a> Messaging<'a> {
    const API_MESSAGING: &'static str = "messaging/";

    #[inline]
    pub(crate) fn new(art_client: &'a ArtStation) -> Self {
        Messaging { art_client }
    }

    pub fn conversations(&self) -> Conversations {
        Conversations::new(self.art_client)
    }

    pub fn messages(&self) -> Messages {
        Messages::new(self.art_client)
    }
}
