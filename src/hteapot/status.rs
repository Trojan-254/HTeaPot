#[derive(Clone, Copy)]
pub enum HttpStatus {
    OK = 200,
    Created = 201,
    Accepted = 202,
    NonAuthoritativeInformation = 203,
    NoContent = 204,
    ResetContent = 205,
    PartialContent = 206,

    MultipleChoices = 300,
    MovedPermanently = 301,
    Found = 302, // (Renombrado de MovedTemporarily)
    SeeOther = 303,
    NotModified = 304,
    UseProxy = 305,
    TemporaryRedirect = 307,
    PermanentRedirect = 308,

    BadRequest = 400,
    Unauthorized = 401,
    PaymentRequired = 402,
    Forbidden = 403,
    NotFound = 404,
    MethodNotAllowed = 405,
    NotAcceptable = 406,
    ProxyAuthenticationRequired = 407,
    RequestTimeout = 408,
    Conflict = 409,
    Gone = 410,
    LengthRequired = 411,
    PreconditionFailed = 412,
    PayloadTooLarge = 413,
    URITooLong = 414,
    UnsupportedMediaType = 415,
    RangeNotSatisfiable = 416,
    ExpectationFailed = 417,
    IAmATeapot = 418,
    MisdirectedRequest = 421,
    UnprocessableEntity = 422,
    Locked = 423,
    FailedDependency = 424,
    TooEarly = 425,
    UpgradeRequired = 426,
    PreconditionRequired = 428,
    TooManyRequests = 429,
    RequestHeaderFieldsTooLarge = 431,

    InternalServerError = 500,
    NotImplemented = 501,
    BadGateway = 502,
    ServiceUnavailable = 503,
    GatewayTimeout = 504,
    HTTPVersionNotSupported = 505,
    VariantAlsoNegotiates = 506,
    InsufficientStorage = 507,
    LoopDetected = 508,
    NotExtended = 510,
    NetworkAuthenticationRequired = 511,
}

impl HttpStatus {
    pub fn from_u16(status: u16) -> Result<HttpStatus, &'static str> {
        match status {
            200 => Ok(HttpStatus::OK),
            201 => Ok(HttpStatus::Created),
            202 => Ok(HttpStatus::Accepted),
            203 => Ok(HttpStatus::NonAuthoritativeInformation),
            204 => Ok(HttpStatus::NoContent),
            300 => Ok(HttpStatus::MultipleChoices),
            301 => Ok(HttpStatus::MovedPermanently),
            302 => Ok(HttpStatus::Found),
            303 => Ok(HttpStatus::SeeOther),
            304 => Ok(HttpStatus::NotModified),
            305 => Ok(HttpStatus::UseProxy),
            307 => Ok(HttpStatus::TemporaryRedirect),
            308 => Ok(HttpStatus::PermanentRedirect),
            400 => Ok(HttpStatus::BadRequest),
            401 => Ok(HttpStatus::Unauthorized),
            402 => Ok(HttpStatus::PaymentRequired),
            403 => Ok(HttpStatus::Forbidden),
            404 => Ok(HttpStatus::NotFound),
            405 => Ok(HttpStatus::MethodNotAllowed),
            406 => Ok(HttpStatus::NotAcceptable),
            407 => Ok(HttpStatus::ProxyAuthenticationRequired),
            408 => Ok(HttpStatus::RequestTimeout),
            409 => Ok(HttpStatus::Conflict),
            410 => Ok(HttpStatus::Gone),
            411 => Ok(HttpStatus::LengthRequired),
            412 => Ok(HttpStatus::PreconditionFailed),
            413 => Ok(HttpStatus::PayloadTooLarge),
            414 => Ok(HttpStatus::URITooLong),
            415 => Ok(HttpStatus::UnsupportedMediaType),
            416 => Ok(HttpStatus::RangeNotSatisfiable),
            418 => Ok(HttpStatus::IAmATeapot),
            421 => Ok(HttpStatus::MisdirectedRequest),
            422 => Ok(HttpStatus::UnprocessableEntity),
            423 => Ok(HttpStatus::Locked),
            424 => Ok(HttpStatus::FailedDependency),
            425 => Ok(HttpStatus::TooEarly),
            426 => Ok(HttpStatus::UpgradeRequired),
            428 => Ok(HttpStatus::PreconditionRequired),
            429 => Ok(HttpStatus::TooManyRequests),
            431 => Ok(HttpStatus::RequestHeaderFieldsTooLarge),
            500 => Ok(HttpStatus::InternalServerError),
            501 => Ok(HttpStatus::NotImplemented),
            502 => Ok(HttpStatus::BadGateway),
            503 => Ok(HttpStatus::ServiceUnavailable),
            504 => Ok(HttpStatus::GatewayTimeout),
            505 => Ok(HttpStatus::HTTPVersionNotSupported),
            506 => Ok(HttpStatus::VariantAlsoNegotiates),
            507 => Ok(HttpStatus::InsufficientStorage),
            508 => Ok(HttpStatus::LoopDetected),
            510 => Ok(HttpStatus::NotExtended),
            511 => Ok(HttpStatus::NetworkAuthenticationRequired),
            _ => Err("Invalid HTTP status"),
        }
    }

    pub fn to_string(&self) -> &str {
        match self {
            HttpStatus::OK => "OK",
            HttpStatus::Created => "Created",
            HttpStatus::Accepted => "Accepted",
            HttpStatus::NoContent => "No Content",
            HttpStatus::MovedPermanently => "Moved Permanently",
            HttpStatus::Found => "Moved Temporarily",
            HttpStatus::NotModified => "Not Modified",
            HttpStatus::BadRequest => "Bad Request",
            HttpStatus::Unauthorized => "Unauthorized",
            HttpStatus::Forbidden => "Forbidden",
            HttpStatus::NotFound => "Not Found",
            HttpStatus::IAmATeapot => "I'm a teapot",
            HttpStatus::InternalServerError => "Internal Server Error",
            HttpStatus::NotImplemented => "Not Implemented",
            HttpStatus::BadGateway => "Bad Gateway",
            HttpStatus::ServiceUnavailable => "Service Unavailable",
            HttpStatus::NonAuthoritativeInformation => "Non Authoritative Information",
            HttpStatus::ResetContent => "Reset Content",
            HttpStatus::PartialContent => "PartialContent",
            HttpStatus::MultipleChoices => "Multiple Choices",
            HttpStatus::SeeOther => "See Other",
            HttpStatus::UseProxy => "Use Proxy",
            HttpStatus::TemporaryRedirect => "Temporary Redirect",
            HttpStatus::PermanentRedirect => "Permanent Redirect",
            HttpStatus::PaymentRequired => "Payment Required",
            HttpStatus::MethodNotAllowed => "Method Not Allowed",
            HttpStatus::NotAcceptable => "Not Acceptable",
            HttpStatus::ProxyAuthenticationRequired => "Proxy Authentication Required",
            HttpStatus::RequestTimeout => "Request Timeout",
            HttpStatus::Conflict => "Conflict",
            HttpStatus::Gone => "Gone",
            HttpStatus::LengthRequired => "Length Required",
            HttpStatus::PreconditionFailed => "Precondition Failed",
            HttpStatus::PayloadTooLarge => "Payload Too Large",
            HttpStatus::URITooLong => "URI Too Long",
            HttpStatus::UnsupportedMediaType => "Unsupported Media Type",
            HttpStatus::RangeNotSatisfiable => "Range Not Satisfiable",
            HttpStatus::ExpectationFailed => "Expectation Failed",
            HttpStatus::MisdirectedRequest => "Misdirected Request",
            HttpStatus::UnprocessableEntity => "Unprocessable Entity",
            HttpStatus::Locked => "Locked",
            HttpStatus::FailedDependency => "Failed Dependency",
            HttpStatus::TooEarly => "Too Early",
            HttpStatus::UpgradeRequired => "Upgrade Required",
            HttpStatus::PreconditionRequired => "Precondition Required",
            HttpStatus::TooManyRequests => "Too Many Requests",
            HttpStatus::RequestHeaderFieldsTooLarge => "Request Header Fields Too Large",
            HttpStatus::GatewayTimeout => "Gateway Timeout",
            HttpStatus::HTTPVersionNotSupported => "HTTP Version Not Supported",
            HttpStatus::VariantAlsoNegotiates => "Variant Also Negotiates",
            HttpStatus::InsufficientStorage => "Insufficient Storage",
            HttpStatus::LoopDetected => "Loop Detected",
            HttpStatus::NotExtended => "Not Extended",
            HttpStatus::NetworkAuthenticationRequired => "Network Authentication Required",
        }
    }
}
