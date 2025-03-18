use std::{fs::File, io::Read};

use log::error;
use quickserving_core::http::server::Server;
use serde_json::Value;
use simple_logger::SimpleLogger;


fn main() {
    let _ = SimpleLogger::new().init();

    let config_file_result = File::options()
        .read(true)
        .create(false)
        .write(false)
        .append(false)
        .truncate(false)
        .open("./quickserving.json");
    let mut config_file = match config_file_result {
        Ok(file) => file,
        Err(_) => {
            println!("Cannot read config file \"quickserving.json\", file not found.");
            return;
        }
    };
    let mut config_str = String::new();
    let read_result = config_file.read_to_string(&mut config_str);

    match read_result {
        Ok(_) => (),
        Err(_) => {
            println!("Cannot read config file \"quickserving.json\", insufficient permissions.");
            return;
        }
    }

    let config_json = match serde_json::from_str::<Value>(config_str.as_str()) {
        Ok(json) => json,
        Err(_) => {
            println!("Cannot read config file \"quickserving.json\", invalid json format.");
            return;
        }
    };

    let server: Server = config_json.into();
    let setup_result = server.listen();

    if setup_result.is_err() {
        error!("Error: {}", setup_result.err().unwrap());
    }
}
