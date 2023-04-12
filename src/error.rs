use std::error;
use std::fmt;

#[derive(Debug)]
struct PlaceHolderError;

impl error::Error for PlaceHolderError {}

impl fmt::Display for PlaceHolderError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        write!(f, "this is merely a placeholder error")
    }
}

pub struct Error {
    error: ErrorKind,
}

enum ErrorKind {
    StatusCode(PlaceHolderError),
    Method(PlaceHolderError),
    Uri(PlaceHolderError),
    UriParts(PlaceHolderError),
    HeaderName(PlaceHolderError),
    HeaderValue(PlaceHolderError),
}

impl Error {
    pub fn get_ref(&self) -> &(dyn error::Error + 'static) {
        use self::ErrorKind::*;

        match self.error {
            StatusCode(ref e) => e,
            Method(ref e) => e,
            Uri(ref e) => e,
            UriParts(ref e) => e,
            HeaderName(ref e) => e,
            HeaderValue(ref e) => e,
        }
    }

    pub fn is<T: error::Error + 'static>(&self) -> bool {
        self.get_ref().is::<T>()
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        fmt::Display::fmt(self.get_ref(), f)
    }
}

impl fmt::Debug for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        f.debug_tuple("http::Error").field(&self.get_ref()).finish()
    }
}
