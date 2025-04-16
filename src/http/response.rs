use crate::http::{Headers, Version};
use serde::{self, Serialize, Deserialize};

// status codes and reasons (messages) fetched from https://status.js.org
#[derive(Serialize, Deserialize, Clone)]
pub enum StatusCode {
    #[serde(rename="Continue")]
    Continue,
    #[serde(rename="Switching Protocols")]
    SwitchingProtocols,
    #[serde(rename="Processing")]
    Processing,
    #[serde(rename="Early Hints")]
    EarlyHints,
    #[serde(rename="OK")]
    OK,
    #[serde(rename="Created")]
    Created,
    #[serde(rename="Accepted")]
    Accepted,
    #[serde(rename="Non Authoritative Information")]
    NonAuthoritativeInformation,
    #[serde(rename="No Content")]
    NoContent,
    #[serde(rename="Reset Content")]
    ResetContent,
    #[serde(rename="Partial Content")]
    PartialContent,
    #[serde(rename="Multi Status")]
    MultiStatus,
    #[serde(rename="Already Reported")]
    AlreadyReported,
    #[serde(rename="This is fine")]
    ThisIsFine,
    #[serde(rename="I'm Used")]
    IMUsed,
    #[serde(rename="Multiple Choices")]
    MultipleChoices,
    #[serde(rename="Moved Permanently")]
    MovedPermanently,
    #[serde(rename="Found")]
    Found,
    #[serde(rename="See Other")]
    SeeOther,
    #[serde(rename="Not Modified")]
    NotModified,
    #[serde(rename="Switch Proxy")]
    SwitchProxy,
    #[serde(rename="Temporary Redirect")]
    TemporaryRedirect,
    #[serde(rename="Resume Incomplete")]
    ResumeIncomplete,
    #[serde(rename="Bad Request")]
    BadRequest,
    #[serde(rename="Unauthorized")]
    Unauthorized,
    #[serde(rename="Payment Required")]
    PaymentRequired,
    #[serde(rename="Forbidden")]
    Forbidden,
    #[serde(rename="Not Found")]
    NotFound,
    #[serde(rename="Method Not Allowed")]
    MethodNotAllowed,
    #[serde(rename="Not Acceptable")]
    NotAcceptable,
    #[serde(rename="Proxy Authentication Required")]
    ProxyAuthenticationRequired,
    #[serde(rename="Request Timeout")]
    RequestTimeout,
    #[serde(rename="Conflict")]
    Conflict,
    #[serde(rename="Gone")]
    Gone,
    #[serde(rename="Length Required")]
    LengthRequired,
    #[serde(rename="Precondition Failed")]
    PreconditionFailed,
    #[serde(rename="Request Entity Too Large")]
    RequestEntityTooLarge,
    #[serde(rename="Request URI Too Long")]
    RequestURITooLong,
    #[serde(rename="Unsupported Media Type")]
    UnsupportedMediaType,
    #[serde(rename="Requested Range Not Satisfiable")]
    RequestedRangeNotSatisfiable,
    #[serde(rename="Expectation Failed")]
    ExpectationFailed,
    #[serde(rename="Im A Teapot")]
    ImATeapot,
    #[serde(rename="Page Expired")]
    PageExpired,
    #[serde(rename="Method Failure")]
    MethodFailure,
    #[serde(rename="Misdirected Request")]
    MisdirectedRequest,
    #[serde(rename="Unprocessable Entity")]
    UnprocessableEntity,
    #[serde(rename="Locked")]
    Locked,
    #[serde(rename="Failed Dependency")]
    FailedDependency,
    #[serde(rename="Upgrade Required")]
    UpgradeRequired,
    #[serde(rename="Precondition Required")]
    PreconditionRequired,
    #[serde(rename="Too Many Requests")]
    TooManyRequests,
    #[serde(rename="Request Header Fields Too Large")]
    RequestHeaderFieldsTooLarge,
    #[serde(rename="Login Timeout")]
    LoginTimeout,
    #[serde(rename="Connection Closed Without Response")]
    ConnectionClosedWithoutResponse,
    #[serde(rename="Retry With")]
    RetryWith,
    #[serde(rename="Blockedby Windows Parental Controls")]
    BlockedbyWindowsParentalControls,
    #[serde(rename="Unavailable For Legal Reasons")]
    UnavailableForLegalReasons,
    #[serde(rename="Request Header Too Large")]
    RequestHeaderTooLarge,
    #[serde(rename="SSL Certificate Error")]
    SSLCertificateError,
    #[serde(rename="SSL Certificate Required")]
    SSLCertificateRequired,
    #[serde(rename="HTTP Request Sentto HTTPS Port")]
    HTTPRequestSenttoHTTPSPort,
    #[serde(rename="Invalid Token")]
    InvalidToken,
    #[serde(rename="Client Closed Request")]
    ClientClosedRequest,
    #[serde(rename="Internal Server Error")]
    InternalServerError,
    #[serde(rename="Not Implemented")]
    NotImplemented,
    #[serde(rename="Bad Gateway")]
    BadGateway,
    #[serde(rename="Service Unavailable")]
    ServiceUnavailable,
    #[serde(rename="Gateway Timeout")]
    GatewayTimeout,
    #[serde(rename="HTTP Version Not Supported")]
    HTTPVersionNotSupported,
    #[serde(rename="Variant Also Negotiates")]
    VariantAlsoNegotiates,
    #[serde(rename="Insufficient Storage")]
    InsufficientStorage,
    #[serde(rename="Loop Detected")]
    LoopDetected,
    #[serde(rename="Bandwidth Limit Exceeded")]
    BandwidthLimitExceeded,
    #[serde(rename="Not Extended")]
    NotExtended,
    #[serde(rename="Network Authentication Required")]
    NetworkAuthenticationRequired,
    #[serde(rename="Unknown Error")]
    UnknownError,
    #[serde(rename="Web Server Is Down")]
    WebServerIsDown,
    #[serde(rename="Connection Timed Out")]
    ConnectionTimedOut,
    #[serde(rename="Origin Is Unreachable")]
    OriginIsUnreachable,
    #[serde(rename="A Timeout Occurred")]
    ATimeoutOccurred,
    #[serde(rename="SSL Handshake Failed")]
    SSLHandshakeFailed,
    #[serde(rename="Invalid SSL Certificate")]
    InvalidSSLCertificate,
    #[serde(rename="Railgun Listener to Origin Error")]
    RailgunListenerToOriginError,
    #[serde(rename="Origin DNS Error")]
    OriginDNSError,
    #[serde(rename="Network Read Timeout Error")]
    NetworkReadTimeoutError
}

impl Into<u16> for StatusCode {
    fn into(self) -> u16 {
        return match self {
            Self::Continue => 100,
            Self::SwitchingProtocols => 101,
            Self::Processing => 102,
            Self::EarlyHints => 103,
            Self::OK => 200,
            Self::Created => 201,
            Self::Accepted => 202,
            Self::NonAuthoritativeInformation => 203,
            Self::NoContent => 204,
            Self::ResetContent => 205,
            Self::PartialContent => 206,
            Self::MultiStatus => 207,
            Self::AlreadyReported => 208,
            Self::ThisIsFine => 218,
            Self::IMUsed => 226,
            Self::MultipleChoices => 300,
            Self::MovedPermanently => 301,
            Self::Found => 302,
            Self::SeeOther => 303,
            Self::NotModified => 304,
            Self::SwitchProxy => 306,
            Self::TemporaryRedirect => 307,
            Self::ResumeIncomplete => 308,
            Self::BadRequest => 400,
            Self::Unauthorized => 401,
            Self::PaymentRequired => 402,
            Self::Forbidden => 403,
            Self::NotFound => 404,
            Self::MethodNotAllowed => 405,
            Self::NotAcceptable => 406,
            Self::ProxyAuthenticationRequired => 407,
            Self::RequestTimeout => 408,
            Self::Conflict => 409,
            Self::Gone => 410,
            Self::LengthRequired => 411,
            Self::PreconditionFailed => 412,
            Self::RequestEntityTooLarge => 413,
            Self::RequestURITooLong => 414,
            Self::UnsupportedMediaType => 415,
            Self::RequestedRangeNotSatisfiable => 416,
            Self::ExpectationFailed => 417,
            Self::ImATeapot => 418,
            Self::PageExpired => 419,
            Self::MethodFailure => 420,
            Self::MisdirectedRequest => 421,
            Self::UnprocessableEntity => 422,
            Self::Locked => 423,
            Self::FailedDependency => 424,
            Self::UpgradeRequired => 426,
            Self::PreconditionRequired => 428,
            Self::TooManyRequests => 429,
            Self::RequestHeaderFieldsTooLarge => 431,
            Self::LoginTimeout => 440,
            Self::ConnectionClosedWithoutResponse => 444,
            Self::RetryWith => 449,
            Self::BlockedbyWindowsParentalControls => 450,
            Self::UnavailableForLegalReasons => 451,
            Self::RequestHeaderTooLarge => 494,
            Self::SSLCertificateError => 495,
            Self::SSLCertificateRequired => 496,
            Self::HTTPRequestSenttoHTTPSPort => 497,
            Self::InvalidToken => 498,
            Self::ClientClosedRequest => 499,
            Self::InternalServerError => 500,
            Self::NotImplemented => 501,
            Self::BadGateway => 502,
            Self::ServiceUnavailable => 503,
            Self::GatewayTimeout => 504,
            Self::HTTPVersionNotSupported => 505,
            Self::VariantAlsoNegotiates => 506,
            Self::InsufficientStorage => 507,
            Self::LoopDetected => 508,
            Self::BandwidthLimitExceeded => 509,
            Self::NotExtended => 510,
            Self::NetworkAuthenticationRequired => 511,
            Self::UnknownError => 520,
            Self::WebServerIsDown => 521,
            Self::ConnectionTimedOut => 522,
            Self::OriginIsUnreachable => 523,
            Self::ATimeoutOccurred => 524,
            Self::SSLHandshakeFailed => 525,
            Self::InvalidSSLCertificate => 526,
            Self::RailgunListenerToOriginError => 527,
            Self::OriginDNSError => 530,
            Self::NetworkReadTimeoutError => 598
        };
    }
}

impl Into<Status> for StatusCode {
    fn into(self) -> Status {
        return Status {
            status_code: self.into()
        };
    }
}

pub struct Status {
    status_code: u16
}

pub struct Response {
    status: Status,
    version: Version,
    headers: Headers,
    body: String,
}

impl Response {
    pub fn new(
        status: Status,
        version: Version,
        headers: Headers,
        body: String,
    ) -> Self {
        return Self {
            status,
            version,
            headers,
            body,
        };
    }
}

impl Into<String> for Response {
    fn into(self) -> String {
        let headers: String = self.headers.into();

        return format!(
            "{} {}\n{}\r\n{}",
            self.version.to_string(),
            self.status.status_code,
            headers,
            self.body
        );
    }
}
