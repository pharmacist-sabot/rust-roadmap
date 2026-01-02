use crate::models::roadmap::{Dependency, Level, Placement, Topic, TopicType};

pub const SECTION_ID: &str = "intro_sec";

pub fn get_topics() -> Vec<Topic> {
    vec![
        // --- Main Spine (Yellow) ---
        Topic {
            id: "intro",
            title: "Introduction",
            section_id: SECTION_ID,
            level: Level::Beginner,
            topic_type: TopicType::Main,
            placement: Placement::Center,
        },
        // --- Right Branch (Beige) ---
        Topic {
            id: "what_is_rust",
            title: "What is Rust?",
            section_id: SECTION_ID,
            level: Level::Beginner,
            topic_type: TopicType::Sub,
            placement: Placement::Right,
        },
        Topic {
            id: "why_rust",
            title: "Why use Rust?",
            section_id: SECTION_ID,
            level: Level::Beginner,
            topic_type: TopicType::Sub,
            placement: Placement::Right,
        },
        Topic {
            id: "env_setup",
            title: "Environment Setup",
            section_id: SECTION_ID,
            level: Level::Beginner,
            topic_type: TopicType::Sub,
            placement: Placement::Right,
        },
        // --- Environment Setup Children ---
        Topic {
            id: "installing",
            title: "Installing Rust and Cargo",
            section_id: SECTION_ID,
            level: Level::Beginner,
            topic_type: TopicType::Sub,
            placement: Placement::Right,
        },
        Topic {
            id: "ides",
            title: "IDEs and Rust Toolchains",
            section_id: SECTION_ID,
            level: Level::Beginner,
            topic_type: TopicType::Sub,
            placement: Placement::Right,
        },
        Topic {
            id: "repl",
            title: "Rust REPL (Rust Playground)",
            section_id: SECTION_ID,
            level: Level::Beginner,
            topic_type: TopicType::Sub,
            placement: Placement::Right,
        },
    ]
}

pub fn get_dependencies() -> Vec<Dependency> {
    vec![
        Dependency {
            from: "intro",
            to: "what_is_rust",
        },
        Dependency {
            from: "intro", // Visual dotted line in PDF usually connects Spine -> Branch
            to: "why_rust",
        },
        Dependency {
            from: "intro",
            to: "env_setup",
        },
        // Env Setup Children
        Dependency {
            from: "env_setup",
            to: "installing",
        },
        Dependency {
            from: "env_setup",
            to: "ides",
        },
        Dependency {
            from: "env_setup",
            to: "repl",
        },
    ]
}
