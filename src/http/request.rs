use std::{error::Error, io::{Read, Write}, net::TcpStream};
use crate::http::{response::Response, server::Server, method::Method, headers::{Headers,HeaderName}, version::Version};

#[derive(Debug)]
pub struct Request {
    pub method: Method,
    pub path: String,
    pub version: Version,
    pub headers: Headers,
    pub body: String,
}

impl Request {
    pub fn new(
        method: Method,
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
        let mut request_buf = [0u8; 8092];
        // this will represent all out decoded data of request
        let mut request = String::new();

        match stream.read(&mut request_buf) {
            Ok(n) => {
                let request_chunk = String::from_utf8_lossy(&request_buf[0..n]).to_string();
                // we decode the request chunk we read from network i/o
                // and append it to out request string
                request.push_str(request_chunk.as_str());
            },
            _ => ()
        };

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
        let lines = string
            .lines()
            .enumerate();

        let mut headers = Headers::new();
        let mut body = "".to_string();

        let mut first_line: Option<(Method, String, Version)> = None;
        let mut is_body = false;

        for (idx, line) in lines {
            match (idx, line) {
                (0, _) => {
                    let p = match process_first_line(line.to_string()) {
                        Ok(p) => p,
                        Err(_) => return Err("Invalid request.".to_string())
                    };

                    first_line = Some(p);
                },
                (i, "") if i > 0 => { 
                    is_body = true;
                    continue;
                },
                (_, line) if is_body => {
                    body += line;
                    body += "\n";
                },
                (_, line) => {
                    let (name, value) = match process_header_line(line.to_string()) {
                        Ok(v) => v,
                        Err(_) => return Err("Invalid request".into())
                    };

                    let _ = headers.insert(name, value);
                }
            }
        }

        let first_line = match first_line {
            Some(first_line) => first_line,
            None => return Err("Invalid request.".to_string())
        };

        let method = match Method::try_from(first_line.0) {
            Ok(method) => method,
            Err(err) => return Err(err.to_string())
        };
        let path = first_line.1;
        let version = match Version::try_from(first_line.2) {
            Ok(version) => version,
            Err(err) => return Err(err.to_string())
        };

        return Ok(Request::new(
            method,
            path,
            version,
            headers,
            body
        ));
    }
}

fn process_first_line(line: String) -> Result<(Method, String, Version), String> {
    let mut method = "".to_string();
    let mut path = "".to_string();
    let mut version = "".to_string();

    let mut i = 0;

    for c in line.chars() {
        if c == ' ' {
            i += 1;
            continue;
        }
        
        match i {
            0 => { method.push(c) },
            1 => { path.push(c) },
            2 => { version.push(c) },
            _ => ()
        };
    }

    let method = Method::try_from(method)?;
    let version = Version::try_from(version)?;

    return Ok((method, path, version));
}

fn process_header_line(line: String) -> Result<(HeaderName, String), String> {
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
            _ => ()
        }
    }
    
    let name = HeaderName::try_from(key)?;

    return Ok((name, value));
}
