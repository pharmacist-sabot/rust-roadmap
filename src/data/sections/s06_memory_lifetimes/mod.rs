use crate::models::roadmap::{Dependency, Level, Placement, Topic, TopicType};

pub mod content;

pub const SECTION_ID: &str = "memory_lifetimes";

pub fn get_topics() -> Vec<Topic> {
    vec![Topic {
        id: "memory_lifetimes_spine",
        title: "Memory & Lifetimes",
        section_id: SECTION_ID,
        level: Level::Intermediate,
        topic_type: TopicType::Main,
        placement: Placement::Center,
        row: None,
    }]
}

pub fn get_dependencies() -> Vec<Dependency> {
    vec![]
}
