pub mod targets;
pub mod version;

use rocket::{get, launch, routes, Build, Rocket};
use rocket_dyn_templates::{context, Template};
use targets::packages;
use version::VERSIONS;

#[get("/")]
async fn index() -> Template {
    let targets = targets::Targets::new().await.unwrap_or_default();
    let windows_targets = targets.windows();
    let linux_targets = targets.linux();
    let macos_targets = targets.macos();
    let versions = VERSIONS.as_slice();
    let packages = packages().await;
    Template::render(
        "index",
        context! {
         windows_targets: windows_targets,
         linux_targets: linux_targets,
         macos_targets: macos_targets,
         releases: versions,
         packages: packages
        },
    )
}

#[launch]
async fn rocket() -> Rocket<Build> {
    rocket::build()
        .attach(Template::fairing())
        .mount("/", routes![index])
}
