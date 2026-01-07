use crate::models::roadmap::TopicContent;

pub fn get_content(id: &str) -> Option<TopicContent> {
    match id {
        "cli_utilities_spine" => Some(TopicContent {
            title: "CLI Utilities",
            description: "Placeholder for CLI Utilities",
            resources: vec![],
        }),
        _ => None,
    }
}
