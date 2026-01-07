use crate::models::roadmap::TopicContent;

pub fn get_content(id: &str) -> Option<TopicContent> {
    match id {
        "documentation_docs_spine" => Some(TopicContent {
            title: "Documentation",
            description: "Placeholder for Documentation",
            resources: vec![],
        }),
        _ => None,
    }
}
