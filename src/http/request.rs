use crate::http::{Headers, Version};
use std::{
    collections::HashMap, error::Error, io::{Read, Write}, net::TcpStream, ops::{Index, IndexMut}
};

use super::{response::Response, server::Server, HeaderName};

pub struct Request {
    pub method: String,
    pub path: String,
    pub version: Version,
    pub headers: Headers,
    pub body: String,
}

impl ToString for Request {
    fn to_string(&self) -> String {
        return format!(
            "{} {} {}\n{}\r\n{}",
            self.method,
            self.path,
            self.version.to_string(),
            self.headers.to_string(),
            self.body
        );
    }
}

impl Request {
    pub fn new(
        method: String,
        path: String,
        version: Version,
        headers: Headers,
        body: String,
    ) -> Self {
        return Self {
            method,
            path,
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

        let method = columns.get(0).unwrap().to_string();

        let path = columns.get(1).unwrap().to_string();

        let version_str = columns.get(2).unwrap().to_string();

        let version_split = version_str.split("/").collect::<Vec<&str>>();

        if version_split.len() < 2 {
            return Err("Invalid request.".into());
        }

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

            let key = match HeaderName::try_from(split.get(0).unwrap().to_string()) {
                Ok(name) => name,
                Err(_) => continue
            };
            let value = split.get(1).unwrap().to_string();

            headers.insert(key, value);
        }

        body = request_parts.get(1).unwrap_or(&"").to_string();

        let request_data = Self::new(method, path, version, headers, body);

        return Ok(request_data);
    }

    pub fn read_from_stream(mut stream: &TcpStream) -> Result<Self, Box<dyn Error>> {
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
        let request = Self::from_string(request);

        if request.is_err() {
            return Err("Error while parsing request. Invalid request.".into());
        }

        let request = request.unwrap();

        return Ok(request);
    }
    
    pub fn respond(self: &Self, create_response: fn(&Server, &Self) -> Response, server: &Server, mut stream: &TcpStream) -> Result<(), Box<dyn Error>> {
        let response = create_response(server, self);
        let response_string = response.to_string();

        stream.write_all(response_string.as_bytes()).unwrap();
        stream.flush().unwrap();

        return Ok(());

    }
}
