use std::collections::HashMap;
use rocket::serde::{Deserialize, Serialize};
use crate::targets::{OS, Package, Packages, Targets};
use crate::version::{Version, VERSIONS};
pub type DownloadScheme = HashMap<String, [OSPackage; 3]>;

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

    (version, [OSPackage::new(OS::Windows, window_pkgs),
        OSPackage::new(OS::Linux, linux_pkgs),
        OSPackage::new(OS::MacOS, mac_pkgs)])
}

pub async fn download_scheme() -> DownloadScheme{
    let mut threads = Vec::new();
    let mut ds = DownloadScheme::new();
    for v in VERSIONS.iter(){
        if v.status == crate::version::Status::Archived{
            continue;
        }
        let t = tokio::spawn(
            os_packages_vers(v.clone())
        );
        threads.push(t)
    }

    let res = futures::future::join_all(threads).await;
    for r in res{
        let r = r.unwrap();
        ds.insert(r.0.vers_with_name(), r.1);
    }

    ds
}