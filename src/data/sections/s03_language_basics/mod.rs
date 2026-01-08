use crate::models::roadmap::{Dependency, Level, Placement, Topic, TopicType};

pub mod content;

pub const SECTION_ID: &str = "basics_sec";

pub fn get_topics() -> Vec<Topic> {
    vec![
        // --- Main Spine ---
        Topic {
            id: "basics",
            title: "Language Basics",
            section_id: SECTION_ID,
            level: Level::Beginner,
            topic_type: TopicType::Main,
            placement: Placement::Center,
            row: None,
        },
        // --- Left Side (Syntax) ---
        Topic {
            id: "syntax_group", // Header for the group
            title: "Syntax and Semantics",
            section_id: SECTION_ID,
            level: Level::Beginner,
            topic_type: TopicType::Sub, // Or distinct style if prefer
            placement: Placement::Left,
            row: None,
        },
        Topic {
            id: "vars",
            title: "Variables, DataTypes",
            section_id: SECTION_ID,
            level: Level::Beginner,
            topic_type: TopicType::Sub,
            placement: Placement::Left,
            row: None,
        },
        Topic {
            id: "control_flow",
            title: "Control Flow",
            section_id: SECTION_ID,
            level: Level::Beginner,
            topic_type: TopicType::Sub,
            placement: Placement::Left,
            row: None,
        },
    ]
}

pub fn get_dependencies() -> Vec<Dependency> {
    vec![
        // Spine -> Left (All direct children)
        Dependency {
            from: "basics",
            to: "syntax_group",
        },
        Dependency {
            from: "basics",
            to: "vars",
        },
        Dependency {
            from: "basics",
            to: "control_flow",
        },
    ]
}
