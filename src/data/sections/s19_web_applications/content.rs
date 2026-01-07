use crate::models::roadmap::TopicContent;

pub fn get_content(id: &str) -> Option<TopicContent> {
    match id {
        "web_applications_spine" => Some(TopicContent {
            title: "Web Applications",
            description: "Placeholder for Web Applications",
            resources: vec![],
        }),
        _ => None,
    }
}
