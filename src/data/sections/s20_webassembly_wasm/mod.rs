use crate::models::roadmap::{Dependency, Level, Placement, Topic, TopicType};

pub mod content;

pub const SECTION_ID: &str = "webassembly_wasm";

pub fn get_topics() -> Vec<Topic> {
    vec![Topic {
        id: "webassembly_wasm_spine",
        title: "WebAssembly (WASM)",
        section_id: SECTION_ID,
        level: Level::Intermediate,
        topic_type: TopicType::Main,
        placement: Placement::Center,
    }]
}

pub fn get_dependencies() -> Vec<Dependency> {
    vec![]
}
