use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};

lazy_static! {
    pub static ref VERSIONS: Vec<Version> = vec![
        Version::new("v0.1.0", "Baloo", Status::Archived),
        Version::new("v0.2.0", "Zula", Status::Released),
        Version::new("v0.3.0", "Iris", Status::Released),
        Version::new("v0.4.0", "Voxl", Status::Released),
        Version::new("v0.5.0", "Luna", Status::Latest),
        Version::new("0.6.0", "Mars", Status::InProgress)
    ];
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Version {
    version: String,
    codename: String,
    pub status: Status,
    link: String,
}

impl Version {
    pub fn new(version: &str, codename: &str, status: Status) -> Self {
        let mut v = Self {
            version: version.to_string(),
            codename: codename.to_string(),
            status,
            link: "".to_string(),
        };
        v.link = v.link();
        v
    }
    pub fn link(&self) -> String {
        match self.status {
            Status::Archived => String::new(),
            Status::InProgress => {
                "https://github.com/Mustafif/MufiZ/releases/tag/next-experimental".to_string()
            }
            Status::Released | Status::Latest => format!(
                "https://github.com/Mustafif/MufiZ/releases/tag/{}",
                &self.version
            ),
        }
    }
    pub fn version(&self) -> String {
        self.version.to_string()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, PartialOrd, Ord)]
pub enum Status {
    Archived,
    Released,
    Latest,
    #[serde(rename = "In Progress")]
    InProgress,
}
