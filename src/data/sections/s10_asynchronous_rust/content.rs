use crate::models::roadmap::TopicContent;

pub fn get_content(id: &str) -> Option<TopicContent> {
    match id {
        "asynchronous_rust_spine" => Some(TopicContent {
            title: "Asynchronous Rust",
            description: "Placeholder for Asynchronous Rust",
            resources: vec![],
        }),
        _ => None,
    }
}
