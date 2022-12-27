use url::{ParseError, Url};

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct URIReference(String);

impl TryInto<Url> for URIReference {
    type Error = ParseError;

    fn try_into(self) -> Result<Url, Self::Error> {
        Url::parse(self.0.as_str())
    }
}
