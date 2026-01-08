use crate::models::roadmap::{Dependency, Level, Placement, Topic, TopicType};

pub mod content;

pub const SECTION_ID: &str = "intro_sec";

pub fn get_topics() -> Vec<Topic> {
    vec![
        // --- Main Spine ---
        Topic {
            id: "intro",
            title: "Introduction",
            section_id: SECTION_ID,
            level: Level::Beginner,
            topic_type: TopicType::Main,
            placement: Placement::Center,
            row: None,
        },
        Topic {
            id: "what_is_rust",
            title: "What is Rust?",
            section_id: SECTION_ID,
            level: Level::Beginner,
            topic_type: TopicType::Sub,
            placement: Placement::Right,
            row: None,
        },
        Topic {
            id: "why_rust",
            title: "Why use Rust?",
            section_id: SECTION_ID,
            level: Level::Beginner,
            topic_type: TopicType::Sub,
            placement: Placement::Right,
            row: None,
        },
        Topic {
            id: "ecosystem",
            title: "Ecosystem & Use Cases",
            section_id: SECTION_ID,
            level: Level::Beginner,
            topic_type: TopicType::Sub,
            placement: Placement::Right,
            row: None,
        },
        Topic {
            id: "community",
            title: "Rust Community",
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
            from: "intro",
            to: "what_is_rust",
        },
        Dependency {
            from: "intro",
            to: "why_rust",
        },
        Dependency {
            from: "intro",
            to: "ecosystem",
        },
        Dependency {
            from: "intro",
            to: "community",
        },
    ]
}
