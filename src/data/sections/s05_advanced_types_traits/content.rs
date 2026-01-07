use crate::models::roadmap::TopicContent;

pub fn get_content(id: &str) -> Option<TopicContent> {
    match id {
        "advanced_types_traits_spine" => Some(TopicContent {
            title: "Advanced Types & Traits",
            description: "Placeholder for Advanced Types & Traits",
            resources: vec![],
        }),
        _ => None,
    }
}
