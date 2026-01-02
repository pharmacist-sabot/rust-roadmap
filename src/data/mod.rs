//! Application data aggregator.

pub mod sections;

use self::sections::*;
use crate::models::roadmap::{Dependency, Section, Topic, TopicContent};

/// Defines the order of sections (The Spine).
pub const SECTIONS: &[Section] = &[
    Section {
        id: s01_introduction::SECTION_ID,
        title: "",
        order: 1,
    },
    Section {
        id: s02_language_basics::SECTION_ID,
        title: "",
        order: 2,
    },
];

/// Aggregates topics from all modular files.
pub fn get_all_topics() -> Vec<Topic> {
    let mut topics = Vec::new();
    topics.extend(s01_introduction::get_topics());
    topics.extend(s02_language_basics::get_topics());
    topics
}

/// Aggregates dependencies from all modular files + Spine connections.
pub fn get_all_dependencies() -> Vec<Dependency> {
    let mut deps = Vec::new();

    // 1. Internal Module Dependencies
    deps.extend(s01_introduction::get_dependencies());
    deps.extend(s02_language_basics::get_dependencies());

    // 2. Spine Connections (Connecting the Yellow Boxes)
    deps.push(Dependency {
        from: "intro",
        to: "basics",
    });

    deps
}

/// Aggregates content lookup from all decentralized content files.
pub fn get_topic_content(id: &str) -> Option<TopicContent> {
    // Try finding content in each section
    if let Some(c) = s01_introduction::content::get_content(id) {
        return Some(c);
    }
    if let Some(c) = s02_language_basics::content::get_content(id) {
        return Some(c);
    }
    None
}
