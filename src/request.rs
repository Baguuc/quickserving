use std::{collections::HashMap, error::Error, ops::{Index, IndexMut}};
use crate::lib::append_field_names;


append_field_names!(
pub struct Request {
    pub method: Option<String>,
    pub path: Option<String>,
    pub version: Option<String>,
    pub a_im: Option<String>,
    pub accept: Option<String>,
    pub accept_charset: Option<String>,
    pub accept_encoding: Option<String>,
    pub accept_language: Option<String>,
    pub accept_datetime: Option<String>,
    pub access_control_request_method: Option<String>,
    pub access_control_request_headers: Option<String>,
    pub authorization: Option<String>,
    pub cache_control: Option<String>,
    pub connection: Option<String>,
    pub content_length: Option<String>,
    pub content_type: Option<String>,
    pub cookie: Option<String>,
    pub date: Option<String>,
    pub expect: Option<String>,
    pub forwarded: Option<String>,
    pub from: Option<String>,
    pub host: Option<String>,
    pub if_match: Option<String>,
    pub if_modified_since: Option<String>,
    pub if_none_match: Option<String>,
    pub if_range: Option<String>,
    pub if_unmodified_since: Option<String>,
    pub max_forwards: Option<String>,
    pub origin: Option<String>,
    pub pragma: Option<String>,
    pub proxy_authorization: Option<String>,
    pub range: Option<String>,
    pub referer: Option<String>,
    pub te: Option<String>,
    pub user_agent: Option<String>,
    pub upgrade: Option<String>,
    pub via: Option<String>,
    pub warning: Option<String>,
    pub dnt: Option<String>,
    pub x_requested_with: Option<String>,
    pub x_csrf_token: Option<String>
}
);


impl Index<&str> for Request {
    type Output = Option<String>;

    fn index(&self, index: &str) -> &Self::Output {
        return match index {
            "method" => &self.method,
            "path" => &self.path,
            "version" => &self.version,
            "a_im" => &self.a_im,
            "accept" => &self.accept,
            "accept_charset" => &self.accept_charset,
            "accept_encoding" => &self.accept_encoding,
            "accept_language" => &self.accept_language,
            "accept_datetime" => &self.accept_datetime,
            "access_control_request_method" => &self.access_control_request_method,
            "access_control_request_headers" => &self.access_control_request_headers,
            "authorization" => &self.authorization,
            "cache_control" => &self.cache_control,
            "connection" => &self.connection,
            "content_length" => &self.content_length,
            "content_type" => &self.content_type,
            "cookie" => &self.cookie,
            "date" => &self.date,
            "expect" => &self.expect,
            "forwarded" => &self.forwarded,
            "from" => &self.from,
            "host" => &self.host,
            "if_match" => &self.if_match,
            "if_modified_since" => &self.if_modified_since,
            "if_none_match" => &self.if_none_match,
            "if_range" => &self.if_range,
            "if_unmodified_since" => &self.if_unmodified_since,
            "max_forwards" => &self.max_forwards,
            "origin" => &self.origin,
            "pragma" => &self.pragma,
            "proxy_authorization" => &self.proxy_authorization,
            "range" => &self.range,
            "referer" => &self.referer,
            "te" => &self.te,
            "user_agent" => &self.user_agent,
            "upgrade" => &self.upgrade,
            "via" => &self.via,
            "warning" => &self.warning,
            "dnt" => &self.dnt,
            "x_requested_with" => &self.x_requested_with,
            "x_csrf_token" => &self.x_csrf_token,
            _ => panic!("Wrong field name provided.")
        };
    }
}


impl IndexMut<&str> for Request {
    fn index_mut(&mut self, index: &str) -> &mut Self::Output {
        return match index {
            "method" => &mut self.method,
            "path" => &mut self.path,
            "version" => &mut self.version,
            "a_im" => &mut self.a_im,
            "accept" => &mut self.accept,
            "accept_charset" => &mut self.accept_charset,
            "accept_encoding" => &mut self.accept_encoding,
            "accept_language" => &mut self.accept_language,
            "accept_datetime" => &mut self.accept_datetime,
            "access_control_request_method" => &mut self.access_control_request_method,
            "access_control_request_headers" => &mut self.access_control_request_headers,
            "authorization" => &mut self.authorization,
            "cache_control" => &mut self.cache_control,
            "connection" => &mut self.connection,
            "content_length" => &mut self.content_length,
            "content_type" => &mut self.content_type,
            "cookie" => &mut self.cookie,
            "date" => &mut self.date,
            "expect" => &mut self.expect,
            "forwarded" => &mut self.forwarded,
            "from" => &mut self.from,
            "host" => &mut self.host,
            "if_match" => &mut self.if_match,
            "if_modified_since" => &mut self.if_modified_since,
            "if_none_match" => &mut self.if_none_match,
            "if_range" => &mut self.if_range,
            "if_unmodified_since" => &mut self.if_unmodified_since,
            "max_forwards" => &mut self.max_forwards,
            "origin" => &mut self.origin,
            "pragma" => &mut self.pragma,
            "proxy_authorization" => &mut self.proxy_authorization,
            "range" => &mut self.range,
            "referer" => &mut self.referer,
            "te" => &mut self.te,
            "user_agent" => &mut self.user_agent,
            "upgrade" => &mut self.upgrade,
            "via" => &mut self.via,
            "warning" => &mut self.warning,
            "dnt" => &mut self.dnt,
            "x_requested_with" => &mut self.x_requested_with,
            "x_csrf_token" => &mut self.x_csrf_token,
            _ => panic!("Wrong field name provided.")
        };
    }
}


impl Request {
    pub fn from_string(string: String) -> Result<Self, Box<dyn Error>> {
        let mut rows = string.lines();
        let first_row = rows.nth(0);

        if first_row.is_none() {
            return Err("Invalid request.".into());
        }
        let first_row = first_row.unwrap();
        
        let columns = first_row
            .split(' ')
            .collect::<Vec<&str>>();

        if columns.len() < 3 {
            return Err("Invalid request.".into());
        }

        let mut request_data = Request {
            method: Some(columns.get(0).unwrap().to_string()),
            path: Some(columns.get(1).unwrap().to_string()),
            version: Some(columns.get(2).unwrap().to_string()),
            a_im: None,
            accept: None,
            accept_charset: None,
            accept_encoding: None,
            accept_language: None,
            accept_datetime: None,
            access_control_request_method: None,
            access_control_request_headers: None,   
            authorization: None,
            cache_control: None,
            connection: None,
            content_length: None,
            content_type: None,
            cookie: None,
            date: None,
            expect: None,
            forwarded: None,
            from: None,
            host: None,
            if_match: None,
            if_modified_since: None,
            if_none_match: None,
            if_range: None,
            if_unmodified_since: None,
            max_forwards: None,
            origin: None,
            pragma: None,
            proxy_authorization: None,
            range: None,
            referer: None,
            te: None,
            user_agent: None,
            upgrade: None,
            via: None,
            warning: None,
            dnt: None,
            x_requested_with: None,
            x_csrf_token: None,
        };
        

        for row in rows {
            let split = row
                .split(": ")
                .collect::<Vec<&str>>();

            if split.len() < 2 {
                continue;
            }

            let key = split.get(0).unwrap().to_lowercase().replace("-", "_");

            if !Request::field_names().contains(&key.as_str()) {
                continue;
            }
            let value = split.get(1).unwrap();

            request_data[key.as_str()] = Some(value.to_string());
        }

        return Ok(request_data);
    }

    
    pub fn to_string(&self) -> String {
        let mut string = format!(
            "{} {} {}\n",
            self.method.clone().unwrap(),
            self.path.clone().unwrap(),
            self.version.clone().unwrap()
        );

        for field_name in Self::field_names() {
            if vec!["method", "path", "version"].contains(field_name) {
                continue;
            }

            let field_value = &self[*field_name];

            if field_value.is_none() {
                continue;
            }

            let field_value = field_value.clone().unwrap();

            let field_name = field_name
                .split("_")
                .map(|part| {
                    let mut chars = part.chars();

                    let first_char = chars.nth(0).unwrap();

                    return format!("{}{}", first_char.to_uppercase(), chars.collect::<String>());
                })
                .collect::<Vec<String>>()
                .join("-");

            string += format!("{}: {}\n", field_name, field_value).as_str();
        }
        
        return string;
    }
}