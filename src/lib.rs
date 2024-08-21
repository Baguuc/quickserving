use std::{collections::HashMap, error::Error};

pub mod config;
pub mod request;
pub mod response;
pub mod server;


mod lib {
    macro_rules! append_field_names {
        (pub struct $name:ident { $(pub $fname:ident : $ftype:ty),* }) => {
            pub struct $name {
                $(pub $fname : $ftype),*
            }
    
            impl $name {
                pub fn field_names() -> &'static [&'static str] {
                    static NAMES: &'static [&'static str] = &[$(stringify!($fname)),*];
                    NAMES
                }
            }
        }
    }

    pub(crate) use append_field_names;    // <-- the trick
}


pub struct Headers(HashMap<String, String>);


impl Headers {
    pub fn new() -> Self {
        return Self(HashMap::new());
    }

    pub fn get(self: &Self, key: &String) -> Option<&String> {
        return self.0.get(key);
    }

    pub fn insert(self: &mut Self, key: &String, value: String) -> Result<(), Box<dyn Error>> {
        let key_formatted = 
            if key.contains("-") {
                key.split("-")
                    .map(|word| {
                        let first_char = word.chars().take(1).collect::<String>().to_uppercase();
                        let other_chars = word.chars().skip(1).collect::<String>().to_lowercase();

                        format!("{}{}", first_char, other_chars)
                    })
                    .collect::<Vec<String>>()
                    .join("-")

            } else {
                let first_char = key.chars().take(1).collect::<String>().to_uppercase();
                let other_chars = key.chars().skip(1).collect::<String>().to_lowercase();

                format!("{}{}", first_char, other_chars)
            };

        let _ = self.0.insert(key_formatted, value);

        return Ok(());
    }
}

impl ToString for Headers {
    fn to_string(&self) -> String {
        let formatted = self.0
            .keys()
            .map(|key| {
                format!("{}: {}\n", key, self.0.get(key).unwrap()).to_string()
            })
            .collect::<String>();

        return formatted
    }
}


pub struct Version {
    name: String,
    version: String
}


impl Version {
    pub fn new(name: String, version: String) -> Self {
        return Self {
            name,
            version
        };
    }
}


impl ToString for Version {
    fn to_string(&self) -> String {
        return format!("{}/{}", self.name, self.version).to_string();
    }
}