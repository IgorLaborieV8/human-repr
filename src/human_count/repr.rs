use crate::metric_prefix::System;
use crate::repr::Sep;
use std::fmt::{self, Display, Formatter};

/// Representation options for Human Count.
#[derive(Debug, Default, Copy, Clone, PartialEq, PartialOrd)]
pub struct ReprCount {
    pub sys: Option<System>,
    pub sep: Option<Sep>,
}

impl Display for ReprCount {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let ReprCount { sys, sep } = *self;
        match (sys, sep) {
            (Some(system), None) => write!(f, "{system:?}"),
            (Some(system), Some(sep)) => write!(f, "{system:?}({sep:?})"),
            (None, Some(sep)) => write!(f, "{sep:?}"),
            (None, None) => Ok(()),
        }
    }
}

impl From<System> for Option<ReprCount> {
    fn from(system: System) -> Self {
        Some(ReprCount {
            sys: Some(system),
            ..Default::default()
        })
    }
}

impl From<Sep> for ReprCount {
    fn from(sep: Sep) -> Self {
        Self {
            sep: Some(sep),
            ..Self::default()
        }
    }
}

impl ReprCount {
    pub fn new(system: impl Into<Option<System>>, sep: impl Into<Option<Sep>>) -> Self {
        Self {
            sys: system.into(),
            sep: sep.into(),
        }
    }
}
