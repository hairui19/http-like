#![allow(unused)]

use std::fmt;

pub struct InvalidMethod {
    _priv: (),
}

impl InvalidMethod {
    pub fn new() -> Self {
        InvalidMethod { _priv: () }
    }
}

impl fmt::Display for InvalidMethod {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "invalid method")
    }
}

impl fmt::Debug for InvalidMethod {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Invalid method")
    }
}

impl std::error::Error for InvalidMethod {}
