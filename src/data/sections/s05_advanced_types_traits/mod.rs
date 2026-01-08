use crate::models::roadmap::{Dependency, Level, Placement, Topic, TopicType};

pub mod content;

pub const SECTION_ID: &str = "advanced_types_traits";

pub fn get_topics() -> Vec<Topic> {
    vec![Topic {
        id: "advanced_types_traits_spine",
        title: "Advanced Types & Traits",
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
