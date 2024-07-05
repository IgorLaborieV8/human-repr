mod human;
mod repr;

use crate::{metric_prefix, utils, Sep};
pub use human::HumanCount;
pub use repr::ReprCount;
use std::borrow::Cow;
use std::fmt::{self, Debug, Display, Formatter};

/// The HumanCount data object, ready to generate Display and Debug representations.
#[derive(PartialEq, PartialOrd)]
pub struct HumanCountData<'a> {
    // TODO change <'a> to <const U: &'static str> as soon as it lands on stable.
    pub val: f64,
    pub unit: Cow<'a, str>,
    pub repr: Option<ReprCount>,
}

impl Display for HumanCountData<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        const SEP_DEFAULT: Sep = match cfg!(feature = "sep_count") {
            true => Sep::WithSep,
            false => Sep::NoSep,
        };

        let HumanCountData { val, repr, .. } = *self;
        let ReprCount { sys, sep } = repr.unwrap_or_default();
        let sep = sep.unwrap_or(SEP_DEFAULT);
        let func = match val.abs() < 1. {
            true => metric_prefix::small_repr,
            false => metric_prefix::large_repr,
        };
        func(val, &self.unit, sys, sep, f)
    }
}

impl Debug for HumanCountData<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let mut d = f.debug_struct("HumanCount");
        d.field("val", &self.val).field("unit", &self.unit);
        if let Some(repr) = &self.repr {
            d.field("repr", &format_args!("{repr}"));
        }
        d.finish()?;
        write!(f, " -> ")?;
        Display::fmt(self, f)
    }
}

impl PartialEq<HumanCountData<'_>> for &str {
    fn eq(&self, other: &HumanCountData<'_>) -> bool {
        utils::compare_display(self, other)
    }
}

impl PartialEq<&str> for HumanCountData<'_> {
    fn eq(&self, other: &&str) -> bool {
        other == self
    }
}

#[cfg(feature = "parse")]
mod parse {
    use super::HumanCountData;

    impl std::str::FromStr for HumanCountData<'_> {
        type Err = &'static str;

        fn from_str(_s: &str) -> Result<Self, Self::Err> {
            todo!()
        }
    }

    #[cfg(test)]
    mod tests {
        #[test]
        fn parse() -> Result<(), serde_json::Error> {
            todo!()
        }
    }
}

#[cfg(feature = "serde")]
mod serde {
    use super::HumanCountData;
    use serde::{de, Deserialize, Deserializer, Serialize, Serializer};

    impl Serialize for HumanCountData<'_> {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            serializer.collect_str(&format_args!("{:#}", self))
        }
    }

    impl<'de> Deserialize<'de> for HumanCountData<'_> {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = <&str>::deserialize(deserializer)?;
            s.parse().map_err(de::Error::custom)
        }
    }

    #[cfg(test)]
    mod tests {
        use crate::{HumanCount, HumanCountData};

        #[test]
        fn serde() -> Result<(), serde_json::Error> {
            let h = 123456.human_count_of("X");
            let ser = serde_json::to_string(&h)?;
            assert_eq!(r#"{"val":123456.0,"unit":"X"}"#, &ser);
            let h2 = serde_json::from_str::<HumanCountData>(&ser)?;
            assert_eq!(h, h2);
            Ok(())
        }
    }
}

#[cfg(all(
    test,
    not(any(feature = "1024", feature = "iec", feature = "sep_count"))
))]
mod tests {
    use crate::*;

    #[test]
    fn types() {
        assert_eq!("123", 123_u8.human_count());
        assert_eq!("123", 123_i8.human_count());
        assert_eq!("123", 123_u16.human_count());
        assert_eq!("123", 123_i16.human_count());
        assert_eq!("123", 123_u32.human_count());
        assert_eq!("123", 123_i32.human_count());
        assert_eq!("123", 123_u64.human_count());
        assert_eq!("123", 123_i64.human_count());
        assert_eq!("123", 123_u128.human_count());
        assert_eq!("123", 123_i128.human_count());
        assert_eq!("123", 123_usize.human_count());
        assert_eq!("123", 123_isize.human_count());
        assert_eq!("123", 123_f32.human_count());
        assert_eq!("123", 123_f64.human_count());

        assert_eq!("-123", (-123_i8).human_count());
        assert_eq!("-123", (-123_i16).human_count());
        assert_eq!("-123", (-123_i32).human_count());
        assert_eq!("-123", (-123_i64).human_count());
        assert_eq!("-123", (-123_i128).human_count());
        assert_eq!("-123", (-123_isize).human_count());
        assert_eq!("-123", (-123_f32).human_count());
        assert_eq!("-123", (-123_f64).human_count());
    }

    #[test]
    #[allow(clippy::needless_borrow)]
    fn ownership() {
        let mut a = 42000;
        assert_eq!("42k", a.human_count());
        assert_eq!("42k", (&a).human_count());
        assert_eq!("42k", (&mut a).human_count());
    }

    #[test]
    fn symmetric() {
        assert_eq!(123000_u64.human_count(), "123k");
    }

    #[test]
    fn eq() {
        let c1 = 0.23403454432.human_count();
        assert_eq!("0.2", c1);
        let c2 = 0.234034.human_count();
        assert_eq!("0.2", c2); // same repr.
        assert_ne!(c1, c2); // but different.

        let c3 = 0.234034.human_count_bytes(); // same value.
        assert_eq!("0.2B", c3); // different unit.
        assert_ne!(c2, c3); // also different.
    }
}
