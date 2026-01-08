use crate::models::roadmap::{Dependency, Level, Placement, Topic, TopicType};

pub mod content;

pub const SECTION_ID: &str = "project_structure";

pub fn get_topics() -> Vec<Topic> {
    vec![
        Topic {
            id: "project_structure_spine",
            title: "Project Structure",
            section_id: SECTION_ID,
            level: Level::Beginner,
            topic_type: TopicType::Main,
            placement: Placement::Center,
            row: None,
        },
        Topic {
            id: "packages_crates",
            title: "Packages & Crates",
            section_id: SECTION_ID,
            level: Level::Beginner,
            topic_type: TopicType::Sub,
            placement: Placement::Right,
            row: Some(0),
        },
        Topic {
            id: "modules_use",
            title: "Modules & Paths",
            section_id: SECTION_ID,
            level: Level::Beginner,
            topic_type: TopicType::Sub,
            placement: Placement::Right,
            row: Some(0),
        },
        Topic {
            id: "workspaces",
            title: "Workspaces",
            section_id: SECTION_ID,
            level: Level::Intermediate,
            topic_type: TopicType::Sub,
            placement: Placement::Right,
            row: Some(1),
        },
    ]
}

pub fn get_dependencies() -> Vec<Dependency> {
    vec![
        Dependency {
            from: "project_structure_spine",
            to: "packages_crates",
        },
        Dependency {
            from: "packages_crates",
            to: "modules_use",
        },
        Dependency {
            from: "project_structure_spine",
            to: "workspaces",
        },
    ]
}
