use std::{
    collections::HashMap,
    error::Error,
    fs::{self, File},
    io::{Read, Write},
    net::{TcpListener, TcpStream},
};

use chrono::Utc;
use log::{info, warn};
use serde::{de, Deserialize};
use serde_json::{Number, Value};

use crate::http::{request::{self, Request}, response::{self, Response}, Headers, Version};

use super::response::{Status, StatusCode};

/**
#[derive(Deserialize)]
pub enum RouteServeType {
    File,
    Text
}

#[derive(Deserialize)]
pub struct Route {
    #[serde(rename = "type")]
    pub _type: RouteServeType,
    pub file: Option<String>,
    pub text: Option<String>
}
*/

#[derive(Deserialize)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum Route {
    Text { text: String },
    File { source: String }
}

pub struct Server {
    pub port: u16,
    pub directory: String,
    pub index_file: String,
    pub not_found_uri: String,
    pub routes: HashMap<String, Route>
}

impl From<Value> for Server {
    fn from(value: Value) -> Self {
        #[derive(Deserialize)]
        struct Deserialized {
            port: Option<u16>,
            directory: Option<String>,
            index_file: Option<String>,
            not_found_uri: Option<String>,
            routes: Option<HashMap<String, Route>>
        }

        let deserialized = serde_json::from_value::<Deserialized>(value)
            .unwrap();
        let default = Self::default();

        return Self {
            port: deserialized.port.unwrap_or(default.port),
            directory: deserialized.directory.unwrap_or(default.directory),
            index_file: deserialized.index_file.unwrap_or(default.index_file),
            not_found_uri: deserialized.not_found_uri.unwrap_or(default.not_found_uri),
            routes: deserialized.routes.unwrap_or(default.routes)
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
            routes: HashMap::new()
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
    let result = server.routes.get(&request.path);

    let route_info = match result {
        Some(route_info) => route_info,
        None => return create_404_response()
    };

    let response = match route_info {
        Route::Text { text } => create_text_response(text),
        Route::File { source } => create_file_response(source)
    };

    return response;
}

fn create_404_response() -> Response {
    let mut headers = Headers::new();
    let _ = headers.insert(
        &"content-type".to_string(),
        "text/html".to_string()
    );
    let _ = headers.insert(&"server".to_string(), "quickserving".to_string());
    let _ = headers.insert(&"date".to_string(), Utc::now().to_string());
    let _ = headers.insert(&"content-lenght".to_string(), 12.to_string());

    return Response::new(
        StatusCode::NotFound.into(),
        Version::new("http".to_string(), "1.1".to_string()),
        headers,
        "<h1>404</h1>".to_string(),
    );
}

fn create_file_response(path: &String) -> Response {
    let resource = match File::open(path) {
        Ok(mut file) => {
            let mut buffer = String::new();
            let _ = file.read_to_string(&mut buffer);
        
            buffer
        },
        Err(_) => return create_404_response()
    };

    let mut headers = Headers::new();
    let _ = headers.insert(
        &"content-type".to_string(),
        "text/html".to_string()
    );
    let _ = headers.insert(&"server".to_string(), "quickserving".to_string());
    let _ = headers.insert(&"date".to_string(), Utc::now().to_string());
    let _ = headers.insert(&"content-lenght".to_string(), resource.len().to_string());

    return Response::new(
        StatusCode::OK.into(),
        Version::new("http".to_string(), "1.1".to_string()),
        headers,
        resource,
    );
}

fn create_text_response(text: &String) -> Response {
    let mut headers = Headers::new();
    let _ = headers.insert(
        &"content-type".to_string(),
        "text/plain".to_string()
    );
    let _ = headers.insert(&"server".to_string(), "quickserving".to_string());
    let _ = headers.insert(&"date".to_string(), Utc::now().to_string());
    let _ = headers.insert(&"content-lenght".to_string(), text.len().to_string());

    return Response::new(
        StatusCode::OK.into(),
        Version::new("http".to_string(), "1.1".to_string()),
        headers,
        text.to_string(),
    );
}
