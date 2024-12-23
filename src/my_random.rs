use std::{
    fmt::Debug,
    ops::{Bound, RangeBounds},
    str::FromStr,
    usize,
};

use chrono::Utc;
use num::Num;

pub fn singl_random<N: Number + FromStr>() -> N
where
    <N as FromStr>::Err: Debug,
{
    let last = Utc::now().time().to_string().chars().last().unwrap();
    last.to_string().trim().parse::<N>().unwrap()
}

pub fn range_random<N, R>(range: R) -> N
where
    N: Number + Debug + FromStr,
    R: RangeBounds<N>,
    <N as FromStr>::Err: Debug,
{
    let range = Range::new(range);
    let mut num = singl_random::<N>();
    let max_digit = if let Some(max_digit) = range.end {
        max_digit.digit()
    } else {
        singl_random::<usize>()
    };
    loop {
        if singl_random::<N>() < N::new(7) {
            if range.is_range(num) {
                break;
            } else {
                num = singl_random();
                continue;
            }
        }
        if num.digit() == max_digit {
            if range.is_range(num) {
                break;
            } else {
                num = singl_random();
                continue;
            }
        }
        if max_digit < num.digit() {
            num.sub_digit();
            if range.is_range(num) {
                break;
            } else {
                num = singl_random();
                continue;
            }
        }
        num = num.add_digit(singl_random());
    }
    num
}

#[derive(Debug)]
pub struct Range<N: Number> {
    start: Option<N>,
    end: Option<N>,
}

impl<N: Number + Debug> Range<N> {
    pub fn new<R: RangeBounds<N>>(range: R) -> Range<N> {
        let start = match range.start_bound() {
            Bound::Included(n) | Bound::Excluded(n) => Some(*n),
            _ => None,
        };
        let end = match range.end_bound() {
            Bound::Excluded(&n) => Some(n - N::one()),
            Bound::Included(n) => Some(*n),
            _ => None,
        };
        Range { start, end }
    }

    /// # Example
    /// ```
    /// use random::Range;
    /// fn main() {
    /// let range: Range<i32> = Range::new(4..10);
    /// assert!(range.is_range(4));
    /// assert!(!range.is_range(10));
    /// let range: Range<i32> = Range::new(10..);
    /// assert!(!range.is_range(4));
    /// assert!(range.is_range(10));
    /// let range: Range<i32> = Range::new(..10);
    /// assert!(range.is_range(4));
    /// assert!(!range.is_range(10));
    /// }
    /// ```
    pub fn is_range(&self, num: N) -> bool {
        if let Some(start) = self.start {
            if let Some(end) = self.end {
                start <= num && num <= end
            } else {
                start <= num
            }
        } else {
            num <= self.end.unwrap()
        }
    }
}

pub trait Number: Num + PartialOrd + Copy {
    fn new<N: Num>(n: u128) -> N;
    fn digit(&self) -> usize;
    fn add_digit(&self, rhs: Self) -> Self;
    fn sub_digit(&mut self);
}

macro_rules! number {
    ($($integer:ty) *) => {
        $(impl Number for $integer {
            fn new<N: Num>(n: u128) -> N {
                let mut num = N::zero();
                for _ in 0..n {
                    num = num.add(N::one());
                }
                num
            }
            fn digit(&self) -> usize {
                self.to_string().chars().count()
            }
            fn add_digit(&self, rhs: Self) -> Self {
                let result = rhs.to_string() + &self.to_string();
                result.trim().parse().unwrap()
            }
            fn sub_digit(&mut self)  {
                self.to_string().pop();
            }
        })*
    };
}

number!(u8 u16 u32 u64 u128 i8 i16 i32 i64 i128 usize isize);

#[cfg(test)]
mod test {
    use termimad::MadSkin;

    use crate::my_random::{range_random, singl_random};

    const DIVIDE: usize = 3;

    fn show_bar_graph<const N: usize>(data: [usize; N]) {
        let mut bars = String::new();

        for (i, &value) in data.iter().enumerate() {
            let p = if i < 10 {
                &format!(" {}: {}\n", i, "█".repeat(value / DIVIDE))
            } else {
                &format!("{}: {}\n", i, "█".repeat(value / DIVIDE))
            };
            bars.push_str(p);
        }

        MadSkin::default().print_text(&bars);
    }

    fn singl_random_repeat<const MAX: usize>() -> [usize; 10] {
        let mut list = [0; 10];
        for _ in 0..MAX {
            list[singl_random::<usize>()] += 1;
        }
        list
    }

    fn range_random_repeat<const N: usize, const MAX: usize>() -> [usize; N] {
        let mut list = [0; N];
        for _ in 0..MAX {
            list[range_random(0..N)] += 1;
        }
        list
    }

    #[test]
    fn singl_random_graph() {
        let data = singl_random_repeat::<1000>();
        show_bar_graph(data);
    }

    #[test]
    fn range_random_graph() {
        let data = range_random_repeat::<15, 1000>();
        show_bar_graph(data);
    }
}
