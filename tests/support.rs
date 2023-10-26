#![allow(unused_macros, dead_code)]

use std::fmt;

#[derive(PartialEq, Eq)]
pub struct MultiLineString<'a>(pub &'a str);

impl<'a> fmt::Debug for MultiLineString<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(self.0)
    }
}

// Modified from https://github.com/colin-kiegel/rust-pretty-assertions/issues/24#issuecomment-520613247
// to optionally turn off the pretty printing so you can copy the actual string.
macro_rules! assert_debug_eq {
    ($left:expr, $right:expr) => {
        if let Ok(_diff) = std::env::var("DIFF") {
            pretty_assertions::assert_eq!(MultiLineString(&format!("{:#?}", $left)), MultiLineString($right));
        } else {
            std::assert_eq!(MultiLineString(&format!("{:#?}", $left)), MultiLineString($right));
        }
    };
}

macro_rules! assert_eq {
    ($left:expr, $right:expr) => {
        if let Ok(_diff) = std::env::var("DIFF") {
            pretty_assertions::assert_eq!($left, $right);
        } else {
            std::assert_eq!($left, $right);
        }
    };
}

pub fn assert_vec_matches<T: PartialEq>(a: &Vec<T>, b: &Vec<T>) {
    let matching = a.iter().zip(b.iter()).filter(|&(a, b)| a == b).count();
    assert!(matching == a.len() && matching == b.len())
}

macro_rules! cast {
    ($target: expr, $pat: path) => {{
        if let $pat(a) = $target {
            // #1
            a
        } else {
            panic!("mismatch variant when cast to {}", stringify!($pat)); // #2
        }
    }};
}
