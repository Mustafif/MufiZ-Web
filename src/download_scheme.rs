use std::collections::HashMap;
use rocket::serde::{Deserialize, Serialize};
use crate::targets::{OS, Package, Packages, Targets};
use crate::version::{Version, VERSIONS};
pub type DownloadScheme = HashMap<Version, [OSPackage; 3]>;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OSPackage{
    os: OS,
    packages: Packages,
}

impl OSPackage {
    pub fn new(os: OS, packages: Packages) -> Self{
        Self{os, packages}
    }
}

pub async fn os_packages_vers(version: Version) -> (Version, [OSPackage; 3]){
    let targets = Targets::new().await.unwrap();

    let mut window_pkgs = Vec::new();

    for w in &targets.windows(){
        window_pkgs.push(Package::new(w, &version));
    }

    let mut linux_pkgs = Vec::new();

    for l in &targets.linux(){
        linux_pkgs.push(Package::new(l, &version));
    }

    let mut mac_pkgs = Vec::new();

    for m in &targets.macos(){
        mac_pkgs.push(Package::new(m, &version));
    }

    (version, [OSPackage::new(OS::Windows, window_pkgs), OSPackage::new(OS::Linux, linux_pkgs), OSPackage::new(OS::MacOS, mac_pkgs)])
}

