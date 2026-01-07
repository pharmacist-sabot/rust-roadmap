use crate::models::roadmap::TopicContent;

pub fn get_content(id: &str) -> Option<TopicContent> {
    match id {
        "serialization_data_spine" => Some(TopicContent {
            title: "Serialization & Data",
            description: "Placeholder for Serialization & Data",
            resources: vec![],
        }),
        _ => None,
    }
}
