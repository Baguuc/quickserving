use std::{
    collections::VecDeque, error::Error, io::Write, net::TcpStream, ops::{Index, IndexMut}
};

use crate::http::{Headers, HeaderName, Version};

use super::request::Request;

// status codes and reasons (messages) fetched from https://status.js.org
#[derive(Clone)]
pub enum StatusCode {
    Continue,
    SwitchingProtocols,
    Processing,
    EarlyHints,
    OK,
    Created,
    Accepted,
    NonAuthoritativeInformation,
    NoContent,
    ResetContent,
    PartialContent,
    MultiStatus,
    AlreadyReported,
    Thisisfine,
    IMUsed,
    MultipleChoices,
    MovedPermanently,
    Found,
    SeeOther,
    NotModified,
    SwitchProxy,
    TemporaryRedirect,
    ResumeIncomplete,
    BadRequest,
    Unauthorized,
    PaymentRequired,
    Forbidden,
    NotFound,
    MethodNotAllowed,
    NotAcceptable,
    ProxyAuthenticationRequired,
    RequestTimeout,
    Conflict,
    Gone,
    LengthRequired,
    PreconditionFailed,
    RequestEntityTooLarge,
    RequestURITooLong,
    UnsupportedMediaType,
    RequestedRangeNotSatisfiable,
    ExpectationFailed,
    ImATeapot,
    PageExpired,
    MethodFailure,
    MisdirectedRequest,
    UnprocessableEntity,
    Locked,
    FailedDependency,
    UpgradeRequired,
    PreconditionRequired,
    TooManyRequests,
    RequestHeaderFieldsTooLarge,
    LoginTimeout,
    ConnectionClosedWithoutResponse,
    RetryWith,
    BlockedbyWindowsParentalControls,
    UnavailableForLegalReasons,
    RequestHeaderTooLarge,
    SSLCertificateError,
    SSLCertificateRequired,
    HTTPRequestSenttoHTTPSPort,
    InvalidToken,
    ClientClosedRequest,
    InternalServerError,
    NotImplemented,
    BadGateway,
    ServiceUnavailable,
    GatewayTimeout,
    HTTPVersionNotSupported,
    VariantAlsoNegotiates,
    InsufficientStorage,
    LoopDetected,
    BandwidthLimitExceeded,
    NotExtended,
    NetworkAuthenticationRequired,
    UnknownError,
    WebServerIsDown,
    ConnectionTimedOut,
    OriginIsUnreachable,
    ATimeoutOccurred,
    SSLHandshakeFailed,
    InvalidSSLCertificate,
    RailgunListenertoOriginError,
    OriginDNSError,
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
            Self::Thisisfine => 218,
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
            Self::RailgunListenertoOriginError => 527,
            Self::OriginDNSError => 530,
            Self::NetworkReadTimeoutError => 598
        };
    }
}

impl Into<String> for StatusCode {
    fn into(self) -> String {
        return match self {
            Self::Continue => "Continue",
            Self::SwitchingProtocols => "Switching Protocols",
            Self::Processing => "Processing",
            Self::EarlyHints => "Early Hints",
            Self::OK => "OK",
            Self::Created => "Created",
            Self::Accepted => "Accepted",
            Self::NonAuthoritativeInformation => "Non-Authoritative Information",
            Self::NoContent => "No Content",
            Self::ResetContent => "Reset Content",
            Self::PartialContent => "Partial Content",
            Self::MultiStatus => "Multi-Status",
            Self::AlreadyReported => "Already Reported",
            Self::Thisisfine => "This is fine",
            Self::IMUsed => "IM Used",
            Self::MultipleChoices => "Multiple Choices",
            Self::MovedPermanently => "Moved Permanently",
            Self::Found => "Found",
            Self::SeeOther => "See Other",
            Self::NotModified => "Not Modified",
            Self::SwitchProxy => "Switch Proxy",
            Self::TemporaryRedirect => "Temporary Redirect",
            Self::ResumeIncomplete => "Resume Incomplete",
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
            Self::RequestEntityTooLarge => "Request Entity Too Large",
            Self::RequestURITooLong => "Request-URI Too Long",
            Self::UnsupportedMediaType => "Unsupported Media Type",
            Self::RequestedRangeNotSatisfiable => "Requested Range Not Satisfiable",
            Self::ExpectationFailed => "Expectation Failed",
            Self::ImATeapot => "I'm a teapot",
            Self::PageExpired => "Page Expired",
            Self::MethodFailure => "Method Failure",
            Self::MisdirectedRequest => "Misdirected Request",
            Self::UnprocessableEntity => "Unprocessable Entity",
            Self::Locked => "Locked",
            Self::FailedDependency => "Failed Dependency",
            Self::UpgradeRequired => "Upgrade Required",
            Self::PreconditionRequired => "Precondition Required",
            Self::TooManyRequests => "Too Many Requests",
            Self::RequestHeaderFieldsTooLarge => "Request Header Fields Too Large",
            Self::LoginTimeout => "Login Time-out",
            Self::ConnectionClosedWithoutResponse => "Connection Closed Without Response",
            Self::RetryWith => "Retry With",
            Self::BlockedbyWindowsParentalControls => "Blocked by Windows Parental Controls",
            Self::UnavailableForLegalReasons => "Unavailable For Legal Reasons",
            Self::RequestHeaderTooLarge => "Request Header Too Large",
            Self::SSLCertificateError => "SSL Certificate Error",
            Self::SSLCertificateRequired => "SSL Certificate Required",
            Self::HTTPRequestSenttoHTTPSPort => "HTTP Request Sent to HTTPS Port",
            Self::InvalidToken => "Invalid Token",
            Self::ClientClosedRequest => "Client Closed Request",
            Self::InternalServerError => "Internal Server Error",
            Self::NotImplemented => "Not Implemented",
            Self::BadGateway => "Bad Gateway",
            Self::ServiceUnavailable => "Service Unavailable",
            Self::GatewayTimeout => "Gateway Timeout",
            Self::HTTPVersionNotSupported => "HTTP Version Not Supported",
            Self::VariantAlsoNegotiates => "Variant Also Negotiates",
            Self::InsufficientStorage => "Insufficient Storage",
            Self::LoopDetected => "Loop Detected",
            Self::BandwidthLimitExceeded => "Bandwidth Limit Exceeded",
            Self::NotExtended => "Not Extended",
            Self::NetworkAuthenticationRequired => "Network Authentication Required",
            Self::UnknownError => "Unknown Error",
            Self::WebServerIsDown => "Web Server Is Down",
            Self::ConnectionTimedOut => "Connection Timed Out",
            Self::OriginIsUnreachable => "Origin Is Unreachable",
            Self::ATimeoutOccurred => "A Timeout Occurred",
            Self::SSLHandshakeFailed => "SSL Handshake Failed",
            Self::InvalidSSLCertificate => "Invalid SSL Certificate",
            Self::RailgunListenertoOriginError => "Railgun Listener to Origin Error",
            Self::OriginDNSError => "Origin DNS Error",
            Self::NetworkReadTimeoutError => "Network Read Timeout Error"
        }
        .to_string();
    }
}

impl Into<Status> for StatusCode {
    fn into(self) -> Status {
        let status_code = self.clone().into();
        let reason = self.into();

        return Status {
            status_code,
            reason
        };
    }
}

impl TryFrom<String> for StatusCode {
    type Error = String;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        return Ok(match value.as_str() {
            "100" => Self::Continue,
            "101" => Self::SwitchingProtocols,
            "102" => Self::Processing,
            "103" => Self::EarlyHints,
            "200" => Self::OK,
            "201" => Self::Created,
            "202" => Self::Accepted,
            "203" => Self::NonAuthoritativeInformation,
            "204" => Self::NoContent,
            "205" => Self::ResetContent,
            "206" => Self::PartialContent,
            "207" => Self::MultiStatus,
            "208" => Self::AlreadyReported,
            "218" => Self::Thisisfine,
            "226" => Self::IMUsed,
            "300" => Self::MultipleChoices,
            "301" => Self::MovedPermanently,
            "302" => Self::Found,
            "303" => Self::SeeOther,
            "304" => Self::NotModified,
            "306" => Self::SwitchProxy,
            "307" => Self::TemporaryRedirect,
            "308" => Self::ResumeIncomplete,
            "400" => Self::BadRequest,
            "401" => Self::Unauthorized,
            "402" => Self::PaymentRequired,
            "403" => Self::Forbidden,
            "404" => Self::NotFound,
            "405" => Self::MethodNotAllowed,
            "406" => Self::NotAcceptable,
            "407" => Self::ProxyAuthenticationRequired,
            "408" => Self::RequestTimeout,
            "409" => Self::Conflict,
            "410" => Self::Gone,
            "411" => Self::LengthRequired,
            "412" => Self::PreconditionFailed,
            "413" => Self::RequestEntityTooLarge,
            "414" => Self::RequestURITooLong,
            "415" => Self::UnsupportedMediaType,
            "416" => Self::RequestedRangeNotSatisfiable,
            "417" => Self::ExpectationFailed,
            "418" => Self::ImATeapot,
            "419" => Self::PageExpired,
            "420" => Self::MethodFailure,
            "421" => Self::MisdirectedRequest,
            "422" => Self::UnprocessableEntity,
            "423" => Self::Locked,
            "424" => Self::FailedDependency,
            "426" => Self::UpgradeRequired,
            "428" => Self::PreconditionRequired,
            "429" => Self::TooManyRequests,
            "431" => Self::RequestHeaderFieldsTooLarge,
            "440" => Self::LoginTimeout,
            "444" => Self::ConnectionClosedWithoutResponse,
            "449" => Self::RetryWith,
            "450" => Self::BlockedbyWindowsParentalControls,
            "451" => Self::UnavailableForLegalReasons,
            "494" => Self::RequestHeaderTooLarge,
            "495" => Self::SSLCertificateError,
            "496" => Self::SSLCertificateRequired,
            "497" => Self::HTTPRequestSenttoHTTPSPort,
            "498" => Self::InvalidToken,
            "499" => Self::ClientClosedRequest,
            "500" => Self::InternalServerError,
            "501" => Self::NotImplemented,
            "502" => Self::BadGateway,
            "503" => Self::ServiceUnavailable,
            "504" => Self::GatewayTimeout,
            "505" => Self::HTTPVersionNotSupported,
            "506" => Self::VariantAlsoNegotiates,
            "507" => Self::InsufficientStorage,
            "508" => Self::LoopDetected,
            "509" => Self::BandwidthLimitExceeded,
            "510" => Self::NotExtended,
            "511" => Self::NetworkAuthenticationRequired,
            "520" => Self::UnknownError,
            "521" => Self::WebServerIsDown,
            "522" => Self::ConnectionTimedOut,
            "523" => Self::OriginIsUnreachable,
            "524" => Self::ATimeoutOccurred,
            "525" => Self::SSLHandshakeFailed,
            "526" => Self::InvalidSSLCertificate,
            "527" => Self::RailgunListenertoOriginError,
            "530" => Self::OriginDNSError,
            "598" => Self::NetworkReadTimeoutError,
            _ => return Err("Invalid code".to_string())
        });
    }
}

impl TryFrom<u16> for StatusCode {
   type Error = String;

   fn try_from(value: u16) -> Result<Self, Self::Error> {
       return value.to_string().try_into();
   }
}

pub struct Status {
    status_code: u16,
    reason: String
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

    pub fn from_string(string: String) -> Result<Self, Box<dyn Error>> {
        let mut rows = string.lines();
        let first_row = rows.nth(0);

        if first_row.is_none() {
            return Err("Invalid request.".into());
        }

        let first_row = first_row.unwrap();

        let columns = first_row.split(' ').collect::<Vec<&str>>();

        if columns.len() < 3 {
            return Err("Invalid request.".into());
        }

        let version_str = columns.get(0).unwrap().to_string();

        let version_split = version_str.split("/").collect::<Vec<&str>>();

        if version_split.len() < 2 {
            return Err("Invalid request.".into());
        }

        let status_code = columns.get(1)
            .unwrap()
            .to_string();

        let status_code = match StatusCode::try_from(status_code) {
            Ok(status) => status,
            Err(_) => return Err("Invalid request.".into())
        };
        let status = status_code.into();

        let version = Version::new(
            version_split.get(0).unwrap().to_string(),
            version_split.get(1).unwrap().to_string(),
        );

        let mut headers = Headers::new();
        let mut body = String::new();

        let request_parts = string.split("\r\n").collect::<Vec<&str>>();

        let header_rows = request_parts.get(0).unwrap_or(&"").lines();

        for row in header_rows {
            let split = row.split(": ").collect::<Vec<&str>>();

            if split.len() < 2 {
                continue;
            }

            let name = match HeaderName::try_from(split.get(0).unwrap().to_string()) {
                Ok(name) => name,
                Err(_) => continue
            };
            let value = split.get(1).unwrap().to_string();

            headers.insert(name, value);
        }

        body = request_parts.get(1).unwrap_or(&"").to_string();

        let request_data = Self::new(status, version, headers, body);

        return Ok(request_data);
    }

    pub fn to_string(&self) -> String {
        return format!(
            "{} {} {}\n{}\r\n{}",
            self.version.to_string(),
            self.status.status_code,
            self.status.reason,
            self.headers.to_string(),
            self.body
        );
    }
}
