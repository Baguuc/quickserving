use crate::http::{headers::Headers, version::Version, status::Status};

pub struct Response {
    status: Status,
    version: Version,
    headers: Headers,
    body: String,
}

impl Response {
    pub fn new(
        status: Status,
        version: Version,
        headers: Headers,
        body: String,
    ) -> Self {
        return Self {
            status,
            version,
            headers,
            body,
        };
    }
}

impl Into<String> for Response {
    fn into(self) -> String {
        let headers: String = self.headers.into();

        return format!(
            "{} {}\n{}\r\n{}",
            self.version.to_string(),
            self.status.status_code,
            headers,
            self.body
        );
    }
}
