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
struct RouteConfig {
    method: Method,
    response: ResponseConfig
}

#[derive(Serialize, Deserialize, Hash, PartialEq, Eq)]
pub struct RequestedRoute {
    pub method: Method,
    pub path: String
}

pub struct ServerConfig {
    pub port: u16,
    pub routes: HashMap<RequestedRoute, ResponseConfig>
}

impl TryFrom<String> for ServerConfig {
    type Error = String;

    fn try_from(s: String) -> Result<Self, Self::Error> {
        #[derive(Deserialize)]
        struct Raw {
            port: u16,
            routes: HashMap<String, Vec<RouteConfig>>
        }

        let raw = match serde_json::from_str::<Raw>(&s) {
            Ok(raw) => raw,
            Err(err) => return Err(err.to_string())
        };

        let mut routes = HashMap::new();

        for (path, route) in &raw.routes {
            for route_config in route {
                let requested_route = RequestedRoute {
                    method: route_config.method.clone(),
                    path: path.to_string()
                };

                let response_config = route_config.response.clone();

                let _ = routes.insert(requested_route, response_config);
            }
        }

        let config = ServerConfig {
            port: raw.port,
            routes
        };

        return Ok(config);
    }
}
