use crate::models::roadmap::{Dependency, Level, Placement, Topic, TopicType};

pub mod content;

pub const SECTION_ID: &str = "embedded_systems";

pub fn get_topics() -> Vec<Topic> {
    vec![Topic {
        id: "embedded_systems_spine",
        title: "Embedded Systems",
        section_id: SECTION_ID,
        level: Level::Intermediate,
        topic_type: TopicType::Main,
        placement: Placement::Center,
    }]
}

pub fn get_dependencies() -> Vec<Dependency> {
    vec![]
}
