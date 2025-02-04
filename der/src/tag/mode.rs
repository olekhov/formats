//! Tag modes.

use crate::{Error, ErrorKind, Result};
use core::{fmt, str::FromStr};

/// Tagging modes: `EXPLICIT` versus `IMPLICIT`.
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
pub enum TagMode {
    /// `EXPLICIT` tagging.
    ///
    /// Tag is added in addition to the inner tag of the type.
    Explicit,

    /// `IMPLICIT` tagging.
    ///
    /// Tag replaces the existing tag of the inner type.
    Implicit,
}

impl Default for TagMode {
    fn default() -> TagMode {
        TagMode::Explicit
    }
}

impl FromStr for TagMode {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        match s {
            "EXPLICIT" | "explicit" => Ok(TagMode::Explicit),
            "IMPLICIT" | "implicit" => Ok(TagMode::Implicit),
            _ => Err(ErrorKind::UnknownTagMode.into()),
        }
    }
}

impl fmt::Display for TagMode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TagMode::Explicit => f.write_str("EXPLICIT"),
            TagMode::Implicit => f.write_str("IMPLICIT"),
        }
    }
}
