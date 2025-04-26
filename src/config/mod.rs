use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use crate::http::{method::Method, headers::Headers};

#[derive(Serialize, Deserialize, Clone)]
pub struct ResponseHTTPConfig {
    pub headers: Headers
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum ResponseConfig {
    Text { 
        text: String,
        http: ResponseHTTPConfig
    },
    File { 
        source: String,
        http: ResponseHTTPConfig
    }
}

#[derive(Serialize, Deserialize)]
pub struct RouteConfig {
    method: Method,
    response: ResponseConfig
}

#[derive(Deserialize)]
pub struct ServerConfig {
    pub port: u16,
    pub routes: HashMap<String, Vec<RouteConfig>>
}

impl TryFrom<String> for ServerConfig {
    type Error = String;

    fn try_from(s: String) -> Result<Self, Self::Error> {
        let config = match serde_json::from_str::<Self>(&s) {
            Ok(config) => config,
            Err(err) => return Err(err.to_string())
        };

        return Ok(config);
    }
}

impl ServerConfig {
    pub fn find_response_config(self: &Self, path: &String, method: &Method) -> Option<ResponseConfig> {
        let result = match self.routes.get(path) {
            Some(result) => result,
            None => return None
        };

        for route_config in result {
            if &route_config.method != method {
                continue;
            }

            return Some(route_config.response.clone());
        }

        return None;
    }
}
