use std::{collections::HashMap, error::Error};
use serde::{self, Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Hash, PartialEq, Eq, Clone)]
pub enum HeaderName {
    #[serde(rename="Content-MD5")]
    ContentMD5,
    #[serde(rename="HTTP2-Settings")]
    HTTP2Settings,
    Warning,
    Pragma,
    #[serde(rename="A-IM")]
    AIM,
    Accept,
    #[serde(rename="Accept-Charset")]
    AcceptCharset,
    #[serde(rename="Accept-Encoding")]
    AcceptEncoding,
    #[serde(rename="Accept-Language")]
    AcceptLanguage,
    Authorization,
    #[serde(rename="Cache-Control")]
    CacheControl,
    Connection,
    #[serde(rename="Content-Encoding")]
    ContentEncoding,
    #[serde(rename="Content-Length")]
    ContentLength,
    #[serde(rename="Content-Type")]
    ContentType,
    Date,
    Expect,
    Forwarded,
    From,
    Host,
    #[serde(rename="If-Match")]
    IfMatch,
    #[serde(rename="If-Modified-Since")]
    IfModifiedSince,
    #[serde(rename="If-None-Match")]
    IfNoneMatch,
    #[serde(rename="If-Range")]
    IfRange,
    #[serde(rename="If-Unmodified-Since")]
    IfUnmodifiedSince,
    #[serde(rename="Max-Forwards")]
    MaxForwards,
    Prefer,
    #[serde(rename="Proxy-Authorization")]
    ProxyAuthorization,
    Range,
    Referer,
    TE,
    Trailer,
    #[serde(rename="Transfer-Encoding")]
    TransferEncoding,
    #[serde(rename="User-Agent")]
    UserAgent,
    Upgrade,
    Via,
    #[serde(rename="Access-Control-Request-Method")]
    AccessControlRequestMethod,
    #[serde(rename="Access-Control-Request-Headers")]
    AccessControlRequestHeaders,
    Cookie,
    Origin,
    #[serde(rename="Accept-Datetime")]
    AcceptDatetime
}

impl TryFrom<String> for HeaderName {
    type Error = String;

    fn try_from(s: String) -> Result<Self, Self::Error> {
        match serde_json::from_str(format!("\"{}\"", s).as_str()) {
            Ok(header) => return Ok(header),
            Err(err) => return Err(err.to_string()),
        };
    }
}

impl Into<String> for HeaderName {
    fn into(self) -> String {
        return serde_json::to_string(&self)
            .unwrap()
            .trim_matches('"')
            .to_string();
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
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
        let result = self.0.insert(name, value);
    
        if result.is_none() {
            return Err("Header value not found.".into());
        }

        return Ok(());
    }

    pub fn remove(self: &mut Self, name: HeaderName) -> Result<(), Box<dyn Error>> {
        let result = self.0.remove(&name);
    
        if result.is_none() {
            return Err("Header value not found.".into());
        }
        
        return Ok(());
    }
}

impl Into<String> for Headers {
    fn into(self) -> String {
        let formatted = self
            .0
            .keys()
            .map(|name| {
                let value = self.get(name).unwrap();
                let name: String = name.clone().into();

                format!("{}: {}\n", name, value)            
            })
            .collect::<String>();

        return formatted;
    }
}

impl TryFrom<String> for Headers {
    type Error = String;

    fn try_from(s: String) -> Result<Self, Self::Error> {
        let mut headers = Self::new();
        let lines = s.lines();

        for line in lines {
            let mut key = "".to_string();
            let mut value = "".to_string();

            let mut i = 0;

            for c in line.chars() {
                match (i, c) {
                    (0, ' ') => {
                        i += 1; 
                        continue;
                    },
                    (0, ':') => { continue; },
                    (0, _) => { key.push(c); },
                    (1, _) => { value.push(c); },
                    (_, _) => ()
                }
            }
            
            let name = HeaderName::try_from(key)?;
            let _ = headers.insert(name, value);
        }
        
        let headers = headers;

        return Ok(headers);
    }
}
