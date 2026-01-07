use crate::models::roadmap::TopicContent;

pub fn get_content(id: &str) -> Option<TopicContent> {
    match id {
        "error_handling_safety_spine" => Some(TopicContent {
            title: "Error Handling & Safety",
            description: "Placeholder for Error Handling & Safety",
            resources: vec![],
        }),
        _ => None,
    }
}
