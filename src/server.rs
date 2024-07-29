use std::{error::Error, fs, io::{Read, Write}, net::{TcpListener, TcpStream}};

use chrono::Utc;
use log::{info, warn};

use crate::{config::Config, request::{self, Request}, response::Response};

pub fn listen(config: Config) -> Result<(), Box<dyn Error>> {
    // we bind our listener to port from config
    let listener = TcpListener::bind(format!("0.0.0.0:{}", config.port));
    
    if listener.is_err() {
        return Err("this port is already in use.".into());
    }

    let listener = listener.unwrap();

    info!("Serving directory {} on port {}.", config.directory, config.port);
    loop {
        // we read every request that the listener has recieved and try to handle it
        let (stream, _) = listener.accept().unwrap();

        let handle = handle_connection(stream, &config);
        
        if handle.is_err() {
            warn!("Error occured while establishing connection with user. {}", handle.err().unwrap());
            continue;
        }
    }
}


fn handle_connection(mut stream: TcpStream, config: &Config) -> Result<(), Box<dyn Error>> {
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
    };

    // we parse our request
    let request = Request::from_string(request);

    if request.is_err() {
        warn!("Error while parsing request. Invalid request. {}", request.err().unwrap());
        return Err("Error while parsing request. Invalid request.".into());
    }

    let mut request = request.unwrap();

    if request.path.clone().unwrap().ends_with("/") {
        request.path = Some(format!("{}{}", request.path.unwrap(), config.index_file).to_string());
    }
    
    info!(
        "{} requested path {}.",
        if request.user_agent.is_some() { request.user_agent.unwrap() } else { "User".to_string() },
        request.path.clone().unwrap()
    );

    let resource_path = format!("{}/{}", config.directory, request.path.clone().unwrap());

    let resource_content = fs::read_to_string(resource_path);

    let response = if resource_content.is_err() {
        // TODO: implement 404 page
        let resource_content = fs::read_to_string(&config.not_found_uri).unwrap_or("404".to_string());
        let resource_len = resource_content.len();
        
        Response {
            content: Some(resource_content),
            content_type: Some("text/html".to_string()),
            content_length: Some(resource_len.to_string()),
            version: Some("HTTP/1.1".to_string()),
            status_code: Some("404".to_string()),
            reason: Some("Resource not found".to_string()),
            server: Some("Quickserving".to_string()),
            accept_patch: None,
            ..Default::default()
        }
    } else {
        let resource_content = resource_content.unwrap();
        let resource_len = resource_content.len();

        Response {
            content: Some(resource_content),
            content_type: Some(mime_guess::from_path(request.path.unwrap()).first().unwrap().to_string()),
            content_length: Some(resource_len.to_string()),
            version: Some("HTTP/1.1".to_string()),
            status_code: Some("200".to_string()),
            reason: Some("OK".to_string()),
            server: Some("Quickserving".to_string()),
            date: Some(Utc::now().to_string()),
            ..Default::default()
        }
    };

    let response_string = response.to_string();

    info!("Responding with: {}", response_string);

    stream.write_all(response_string.as_bytes()).unwrap();
    stream.flush().unwrap();

    return Ok(());
}
