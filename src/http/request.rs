use crate::http::{Headers, Version};
use std::{error::Error, io::{Read, Write}, net::TcpStream};

use super::{response::Response, server::Server};

pub struct Request {
    pub method: String,
    pub path: String,
    pub version: Version,
    pub headers: Headers,
    pub body: String,
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
        let request = Self::try_from(request);

        if request.is_err() {
            return Err("Error while parsing request. Invalid request.".into());
        }

        let request = request.unwrap();

        return Ok(request);
    }
    
    pub fn respond(self: &Self, create_response: fn(&Server, &Self) -> Response, server: &Server, mut stream: &TcpStream) -> Result<(), Box<dyn Error>> {
        let response = create_response(server, self);
        let response_string: String = response.into();

        stream.write_all(response_string.as_bytes()).unwrap();
        stream.flush().unwrap();

        return Ok(());

    }
}

impl TryFrom<String> for Request {
    type Error = String;

    fn try_from(string: String) -> Result<Self, Self::Error> {
        let mut rows = string.lines();
        let first_row = match rows.nth(0) {
            Some(row) => row,
            None => return Err("Invalid request.".to_string()) 
        };
        
        let columns = first_row.split(' ').collect::<Vec<&str>>();

        if columns.len() < 3 {
            return Err("Invalid request.".into());
        }

        let method = columns.get(0)
            .unwrap()
            .to_string();
        let path = columns.get(1)
            .unwrap()
            .to_string();

        let version = {
            let s = match columns.get(2) {
                Some(s) => s,
                None => return Err("Invalid request.".into())
            }
            .to_string();
            
            match Version::try_from(s) {
                Ok(version) => version,
                Err(err) => return Err(err.into())
            }
        };

        let request_parts = string.split("\r\n").collect::<Vec<&str>>();

        let headers = {
            let s = request_parts
                .get(0)
                .unwrap_or(&"")
                .to_string();

            Headers::from(s)
        };

        let body = request_parts
            .get(1)
            .unwrap_or(&"")
            .to_string();

        return Ok(Self::new(method, path, version, headers, body));
    }
}
