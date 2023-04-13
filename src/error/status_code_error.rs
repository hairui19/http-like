#![allow(unused)]

use std::fmt;

pub struct InvalidStatusCode {
    _priv: (),
}

impl InvalidStatusCode {
    pub fn new() -> Self {
        Self { _priv: () }
    }
}

impl fmt::Display for InvalidStatusCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        write!(f, "Invalid status code error")
    }
}

impl fmt::Debug for InvalidStatusCode {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "Invalid status code error")
    }
}

impl std::error::Error for InvalidStatusCode {}
