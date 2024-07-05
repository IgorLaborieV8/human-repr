use std::fmt::{self, Display, Formatter};

/// The separator used between numbers and the prefixes.
#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
pub enum Sep {
    WithSep,
    NoSep,
}

impl Display for Sep {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.resolve())
    }
}

impl Sep {
    pub fn resolve(&self) -> &'static str {
        match self {
            Self::WithSep => " ",
            Self::NoSep => "",
        }
    }
}
