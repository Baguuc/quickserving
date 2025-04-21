use std::{fs::File, io::Read};

use quickserving_core::{
    logging::{LogLevel, log},
    http::server::Server,
    config::ServerConfig
};

fn main() {
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
            log(LogLevel::ERROR, "Cannot read config file \"quickserving.json\", file not found.".to_string());
            return;
        }
    };
    let mut config_str = String::new();
    let read_result = config_file.read_to_string(&mut config_str);

    match read_result {
        Ok(_) => (),
        Err(_) => {
            log(LogLevel::ERROR, "Cannot read config file \"quickserving.json\", insufficient permissions.".to_string());
            return;
        }
    };

    let config: ServerConfig = config_str.try_into().unwrap();
    let server = Server::new(config);
    let setup_result = server.listen();

    if setup_result.is_err() {
        log(LogLevel::ERROR, setup_result.err().unwrap().to_string());
    }
}
