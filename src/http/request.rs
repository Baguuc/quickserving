use crate::http::{Headers, Version};
use std::{
    collections::HashMap,
    error::Error,
    ops::{Index, IndexMut},
};

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

            let key = split.get(0).unwrap().to_string();
            let value = split.get(1).unwrap().to_string();

            headers.insert(&key, value);
        }

        body = request_parts.get(1).unwrap_or(&"").to_string();

        let request_data = Self::new(method, path, version, headers, body);

        return Ok(request_data);
    }

    pub fn to_string(&self) -> String {
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
