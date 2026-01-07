use crate::models::roadmap::TopicContent;

pub fn get_content(id: &str) -> Option<TopicContent> {
    match id {
        "testing_tdd_spine" => Some(TopicContent {
            title: "Testing & TDD",
            description: "Placeholder for Testing & TDD",
            resources: vec![],
        }),
        _ => None,
    }
}
