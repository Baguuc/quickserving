use std::{
    collections::HashMap,
    error::Error,
    fs,
    io::{Read, Write},
    net::{TcpListener, TcpStream},
};

use chrono::Utc;
use log::{info, warn};

use crate::http::{request::Request, response::Response, Headers, Version};

pub struct Server {
    pub port: u16,
    pub directory: String,
    pub index_file: String,
    pub not_found_uri: String,
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

            let handle = self.handle_connection(stream);

            if handle.is_err() {
                warn!(
                    "Error occured while establishing connection with user. {}",
                    handle.err().unwrap()
                );
                continue;
            }
        }
    }

    fn handle_connection(self: &Self, mut stream: TcpStream) -> Result<(), Box<dyn Error>> {
        // we initialize out request buffer that we will be reading request's data into
        let mut request_buf = [0u8; 4096];
        // this will represent all out decoded data of request
        let mut request = String::new();

        // as because we cannot simply read the entire request from the network i/o
        // we read the bytes of it in chunks, sequentially
        loop {
            let bytes_read = stream.read(&mut request_buf).unwrap();
            if bytes_read == 0 {
                // if we had read 0 bytes it means that we read entirity of the request
                // so we stop reading it
                break;
            }

            // we decode the request chunk we read from network i/o
            // and append it to out request string
            let request_chunk = String::from_utf8_lossy(&request_buf[0..bytes_read]).to_string();
            request.push_str(request_chunk.as_str());

            // the full request has been recieved
            if request_chunk.ends_with("\r\n\r\n") {
                break;
            }
        }

        // we parse our request
        let request = Request::from_string(request);

        if request.is_err() {
            warn!(
                "Error while parsing request. Invalid request. {}",
                request.err().unwrap()
            );
            return Err("Error while parsing request. Invalid request.".into());
        }

        let mut request = request.unwrap();

        if request.path.ends_with("/") {
            request.path = format!("{}{}", request.path, self.index_file).to_string();
        }

        info!("Requested path {}.", request.path);

        let resource_path = format!("{}/{}", self.directory.trim_end_matches("/"), request.path);
        let resource_content = fs::read_to_string(resource_path);

        let response = if resource_content.is_err() {
            let resource_content = fs::read_to_string(format!(
                "{}/{}",
                &self.directory.trim_end_matches('/'),
                &self.not_found_uri
            ))
            .unwrap_or("404".to_string());
            let resource_len = resource_content.len();

            let mut headers = Headers::new();

            headers.insert(&"Content-Type".to_string(), "text/html".to_string());
            headers.insert(&"Content-Lenght".to_string(), resource_len.to_string());
            headers.insert(&"Server".to_string(), "Quickserving".to_string());

            Response::new(
                404,
                "Resource not found".to_string(),
                Version::new("HTTP".to_string(), "1.1".to_string()),
                headers,
                resource_content,
            )
        } else {
            let resource_content = resource_content.unwrap();
            let resource_len = resource_content.len();

            let mut headers = Headers::new();

            let _ = headers.insert(
                &"Content-Type".to_string(),
                mime_guess::from_path(request.path)
                    .first()
                    .unwrap()
                    .to_string(),
            );
            let _ = headers.insert(&"Content-Lenght".to_string(), resource_len.to_string());
            let _ = headers.insert(&"Server".to_string(), "Quickserving".to_string());
            let _ = headers.insert(&"Date".to_string(), Utc::now().to_string());

            Response::new(
                200,
                "OK".to_string(),
                Version::new("HTTP".to_string(), "1.1".to_string()),
                headers,
                resource_content,
            )
        };

        let response_string = response.to_string();

        info!("Responding with: {}", response_string);

        stream.write_all(response_string.as_bytes()).unwrap();
        stream.flush().unwrap();

        return Ok(());
    }
}
