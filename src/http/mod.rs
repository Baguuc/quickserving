use std::{collections::HashMap, error::Error};
use serde::{Serialize, Deserialize};

pub mod request;
pub mod response;
pub mod server;

#[derive(Serialize, Deserialize, Debug, Hash, PartialEq, Eq)]
#[serde(untagged)]
pub enum HeaderName {
    ContentMD5,
    HTTP2Settings,
    Warning,
    Pragma,
    AIM,
    Accept,
    AcceptCharset,
    AcceptEncoding,
    AcceptLanguage,
    Authorization,
    CacheControl,
    Connection,
    ContentEncoding,
    ContentLength,
    ContentType,
    Date,
    Expect,
    Forwarded,
    From,
    Host,
    IfMatch,
    IfModifiedSince,
    IfNoneMatch,
    IfRange,
    IfUnmodifiedSince,
    MaxForwards,
    Prefer,
    ProxyAuthorization,
    Range,
    Referer,
    TE,
    Trailer,
    TransferEncoding,
    UserAgent,
    Upgrade,
    Via,
    AccessControlRequestMethod,
    AccessControlRequestHeaders,
    Cookie,
    Origin,
    AcceptDatetime
}

impl ToString for HeaderName {
    fn to_string(&self) -> String {
        return match self {
            HeaderName::ContentMD5 => "Content-MD5",
            HeaderName::HTTP2Settings => "HTTP2-Settings",
            HeaderName::Warning => "Warning",
            HeaderName::Pragma => "Pragma",
            HeaderName::AIM => "A-IM",
            HeaderName::Accept => "Accept",
            HeaderName::AcceptCharset => "Accept-Charset",
            HeaderName::AcceptEncoding => "Accept-Encoding",
            HeaderName::AcceptLanguage => "Accept-Language",
            HeaderName::Authorization => "Authorization",
            HeaderName::CacheControl => "Cache-Control",
            HeaderName::Connection => "Connection",
            HeaderName::ContentEncoding => "Content-Encoding",
            HeaderName::ContentLength => "Content-Length",
            HeaderName::ContentType => "Content-Type",
            HeaderName::Date => "Date",
            HeaderName::Expect => "Expect",
            HeaderName::Forwarded => "Forwarded",
            HeaderName::From => "From",
            HeaderName::Host => "Host",
            HeaderName::IfMatch => "If-Match",
            HeaderName::IfModifiedSince => "If-Modified-Since",
            HeaderName::IfNoneMatch => "If-None-Match",
            HeaderName::IfRange => "If-Range",
            HeaderName::IfUnmodifiedSince => "If-Unmodified-Since",
            HeaderName::MaxForwards => "Max-Forwards",
            HeaderName::Prefer => "Prefer",
            HeaderName::ProxyAuthorization => "Proxy-Authorization",
            HeaderName::Range => "Range",
            HeaderName::Referer => "Referer",
            HeaderName::TE => "TE",
            HeaderName::Trailer => "Trailer",
            HeaderName::TransferEncoding => "Transfer-Encoding",
            HeaderName::UserAgent => "User-Agent",
            HeaderName::Upgrade => "Upgrade",
            HeaderName::Via => "Via",
            HeaderName::AccessControlRequestMethod => "Access-Control-Request-Method",
            HeaderName::AccessControlRequestHeaders => "Access-Control-Request-Headers",
            HeaderName::Cookie => "Cookie",
            HeaderName::Origin => "Origin",
            HeaderName::AcceptDatetime => "Accept-Datetime"
        }
        .to_string();
    }
}

impl TryFrom<String> for HeaderName {
    type Error = String;

    fn try_from(s: String) -> Result<Self, Self::Error> {
        return match s.as_str() {
            "Content-MD5" => Ok(Self::ContentMD5),
            "HTTP2-Settings" => Ok(Self::HTTP2Settings),
            "Warning" => Ok(Self::Warning),
            "Pragma" => Ok(Self::Pragma),
            "A-IM" => Ok(Self::AIM),
            "Accept" => Ok(Self::Accept),
            "Accept-Charset" => Ok(Self::AcceptCharset),
            "Accept-Encoding" => Ok(Self::AcceptEncoding),
            "Accept-Language" => Ok(Self::AcceptLanguage),
            "Authorization" => Ok(Self::Authorization),
            "Cache-Control" => Ok(Self::CacheControl),
            "Connection" => Ok(Self::Connection),
            "Content-Encoding" => Ok(Self::ContentEncoding),
            "Content-Length" => Ok(Self::ContentLength),
            "Content-Type" => Ok(Self::ContentType),
            "Date" => Ok(Self::Date),
            "Expect" => Ok(Self::Expect),
            "Forwarded" => Ok(Self::Forwarded),
            "From" => Ok(Self::From),
            "Host" => Ok(Self::Host),
            "If-Match" => Ok(Self::IfMatch),
            "If-Modified-Since" => Ok(Self::IfModifiedSince),
            "If-None-Match" => Ok(Self::IfNoneMatch),
            "If-Range" => Ok(Self::IfRange),
            "If-Unmodified-Since" => Ok(Self::IfUnmodifiedSince),
            "Max-Forwards" => Ok(Self::MaxForwards),
            "Prefer" => Ok(Self::Prefer),
            "Proxy-Authorization" => Ok(Self::ProxyAuthorization),
            "Range" => Ok(Self::Range),
            "Referer" => Ok(Self::Referer),
            "TE" => Ok(Self::TE),
            "Trailer" => Ok(Self::Trailer),
            "Transfer-Encoding" => Ok(Self::TransferEncoding),
            "User-Agent" => Ok(Self::UserAgent),
            "Upgrade" => Ok(Self::Upgrade),
            "Via" => Ok(Self::Via),
            "Access-Control-Request-Method" => Ok(Self::AccessControlRequestMethod),
            "Access-Control-Request-Headers" => Ok(Self::AccessControlRequestHeaders),
            "Cookie" => Ok(Self::Cookie),
            "Origin" => Ok(Self::Origin),
            "Accept-Datetime " => Ok(Self::AcceptDatetime),
            _ => return Err("Wrong value".to_string())
        };
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Headers(
    HashMap<HeaderName, String>
);

impl Headers {
    pub fn new() -> Self {
        return Self(HashMap::new());
    }

    pub fn get(self: &Self, name: &HeaderName) -> Option<&String> {
        return self.0.get(name);
    }

    pub fn insert(self: &mut Self, name: HeaderName, value: String) -> Result<(), Box<dyn Error>> {
        let _ = self.0.insert(name, value);

        return Ok(());
    }
}

impl ToString for Headers {
    fn to_string(&self) -> String {
        let formatted = self
            .0
            .keys()
            .map(|name| format!("{}: {}\n", name.to_string(), self.0.get(name).unwrap()).to_string())
            .collect::<String>();

        return formatted;
    }
}

impl From<String> for Headers {
    fn from(s: String) -> Self {
        let mut headers = Self::new();
        let rows = s.lines();
        
        for row in rows {
            let split = row.split(": ").collect::<Vec<&str>>();

            if split.len() < 2 {
                continue;
            }

            let key = match HeaderName::try_from(split.get(0).unwrap().to_string()) {
                Ok(name) => name,
                Err(_) => continue
            };
            let value = split.get(1).unwrap().to_string();

            let _ = headers.insert(key, value);
        }
        let headers = headers;

        return headers;
    }
}

pub struct Version {
    name: String,
    version: String,
}

impl Version {
    pub fn new(name: String, version: String) -> Self {
        return Self { name, version };
    }
}

impl ToString for Version {
    fn to_string(&self) -> String {
        return format!("{}/{}", self.name, self.version).to_string();
    }
}

impl TryFrom<String> for Version {
    type Error = String;
    
    fn try_from(s: String) -> Result<Self, Self::Error> {
        let version_split = s.split("/").collect::<Vec<&str>>();

        if version_split.len() < 2 {
            return Err("Invalid version string.".into());
        }

        let version = Version::new(
            version_split.get(0).unwrap().to_string(),
            version_split.get(1).unwrap().to_string()
        );

        return Ok(version);
    }
}
