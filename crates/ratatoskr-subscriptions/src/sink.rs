use crate::types::URIReference;
use chrono::{DateTime, Utc};

/// A `Sink` structure is representing the address where the events collected
/// on behalf `ratatoskr` process for the subscriber should be sent.
///
/// The sink's address can also be associated to credentials, which are stored
/// within [`SinkCredentials`] enum.
#[derive(Clone, Debug)]
pub struct Sink {
    pub(crate) sink: URIReference,
    pub(crate) credentials: Option<SinkCredentials>,
}

impl Sink {
    /// Return a new instance of a [`Sink`] structure with an URI reference
    /// which we'll be used to send events to the subscriber.
    #[inline]
    pub fn new(sink: URIReference) -> Self {
        Sink {
            sink,
            credentials: None,
        }
    }

    /// Retrieves the URI-reference which events destined to the subscriber
    /// will be sinked.
    #[inline]
    pub fn sink(&self) -> &URIReference {
        &self.sink
    }

    /// Optional credentials associated to the sink endpoint.
    #[inline]
    pub fn credentials(&self) -> Option<&SinkCredentials> {
        self.credentials.as_ref()
    }

    /// Store [`SinkCredentials`] enum variant in the current [`SinkCredentials`]
    /// instance.
    pub fn with_credentials(mut self, credentials: SinkCredentials) -> Self {
        self.credentials = Some(credentials);
        self
    }
}

/// This enumeration represents the credentials that should be used while
/// sending events to the subscriber endpoint.
///
/// There is three kinds of credentials:
///
/// 1. [`SinkCredentials::AccessToken`] which is providing an OAuth2-based
/// authentication
/// 2. [`SinkCredentials::Plain`] which is providing a classic identifier
/// and secret pair
/// 3. [`SinkCredentials::RefreshToken`] which is also providing a OAuth2-based
/// authentication, but with a refresh mechanism enabled.
#[derive(Clone, Debug)]
pub enum SinkCredentials {
    /// [`SinkCredentials::AccessToken`] is the variant storing credentials
    /// informations for an OAuth2-based authentication.
    ///
    /// This variant is storing the OAuth2 access token, and its granting type,
    /// with the expiration date associated to it.
    AccessToken {
        token: String,
        ty: String,
        expires_at: DateTime<Utc>,
    },

    /// [`SinkCredentials::Plain`] is the variant storing credentials informations
    /// stored as a couple of an identifier and a secret string. It's generally an
    /// account or a username associated to a password, passphrase or key.
    Plain { identifier: String, secret: String },

    /// [`SinkCredentials::RefreshToken`] is the variant storing credentials
    /// informations for an OAuth2-based authentication with a refresh mechanism.
    ///
    /// Like [`SinkCredentials::AccessToken`], this variant is storing the access
    /// token, its expiration timestamp and its granting type, but it's also storing
    /// the one-time usage refresh token and the endpoint that should be used to
    /// fetch a new access token once the previous one has expired.
    RefreshToken {
        access_token: String,
        refresh_token: String,
        ty: String,
        expires_at: DateTime<Utc>,
        refresh_endpoint: URIReference,
    },
}

impl SinkCredentials {
    /// Retrieve a reference to the credentials access token if available.
    pub fn access_token(&self) -> Option<&str> {
        match *self {
            Self::AccessToken { ref token, .. } => Some(token.as_str()),
            Self::RefreshToken {
                ref access_token, ..
            } => Some(access_token.as_str()),
            _ => None,
        }
    }

    /// Retrieve a reference to the expiration timestamps of the [`SinkCredentials`]
    /// access token if available.
    pub fn access_token_expires_at(&self) -> Option<&DateTime<Utc>> {
        match *self {
            Self::AccessToken { ref expires_at, .. } => Some(expires_at),
            Self::RefreshToken { ref expires_at, .. } => Some(expires_at),
            _ => None,
        }
    }

    /// Retrieve a reference to [`SinkCredentials`] access token type if available.
    pub fn access_token_type(&self) -> Option<&str> {
        match *self {
            Self::AccessToken { ref ty, .. } => Some(ty.as_str()),
            Self::RefreshToken { ref ty, .. } => Some(ty.as_str()),
            _ => None,
        }
    }

    /// Retrieve a reference of the plain identifier if available.
    pub fn identifier(&self) -> Option<&str> {
        match *self {
            Self::Plain { ref identifier, .. } => Some(identifier.as_str()),
            _ => None,
        }
    }

    /// Retrieve a reference to [`SinkCredentials`] refresh token if available.
    pub fn refresh_token(&self) -> Option<&str> {
        match *self {
            Self::RefreshToken {
                ref refresh_token, ..
            } => Some(refresh_token.as_str()),
            _ => None,
        }
    }

    /// Retrieve a reference to [`SinkCredentials`] refresh endpoint if available.
    pub fn refresh_token_endpoint(&self) -> Option<&URIReference> {
        match *self {
            Self::RefreshToken {
                ref refresh_endpoint,
                ..
            } => Some(refresh_endpoint),
            _ => None,
        }
    }

    /// Retrieve a reference of the secret if available.
    pub fn secret(&self) -> Option<&str> {
        match *self {
            Self::Plain { ref secret, .. } => Some(secret.as_str()),
            _ => None,
        }
    }

    /// Retrieve a string representation of the type based on the variant of
    /// the current [`SinkCredentials`] instance.
    pub fn ty(&self) -> &'static str {
        match *self {
            Self::AccessToken { .. } => "ACCESSTOKEN",
            Self::Plain { .. } => "PLAIN",
            Self::RefreshToken { .. } => "REFRESHTOKEN",
        }
    }
}
