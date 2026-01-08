use crate::models::roadmap::{Dependency, Level, Placement, Topic, TopicType};

pub mod content;

pub const SECTION_ID: &str = "error_handling_safety";

pub fn get_topics() -> Vec<Topic> {
    vec![Topic {
        id: "error_handling_safety_spine",
        title: "Error Handling & Safety",
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
