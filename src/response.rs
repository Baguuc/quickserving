use std::{collections::HashMap, error::Error, ops::{Index, IndexMut}};


pub struct Response {
    pub accept_patch: Option<String>,
    pub accept_ranges: Option<String>,
    pub age: Option<String>,
    pub allow: Option<String>,
    pub alt_svc: Option<String>,
    pub cache_control: Option<String>,
    pub connection: Option<String>,
    pub content_disposition: Option<String>,
    pub content_encoding: Option<String>,
    pub content_language: Option<String>,
    pub content_length: Option<String>,
    pub content_location: Option<String>,
    pub content_range: Option<String>,
    pub content_type: Option<String>,
    pub date: Option<String>,
    pub delta_base: Option<String>,
    pub etag: Option<String>,
    pub expires: Option<String>,
    pub im: Option<String>,
    pub last_modified: Option<String>,
    pub link: Option<String>,
    pub location: Option<String>,
    pub pragma: Option<String>,
    pub proxy_authenticate: Option<String>,
    pub public_key_pins: Option<String>,
    pub retry_after: Option<String>,
    pub server: Option<String>,
    pub set_cookie: Option<String>,
    pub strict_transport_security: Option<String>,
    pub trailer: Option<String>,
    pub transfer_encoding: Option<String>,
    pub tk: Option<String>,
    pub upgrade: Option<String>,
    pub vary: Option<String>,
    pub via: Option<String>,
    pub warning: Option<String>,
    pub www_authenticate: Option<String>
}


impl Index<&str> for Response {
    type Output = Option<String>;

    fn index(&self, index: &str) -> &Self::Output {
        return match index {
            "accept_patch" => &self.accept_patch,
            "accept_ranges" => &self.accept_ranges,
            "age" => &self.age,
            "allow" => &self.allow,
            "alt_svc" => &self.alt_svc,
            "cache_control" => &self.cache_control,
            "connection" => &self.connection,
            "content_disposition" => &self.content_disposition,
            "content_encoding" => &self.content_encoding,
            "content_language" => &self.content_language,
            "content_length" => &self.content_length,
            "content_location" => &self.content_location,
            "content_range" => &self.content_range,
            "content_type" => &self.content_type,
            "date" => &self.date,
            "delta_base" => &self.delta_base,
            "etag" => &self.etag,
            "expires" => &self.expires,
            "im" => &self.im,
            "last_modified" => &self.last_modified,
            "link" => &self.link,
            "location" => &self.location,
            "pragma" => &self.pragma,
            "proxy_authenticate" => &self.proxy_authenticate,
            "public_key_pins" => &self.public_key_pins,
            "retry_after" => &self.retry_after,
            "server" => &self.server,
            "set_cookie" => &self.set_cookie,
            "strict_transport_security" => &self.strict_transport_security,
            "trailer" => &self.trailer,
            "transfer_encoding" => &self.transfer_encoding,
            "tk" => &self.tk,
            "upgrade" => &self.upgrade,
            "vary" => &self.vary,
            "via" => &self.via,
            "warning" => &self.warning,
            "www_authenticate" => &self.www_authenticate,
            _ => panic!("Wrong field name provided.")
        };
    }
}


impl IndexMut<&str> for Response {
    fn index_mut(&mut self, index: &str) -> &mut Self::Output {
        return match index {
            "accept_patch" => &mut self.accept_patch,
            "accept_ranges" => &mut self.accept_ranges,
            "age" => &mut self.age,
            "allow" => &mut self.allow,
            "alt_svc" => &mut self.alt_svc,
            "cache_control" => &mut self.cache_control,
            "connection" => &mut self.connection,
            "content_disposition" => &mut self.content_disposition,
            "content_encoding" => &mut self.content_encoding,
            "content_language" => &mut self.content_language,
            "content_length" => &mut self.content_length,
            "content_location" => &mut self.content_location,
            "content_range" => &mut self.content_range,
            "content_type" => &mut self.content_type,
            "date" => &mut self.date,
            "delta_base" => &mut self.delta_base,
            "etag" => &mut self.etag,
            "expires" => &mut self.expires,
            "im" => &mut self.im,
            "last_modified" => &mut self.last_modified,
            "link" => &mut self.link,
            "location" => &mut self.location,
            "pragma" => &mut self.pragma,
            "proxy_authenticate" => &mut self.proxy_authenticate,
            "public_key_pins" => &mut self.public_key_pins,
            "retry_after" => &mut self.retry_after,
            "server" => &mut self.server,
            "set_cookie" => &mut self.set_cookie,
            "strict_transport_security" => &mut self.strict_transport_security,
            "trailer" => &mut self.trailer,
            "transfer_encoding" => &mut self.transfer_encoding,
            "tk" => &mut self.tk,
            "upgrade" => &mut self.upgrade,
            "vary" => &mut self.vary,
            "via" => &mut self.via,
            "warning" => &mut self.warning,
            "www_authenticate" => &mut self.www_authenticate,
            _ => panic!("Wrong field name provided.")
        };
    }
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
        

        for row in rows {
            let split = row
                .split(": ")
                .collect::<Vec<&str>>();

            if split.len() < 2 {
                continue;
            }

            let key = split.get(0).unwrap();
            let value = split.get(1).unwrap();


            response_data[key.to_lowercase().replace("-", "_").as_str()] = Some(value.to_string());
        }

        return Ok(response_data);
    }
}