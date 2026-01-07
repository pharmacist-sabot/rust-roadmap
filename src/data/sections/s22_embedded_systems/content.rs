use crate::models::roadmap::TopicContent;

pub fn get_content(id: &str) -> Option<TopicContent> {
    match id {
        "embedded_systems_spine" => Some(TopicContent {
            title: "Embedded Systems",
            description: "Placeholder for Embedded Systems",
            resources: vec![],
        }),
        _ => None,
    }
}
