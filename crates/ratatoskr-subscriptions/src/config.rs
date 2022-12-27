use std::fmt;

/// An enumeration to represents all possible values of a CloudEvent
/// subscription config mapping.
#[derive(Clone, Debug)]
pub enum ConfigValue {
    /// Represents a [`bool`] value.
    Boolean(bool),

    /// Represents a [`float`] value.
    Float(f64),

    /// Represents an integer [`i64`] value.
    Integer(i64),

    /// Represents a [`String`] value.
    String(String),
}

impl From<bool> for ConfigValue {
    fn from(value: bool) -> Self {
        Self::Boolean(value)
    }
}

impl From<f32> for ConfigValue {
    fn from(value: f32) -> Self {
        Self::Float(value.into())
    }
}

impl From<f64> for ConfigValue {
    fn from(value: f64) -> Self {
        Self::Float(value)
    }
}

impl From<i8> for ConfigValue {
    fn from(value: i8) -> Self {
        Self::Integer(value.into())
    }
}

impl From<i16> for ConfigValue {
    fn from(value: i16) -> Self {
        Self::Integer(value.into())
    }
}

impl From<i32> for ConfigValue {
    fn from(value: i32) -> Self {
        Self::Integer(value.into())
    }
}

impl From<i64> for ConfigValue {
    fn from(value: i64) -> Self {
        Self::Integer(value)
    }
}

impl From<u8> for ConfigValue {
    fn from(value: u8) -> Self {
        Self::Integer(value.into())
    }
}

impl From<u16> for ConfigValue {
    fn from(value: u16) -> Self {
        Self::Integer(value.into())
    }
}

impl From<u32> for ConfigValue {
    fn from(value: u32) -> Self {
        Self::Integer(value.into())
    }
}

impl From<&str> for ConfigValue {
    fn from(value: &str) -> Self {
        Self::String(String::from(value))
    }
}

impl From<String> for ConfigValue {
    fn from(value: String) -> Self {
        Self::String(value)
    }
}

impl Eq for ConfigValue {}

impl PartialEq for ConfigValue {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Boolean(a), Self::Boolean(b)) => *a == *b,
            (Self::Float(a), Self::Float(b)) => *a == *b,
            (Self::Integer(a), Self::Integer(b)) => *a == *b,
            (Self::String(a), Self::String(b)) => *a == *b,
            _ => false,
        }
    }
}

impl PartialEq<bool> for ConfigValue {
    fn eq(&self, other: &bool) -> bool {
        match self {
            Self::Boolean(value) => *value == *other,
            _ => false,
        }
    }
}

impl PartialEq<f32> for ConfigValue {
    fn eq(&self, other: &f32) -> bool {
        match self {
            Self::Float(value) => *value == *other as f64,
            _ => false,
        }
    }
}

impl PartialEq<f64> for ConfigValue {
    fn eq(&self, other: &f64) -> bool {
        match self {
            Self::Float(value) => *value == *other,
            _ => false,
        }
    }
}

impl PartialEq<i8> for ConfigValue {
    fn eq(&self, other: &i8) -> bool {
        match self {
            Self::Integer(value) => *value == *other as i64,
            _ => false,
        }
    }
}

impl PartialEq<i16> for ConfigValue {
    fn eq(&self, other: &i16) -> bool {
        match self {
            Self::Integer(value) => *value == *other as i64,
            _ => false,
        }
    }
}

impl PartialEq<i32> for ConfigValue {
    fn eq(&self, other: &i32) -> bool {
        match self {
            Self::Integer(value) => *value == *other as i64,
            _ => false,
        }
    }
}

impl PartialEq<i64> for ConfigValue {
    fn eq(&self, other: &i64) -> bool {
        match self {
            Self::Integer(value) => *value == *other,
            _ => false,
        }
    }
}

impl PartialEq<u8> for ConfigValue {
    fn eq(&self, other: &u8) -> bool {
        match self {
            Self::Integer(value) => *value == *other as i64,
            _ => false,
        }
    }
}

impl PartialEq<u16> for ConfigValue {
    fn eq(&self, other: &u16) -> bool {
        match self {
            Self::Integer(value) => *value == *other as i64,
            _ => false,
        }
    }
}

impl PartialEq<u32> for ConfigValue {
    fn eq(&self, other: &u32) -> bool {
        match self {
            Self::Integer(value) => *value == *other as i64,
            _ => false,
        }
    }
}

impl PartialEq<&str> for ConfigValue {
    fn eq(&self, other: &&str) -> bool {
        match self {
            Self::String(value) => *value == *other,
            _ => false,
        }
    }
}

impl PartialEq<String> for ConfigValue {
    fn eq(&self, other: &String) -> bool {
        match self {
            Self::String(value) => *value == *other,
            _ => false,
        }
    }
}

impl fmt::Display for ConfigValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Boolean(value) => write!(f, "{}", value),
            Self::Float(value) => write!(f, "{}", value),
            Self::Integer(value) => write!(f, "{}", value),
            Self::String(value) => write!(f, "{}", value),
        }
    }
}
