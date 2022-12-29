use url::{ParseError, Url};

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct URIReference(pub(crate) String);

impl URIReference {
    #[inline]
    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}

impl TryInto<Url> for URIReference {
    type Error = ParseError;

    fn try_into(self) -> Result<Url, Self::Error> {
        Url::parse(self.0.as_str())
    }
}
