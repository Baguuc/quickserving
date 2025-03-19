use std::{
    collections::VecDeque, error::Error, io::Write, net::TcpStream, ops::{Index, IndexMut}
};

use crate::http::{Headers, Version};

use super::request::Request;

pub struct Response {
    status_code: u16,
    reason: String,
    version: Version,
    headers: Headers,
    body: String,
}

impl Response {
    pub fn new(
        status_code: u16,
        reason: String,
        version: Version,
        headers: Headers,
        body: String,
    ) -> Self {
        return Self {
            status_code,
            reason,
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

        let version_str = columns.get(0).unwrap().to_string();

        let version_split = version_str.split("/").collect::<Vec<&str>>();

        if version_split.len() < 2 {
            return Err("Invalid request.".into());
        }

        let status_code = columns.get(1).unwrap().parse::<u16>();

        if status_code.is_err() {
            return Err("Invalid request.".into());
        }

        let status_code = status_code.unwrap();

        let reason = columns.get(2).unwrap().to_string();

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

        let request_data = Self::new(status_code, reason, version, headers, body);

        return Ok(request_data);
    }

    pub fn to_string(&self) -> String {
        return format!(
            "{} {} {}\n{}\r\n{}",
            self.version.to_string(),
            self.status_code,
            self.reason,
            self.headers.to_string(),
            self.body
        );
    }
}
