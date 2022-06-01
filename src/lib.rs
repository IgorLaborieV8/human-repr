//! ### Generate beautiful human representations of bytes, durations and even throughputs!
//!
//! Easily generate human-readable descriptions directly on primitive numbers, of several kinds:
//! - counts: which get SI prefixes: "k", "M", "G", "T", "P", "E", "Z", "Y";
//! - durations: with support for nanoseconds, millis, µs, secs, and even HH:MM:SS;
//! - throughputs: which get, in addition to SI prefixes, support for /day, /hour, /month, and /sec!!
//!
//! They work on the following Rust primitive types: `u8, u16, u32, u64, u128, usize, f32, f64, i8, i16, i32, i64, i128, isize`.
//! <br>The entity they refer to is configurable, so you can send "B" for bytes, or "it" for iterations, or "errors", etc.
//! <br>Bytes have dedicated methods for convenience.
//!
//! It is also blazingly fast, taking only ~80 ns to generate, and well-tested.
//!
//! You can, for example:
//!
//! ```rust
//! use human_repr::HumanRepr;
//!
//! // counts (bytes or anything)
//! assert_eq!("43.2 MB", 43214321_u32.human_count_bytes());
//! assert_eq!("123.5 kPackets", 123456_u64.human_count("Packets"));
//!
//! // durations
//! assert_eq!("15.6 µs", 0.0000156.human_duration());
//! assert_eq!("10 ms", 0.01.human_duration());
//! assert_eq!("1:14:48", 4488.395.human_duration());
//!
//! // throughputs (bytes or anything)
//! assert_eq!("1.2 MB/s", (1234567. / 1.).human_throughput_bytes());
//! assert_eq!("6.1 tests/m", (10. / 99.).human_throughput("tests"));
//! assert_eq!("9 errors/d", (125. / 1200000.).human_throughput("errors"));
//!
//! ```
//!
//! ## How to use it
//!
//! Add this dependency to your Cargo.toml file:
//!
//! ```toml
//! human-repr = "0"
//! ```
//!
//! Use the trait:
//!
//! ```rust
//! use human_repr::HumanRepr;
//! ```
//!
//! That's it! You can now call on any number:
//!
//! ```rust
//!     fn human_count(self, what: &str) -> String;
//!     fn human_count_bytes(self) -> String;
//!
//!     fn human_duration(self) -> String;
//!
//!     fn human_throughput(self, what: &str) -> String;
//!     fn human_throughput_bytes(self) -> String;
//! ```

mod human_count;
mod human_duration;
mod human_throughput;

const BYTES: &str = "B";

/// Human representation trait, already implemented for all Rust primitive number types.
pub trait HumanRepr: sealed::Sealed + Sized {
    /// Generate a beautiful human count.
    ///
    /// ```
    /// use human_repr::HumanRepr;
    /// assert_eq!("43.2 Mcoins", 43214321u32.human_count("coins"));
    /// ```
    fn human_count(self, what: &str) -> String;
    /// Generate a beautiful human count.
    ///
    /// ```
    /// use human_repr::HumanRepr;
    /// assert_eq!("43.2 MB", 43214321u32.human_count_bytes());
    /// ```
    fn human_count_bytes(self) -> String {
        self.human_count(BYTES)
    }

    /// Generate a beautiful human duration.
    ///
    /// ```
    /// use human_repr::HumanRepr;
    /// assert_eq!("160 ms", 0.1599999.human_duration());
    /// ```
    fn human_duration(self) -> String;

    /// Generate a beautiful human throughput.
    ///
    /// ```
    /// use human_repr::HumanRepr;
    /// assert_eq!("1.2 Mcoins/s", 1234567.8.human_throughput("coins"));
    /// ```
    fn human_throughput(self, what: &str) -> String;
    /// Generate a beautiful human throughput.
    ///
    /// ```
    /// use human_repr::HumanRepr;
    /// assert_eq!("1.2 MB/s", 1234567.8.human_throughput_bytes());
    /// ```
    fn human_throughput_bytes(self) -> String {
        self.human_throughput(BYTES)
    }
}

macro_rules! impl_human {
    {$($t:ty),+} => {$(
        impl HumanRepr for $t {
            fn human_count(self, what: &str) -> String {
                human_count::conv(self as f64, what)
            }
            fn human_duration(self) -> String {
                human_duration::conv(self as f64)
            }
            fn human_throughput(self, what: &str) -> String {
                human_throughput::conv(self as f64, what)
            }
        }
    )+}
}
impl_human!(u8, u16, u32, u64, u128, usize, f32, f64, i8, i16, i32, i64, i128, isize);

mod sealed {
    pub trait Sealed {}
    macro_rules! impl_sealed {
        {$($t:ty),+} => {
            $(impl Sealed for $t {})+
        }
    }
    impl_sealed!(u8, u16, u32, u64, u128, usize, f32, f64, i8, i16, i32, i64, i128, isize);
}

const SPACE: &str = {
    match cfg!(feature = "nospace") {
        true => "",
        false => " ",
    }
};

#[inline]
fn rounded(val: f64, dec: usize) -> f64 {
    match dec {
        1 => (val * 10.).round() / 10.,
        2 => (val * 100.).round() / 100.,
        // 0 => val.round(),
        _ => unreachable!(),
    }
}
