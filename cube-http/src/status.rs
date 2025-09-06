use std::fmt;

use cube_core::error::Error;

/// https://developer.mozilla.org/en-US/docs/Web/HTTP/Reference/Status
#[repr(u16)]
#[derive(Debug, Clone, Copy, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Status {
    /// https://developer.mozilla.org/en-US/docs/Web/HTTP/Reference/Status/100
    #[cfg_attr(feature = "serde", serde(rename = "Continue"))]
    Continue = 100,

    /// https://developer.mozilla.org/en-US/docs/Web/HTTP/Reference/Status/101
    #[cfg_attr(feature = "serde", serde(rename = "Switching Protocols"))]
    SwitchingProtocols = 101,

    /// https://developer.mozilla.org/en-US/docs/Web/HTTP/Reference/Status/101
    #[cfg_attr(feature = "serde", serde(rename = "Processing"))]
    Processing = 102,

    /// https://developer.mozilla.org/en-US/docs/Web/HTTP/Reference/Status/103
    #[cfg_attr(feature = "serde", serde(rename = "Early Hints"))]
    EarlyHints = 103,

    /// https://developer.mozilla.org/en-US/docs/Web/HTTP/Reference/Status/200
    #[cfg_attr(feature = "serde", serde(rename = "Ok"))]
    Ok = 200,

    /// https://developer.mozilla.org/en-US/docs/Web/HTTP/Reference/Status/201
    #[cfg_attr(feature = "serde", serde(rename = "Created"))]
    Created = 201,

    /// https://developer.mozilla.org/en-US/docs/Web/HTTP/Reference/Status/202
    #[cfg_attr(feature = "serde", serde(rename = "Accepted"))]
    Accepted = 202,

    /// https://developer.mozilla.org/en-US/docs/Web/HTTP/Reference/Status/203
    #[cfg_attr(feature = "serde", serde(rename = "Non-Authoritative Information"))]
    NonAuthoritativeInformation = 203,

    /// https://developer.mozilla.org/en-US/docs/Web/HTTP/Reference/Status/204
    #[cfg_attr(feature = "serde", serde(rename = "No Content"))]
    NoContent = 204,

    /// https://developer.mozilla.org/en-US/docs/Web/HTTP/Reference/Status/205
    #[cfg_attr(feature = "serde", serde(rename = "Reset Content"))]
    ResetContent = 205,

    /// https://developer.mozilla.org/en-US/docs/Web/HTTP/Reference/Status/206
    #[cfg_attr(feature = "serde", serde(rename = "Partial Content"))]
    PartialContent = 206,

    /// https://developer.mozilla.org/en-US/docs/Web/HTTP/Reference/Status/207
    #[cfg_attr(feature = "serde", serde(rename = "Multi-Status"))]
    MultiStatus = 207,

    /// https://developer.mozilla.org/en-US/docs/Web/HTTP/Reference/Status/208
    #[cfg_attr(feature = "serde", serde(rename = "Already Reported"))]
    AlreadyReported = 208,

    /// https://developer.mozilla.org/en-US/docs/Web/HTTP/Reference/Status/226
    #[cfg_attr(feature = "serde", serde(rename = "IM Used"))]
    IMUsed = 226,

    /// https://developer.mozilla.org/en-US/docs/Web/HTTP/Reference/Status/300
    #[cfg_attr(feature = "serde", serde(rename = "Multiple Choices"))]
    MultipleChoices = 300,

    /// https://developer.mozilla.org/en-US/docs/Web/HTTP/Reference/Status/301
    #[cfg_attr(feature = "serde", serde(rename = "Moved Permanently"))]
    MovedPermanently = 301,

    /// https://developer.mozilla.org/en-US/docs/Web/HTTP/Reference/Status/302
    #[cfg_attr(feature = "serde", serde(rename = "Found"))]
    Found = 302,

    /// https://developer.mozilla.org/en-US/docs/Web/HTTP/Reference/Status/303
    #[cfg_attr(feature = "serde", serde(rename = "See Other"))]
    SeeOther = 303,

    /// https://developer.mozilla.org/en-US/docs/Web/HTTP/Reference/Status/304
    #[cfg_attr(feature = "serde", serde(rename = "Not Modified"))]
    NotModified = 304,

    /// https://developer.mozilla.org/en-US/docs/Web/HTTP/Reference/Status/305
    #[cfg_attr(feature = "serde", serde(rename = "Use Proxy"))]
    UseProxy = 305,

    /// https://developer.mozilla.org/en-US/docs/Web/HTTP/Reference/Status/306
    #[cfg_attr(feature = "serde", serde(rename = "Unused"))]
    Unused = 306,

    /// https://developer.mozilla.org/en-US/docs/Web/HTTP/Reference/Status/307
    #[cfg_attr(feature = "serde", serde(rename = "Temporary Redirect"))]
    TemporaryRedirect = 307,

    /// https://developer.mozilla.org/en-US/docs/Web/HTTP/Reference/Status/308
    #[cfg_attr(feature = "serde", serde(rename = "Permanent Redirect"))]
    PermanentRedirect = 308,

    /// https://developer.mozilla.org/en-US/docs/Web/HTTP/Reference/Status/400
    #[cfg_attr(feature = "serde", serde(rename = "Bad Request"))]
    BadRequest = 400,

    /// https://developer.mozilla.org/en-US/docs/Web/HTTP/Reference/Status/401
    #[cfg_attr(feature = "serde", serde(rename = "Unauthorized"))]
    Unauthorized = 401,

    /// https://developer.mozilla.org/en-US/docs/Web/HTTP/Reference/Status/402
    #[cfg_attr(feature = "serde", serde(rename = "Payment Required"))]
    PaymentRequired = 402,

    /// https://developer.mozilla.org/en-US/docs/Web/HTTP/Reference/Status/403
    #[cfg_attr(feature = "serde", serde(rename = "Forbidden"))]
    Forbidden = 403,

    /// https://developer.mozilla.org/en-US/docs/Web/HTTP/Reference/Status/404
    #[cfg_attr(feature = "serde", serde(rename = "Not Found"))]
    NotFound = 404,

    /// https://developer.mozilla.org/en-US/docs/Web/HTTP/Reference/Status/405
    #[cfg_attr(feature = "serde", serde(rename = "Method Not Allowed"))]
    MethodNotAllowed = 405,

    /// https://developer.mozilla.org/en-US/docs/Web/HTTP/Reference/Status/406
    #[cfg_attr(feature = "serde", serde(rename = "Not Acceptable"))]
    NotAcceptable = 406,

    /// https://developer.mozilla.org/en-US/docs/Web/HTTP/Reference/Status/407
    #[cfg_attr(feature = "serde", serde(rename = "Proxy Authentication Required"))]
    ProxyAuthenticationRequired = 407,

    /// https://developer.mozilla.org/en-US/docs/Web/HTTP/Reference/Status/408
    #[cfg_attr(feature = "serde", serde(rename = "Request Timeout"))]
    RequestTimeout = 408,

    /// https://developer.mozilla.org/en-US/docs/Web/HTTP/Reference/Status/409
    #[cfg_attr(feature = "serde", serde(rename = "Conflict"))]
    Conflict = 409,

    /// https://developer.mozilla.org/en-US/docs/Web/HTTP/Reference/Status/410
    #[cfg_attr(feature = "serde", serde(rename = "Gone"))]
    Gone = 410,

    /// https://developer.mozilla.org/en-US/docs/Web/HTTP/Reference/Status/411
    #[cfg_attr(feature = "serde", serde(rename = "Length Required"))]
    LengthRequired = 411,

    /// https://developer.mozilla.org/en-US/docs/Web/HTTP/Reference/Status/412
    #[cfg_attr(feature = "serde", serde(rename = "Precondition Failed"))]
    PreconditionFailed = 412,

    /// https://developer.mozilla.org/en-US/docs/Web/HTTP/Reference/Status/413
    #[cfg_attr(feature = "serde", serde(rename = "Content Too Large"))]
    ContentTooLarge = 413,

    /// https://developer.mozilla.org/en-US/docs/Web/HTTP/Reference/Status/414
    #[cfg_attr(feature = "serde", serde(rename = "URI Too Long"))]
    URITooLong = 414,

    /// https://developer.mozilla.org/en-US/docs/Web/HTTP/Reference/Status/415
    #[cfg_attr(feature = "serde", serde(rename = "Unsupported Media Type"))]
    UnsupportedMediaType = 415,

    /// https://developer.mozilla.org/en-US/docs/Web/HTTP/Reference/Status/416
    #[cfg_attr(feature = "serde", serde(rename = "Range Not Satisfiable"))]
    RangeNotSatisfiable = 416,

    /// https://developer.mozilla.org/en-US/docs/Web/HTTP/Reference/Status/417
    #[cfg_attr(feature = "serde", serde(rename = "Expectation Failed"))]
    ExpectationFailed = 417,

    /// https://developer.mozilla.org/en-US/docs/Web/HTTP/Reference/Status/418
    #[cfg_attr(feature = "serde", serde(rename = "Teampot"))]
    Teapot = 418,

    /// https://developer.mozilla.org/en-US/docs/Web/HTTP/Reference/Status/421
    #[cfg_attr(feature = "serde", serde(rename = "Misdirected Request"))]
    MisdirectedRequest = 421,

    /// https://developer.mozilla.org/en-US/docs/Web/HTTP/Reference/Status/422
    #[cfg_attr(feature = "serde", serde(rename = "Unprocessable Content"))]
    UnprocessableContent = 422,

    /// https://developer.mozilla.org/en-US/docs/Web/HTTP/Reference/Status/423
    #[cfg_attr(feature = "serde", serde(rename = "Locked"))]
    Locked = 423,

    /// https://developer.mozilla.org/en-US/docs/Web/HTTP/Reference/Status/424
    #[cfg_attr(feature = "serde", serde(rename = "Failed Dependency"))]
    FailedDependency = 424,

    /// https://developer.mozilla.org/en-US/docs/Web/HTTP/Reference/Status/425
    #[cfg_attr(feature = "serde", serde(rename = "Too Early"))]
    TooEarly = 425,

    /// https://developer.mozilla.org/en-US/docs/Web/HTTP/Reference/Status/426
    #[cfg_attr(feature = "serde", serde(rename = "Upgrade Required"))]
    UpgradeRequired = 426,

    /// https://developer.mozilla.org/en-US/docs/Web/HTTP/Reference/Status/428
    #[cfg_attr(feature = "serde", serde(rename = "Precondition Required"))]
    PreconditionRequired = 428,

    /// https://developer.mozilla.org/en-US/docs/Web/HTTP/Reference/Status/429
    #[cfg_attr(feature = "serde", serde(rename = "Too Many Requests"))]
    TooManyRequests = 429,

    /// https://developer.mozilla.org/en-US/docs/Web/HTTP/Reference/Status/431
    #[cfg_attr(feature = "serde", serde(rename = "Request Header Fields Too Large"))]
    RequestHeaderFieldsTooLarge = 431,

    /// https://developer.mozilla.org/en-US/docs/Web/HTTP/Reference/Status/451
    #[cfg_attr(feature = "serde", serde(rename = "Unavailable For Legal Reasons"))]
    UnavailableForLegalReasons = 451,

    /// https://developer.mozilla.org/en-US/docs/Web/HTTP/Reference/Status/500
    #[cfg_attr(feature = "serde", serde(rename = "Internal Server Error"))]
    InternalServerError = 500,

    /// https://developer.mozilla.org/en-US/docs/Web/HTTP/Reference/Status/501
    #[cfg_attr(feature = "serde", serde(rename = "Not Implemented"))]
    NotImplemented = 501,

    /// https://developer.mozilla.org/en-US/docs/Web/HTTP/Reference/Status/208
    #[cfg_attr(feature = "serde", serde(rename = "Bad Gateway"))]
    BadGateway = 502,

    /// https://developer.mozilla.org/en-US/docs/Web/HTTP/Reference/Status/503
    #[cfg_attr(feature = "serde", serde(rename = "Service Unavailable"))]
    ServiceUnavailable = 503,

    /// https://developer.mozilla.org/en-US/docs/Web/HTTP/Reference/Status/504
    #[cfg_attr(feature = "serde", serde(rename = "Gateway Timeout"))]
    GatewayTimeout = 504,

    /// https://developer.mozilla.org/en-US/docs/Web/HTTP/Reference/Status/505
    #[cfg_attr(feature = "serde", serde(rename = "HTTP Version Not Supported"))]
    HTTPVersionNotSupported = 505,

    /// https://developer.mozilla.org/en-US/docs/Web/HTTP/Reference/Status/506
    #[cfg_attr(feature = "serde", serde(rename = "Variant Also Negotiates"))]
    VariantAlsoNegotiates = 506,

    /// https://developer.mozilla.org/en-US/docs/Web/HTTP/Reference/Status/507
    #[cfg_attr(feature = "serde", serde(rename = "Insufficient Storage"))]
    InsufficientStorage = 507,

    /// https://developer.mozilla.org/en-US/docs/Web/HTTP/Reference/Status/508
    #[cfg_attr(feature = "serde", serde(rename = "Loop Detected"))]
    LoopDetected = 508,

    /// https://developer.mozilla.org/en-US/docs/Web/HTTP/Reference/Status/510
    #[cfg_attr(feature = "serde", serde(rename = "Not Extended"))]
    NotExtended = 510,

    /// https://developer.mozilla.org/en-US/docs/Web/HTTP/Reference/Status/511
    #[cfg_attr(feature = "serde", serde(rename = "Network Authentication Required"))]
    NetworkAuthenticationRequired = 511,
}

impl Status {
    #[inline]
    pub const fn as_u16(self) -> u16 {
        return self as u16;
    }

    #[inline]
    pub const fn reason(self) -> &'static str {
        return match self {
            Self::Continue => "Continue",
            Self::SwitchingProtocols => "Switching Protocols",
            Self::Processing => "Processing",
            Self::EarlyHints => "Early Hints",

            Self::Ok => "OK",
            Self::Created => "Created",
            Self::Accepted => "Accepted",
            Self::NonAuthoritativeInformation => "Non-Authoritative Information",
            Self::NoContent => "No Content",
            Self::ResetContent => "Reset Content",
            Self::PartialContent => "Partial Content",
            Self::MultiStatus => "Multi-Status",
            Self::AlreadyReported => "Already Reported",
            Self::IMUsed => "IM Used",

            Self::MultipleChoices => "Multiple Choices",
            Self::MovedPermanently => "Moved Permanently",
            Self::Found => "Found",
            Self::SeeOther => "See Other",
            Self::NotModified => "Not Modified",
            Self::UseProxy => "Use Proxy",
            Self::Unused => "Unused",
            Self::TemporaryRedirect => "Temporary Redirect",
            Self::PermanentRedirect => "Permanent Redirect",

            Self::BadRequest => "Bad Request",
            Self::Unauthorized => "Unauthorized",
            Self::PaymentRequired => "Payment Required",
            Self::Forbidden => "Forbidden",
            Self::NotFound => "Not Found",
            Self::MethodNotAllowed => "Method Not Allowed",
            Self::NotAcceptable => "Not Acceptable",
            Self::ProxyAuthenticationRequired => "Proxy Authentication Required",
            Self::RequestTimeout => "Request Timeout",
            Self::Conflict => "Conflict",
            Self::Gone => "Gone",
            Self::LengthRequired => "Length Required",
            Self::PreconditionFailed => "Precondition Failed",
            Self::ContentTooLarge => "Content Too Large",
            Self::URITooLong => "URI Too Long",
            Self::UnsupportedMediaType => "Unsupported Media Type",
            Self::RangeNotSatisfiable => "Range Not Satisfiable",
            Self::ExpectationFailed => "Expectation Failed",
            Self::Teapot => "I'm a teapot",
            Self::MisdirectedRequest => "Misdirected Request",
            Self::UnprocessableContent => "Unprocessable Content",
            Self::Locked => "Locked",
            Self::FailedDependency => "Failed Dependency",
            Self::TooEarly => "Too Early",
            Self::UpgradeRequired => "Upgrade Required",
            Self::PreconditionRequired => "Precondition Required",
            Self::TooManyRequests => "Too Many Requests",
            Self::RequestHeaderFieldsTooLarge => "Request Header Fields Too Large",
            Self::UnavailableForLegalReasons => "Unavailable For Legal Reasons",

            Self::InternalServerError => "Internal Server Error",
            Self::NotImplemented => "Not Implemented",
            Self::BadGateway => "Bad Gateway",
            Self::ServiceUnavailable => "Service Unavailable",
            Self::GatewayTimeout => "Gateway Timeout",
            Self::HTTPVersionNotSupported => "HTTP Version Not Supported",
            Self::VariantAlsoNegotiates => "Variant Also Negotiates",
            Self::InsufficientStorage => "Insufficient Storage",
            Self::LoopDetected => "Loop Detected",
            Self::NotExtended => "Not Extended",
            Self::NetworkAuthenticationRequired => "Network Authentication Required",
        };
    }
}

impl From<Status> for u16 {
    fn from(status: Status) -> Self {
        return status as u16;
    }
}

impl TryFrom<u16> for Status {
    type Error = Error;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        return match value {
            100 => Ok(Self::Continue),
            101 => Ok(Self::SwitchingProtocols),
            102 => Ok(Self::Processing),
            103 => Ok(Self::EarlyHints),
            200 => Ok(Self::Ok),
            201 => Ok(Self::Created),
            202 => Ok(Self::Accepted),
            203 => Ok(Self::NonAuthoritativeInformation),
            204 => Ok(Self::NoContent),
            205 => Ok(Self::ResetContent),
            206 => Ok(Self::PartialContent),
            207 => Ok(Self::MultiStatus),
            208 => Ok(Self::AlreadyReported),
            226 => Ok(Self::IMUsed),
            300 => Ok(Self::MultipleChoices),
            301 => Ok(Self::MovedPermanently),
            302 => Ok(Self::Found),
            303 => Ok(Self::SeeOther),
            304 => Ok(Self::NotModified),
            305 => Ok(Self::UseProxy),
            306 => Ok(Self::Unused),
            307 => Ok(Self::TemporaryRedirect),
            308 => Ok(Self::PermanentRedirect),
            400 => Ok(Self::BadRequest),
            401 => Ok(Self::Unauthorized),
            402 => Ok(Self::PaymentRequired),
            403 => Ok(Self::Forbidden),
            404 => Ok(Self::NotFound),
            405 => Ok(Self::MethodNotAllowed),
            406 => Ok(Self::NotAcceptable),
            407 => Ok(Self::ProxyAuthenticationRequired),
            408 => Ok(Self::RequestTimeout),
            409 => Ok(Self::Conflict),
            410 => Ok(Self::Gone),
            411 => Ok(Self::LengthRequired),
            412 => Ok(Self::PreconditionFailed),
            413 => Ok(Self::ContentTooLarge),
            414 => Ok(Self::URITooLong),
            415 => Ok(Self::UnsupportedMediaType),
            416 => Ok(Self::RangeNotSatisfiable),
            417 => Ok(Self::ExpectationFailed),
            418 => Ok(Self::Teapot),
            421 => Ok(Self::MisdirectedRequest),
            422 => Ok(Self::UnprocessableContent),
            423 => Ok(Self::Locked),
            424 => Ok(Self::FailedDependency),
            425 => Ok(Self::TooEarly),
            426 => Ok(Self::UpgradeRequired),
            428 => Ok(Self::PreconditionRequired),
            429 => Ok(Self::TooManyRequests),
            431 => Ok(Self::RequestHeaderFieldsTooLarge),
            451 => Ok(Self::UnavailableForLegalReasons),
            500 => Ok(Self::InternalServerError),
            501 => Ok(Self::NotImplemented),
            502 => Ok(Self::BadGateway),
            503 => Ok(Self::ServiceUnavailable),
            504 => Ok(Self::GatewayTimeout),
            505 => Ok(Self::HTTPVersionNotSupported),
            506 => Ok(Self::VariantAlsoNegotiates),
            507 => Ok(Self::InsufficientStorage),
            508 => Ok(Self::LoopDetected),
            510 => Ok(Self::NotExtended),
            511 => Ok(Self::NetworkAuthenticationRequired),
            v => Err(Error::from(format!(
                "[cube::http::status] => status code \"{}\" is out of range",
                v
            ))),
        };
    }
}

impl TryFrom<&str> for Status {
    type Error = Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let parsed: u16 = value.parse()?;
        return Self::try_from(parsed);
    }
}

impl fmt::Display for Status {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        return write!(f, "{}", self.reason());
    }
}
