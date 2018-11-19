mod conversations;

use self::conversations::Conversations;
use ArtStation;

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
}
