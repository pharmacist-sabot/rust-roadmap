use crate::models::roadmap::TopicContent;

pub fn get_content(id: &str) -> Option<TopicContent> {
    match id {
        "performance_optimization_spine" => Some(TopicContent {
            title: "Performance Optimization",
            description: "Placeholder for Performance Optimization",
            resources: vec![],
        }),
        _ => None,
    }
}
