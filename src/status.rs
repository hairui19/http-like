#![allow(unused)]

use std::fmt;
use std::num::NonZeroU16;
use std::str;

use crate::error::status_code_error::InvalidStatusCode;

#[derive(Copy, Clone)]
struct StatusCode(NonZeroU16);

impl StatusCode {
    /// Converts a u16 to a status code.
    pub fn from_u16(src: u16) -> Result<StatusCode, InvalidStatusCode> {
        if src < 100 || src > 999 {
            return Err(InvalidStatusCode::new());
        }
        NonZeroU16::new(src)
            .map(StatusCode)
            .ok_or_else(InvalidStatusCode::new)
    }

    /// Converts a ascii &[u8] to a status code.
    pub fn from_ascii_bytes(src: &[u8]) -> Result<StatusCode, InvalidStatusCode> {
        if src.len() != 3 {
            return Err(InvalidStatusCode::new());
        }

        let a = src[0].wrapping_sub(b'0') as u16;
        let b = src[1].wrapping_sub(b'0') as u16;
        let c = src[2].wrapping_sub(b'0') as u16;

        if a == 0 || a > 9 || b > 9 || c > 9 {
            return Err(InvalidStatusCode::new());
        }

        NonZeroU16::new(a * 100 + b * 10 + c)
            .map(StatusCode)
            .ok_or_else(InvalidStatusCode::new)
    }

    pub fn as_u16(&self) -> u16 {
        (*self).into()
    }

    pub fn as_str(&self) -> &str {
        let offset: usize = (self.as_u16() - 100) as usize * 3;
        &CODE_DIGITS[offset..offset + 3]
    }

    pub fn canonical_reason(&self) -> Option<&'static str> {
        canonical_reason(self.as_u16())
    }

    /// Check if status is 100-199
    pub fn is_informational(&self) -> bool {
        200 > self.as_u16() && self.as_u16() >= 100
    }

    /// Check if status is within 200-299
    pub fn is_success(&self) -> bool {
        300 > self.as_u16() && self.as_u16() >= 200
    }

    /// Check if the status is within 300-399
    pub fn is_redirection(&self) -> bool {
        400 > self.as_u16() && self.as_u16() >= 300
    }

    /// Check if the status is within 400-499
    pub fn is_client_error(&self) -> bool {
        500 > self.as_u16() && self.as_u16() >= 400
    }

    /// Check if the status is within 500-599
    pub fn is_server_error(&self) -> bool {
        600 > self.as_u16() && self.as_u16() >= 500
    }
}

// Trait Implementaions for StatusCode.
impl fmt::Display for StatusCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        fmt::Display::fmt(&self.0, f)
    }
}

impl fmt::Debug for StatusCode {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        fmt::Debug::fmt(&self.0, f)
    }
}

impl Default for StatusCode {
    fn default() -> Self {
        StatusCode::OK
    }
}

impl PartialEq<StatusCode> for StatusCode {
    fn eq(&self, other: &StatusCode) -> bool {
        self.0 == other.0
    }
}

impl PartialEq<StatusCode> for u16 {
    fn eq(&self, other: &StatusCode) -> bool {
        *self == other.as_u16()
    }
}

impl str::FromStr for StatusCode {
    type Err = InvalidStatusCode;
    fn from_str(s: &str) -> Result<Self, InvalidStatusCode> {
        StatusCode::from_ascii_bytes(s.as_bytes())
    }
}

impl From<StatusCode> for u16 {
    fn from(status_code: StatusCode) -> u16 {
        status_code.0.get()
    }
}

impl<'a> From<&'a StatusCode> for StatusCode {
    fn from(status_code: &'a StatusCode) -> Self {
        status_code.clone()
    }
}

impl<'a> TryFrom<&'a [u8]> for StatusCode {
    type Error = InvalidStatusCode;

    fn try_from(value: &'a [u8]) -> Result<StatusCode, InvalidStatusCode> {
        StatusCode::from_ascii_bytes(value)
    }
}

impl<'a> TryFrom<&'a str> for StatusCode {
    type Error = InvalidStatusCode;

    fn try_from(s: &'a str) -> Result<StatusCode, InvalidStatusCode> {
        StatusCode::from_ascii_bytes(s.as_bytes())
    }
}

impl TryFrom<u16> for StatusCode {
    type Error = InvalidStatusCode;
    fn try_from(src: u16) -> Result<StatusCode, InvalidStatusCode> {
        StatusCode::from_u16(src)
    }
}

macro_rules! status_codes {
    (
        $(
            $(#[$docs:meta])*
            ($num:expr, $konst:ident, $phrase:expr);
        )+
    ) => {
        impl StatusCode {
        $(
            $(#[$docs])*
            pub const $konst: StatusCode = StatusCode(unsafe { NonZeroU16::new_unchecked($num) });
        )+

        }

        fn canonical_reason(num: u16) -> Option<&'static str> {
            match num {
                $(
                $num => Some($phrase),
                )+
                _ => None
            }
        }
    }
}

status_codes! {
    /// 100 Continue
    /// [[RFC7231, Section 6.2.1](https://tools.ietf.org/html/rfc7231#section-6.2.1)]
    (100, CONTINUE, "Continue");
    /// 101 Switching Protocols
    /// [[RFC7231, Section 6.2.2](https://tools.ietf.org/html/rfc7231#section-6.2.2)]
    (101, SWITCHING_PROTOCOLS, "Switching Protocols");
    /// 102 Processing
    /// [[RFC2518](https://tools.ietf.org/html/rfc2518)]
    (102, PROCESSING, "Processing");

    /// 200 OK
    /// [[RFC7231, Section 6.3.1](https://tools.ietf.org/html/rfc7231#section-6.3.1)]
    (200, OK, "OK");
    /// 201 Created
    /// [[RFC7231, Section 6.3.2](https://tools.ietf.org/html/rfc7231#section-6.3.2)]
    (201, CREATED, "Created");
    /// 202 Accepted
    /// [[RFC7231, Section 6.3.3](https://tools.ietf.org/html/rfc7231#section-6.3.3)]
    (202, ACCEPTED, "Accepted");
    /// 203 Non-Authoritative Information
    /// [[RFC7231, Section 6.3.4](https://tools.ietf.org/html/rfc7231#section-6.3.4)]
    (203, NON_AUTHORITATIVE_INFORMATION, "Non Authoritative Information");
    /// 204 No Content
    /// [[RFC7231, Section 6.3.5](https://tools.ietf.org/html/rfc7231#section-6.3.5)]
    (204, NO_CONTENT, "No Content");
    /// 205 Reset Content
    /// [[RFC7231, Section 6.3.6](https://tools.ietf.org/html/rfc7231#section-6.3.6)]
    (205, RESET_CONTENT, "Reset Content");
    /// 206 Partial Content
    /// [[RFC7233, Section 4.1](https://tools.ietf.org/html/rfc7233#section-4.1)]
    (206, PARTIAL_CONTENT, "Partial Content");
    /// 207 Multi-Status
    /// [[RFC4918](https://tools.ietf.org/html/rfc4918)]
    (207, MULTI_STATUS, "Multi-Status");
    /// 208 Already Reported
    /// [[RFC5842](https://tools.ietf.org/html/rfc5842)]
    (208, ALREADY_REPORTED, "Already Reported");

    /// 226 IM Used
    /// [[RFC3229](https://tools.ietf.org/html/rfc3229)]
    (226, IM_USED, "IM Used");

    /// 300 Multiple Choices
    /// [[RFC7231, Section 6.4.1](https://tools.ietf.org/html/rfc7231#section-6.4.1)]
    (300, MULTIPLE_CHOICES, "Multiple Choices");
    /// 301 Moved Permanently
    /// [[RFC7231, Section 6.4.2](https://tools.ietf.org/html/rfc7231#section-6.4.2)]
    (301, MOVED_PERMANENTLY, "Moved Permanently");
    /// 302 Found
    /// [[RFC7231, Section 6.4.3](https://tools.ietf.org/html/rfc7231#section-6.4.3)]
    (302, FOUND, "Found");
    /// 303 See Other
    /// [[RFC7231, Section 6.4.4](https://tools.ietf.org/html/rfc7231#section-6.4.4)]
    (303, SEE_OTHER, "See Other");
    /// 304 Not Modified
    /// [[RFC7232, Section 4.1](https://tools.ietf.org/html/rfc7232#section-4.1)]
    (304, NOT_MODIFIED, "Not Modified");
    /// 305 Use Proxy
    /// [[RFC7231, Section 6.4.5](https://tools.ietf.org/html/rfc7231#section-6.4.5)]
    (305, USE_PROXY, "Use Proxy");
    /// 307 Temporary Redirect
    /// [[RFC7231, Section 6.4.7](https://tools.ietf.org/html/rfc7231#section-6.4.7)]
    (307, TEMPORARY_REDIRECT, "Temporary Redirect");
    /// 308 Permanent Redirect
    /// [[RFC7238](https://tools.ietf.org/html/rfc7238)]
    (308, PERMANENT_REDIRECT, "Permanent Redirect");

    /// 400 Bad Request
    /// [[RFC7231, Section 6.5.1](https://tools.ietf.org/html/rfc7231#section-6.5.1)]
    (400, BAD_REQUEST, "Bad Request");
    /// 401 Unauthorized
    /// [[RFC7235, Section 3.1](https://tools.ietf.org/html/rfc7235#section-3.1)]
    (401, UNAUTHORIZED, "Unauthorized");
    /// 402 Payment Required
    /// [[RFC7231, Section 6.5.2](https://tools.ietf.org/html/rfc7231#section-6.5.2)]
    (402, PAYMENT_REQUIRED, "Payment Required");
    /// 403 Forbidden
    /// [[RFC7231, Section 6.5.3](https://tools.ietf.org/html/rfc7231#section-6.5.3)]
    (403, FORBIDDEN, "Forbidden");
    /// 404 Not Found
    /// [[RFC7231, Section 6.5.4](https://tools.ietf.org/html/rfc7231#section-6.5.4)]
    (404, NOT_FOUND, "Not Found");
    /// 405 Method Not Allowed
    /// [[RFC7231, Section 6.5.5](https://tools.ietf.org/html/rfc7231#section-6.5.5)]
    (405, METHOD_NOT_ALLOWED, "Method Not Allowed");
    /// 406 Not Acceptable
    /// [[RFC7231, Section 6.5.6](https://tools.ietf.org/html/rfc7231#section-6.5.6)]
    (406, NOT_ACCEPTABLE, "Not Acceptable");
    /// 407 Proxy Authentication Required
    /// [[RFC7235, Section 3.2](https://tools.ietf.org/html/rfc7235#section-3.2)]
    (407, PROXY_AUTHENTICATION_REQUIRED, "Proxy Authentication Required");
    /// 408 Request Timeout
    /// [[RFC7231, Section 6.5.7](https://tools.ietf.org/html/rfc7231#section-6.5.7)]
    (408, REQUEST_TIMEOUT, "Request Timeout");
    /// 409 Conflict
    /// [[RFC7231, Section 6.5.8](https://tools.ietf.org/html/rfc7231#section-6.5.8)]
    (409, CONFLICT, "Conflict");
    /// 410 Gone
    /// [[RFC7231, Section 6.5.9](https://tools.ietf.org/html/rfc7231#section-6.5.9)]
    (410, GONE, "Gone");
    /// 411 Length Required
    /// [[RFC7231, Section 6.5.10](https://tools.ietf.org/html/rfc7231#section-6.5.10)]
    (411, LENGTH_REQUIRED, "Length Required");
    /// 412 Precondition Failed
    /// [[RFC7232, Section 4.2](https://tools.ietf.org/html/rfc7232#section-4.2)]
    (412, PRECONDITION_FAILED, "Precondition Failed");
    /// 413 Payload Too Large
    /// [[RFC7231, Section 6.5.11](https://tools.ietf.org/html/rfc7231#section-6.5.11)]
    (413, PAYLOAD_TOO_LARGE, "Payload Too Large");
    /// 414 URI Too Long
    /// [[RFC7231, Section 6.5.12](https://tools.ietf.org/html/rfc7231#section-6.5.12)]
    (414, URI_TOO_LONG, "URI Too Long");
    /// 415 Unsupported Media Type
    /// [[RFC7231, Section 6.5.13](https://tools.ietf.org/html/rfc7231#section-6.5.13)]
    (415, UNSUPPORTED_MEDIA_TYPE, "Unsupported Media Type");
    /// 416 Range Not Satisfiable
    /// [[RFC7233, Section 4.4](https://tools.ietf.org/html/rfc7233#section-4.4)]
    (416, RANGE_NOT_SATISFIABLE, "Range Not Satisfiable");
    /// 417 Expectation Failed
    /// [[RFC7231, Section 6.5.14](https://tools.ietf.org/html/rfc7231#section-6.5.14)]
    (417, EXPECTATION_FAILED, "Expectation Failed");
    /// 418 I'm a teapot
    /// [curiously not registered by IANA but [RFC2324](https://tools.ietf.org/html/rfc2324)]
    (418, IM_A_TEAPOT, "I'm a teapot");

    /// 421 Misdirected Request
    /// [RFC7540, Section 9.1.2](http://tools.ietf.org/html/rfc7540#section-9.1.2)
    (421, MISDIRECTED_REQUEST, "Misdirected Request");
    /// 422 Unprocessable Entity
    /// [[RFC4918](https://tools.ietf.org/html/rfc4918)]
    (422, UNPROCESSABLE_ENTITY, "Unprocessable Entity");
    /// 423 Locked
    /// [[RFC4918](https://tools.ietf.org/html/rfc4918)]
    (423, LOCKED, "Locked");
    /// 424 Failed Dependency
    /// [[RFC4918](https://tools.ietf.org/html/rfc4918)]
    (424, FAILED_DEPENDENCY, "Failed Dependency");

    /// 426 Upgrade Required
    /// [[RFC7231, Section 6.5.15](https://tools.ietf.org/html/rfc7231#section-6.5.15)]
    (426, UPGRADE_REQUIRED, "Upgrade Required");

    /// 428 Precondition Required
    /// [[RFC6585](https://tools.ietf.org/html/rfc6585)]
    (428, PRECONDITION_REQUIRED, "Precondition Required");
    /// 429 Too Many Requests
    /// [[RFC6585](https://tools.ietf.org/html/rfc6585)]
    (429, TOO_MANY_REQUESTS, "Too Many Requests");

    /// 431 Request Header Fields Too Large
    /// [[RFC6585](https://tools.ietf.org/html/rfc6585)]
    (431, REQUEST_HEADER_FIELDS_TOO_LARGE, "Request Header Fields Too Large");

    /// 451 Unavailable For Legal Reasons
    /// [[RFC7725](http://tools.ietf.org/html/rfc7725)]
    (451, UNAVAILABLE_FOR_LEGAL_REASONS, "Unavailable For Legal Reasons");

    /// 500 Internal Server Error
    /// [[RFC7231, Section 6.6.1](https://tools.ietf.org/html/rfc7231#section-6.6.1)]
    (500, INTERNAL_SERVER_ERROR, "Internal Server Error");
    /// 501 Not Implemented
    /// [[RFC7231, Section 6.6.2](https://tools.ietf.org/html/rfc7231#section-6.6.2)]
    (501, NOT_IMPLEMENTED, "Not Implemented");
    /// 502 Bad Gateway
    /// [[RFC7231, Section 6.6.3](https://tools.ietf.org/html/rfc7231#section-6.6.3)]
    (502, BAD_GATEWAY, "Bad Gateway");
    /// 503 Service Unavailable
    /// [[RFC7231, Section 6.6.4](https://tools.ietf.org/html/rfc7231#section-6.6.4)]
    (503, SERVICE_UNAVAILABLE, "Service Unavailable");
    /// 504 Gateway Timeout
    /// [[RFC7231, Section 6.6.5](https://tools.ietf.org/html/rfc7231#section-6.6.5)]
    (504, GATEWAY_TIMEOUT, "Gateway Timeout");
    /// 505 HTTP Version Not Supported
    /// [[RFC7231, Section 6.6.6](https://tools.ietf.org/html/rfc7231#section-6.6.6)]
    (505, HTTP_VERSION_NOT_SUPPORTED, "HTTP Version Not Supported");
    /// 506 Variant Also Negotiates
    /// [[RFC2295](https://tools.ietf.org/html/rfc2295)]
    (506, VARIANT_ALSO_NEGOTIATES, "Variant Also Negotiates");
    /// 507 Insufficient Storage
    /// [[RFC4918](https://tools.ietf.org/html/rfc4918)]
    (507, INSUFFICIENT_STORAGE, "Insufficient Storage");
    /// 508 Loop Detected
    /// [[RFC5842](https://tools.ietf.org/html/rfc5842)]
    (508, LOOP_DETECTED, "Loop Detected");

    /// 510 Not Extended
    /// [[RFC2774](https://tools.ietf.org/html/rfc2774)]
    (510, NOT_EXTENDED, "Not Extended");
    /// 511 Network Authentication Required
    /// [[RFC6585](https://tools.ietf.org/html/rfc6585)]
    (511, NETWORK_AUTHENTICATION_REQUIRED, "Network Authentication Required");
}

const CODE_DIGITS: &'static str = "\
100101102103104105106107108109110111112113114115116117118119\
120121122123124125126127128129130131132133134135136137138139\
140141142143144145146147148149150151152153154155156157158159\
160161162163164165166167168169170171172173174175176177178179\
180181182183184185186187188189190191192193194195196197198199\
200201202203204205206207208209210211212213214215216217218219\
220221222223224225226227228229230231232233234235236237238239\
240241242243244245246247248249250251252253254255256257258259\
260261262263264265266267268269270271272273274275276277278279\
280281282283284285286287288289290291292293294295296297298299\
300301302303304305306307308309310311312313314315316317318319\
320321322323324325326327328329330331332333334335336337338339\
340341342343344345346347348349350351352353354355356357358359\
360361362363364365366367368369370371372373374375376377378379\
380381382383384385386387388389390391392393394395396397398399\
400401402403404405406407408409410411412413414415416417418419\
420421422423424425426427428429430431432433434435436437438439\
440441442443444445446447448449450451452453454455456457458459\
460461462463464465466467468469470471472473474475476477478479\
480481482483484485486487488489490491492493494495496497498499\
500501502503504505506507508509510511512513514515516517518519\
520521522523524525526527528529530531532533534535536537538539\
540541542543544545546547548549550551552553554555556557558559\
560561562563564565566567568569570571572573574575576577578579\
580581582583584585586587588589590591592593594595596597598599\
600601602603604605606607608609610611612613614615616617618619\
620621622623624625626627628629630631632633634635636637638639\
640641642643644645646647648649650651652653654655656657658659\
660661662663664665666667668669670671672673674675676677678679\
680681682683684685686687688689690691692693694695696697698699\
700701702703704705706707708709710711712713714715716717718719\
720721722723724725726727728729730731732733734735736737738739\
740741742743744745746747748749750751752753754755756757758759\
760761762763764765766767768769770771772773774775776777778779\
780781782783784785786787788789790791792793794795796797798799\
800801802803804805806807808809810811812813814815816817818819\
820821822823824825826827828829830831832833834835836837838839\
840841842843844845846847848849850851852853854855856857858859\
860861862863864865866867868869870871872873874875876877878879\
880881882883884885886887888889890891892893894895896897898899\
900901902903904905906907908909910911912913914915916917918919\
920921922923924925926927928929930931932933934935936937938939\
940941942943944945946947948949950951952953954955956957958959\
960961962963964965966967968969970971972973974975976977978979\
980981982983984985986987988989990991992993994995996997998999";
