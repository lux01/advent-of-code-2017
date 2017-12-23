//! A collection of utility functions used across the advent calendar

use std::fs::File;
use std::path::Path;
use std::io::Read;

use nom::digit;
use std::str::FromStr;

/// Open a file by name and read its contents into a string. Panics if an error occurs.
pub fn file_as_string<P: AsRef<Path>>(path: P) -> String {
    let mut buffer = String::new();
    let mut file = File::open(path).expect("Failed to open input file.");
    file.read_to_string(&mut buffer).expect(
        "Failed to read input file.",
    );
    buffer
}


named!(pub parse_usize(&str) -> usize,
    map!(digit, |s| FromStr::from_str(s).unwrap())
);

named!(pub parse_isize(&str) -> isize,
    map!(digit, |s| FromStr::from_str(s).unwrap())
);