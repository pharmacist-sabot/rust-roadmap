use crate::models::roadmap::{Resource, TopicContent};

pub fn get_content(id: &str) -> Option<TopicContent> {
    match id {
        "project_structure_spine" => Some(TopicContent {
            title: "Project Structure",
            description: "Managing growing projects involves packages, crates, and modules.",
            resources: vec![],
        }),
        "packages_crates" => Some(TopicContent {
            title: "Packages & Crates",
            description: "A package is a Cargo feature that lets you build, test, and share crates.",
            resources: vec![Resource {
                label: "Packages and Crates",
                url: "https://doc.rust-lang.org/book/ch07-01-packages-and-crates.html",
                badge: crate::models::roadmap::BadgeKind::Official,
            }],
        }),
        "modules_use" => Some(TopicContent {
            title: "Modules & Paths",
            description: "Controlling scope and privacy, and bringing paths into scope with use.",
            resources: vec![Resource {
                label: "Modules",
                url: "https://doc.rust-lang.org/book/ch07-02-defining-modules-to-control-scope-and-privacy.html",
                badge: crate::models::roadmap::BadgeKind::Official,
            }],
        }),
        "workspaces" => Some(TopicContent {
            title: "Workspaces",
            description: "Managing multiple related packages that are developed together.",
            resources: vec![Resource {
                label: "Cargo Workspaces",
                url: "https://doc.rust-lang.org/book/ch14-03-cargo-workspaces.html",
                badge: crate::models::roadmap::BadgeKind::Official,
            }],
        }),
        _ => None,
    }
}
