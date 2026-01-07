use crate::models::roadmap::TopicContent;

pub fn get_content(id: &str) -> Option<TopicContent> {
    match id {
        "package_management_deepdive_spine" => Some(TopicContent {
            title: "Package Management Deep Dive",
            description: "Placeholder for Package Management Deep Dive",
            resources: vec![],
        }),
        _ => None,
    }
}
