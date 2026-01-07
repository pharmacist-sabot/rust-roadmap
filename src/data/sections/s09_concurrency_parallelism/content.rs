use crate::models::roadmap::TopicContent;

pub fn get_content(id: &str) -> Option<TopicContent> {
    match id {
        "concurrency_parallelism_spine" => Some(TopicContent {
            title: "Concurrency & Parallelism",
            description: "Placeholder for Concurrency & Parallelism",
            resources: vec![],
        }),
        _ => None,
    }
}
