use crate::models::roadmap::{Dependency, Level, Placement, Topic, TopicType};

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
        },
        // --- Right Side (Setup) ---
        Topic {
            id: "env_setup",
            title: "Environment Setup",
            section_id: SECTION_ID,
            level: Level::Beginner,
            topic_type: TopicType::Sub,
            placement: Placement::Right,
        },
        Topic {
            id: "install",
            title: "Installing Rust and Cargo",
            section_id: SECTION_ID,
            level: Level::Beginner,
            topic_type: TopicType::Sub,
            placement: Placement::Right,
        },
        Topic {
            id: "ides",
            title: "IDEs and Rust Toolchains",
            section_id: SECTION_ID,
            level: Level::Beginner,
            topic_type: TopicType::Sub,
            placement: Placement::Right,
        },
        // --- Left Side (Syntax) ---
        Topic {
            id: "syntax_group", // Header for the group
            title: "Syntax and Semantics",
            section_id: SECTION_ID,
            level: Level::Beginner,
            topic_type: TopicType::Sub, // Or distinct style if prefer
            placement: Placement::Left,
        },
        Topic {
            id: "vars",
            title: "Variables, DataTypes",
            section_id: SECTION_ID,
            level: Level::Beginner,
            topic_type: TopicType::Sub,
            placement: Placement::Left,
        },
        Topic {
            id: "control_flow",
            title: "Control Flow",
            section_id: SECTION_ID,
            level: Level::Beginner,
            topic_type: TopicType::Sub,
            placement: Placement::Left,
        },
    ]
}

pub fn get_dependencies() -> Vec<Dependency> {
    vec![
        // Spine -> Right
        Dependency {
            from: "basics",
            to: "env_setup",
        },
        Dependency {
            from: "env_setup",
            to: "install",
        },
        Dependency {
            from: "install",
            to: "ides",
        },
        // Spine -> Left
        Dependency {
            from: "basics",
            to: "syntax_group",
        },
        Dependency {
            from: "syntax_group",
            to: "vars",
        },
        Dependency {
            from: "vars",
            to: "control_flow",
        },
    ]
}
