use crate::models::roadmap::{BadgeKind, Resource, TopicContent};

pub fn get_content(id: &str) -> Option<TopicContent> {
    match id {
        "setup_env" => Some(TopicContent {
            title: "Setup & Tooling",
            description: "Install Rust and get familiar with the official toolchain.",
            resources: vec![Resource {
                label: "Install Rust",
                url: "https://www.rust-lang.org/tools/install",
                badge: BadgeKind::Official,
            }],
        }),
        "rustup" => Some(TopicContent {
            title: "Rustup",
            description: "The Rust toolchain installer and version manager.",
            resources: vec![Resource {
                label: "The Rustup Book",
                url: "https://rust-lang.github.io/rustup/",
                badge: BadgeKind::Official,
            }],
        }),
        "cargo_basics" => Some(TopicContent {
            title: "Cargo Basics",
            description: "Rust's package manager and build system.",
            resources: vec![Resource {
                label: "The Cargo Book",
                url: "https://doc.rust-lang.org/cargo/",
                badge: BadgeKind::Official,
            }],
        }),
        _ => None,
    }
}
