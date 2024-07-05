use crate::repr::Sep;
use crate::utils;
use std::fmt;
use std::fmt::Formatter;

/// The system used to represent prefixes.
#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
pub enum System {
    /// SI system (1000 divisor).
    SI,
    /// SI system with binary prefix (1024 divisor).
    SI2,
    /// IEC system (1024 divisor).
    IEC,
}

fn spec_div<'a>(sys: Option<System>, specs: [&'a [&'a str]; 3]) -> (&'a [&'a str], f64) {
    const SYS_DEFAULT: System = match (cfg!(feature = "iec"), cfg!(feature = "1024")) {
        (false, false) => System::SI,
        (false, true) => System::SI2,
        (true, _) => System::IEC,
    };
    match sys.unwrap_or(SYS_DEFAULT) {
        System::SI => (specs[0], 1000.),
        System::SI2 => (specs[1], 1024.),
        System::IEC => (specs[2], 1024.),
    }
}

/// Human metric prefix representation for large values, i.e., abs(val) >= 1.
pub fn large_repr(
    val: f64,
    unit: &str,
    sys: Option<System>,
    sep: Sep, // separator varies per entity.
    f: &mut Formatter<'_>,
) -> fmt::Result {
    const M: usize = 11;
    const SPEC_SI: [&str; M] = ["", "k", "M", "G", "T", "P", "E", "Z", "Y", "R", "Q"];
    const SPEC_SI2: [&str; M] = const {
        let mut spec = SPEC_SI;
        spec[1] = "K"; // only k is different from SI (1000).
        spec
    };
    const SPEC_IEC: [&str; M] = [
        "", "Ki", "Mi", "Gi", "Ti", "Pi", "Ei", "Zi", "Yi", "Ri", "Qi",
    ];
    const DECIMALS: [usize; M] = [1, 1, 1, 2, 2, 2, 2, 2, 2, 2, 2];

    let (spec, div) = spec_div(sys, [&SPEC_SI, &SPEC_SI2, &SPEC_IEC]);

    let (mut int, mut fract) = (val.trunc(), val.fract());
    let mut it = spec.iter().zip(DECIMALS).peekable();
    while let Some((prefix, dec)) = it.next() {
        match utils::rounded(int + fract, dec) {
            r if r.abs() >= div && it.peek().is_some() => {
                fract = (fract + int % div) / div;
                int = (int / div).trunc() + fract.trunc();
                fract = fract.fract();
            }
            r => {
                match f.alternate() {
                    true => write!(f, "{}", int + fract)?, // alternate is precise.
                    false => write!(f, "{r:.*}", utils::decimals(r))?, // rounded with up to dec decimals.
                }
                return match !prefix.is_empty() || !unit.is_empty() {
                    true => write!(f, "{sep}{prefix}{unit}"),
                    false => Ok(()), // avoid "123 " when no prefix and no unit but Sep.
                };
            }
        }
    }
    unreachable!()
}

/// Human metric prefix representation for small values, i.e., abs(val) < 1.
pub fn small_repr(
    val: f64,
    unit: &str,
    sys: Option<System>,
    sep: Sep,
    f: &mut Formatter<'_>,
) -> fmt::Result {
    large_repr(val, unit, sys, sep, f)
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! base {
        ($fmt:literal => $val:expr, $unit:expr, $sys:expr, $sep:expr) => {{
            struct H;
            impl fmt::Display for H {
                fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
                    large_repr($val as f64, $unit, $sys, $sep, f)
                }
            }
            format!($fmt)
        }};
    }

    #[test]
    fn operation() {
        macro_rules! case {
            ($val:expr) => {
                base!("{H}" => $val, "", Some(System::SI), Sep::NoSep)
            };
        }
        assert_eq!("123k", case!(123000_u64));
        assert_eq!("123.5k", case!(123456_u64));
        assert_eq!("1k", case!(999.96));
        assert_eq!("23", case!(23u8));
        assert_eq!("23", case!(23i8));
        assert_eq!("23.5", case!(23.5123));
        assert_eq!("-23", case!((-23i8)));
        assert_eq!("1k", case!(1025u16));
        assert_eq!("-1k", case!((-1025i16)));
        assert_eq!("43.2M", case!(43214321u32));
        assert_eq!("23.4G", case!(23403454432_u64));
        assert_eq!("0.2", case!(0.23403454432));
        assert_eq!("23.43G", case!(23433454432_u64));
        assert_eq!("18.45E", case!(u64::MAX));
        assert_eq!("9.22E", case!(i64::MAX));
        assert_eq!("-9.22E", case!(i64::MIN));
        assert_eq!("1R", case!(999.999e24));
        assert_eq!("1.12R", case!(1.123456e27));
        assert_eq!("1.12Q", case!(1.123456e30));
        assert_eq!("1123.46Q", case!(1.123456e33));
    }

    #[test]
    fn precision() {
        macro_rules! case {
            ($val:expr) => {
                base!("{H:#}" => $val, "", Some(System::SI), Sep::NoSep)
            };
        }
        assert_eq!("123k", case!(123000_u64));
        assert_eq!("123.456k", case!(123456_u64));
        assert_eq!("23.5123", case!(23.5123));
        assert_eq!("-23", case!(-23i8));
        assert_eq!("1.025k", case!(1025u16));
        assert_eq!("0.23403454432", case!(0.23403454432));
        assert_eq!("23G", case!(23e9));
        assert_eq!("23.000000001G", case!(23e9 + 1.));
        assert_eq!("0.999999999999R", case!(999.999999999e24));
        assert_eq!("1.123456R", case!(1.123456e27));
    }

    #[test]
    fn units() {
        macro_rules! case {
            ($val:expr, $unit:expr) => {
                base!("{H}" => $val, $unit, Some(System::SI), Sep::NoSep)
            };
        }
        assert_eq!("123", case!(123, ""));
        assert_eq!("123Crabs", case!(123, "Crabs"));
        assert_eq!("123🦀", case!(123, "🦀"));
        assert_eq!("123°C", case!(123, "°C"));

        assert_eq!("123.5", case!(123.5, ""));
        assert_eq!("123.5Crabs", case!(123.5, "Crabs"));
        assert_eq!("123.5🦀", case!(123.5, "🦀"));
        assert_eq!("123.5°C", case!(123.5, "°C"));

        assert_eq!("123k", case!(123e3, ""));
        assert_eq!("123kCrabs", case!(123e3, "Crabs"));
        assert_eq!("123k🦀", case!(123e3, "🦀"));
        assert_eq!("123k°C", case!(123e3, "°C"));

        assert_eq!("123.5M", case!(123.5e6, ""));
        assert_eq!("123.5MCrabs", case!(123.5e6, "Crabs"));
        assert_eq!("123.5M🦀", case!(123.5e6, "🦀"));
        assert_eq!("123.5M°C", case!(123.5e6, "°C"));
    }

    #[test]
    fn separators() {
        macro_rules! case {
            ($val:expr, $unit:expr) => {
                base!("{H}" => $val, $unit, Some(System::SI), Sep::WithSep)
            };
        }
        assert_eq!("123", case!(123, ""));
        assert_eq!("123 Crabs", case!(123, "Crabs"));
        assert_eq!("123 🦀", case!(123, "🦀"));
        assert_eq!("123 °C", case!(123, "°C"));

        assert_eq!("123.5", case!(123.5, ""));
        assert_eq!("123.5 Crabs", case!(123.5, "Crabs"));
        assert_eq!("123.5 🦀", case!(123.5, "🦀"));
        assert_eq!("123.5 °C", case!(123.5, "°C"));

        assert_eq!("123 k", case!(123e3, ""));
        assert_eq!("123 kCrabs", case!(123e3, "Crabs"));
        assert_eq!("123 k🦀", case!(123e3, "🦀"));
        assert_eq!("123 k°C", case!(123e3, "°C"));

        assert_eq!("123.5 M", case!(123.5e6, ""));
        assert_eq!("123.5 MCrabs", case!(123.5e6, "Crabs"));
        assert_eq!("123.5 M🦀", case!(123.5e6, "🦀"));
        assert_eq!("123.5 M°C", case!(123.5e6, "°C"));
    }

    #[test]
    fn systems() {
        macro_rules! case {
            ($val:expr, $sys:expr) => {
                base!("{H}" => $val, "", Some($sys), Sep::NoSep)
            };
        }
        assert_eq!("1Ki", case!(1024, System::IEC));
        assert_eq!("1Mi", case!(1048576, System::IEC));
        assert_eq!("1Gi", case!(1073741824, System::IEC));
        assert_eq!("1Ti", case!(1099511627776u64, System::IEC));
        assert_eq!("1Pi", case!(1125899906842624u64, System::IEC));
        assert_eq!("1Ei", case!(1152921504606846976u64, System::IEC));

        assert_eq!("1k", case!(1024, System::SI));
        assert_eq!("1M", case!(1048576, System::SI));
        assert_eq!("1.07G", case!(1073741824, System::SI));
        assert_eq!("1.1T", case!(1099511627776u64, System::SI));
        assert_eq!("1.13P", case!(1125899906842624u64, System::SI));
        assert_eq!("1.15E", case!(1152921504606846976u64, System::SI));
    }
}
