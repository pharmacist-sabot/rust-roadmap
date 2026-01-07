use crate::models::roadmap::TopicContent;

pub fn get_content(id: &str) -> Option<TopicContent> {
    match id {
        "project_structure_spine" => Some(TopicContent {
            title: "Project Structure",
            description: "Placeholder for Project Structure",
            resources: vec![],
        }),
        _ => None,
    }
}
