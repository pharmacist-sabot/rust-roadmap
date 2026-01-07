use crate::models::roadmap::TopicContent;

pub fn get_content(id: &str) -> Option<TopicContent> {
    match id {
        "debugging_tools_spine" => Some(TopicContent {
            title: "Debugging Tools",
            description: "Placeholder for Debugging Tools",
            resources: vec![],
        }),
        _ => None,
    }
}
