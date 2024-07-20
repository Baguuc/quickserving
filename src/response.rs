use std::{collections::HashMap, error::Error};


pub struct Response {
    accept_patch: Option<String>,
    accept_ranges: Option<String>,
    age: Option<String>,
    allow: Option<String>,
    alt_svc: Option<String>,
    cache_control: Option<String>,
    connection: Option<String>,
    content_disposition: Option<String>,
    content_encoding: Option<String>,
    content_language: Option<String>,
    content_length: Option<String>,
    content_location: Option<String>,
    content_range: Option<String>,
    content_type: Option<String>,
    date: Option<String>,
    delta_base: Option<String>,
    etag: Option<String>,
    expires: Option<String>,
    im: Option<String>,
    last_modified: Option<String>,
    link: Option<String>,
    location: Option<String>,
    pragma: Option<String>,
    proxy_authenticate: Option<String>,
    public_key_pins: Option<String>,
    retry_after: Option<String>,
    server: Option<String>,
    set_cookie: Option<String>,
    strict_transport_security: Option<String>,
    trailer: Option<String>,
    transfer_encoding: Option<String>,
    tk: Option<String>,
    upgrade: Option<String>,
    vary: Option<String>,
    via: Option<String>,
    warning: Option<String>,
    www_authenticate: Option<String>
}


impl Response {
    pub fn from_string(string: String) -> Result<Self, Box<dyn Error>> {
        let rows = string.lines();

        let mut response_data = Response {
            accept_patch: None,
            accept_ranges: None,
            age: None,
            allow: None,
            alt_svc: None,
            cache_control: None,
            connection: None,
            content_disposition: None,
            content_encoding: None,
            content_language: None,
            content_length: None,
            content_location: None,
            content_range: None,
            content_type: None,
            date: None,
            delta_base: None,
            etag: None,
            expires: None,
            im: None,
            last_modified: None,
            link: None,
            location: None,
            pragma: None,
            proxy_authenticate: None,
            public_key_pins: None,
            retry_after: None,
            server: None,
            set_cookie: None,
            strict_transport_security: None,
            trailer: None,
            transfer_encoding: None,
            tk: None,
            upgrade: None,
            vary: None,
            via: None,
            warning: None,
            www_authenticate: None,
        };

        let mut operations: HashMap<&str, fn(response_data: &mut Response, value: String) -> ()> = HashMap::new();
        operations.insert("Accept-Patch", |response_data, value| response_data.accept_patch = Some(value));
        operations.insert("Accept-Ranges", |response_data, value| response_data.accept_ranges = Some(value));
        operations.insert("Age", |response_data, value| response_data.age = Some(value));
        operations.insert("Allow", |response_data, value| response_data.allow = Some(value));
        operations.insert("Alt-Svc", |response_data, value| response_data.alt_svc = Some(value));
        operations.insert("Cache-Control", |response_data, value| response_data.cache_control = Some(value));
        operations.insert("Connection", |response_data, value| response_data.connection = Some(value));
        operations.insert("Content-Disposition", |response_data, value| response_data.content_disposition = Some(value));
        operations.insert("Content-Encoding", |response_data, value| response_data.content_encoding = Some(value));
        operations.insert("Content-Language", |response_data, value| response_data.content_language = Some(value));
        operations.insert("Content-Length", |response_data, value| response_data.content_length = Some(value));
        operations.insert("Content-Location", |response_data, value| response_data.content_location = Some(value));
        operations.insert("Content-Range", |response_data, value| response_data.content_range = Some(value));
        operations.insert("Content-Type", |response_data, value| response_data.content_type = Some(value));
        operations.insert("Date", |response_data, value| response_data.date = Some(value));
        operations.insert("Delta-Base", |response_data, value| response_data.delta_base = Some(value));
        operations.insert("ETag", |response_data, value| response_data.etag = Some(value));
        operations.insert("Expires", |response_data, value| response_data.expires = Some(value));
        operations.insert("IM", |response_data, value| response_data.im = Some(value));
        operations.insert("Last-Modified", |response_data, value| response_data.last_modified = Some(value));
        operations.insert("Link", |response_data, value| response_data.link = Some(value));
        operations.insert("Location", |response_data, value| response_data.location = Some(value));
        operations.insert("Pragma", |response_data, value| response_data.pragma = Some(value));
        operations.insert("Proxy-Authenticate", |response_data, value| response_data.proxy_authenticate = Some(value));
        operations.insert("Public-Key-Pins", |response_data, value| response_data.public_key_pins = Some(value));
        operations.insert("Retry-After", |response_data, value| response_data.retry_after = Some(value));
        operations.insert("Server", |response_data, value| response_data.server = Some(value));
        operations.insert("Set-Cookie", |response_data, value| response_data.set_cookie = Some(value));
        operations.insert("Strict-Transport-Security", |response_data, value| response_data.strict_transport_security = Some(value));
        operations.insert("Trailer", |response_data, value| response_data.trailer = Some(value));
        operations.insert("Transfer-Encoding", |response_data, value| response_data.transfer_encoding = Some(value));
        operations.insert("Tk", |response_data, value| response_data.tk = Some(value));
        operations.insert("Upgrade", |response_data, value| response_data.upgrade = Some(value));
        operations.insert("Vary", |response_data, value| response_data.vary = Some(value));
        operations.insert("Via", |response_data, value| response_data.via = Some(value));
        operations.insert("Warning", |response_data, value| response_data.warning = Some(value));
        operations.insert("WWW-Authenticate", |response_data, value| response_data.www_authenticate = Some(value));
        

        for row in rows {
            let split = row
                .split(": ")
                .collect::<Vec<&str>>();

            if split.len() < 2 {
                continue;
            }

            let key = split.get(0).unwrap();
            let value = split.get(1).unwrap();


            let set_value = operations.get(key.to_uppercase().as_str());

            if set_value.is_none() {
                continue;
            }
            let set_value = set_value.unwrap();

            set_value(&mut response_data, value.to_string());
        }

        return Ok(response_data);
    }
}