use crate::models::roadmap::{Dependency, Level, Placement, Topic, TopicType};

pub mod content;

pub const SECTION_ID: &str = "unsafe_rust";

pub fn get_topics() -> Vec<Topic> {
    vec![Topic {
        id: "unsafe_rust_spine",
        title: "Unsafe Rust",
        section_id: SECTION_ID,
        level: Level::Intermediate,
        topic_type: TopicType::Main,
        placement: Placement::Center,
    }]
}

pub fn get_dependencies() -> Vec<Dependency> {
    vec![]
}
