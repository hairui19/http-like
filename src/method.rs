#![allow(unused)]

use std::fmt;
use crate::error::method_error::InvalidMethod;

#[derive(Copy, Clone, PartialEq)]
pub enum Method {
    Options,
    Get,
    Post,
    Put,
    Delete,
    Head,
    Trace,
    Connect,
    Patch,
}

impl Method {
    pub fn from_ascii_bytes(src: &[u8]) -> Result<Method, InvalidMethod> {
        match src.len() {
            0 => Err(InvalidMethod::new()),
            3 => match src {
                b"GET" => Ok(Method::Get),
                b"PUT" => Ok(Method::Put),
                _ => Err(InvalidMethod::new()),
            },
            4 => match src {
                b"POST" => Ok(Method::Post),
                b"HEAD" => Ok(Method::Head),
                _ => Err(InvalidMethod::new()),
            },
            5 => match src {
                b"TRACE" => Ok(Method::Trace),
                b"PATCH" => Ok(Method::Patch),
                _ => Err(InvalidMethod::new()),
            },
            6 => match src {
                b"DELETE" => Ok(Method::Delete),
                _ => Err(InvalidMethod::new()),
            },
            7 => match src {
                b"CONNECT" => Ok(Method::Connect),
                b"OPTIONS" => Ok(Method::Options),
                _ => Err(InvalidMethod::new()),
            },
            _ => Err(InvalidMethod::new()),
        }
    }

    /// Whether a method is considered "safe",
    /// meaning that the request is essentially read-only.
    pub fn is_safe(&self) -> bool {
        match self {
            Method::Get | Method::Head | Method::Options | Method::Trace => true,
            _ => false,
        }
    }

    /// Whether a method is considered "idempotent", meaning that
    /// the request will have the same result even if executed multiple time.
    pub fn is_idempotent(&self) -> bool {
        match self {
            Method::Delete | Method::Put => true,
            _ => false,
        }
    }

    pub fn as_str(&self) -> &str {
        match self {
            Method::Options => "OPTIONS",
            Method::Get => "GET",
            Method::Post => "POST",
            Method::Put => "PUT",
            Method::Delete => "DELETE",
            Method::Head => "HEAD",
            Method::Trace => "TRACE",
            Method::Connect => "CONNECT",
            Method::Patch => "PATCH",
        }
    }
}

impl AsRef<str> for Method {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl<'a> PartialEq<&'a Method> for Method {
    fn eq(&self, other: &&'a Method) -> bool {
        self == *other
    }
}

impl<'a> PartialEq<Method> for &'a Method {
    fn eq(&self, other: &Method) -> bool {
        *self == other
    }
}

impl PartialEq<str> for Method {
    fn eq(&self, other: &str) -> bool {
        self.as_str() == other
    }
}

impl PartialEq<Method> for str {
    fn eq(&self, other: &Method) -> bool {
        self == other.as_str()
    }
}

impl<'a> PartialEq<&'a str> for Method {
    fn eq(&self, other: &&'a str) -> bool {
        self.as_str() == *other
    }
}

impl<'a> PartialEq<Method> for &'a str {
    fn eq(&self, other: &Method) -> bool {
        *self == other.as_str()
    }
}

impl fmt::Debug for Method {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        f.debug_tuple("http:Method").field(&self.as_str()).finish()
    }
}

impl fmt::Display for Method {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        f.debug_tuple("http:Method").field(&self.as_str()).finish()
    }
}

impl Default for Method {
    fn default() -> Method {
        Method::Get
    }
}

impl<'a> From<&'a Method> for Method {
    fn from(value: &'a Method) -> Method {
        value.clone()
    }
}

impl<'a> TryFrom<&'a str> for Method {
    type Error = InvalidMethod; 

    fn try_from(value: &'a str) -> Result<Method, InvalidMethod> {
        Method::from_ascii_bytes(value.as_bytes())
    }
}
