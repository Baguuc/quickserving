#[derive(Debug)]
pub struct Version {
    pub name: String,
    pub version: String,
}

impl Version {
    pub fn new(name: String, version: String) -> Self {
        return Self { name, version };
    }
}

impl ToString for Version {
    fn to_string(&self) -> String {
        return format!("{}/{}", self.name, self.version).to_string();
    }
}

impl TryFrom<String> for Version {
    type Error = String;

    fn try_from(s: String) -> Result<Self, Self::Error> {
        let version_split = s.split("/").collect::<Vec<&str>>();

        if version_split.len() < 2 {
            return Err("Invalid version string.".to_string());
        }

        let version = Version::new(
            version_split.get(0).unwrap().to_string(),
            version_split.get(1).unwrap().to_string()
        );

        return Ok(version);
    }
}
