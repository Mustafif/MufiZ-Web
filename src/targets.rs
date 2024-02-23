use crate::version::{Status, VERSIONS};
use anyhow::Result;
use rocket::form::validate::Contains;
use serde::{Deserialize, Serialize};
const LINK: &str = "https://raw.githubusercontent.com/Mustafif/MufiZ/next/targets.json";

pub type Packages = Vec<Package>;

pub async fn packages() -> Packages {
    let targets = Targets::new().await.unwrap_or_default();
    let mut packages = vec![];
    let latest_version = VERSIONS
        .iter()
        .find(|x| x.status == Status::Latest)
        .unwrap();

    for t in targets.targets {
        packages.push(Package::new(t.as_str(), latest_version.version().as_str()));
    }

    packages
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Package {
    target: String,
    version: String,
    zip: String,
    deb: Option<String>,
    rpm: Option<String>,
}

impl Package {
    pub fn new(target: &str, version: &str) -> Self {
        let mut deb: Option<String> = None;
        let mut rpm: Option<String> = None;
        let version_orig = version;
        let mut version = version.to_string();
        version.remove(0);
        if target.contains("linux") {
            deb = Some(format!("https://github.com/Mustafif/MufiZ/releases/download/{version_orig}/mufiz_{version}_{target}.deb"));
            rpm = Some(format!("https://github.com/Mustafif/MufiZ/releases/download/{version_orig}/mufiz_{version}_{target}.rpm"));
        }

        Self{
            target: target.to_string(),
            version: version_orig.to_string(),
            zip: format!("https://github.com/Mustafif/MufiZ/releases/download/{version_orig}/mufiz_{version}_{target}.zip"),
            deb,
            rpm
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Targets {
    targets: Vec<String>,
}

impl Targets {
    pub async fn new() -> Result<Self> {
        let req = reqwest::get(LINK).await?;
        let j: Targets = req.json().await?;
        Ok(j)
    }

    pub fn count(&self) -> usize {
        self.targets.len()
    }

    pub fn targets(&self) -> Vec<String> {
        self.targets.clone()
    }

    pub fn filter(&self, os: OS) -> Vec<String> {
        match os {
            OS::Linux => self.linux(),
            OS::Windows => self.windows(),
            OS::MacOS => self.macos(),
        }
    }

    pub fn linux(&self) -> Vec<String> {
        self.targets
            .iter()
            .filter(|x| x.contains("linux"))
            .map(|x| x.to_string())
            .collect()
    }

    pub fn windows(&self) -> Vec<String> {
        self.targets
            .iter()
            .filter(|x| x.contains("windows"))
            .map(|x| x.to_string())
            .collect()
    }

    pub fn macos(&self) -> Vec<String> {
        self.targets
            .iter()
            .filter(|x| x.contains("macos"))
            .map(|x| x.to_string())
            .collect()
    }
}

#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub enum OS {
    Windows,
    MacOS,
    Linux,
}
