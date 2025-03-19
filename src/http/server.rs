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

#[derive(Deserialize)]
pub struct Route {
    pub file: String
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
    let resource_path = if let Some(route) = server.routes.get(&request.path) {
        format!("{}/{}", server.directory, route.file)
    } else {
        server.not_found_uri.to_string()
    };

    let file_opening_result = File::options()
        .read(true)
        .write(false)
        .open(&resource_path);
    
    let mut resource = match file_opening_result {
        Ok(resource) => resource,
        Err(_) => {
            let mut headers = Headers::new();
            let _ = headers.insert(
                &"content-type".to_string(),
                mime_guess::from_path(resource_path)
                    .first()
                    .unwrap()
                    .to_string(),
            );
            let _ = headers.insert(&"content-lenght".to_string(), 3.to_string());
            let _ = headers.insert(&"server".to_string(), "quickserving".to_string());
            let _ = headers.insert(&"date".to_string(), Utc::now().to_string());    

            return Response::new(
                404,
                "Resource not found".to_string(),
                Version::new("http".to_string(), "1.1".to_string()),
                headers,
                404.to_string(),
            );
        }
    };
    let mut res_buf = String::new();
    resource.read_to_string(&mut res_buf);
    
    // limit mutability
    let resource_content = res_buf;
    let resource_len = resource_content.len();
    
    let mut headers = Headers::new();
    let _ = headers.insert(
        &"content-type".to_string(),
        mime_guess::from_path(resource_path)
            .first()
            .unwrap()
            .to_string(),
    );
    let _ = headers.insert(&"content-lenght".to_string(), resource_len.to_string());
    let _ = headers.insert(&"server".to_string(), "quickserving".to_string());
    let _ = headers.insert(&"date".to_string(), Utc::now().to_string());

    let response = Response::new(
        200,
        "OK".to_string(),
        Version::new("http".to_string(), "1.1".to_string()),
        headers,
        resource_content,
    );

    return response;
}
