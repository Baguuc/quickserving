pub struct Config {
    pub port: u16,
    pub directory: String,
    pub index_file: String,
    pub not_found_uri: String
}


impl Default for Config {
    fn default() -> Self {
        return Config {
            port: 3000,
            directory: "./".to_string(),
            index_file: "index.html".to_string(),
            not_found_uri: "404.html".to_string()
        }
    }
}
