use std::{collections::HashMap, error::Error};


pub struct Request {
    method: String,
    path: String,
    version: String,
    a_im: Option<String>,
    accept: Option<String>,
    accept_charset: Option<String>,
    accept_encoding: Option<String>,
    accept_language: Option<String>,
    accept_datetime: Option<String>,
    access_control_request_method: Option<String>,
    access_control_request_headers: Option<String>,
    authorization: Option<String>,
    cache_control: Option<String>,
    connection: Option<String>,
    content_length: Option<String>,
    content_type: Option<String>,
    cookie: Option<String>,
    date: Option<String>,
    expect: Option<String>,
    forwarded: Option<String>,
    from: Option<String>,
    host: Option<String>,
    if_match: Option<String>,
    if_modified_since: Option<String>,
    if_none_match: Option<String>,
    if_range: Option<String>,
    if_unmodified_since: Option<String>,
    max_forwards: Option<String>,
    origin: Option<String>,
    pragma: Option<String>,
    proxy_authorization: Option<String>,
    range: Option<String>,
    referer: Option<String>,
    te: Option<String>,
    user_agent: Option<String>,
    upgrade: Option<String>,
    via: Option<String>,
    warning: Option<String>,
    dnt: Option<String>,
    x_requested_with: Option<String>,
    x_csrf_token: Option<String>
}


impl Request {
    pub fn from_string(string: String) -> Result<Self, Box<dyn Error>> {
        let mut rows = string.lines();
        let first_row = rows.nth(0);

        if first_row.is_none() {
            return Err("Invalid request.".into());
        }
        let first_row = first_row.unwrap();

        let rows = rows.skip(1);

        let columns = first_row
            .split(' ')
            .collect::<Vec<&str>>();

        if columns.len() < 3 {
            return Err("Invalid request.".into());
        }

        let mut request_data = Request {
            method: columns.get(0).unwrap().to_string(),
            path: columns.get(1).unwrap().to_string(),
            version: columns.get(2).unwrap().to_string(),
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

        let mut operations: HashMap<&str, fn(request_data: &mut Request, value: String) -> ()> = HashMap::new();
        operations.insert("A-IM", |request_data, value| request_data.a_im = Some(value));
        operations.insert("ACCEPT", |request_data, value| request_data.accept = Some(value));
        operations.insert("ACCEPT-CHARSET", |request_data, value| request_data.accept_charset = Some(value));
        operations.insert("ACCEPT-ENCODING", |request_data, value| request_data.accept_encoding = Some(value));
        operations.insert("ACCEPT-LANGUAGE", |request_data, value| request_data.accept_language = Some(value));
        operations.insert("ACCEPT-DATETIME", |request_data, value| request_data.accept_datetime = Some(value));
        operations.insert("ACCESS-CONTROL-REQUEST-METHOD", |request_data, value| request_data.access_control_request_method = Some(value));
        operations.insert("ACCESS-CONTROL-REQUEST-HEADERS", |request_data, value| request_data.access_control_request_headers = Some(value));
        operations.insert("AUTHORIZATION", |request_data, value| request_data.authorization = Some(value));
        operations.insert("CACHE-CONTROL", |request_data, value| request_data.cache_control = Some(value));
        operations.insert("CONNECTION", |request_data, value| request_data.connection = Some(value));
        operations.insert("CONTENT-LENGTH", |request_data, value| request_data.content_length = Some(value));
        operations.insert("CONTENT-TYPE", |request_data, value| request_data.content_type = Some(value));
        operations.insert("COOKIE", |request_data, value| request_data.cookie = Some(value));
        operations.insert("DATE", |request_data, value| request_data.date = Some(value));
        operations.insert("EXPECT", |request_data, value| request_data.expect = Some(value));
        operations.insert("FORWARDED", |request_data, value| request_data.forwarded = Some(value));
        operations.insert("FROM", |request_data, value| request_data.from = Some(value));
        operations.insert("HOST", |request_data, value| request_data.host = Some(value));
        operations.insert("IF-MATCH", |request_data, value| request_data.if_match = Some(value));
        operations.insert("IF-MODIFIED-SINCE", |request_data, value| request_data.if_modified_since = Some(value));
        operations.insert("IF-NONE-MATCH", |request_data, value| request_data.if_none_match = Some(value));
        operations.insert("IF-RANGE", |request_data, value| request_data.if_range = Some(value));
        operations.insert("IF-UNMODIFIED-SINCE", |request_data, value| request_data.if_unmodified_since = Some(value));
        operations.insert("MAX-FORWARDS", |request_data, value| request_data.authorization = Some(value));
        operations.insert("ORIGIN", |request_data, value| request_data.origin = Some(value));
        operations.insert("PRAGMA", |request_data, value| request_data.pragma = Some(value));
        operations.insert("PROXY-AUTHORIZATION", |request_data, value| request_data.proxy_authorization = Some(value));
        operations.insert("RANGE", |request_data, value| request_data.range = Some(value));
        operations.insert("REFERER", |request_data, value| request_data.referer = Some(value));
        operations.insert("TE", |request_data, value| request_data.te = Some(value));
        operations.insert("USER-AGENT", |request_data, value| request_data.user_agent = Some(value));
        operations.insert("UPGRADE", |request_data, value| request_data.upgrade = Some(value));
        operations.insert("VIA", |request_data, value| request_data.via = Some(value));
        operations.insert("WARNING", |request_data, value| request_data.warning = Some(value));
        operations.insert("DNT", |request_data, value| request_data.dnt = Some(value));
        operations.insert("X-REQUESTED-WITH", |request_data, value| request_data.x_requested_with = Some(value));
        operations.insert("X-CSRF-TOKEN", |request_data, value| request_data.x_csrf_token = Some(value));
        

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

            set_value(&mut request_data, value.to_string());
        }

        return Ok(request_data);
    }
}