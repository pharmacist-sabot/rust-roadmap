use crate::models::roadmap::{Dependency, Level, Placement, Topic, TopicType};

pub mod content;

pub const SECTION_ID: &str = "networking_io";

pub fn get_topics() -> Vec<Topic> {
    vec![Topic {
        id: "networking_io_spine",
        title: "Networking & I/O",
        section_id: SECTION_ID,
        level: Level::Intermediate,
        topic_type: TopicType::Main,
        placement: Placement::Center,
    }]
}

pub fn get_dependencies() -> Vec<Dependency> {
    vec![]
}
