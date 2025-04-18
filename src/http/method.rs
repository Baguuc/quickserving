use serde::{self, Serialize, Deserialize};

#[derive(Serialize, Deserialize, PartialEq, Eq)]
pub enum Method {
    GET,
    HEAD,
    OPTIONS,
    TRACE,
    PUT,
    DELETE,
    POST,
    PATCH,
    CONNECT
}

impl TryFrom<String> for Method {
    type Error = String;

    fn try_from(s: String) -> Result<Self, Self::Error> {
        match serde_json::from_str(format!("\"{}\"", s).as_str()) {
            Ok(method) => return Ok(method),
            Err(err) => return Err(err.to_string()),
        };
    }
}

impl Into<String> for Method {
    fn into(self) -> String {
        return serde_json::to_string(&self)
            .unwrap()
            .trim_matches('"')
            .to_string();
    }
}

