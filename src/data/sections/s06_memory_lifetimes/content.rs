use crate::models::roadmap::TopicContent;

pub fn get_content(id: &str) -> Option<TopicContent> {
    match id {
        "memory_lifetimes_spine" => Some(TopicContent {
            title: "Memory & Lifetimes",
            description: "Placeholder for Memory & Lifetimes",
            resources: vec![],
        }),
        _ => None,
    }
}
