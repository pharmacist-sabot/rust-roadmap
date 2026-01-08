use crate::models::roadmap::{Dependency, Level, Placement, Topic, TopicType};

pub mod content;

pub const SECTION_ID: &str = "setup_sec";

pub fn get_topics() -> Vec<Topic> {
    vec![
        // --- Main Spine ---
        Topic {
            id: "setup_env",
            title: "Setup & Tooling",
            section_id: SECTION_ID,
            level: Level::Beginner,
            topic_type: TopicType::Main,
            placement: Placement::Center,
            row: None,
        },
        // --- Branches ---
        Topic {
            id: "rustup",
            title: "Rustup",
            section_id: SECTION_ID,
            level: Level::Beginner,
            topic_type: TopicType::Sub,
            placement: Placement::Left,
            row: None,
        },
        Topic {
            id: "cargo_basics",
            title: "Cargo Basics",
            section_id: SECTION_ID,
            level: Level::Beginner,
            topic_type: TopicType::Sub,
            placement: Placement::Right,
            row: None,
        },
    ]
}

pub fn get_dependencies() -> Vec<Dependency> {
    vec![
        Dependency {
            from: "setup_env",
            to: "rustup",
        },
        Dependency {
            from: "setup_env",
            to: "cargo_basics",
        },
    ]
}
