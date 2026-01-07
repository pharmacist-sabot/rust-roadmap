use crate::models::roadmap::TopicContent;

pub fn get_content(id: &str) -> Option<TopicContent> {
    match id {
        "databases_orm_spine" => Some(TopicContent {
            title: "Databases & ORM",
            description: "Placeholder for Databases & ORM",
            resources: vec![],
        }),
        _ => None,
    }
}
