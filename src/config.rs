pub struct Config {
    pub port: u16,
    pub directory: String,
    pub index_file: String
}


impl Default for Config {
    fn default() -> Self {
        return Config {
            port: 3000,
            directory: "./".to_string(),
            index_file: "index.html".to_string()
        }
    }
}
