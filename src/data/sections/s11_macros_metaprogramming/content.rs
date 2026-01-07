use crate::models::roadmap::TopicContent;

pub fn get_content(id: &str) -> Option<TopicContent> {
    match id {
        "macros_metaprogramming_spine" => Some(TopicContent {
            title: "Macros & Metaprogramming",
            description: "Placeholder for Macros & Metaprogramming",
            resources: vec![],
        }),
        _ => None,
    }
}
