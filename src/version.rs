use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};

lazy_static! {
    pub static ref VERSIONS: Vec<Version> = vec![
        Version::new("v0.1.0", "Baloo", Status::Archived),
        Version::new("v0.2.0", "Zula", Status::Released),
        Version::new("v0.3.0", "Iris", Status::Released),
        Version::new("v0.4.0", "Voxl", Status::Released),
        Version::new("v0.5.0", "Luna", Status::Released),
        Version::new("v0.6.0", "Mars", Status::Latest),
        Version::new("v0.7.0", "Jade", Status::InProgress),
    ];
}
#[derive(Debug, Clone, Deserialize, Serialize, Hash, Ord, PartialOrd, Eq, PartialEq)]
pub struct Version {
    version: String,
    tag_v: String,
    codename: String,
    pub status: Status,
    link: String,
}

impl Version {
    pub fn new(tag_v: &str, codename: &str, status: Status) -> Self {
        let mut version = tag_v.to_string();
        version.remove(0);
        let mut v = Self {
            version,
            codename: codename.to_string(),
            tag_v: tag_v.to_string(),
            status,
            link: "".to_string(),
        };
        if v.status == Status::InProgress {
            v.tag_v = "next-experimental".to_string();
        }
        v.link = v.link();
        v
    }
    pub fn link(&self) -> String {
        match self.status {
            Status::Archived => String::new(),
            _ => format!(
                "https://github.com/Mustafif/MufiZ/releases/tag/{}",
                &self.tag_v
            ),
        }
    }
    pub fn version(&self) -> String {
        self.version.to_string()
    }

    pub fn tag_v(&self) -> String {
        self.tag_v.to_string()
    }

    pub fn vers_with_name(&self) -> String{
        format!("v{} ({} Release)", &self.version, &self.codename)
    }
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Status {
    Archived,
    Released,
    Latest,
    #[serde(rename = "In Progress")]
    InProgress,
}
