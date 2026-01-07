use crate::models::roadmap::{Dependency, Level, Placement, Topic, TopicType};

pub mod content;

pub const SECTION_ID: &str = "package_management_deepdive";

pub fn get_topics() -> Vec<Topic> {
    vec![Topic {
        id: "package_management_deepdive_spine",
        title: "Package Management Deep Dive",
        section_id: SECTION_ID,
        level: Level::Intermediate,
        topic_type: TopicType::Main,
        placement: Placement::Center,
    }]
}

pub fn get_dependencies() -> Vec<Dependency> {
    vec![]
}
