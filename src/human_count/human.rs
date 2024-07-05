use super::HumanCountData;
use crate::{sealed, ReprCount, BYTES};
use std::borrow::Cow;

/// Generate beautiful human-friendly counts.
pub trait HumanCount: sealed::Sealed + Sized {
    /// Generate a beautiful human-friendly count with automatic prefixes.
    #[cfg_attr(
        not(any(feature = "1024", feature = "iec", feature = "sep_count")),
        doc = r#"

```
use human_repr::HumanCount;
assert_eq!("4.2M", 4221432u32.human_count());
```
"#
    )]
    #[inline]
    fn human_count(self) -> HumanCountData<'static> {
        self.human_count_of("")
    }

    /// Generate a beautiful human-friendly count with automatic prefixes and `"B"` (bytes) unit.
    #[cfg_attr(
        not(any(feature = "1024", feature = "iec", feature = "sep_count")),
        doc = r#"

```
use human_repr::HumanCount;
assert_eq!("4.2MB", 4221432u32.human_count_bytes());
```
"#
    )]
    #[inline]
    fn human_count_bytes(self) -> HumanCountData<'static> {
        self.human_count_of(BYTES)
    }

    /// Generate a beautiful human-friendly count with automatic prefixes and a custom unit.
    #[cfg_attr(
        not(any(feature = "1024", feature = "iec", feature = "sep_count")),
        doc = r#"

```
use human_repr::HumanCount;
assert_eq!("4.2Mcoins", 4221432u32.human_count_of("coins"));
```
"#
    )]
    #[inline]
    fn human_count_of<'a>(self, unit: impl Into<Cow<'a, str>>) -> HumanCountData<'a> {
        self.human_count_with(unit, None)
    }

    /// Generate a beautiful human-friendly count with automatic prefixes and custom unit and representation.
    ///
    /// ```
    /// use human_repr::{HumanCount, System};
    /// assert_eq!("4Mibytes", 4221432u32.human_count_with("bytes", System::IEC));
    /// ```
    fn human_count_with<'a>(
        self,
        unit: impl Into<Cow<'a, str>>,
        repr: impl Into<Option<ReprCount>>,
    ) -> HumanCountData<'a>;
}
