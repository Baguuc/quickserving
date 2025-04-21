use std::{
    error::Error,
    fs::File,
    io::Read,
    net::TcpListener,
};
use chrono::Utc;
use crate::{logging::{LogLevel, log}, http::{request::Request, response::Response, headers::{Headers,HeaderName}, version::Version, status::StatusCode}, config::{ServerConfig, RequestedRoute, ResponseConfig, ResponseHTTPConfig}};


pub struct Server {
    config: ServerConfig
}

impl Server {
    pub fn new(config: ServerConfig) -> Self {
        return Self { config };
    }

    pub fn listen(self: Self) -> Result<(), Box<dyn Error>> {
        // we bind our listener to port from self
        let listener = TcpListener::bind(format!("0.0.0.0:{}", self.config.port));

        if listener.is_err() {
            return Err("this port is already in use.".into());
        }

        let listener = listener.unwrap();
        
        log(LogLevel::INFO, format!("Serving on port {}.", self.config.port));
        
        loop {
            // we read every request that the listener has recieved and try to handle it
            let (stream, _) = listener.accept().unwrap();
            
            let request = Request::read_from_stream(&stream)
                .unwrap();

            let _ = request.respond(
                create_response,
                &self,
                &stream
            );
        }
    }

}

fn create_response(server: &Server, request: &Request) -> Response {
    let result = server.config.routes.get(&RequestedRoute {
        method: request.method.clone(),
        path: request.path.clone()
    });
    
    let response_config = match result {
        Some(route_info) => route_info,
        None => return create_404_response()
    };

    let response = match response_config {
        ResponseConfig::Text { text, http } => create_text_response(
            &text,
            &http
        ),
        ResponseConfig::File { source, http } => create_file_response(
            &source,
            &http
        )
    };

    return response;
}

fn create_404_response() -> Response { 
    let mut headers = Headers::new();
    let _ = headers.insert(HeaderName::ContentType, "text/html".to_string());
    let _ = headers.insert(HeaderName::Host, "quickserving".to_string());
    let _ = headers.insert(HeaderName::Date, Utc::now().to_string());
    let _ = headers.insert(HeaderName::ContentLength, 12.to_string());

    return Response::new(
        StatusCode::NotFound.into(),
        Version::new("HTTP".to_string(), "1.1".to_string()),
        headers,
        "<h1>404</h1>".to_string(),
    );
}

fn create_text_response(
    text: &String, 
    response_config: &ResponseHTTPConfig
) -> Response {
    return Response::new(
        StatusCode::OK.into(),
        Version::new("HTTP".to_string(), "1.1".to_string()),
        response_config.headers.clone(),
        text.to_string()
    );
}

fn create_file_response(
    path: &String, 
    response_config: &ResponseHTTPConfig
) -> Response {
    let resource = match File::open(path) {
        Ok(mut file) => {
            let mut buffer = String::new();
            let _ = file.read_to_string(&mut buffer);
        
            buffer
        },
        Err(_) => return create_404_response()
    };

    let mut headers = response_config.headers.clone();
    let _ = headers.remove(HeaderName::ContentLength);
    let _ = headers.insert(HeaderName::ContentLength, resource.len().to_string());

    return Response::new(
        StatusCode::OK.into(),
        Version::new("HTTP".to_string(), "1.1".to_string()),
        headers,
        resource,
    );
}
