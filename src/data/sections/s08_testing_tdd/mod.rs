use crate::models::roadmap::{Dependency, Level, Placement, Topic, TopicType};

pub mod content;

pub const SECTION_ID: &str = "testing_tdd";

pub fn get_topics() -> Vec<Topic> {
    vec![Topic {
        id: "testing_tdd_spine",
        title: "Testing & TDD",
        section_id: SECTION_ID,
        level: Level::Intermediate,
        topic_type: TopicType::Main,
        placement: Placement::Center,
    }]
}

pub fn get_dependencies() -> Vec<Dependency> {
    vec![]
}
