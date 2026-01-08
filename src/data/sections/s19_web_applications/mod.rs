use crate::models::roadmap::{Dependency, Level, Placement, Topic, TopicType};

pub mod content;

pub const SECTION_ID: &str = "web_applications";

pub fn get_topics() -> Vec<Topic> {
    vec![Topic {
        id: "web_applications_spine",
        title: "Web Applications",
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
