use std::{
    collections::HashMap,
    error::Error,
    fs,
    io::{Read, Write},
    net::{TcpListener, TcpStream},
};

use chrono::Utc;
use log::{info, warn};
use serde_json::{Number, Value};

use crate::http::{request::{self, Request}, response::{self, Response}, Headers, Version};

pub struct Server {
    pub port: u16,
    pub directory: String,
    pub index_file: String,
    pub not_found_uri: String,
}

impl From<Value> for Server {
    fn from(value: Value) -> Self {
        let default = Self::default();

        let port = value.get("port")
            .unwrap_or(&Value::Number(Number::from_u128(default.port as u128).unwrap()))
            .as_u64()
            .unwrap();

        let port = if port > (u16::MAX as u64) {
            default.port
        } else { 
            port as u16
        };

        let directory = value.get("directory")
            .unwrap_or(&Value::String(default.directory))
            .as_str()
            .unwrap()
            .to_string();
        let index_file = value.get("index_file")
            .unwrap_or(&Value::String(default.index_file))
            .as_str()
            .unwrap()
            .to_string();
        let not_found_uri = value.get("not_found_uri")
            .unwrap_or(&Value::String(default.not_found_uri))
            .as_str()
            .unwrap()
            .to_string();

        return Self {
            port,
            directory,
            index_file,
            not_found_uri
        };
    }
}

impl Default for Server {
    fn default() -> Self {
        return Self {
            port: 3000,
            directory: "./".to_string(),
            index_file: "index.html".to_string(),
            not_found_uri: "404.html".to_string(),
        };
    }
}

impl Server {
    pub fn listen(self: Self) -> Result<(), Box<dyn Error>> {
        // we bind our listener to port from self
        let listener = TcpListener::bind(format!("0.0.0.0:{}", self.port));

        if listener.is_err() {
            return Err("this port is already in use.".into());
        }

        let listener = listener.unwrap();

        info!(
            "Serving directory {} on port {}.",
            self.directory, self.port
        );
        loop {
            // we read every request that the listener has recieved and try to handle it
            let (stream, _) = listener.accept().unwrap();
            
            let request = Request::read_from_stream(&stream)
                .unwrap();

            let result = request.respond(
                create_response,
                &self,
                &stream
            );
        }
    }

}

fn create_response(server: &Server, request: &Request) -> Response {
    let mut path = request.path.clone();

    if path.ends_with("/") {
        path = format!("{}{}", request.path, server.index_file).to_string();
    }

    info!("requested path {}.", path);

    let resource_path = format!("{}/{}", server.directory.trim_end_matches("/"), path);
    let resource_content = fs::read_to_string(resource_path);

    let response = if resource_content.is_err() {
        let resource_content = fs::read_to_string(format!(
            "{}/{}",
            &server.directory.trim_end_matches('/'),
            &server.not_found_uri
        ))
        .unwrap_or("404".to_string());
        let resource_len = resource_content.len();

        let mut headers = Headers::new();

        headers.insert(&"content-type".to_string(), "text/html".to_string());
        headers.insert(&"content-lenght".to_string(), resource_len.to_string());
        headers.insert(&"server".to_string(), "quickserving".to_string());

        Response::new(
            404,
            "Resource not found".to_string(),
            Version::new("http".to_string(), "1.1".to_string()),
            headers,
            resource_content,
        )
    } else {
        let resource_content = resource_content.unwrap();
        let resource_len = resource_content.len();

        let mut headers = Headers::new();

        let _ = headers.insert(
            &"content-type".to_string(),
            mime_guess::from_path(path)
                .first()
                .unwrap()
                .to_string(),
        );
        let _ = headers.insert(&"content-lenght".to_string(), resource_len.to_string());
        let _ = headers.insert(&"server".to_string(), "quickserving".to_string());
        let _ = headers.insert(&"date".to_string(), Utc::now().to_string());

        Response::new(
            200,
            "OK".to_string(),
            Version::new("http".to_string(), "1.1".to_string()),
            headers,
            resource_content,
        )
    };

    return response;
}
