pub struct Config {
    pub port: u16,
    pub directory: String
}


impl Config {
    pub fn directory(mut self, path: impl Into<String>) -> Config {
        self.directory = path.into();

        return self;
    }

    pub fn at(mut self, port: u16) -> Config {
        self.port = port;

        return self;
    }
}

impl Default for Config {
    fn default() -> Self {
        return Config {
            port: 3000,
            directory: "./".to_string(),
        }
    }
}
