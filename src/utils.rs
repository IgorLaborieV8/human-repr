use std::fmt::{self, Display, Write};

/// Round a value to the given number of decimals.
pub fn rounded(val: f64, dec: usize) -> f64 {
    match dec {
        0 => val.round(),
        1 => (val * 10.).round() / 10.,
        2 => (val * 100.).round() / 100.,
        _ => unreachable!(),
    }
}

/// Return the minimum number of decimals to display.
pub fn decimals(r: f64) -> usize {
    match r {
        _ if r.fract() == 0. => 0,
        _ if (r * 10.).fract() == 0. => 1,
        _ => 2,
    }
}

#[doc(hidden)]
pub struct HeapLessCompare<'a, I>(&'a mut I);

impl<I: Iterator<Item = u8>> Write for HeapLessCompare<'_, I> {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        s.bytes().try_for_each(|c| match self.0.next() {
            Some(ex) if c == ex => Ok(()),
            _ => Err(fmt::Error),
        })
    }
}

pub fn compare_display(expected: &str, human: &impl Display) -> bool {
    let mut it = expected.bytes();
    write!(HeapLessCompare(it.by_ref()), "{human}").is_ok_and(|()| it.len() == 0)
}
