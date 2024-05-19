use std::{fs, io::{Read, Write}, net::{TcpListener, TcpStream}, path::Path};

use chrono::Utc;
use clap::{command, Parser};
use log::{info, warn};
use simple_logger::SimpleLogger;

#[derive(Debug, Clone)]
struct Error(String);

#[derive(Parser, Debug, Clone)]
#[command(version, about = "A simple HTTP server.")]
struct AppConfig {
    #[arg(short, long, default_value = "8080", help = "The port that server will be listening for requests on.")]
    port: String,
    #[arg(short, long, default_value = ".", help = "The directoryectory that will be served.")]
    directory: String
}


fn respond(mut tcp_stream: TcpStream, text_content: String, content_type: String) -> Result<(), Error> {
    let now = Utc::now();
    let date = now.to_string();

    let response: String = format!("HTTP/1.1 200 OK\r\nServer: Quickserving\r\nDate: {}\r\nContent-Length: {}\r\nContent-Type: text/{}\r\n\r\n{}", date, text_content.len(), content_type, text_content);
    info!("Responding with:\n{}", response);

    tcp_stream.write_all(response.as_bytes()).unwrap();
    tcp_stream.flush().unwrap();

    return Ok(());
}


fn get_resource(filepath: String, app_config: &AppConfig, depth: u8) -> (Result<String, Error>, Result<String, Error>) {
    // if the depth is 3 and nothing could be read then the resource cannot be found in any way
    // and we can just return an error
    if depth == 3 {
        return (Err(Error("Resource not found.".to_string())), Err(Error("Resource not found.".to_string())));
    }

    if !filepath.chars().skip(1).collect::<String>().contains(".") && !filepath.ends_with("/") {
        let (content, content_type) = get_resource(filepath.clone() + ".html", app_config, depth+1);

        if content.is_err() {
            return get_resource(filepath.clone()+"/index.html", app_config, depth+1);
        }

        return (content, content_type);
    }

    if filepath.ends_with("/") {
        return get_resource(filepath+"index.html", app_config, depth+1);
    }

    let res_path_string = format!("{}", filepath.clone());
    let path = Path::new(res_path_string.as_str());
    let content = fs::read_to_string(path);

    if content.is_err() {
        return (Err(Error("Resource not found.".to_string())), Err(Error("Resource not found.".to_string())));
    }

    let content_type = res_path_string.split('.').last().unwrap();

    return (Ok(content.unwrap_or("".to_string())), Ok(content_type.to_string()));
} 


fn handle_request(request_string: String, tcp_stream: TcpStream, app_config: &AppConfig) -> Result<(), Error> {
    // we retrieve the resource path from first line and second column (splitted by the whitespace) like shown below
    //    |                  |
    // GET /public/styles.css HTTP/1.1
    // Host: localhost:8000
    // Connection: keep-alive
    // ...
    let resource_path = request_string
        .lines()
        .nth(0).unwrap()
        .split_whitespace()
        .nth(1).unwrap();
    
    let resource_path = app_config.directory.clone() + resource_path;

    let (resource, content_type) = get_resource(resource_path, app_config, 1);

    if resource.is_err() {
        let (content, _) = get_resource(app_config.directory.clone() + "/404.html", app_config, 1);

        return respond(tcp_stream, content.unwrap_or(String::from("404")), "html".to_string());
    }

    // we know it has not errored in any way
    let content = resource.unwrap();
    let content_type = content_type.unwrap();
    
    return respond(tcp_stream, content, content_type);
}


fn handle_stream_connection(mut tcp_stream: TcpStream, app_config: &AppConfig) -> Result<(), Error> {
    let mut request_buf = [0u8; 4096];
    let mut request = String::new();

    loop {
        let bytes_read = tcp_stream.read(&mut request_buf).unwrap();
        if bytes_read == 0 {
            // the recieved request has invalid lenght
            break;
        }

        let request_part = String::from_utf8_lossy(&request_buf[0..bytes_read]).to_string();
        request.push_str(request_part.as_str());

        info!("{}", request_part);
        if request_part.ends_with("\r\n\r\n") {
            // the full request has been recieved
            break;
        }
    }
    
    return handle_request(request, tcp_stream, app_config);
}


#[tokio::main]
async fn main() {
    SimpleLogger::new().init().unwrap();

    let config = AppConfig::parse();
    let listener = TcpListener::bind(format!("0.0.0.0:{}", config.port)).unwrap();

    info!("Serving directoryectory {} on port {}.", config.directory, config.port);

    loop {
        let config = config.clone();
        let (stream, _) = listener.accept().unwrap();

        tokio::spawn(async move {
            let _ = match handle_stream_connection(stream, &config) {
                Ok(_) => {
                    dbg!("Request handled successfully.");
                    ()
                },
                Err(e) => warn!("Error: {}", e.0)
            };
        });
    }
}