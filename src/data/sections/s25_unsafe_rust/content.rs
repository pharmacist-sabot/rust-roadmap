use crate::models::roadmap::TopicContent;

pub fn get_content(id: &str) -> Option<TopicContent> {
    match id {
        "unsafe_rust_spine" => Some(TopicContent {
            title: "Unsafe Rust",
            description: "Placeholder for Unsafe Rust",
            resources: vec![],
        }),
        _ => None,
    }
}
