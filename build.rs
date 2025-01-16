// manifest.json = {
// 	"id": "sample-plugin",
// 	"name": "obsidian-gtd",
// 	"version": "1.0.0",
// 	"minAppVersion": "0.15.0",
// 	"description": "Demonstrates some of the capabilities of the Obsidian API.",
// 	"author": "Obsidian",
// 	"authorUrl": "https://obsidian.md",
// 	"fundingUrl": "https://obsidian.md/pricing",
// 	"isDesktopOnly": false
// }
//
// versions.json = {
// 	"1.0.0": "0.15.0"
// }

#[derive(serde::Serialize)]
struct Manifest {
    id: String, // from Cargo.toml
    name: String,
    version: String, // from Cargo.toml
    #[serde(rename = "minAppVersion")]
    min_app_version: &'static str,
    description: String, // from Cargo.toml
    author: String,      // from Cargo.toml
    #[serde(rename = "authorUrl")]
    author_url: String, // from Cargo.toml
    #[serde(rename = "fundingUrl")]
    funding_url: String,
    #[serde(rename = "isDesktopOnly")]
    is_desktop_only: bool,
}

impl Manifest {
    fn new() -> Self {
        Self {
            id: todo!(),
            name: todo!(),
            version: todo!(),
            min_app_version: "1.7.7",
            description: todo!(),
            author: todo!(),
            author_url: todo!(),
            funding_url: todo!(),
            is_desktop_only: false,
        }
    }
}

fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
    let json = serde_json::to_string_pretty(&Manifest::new())?;
    std::fs::write("manifest.json", json)?;
    Ok(())
}
