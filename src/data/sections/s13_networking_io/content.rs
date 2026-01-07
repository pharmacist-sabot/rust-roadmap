use crate::models::roadmap::TopicContent;

pub fn get_content(id: &str) -> Option<TopicContent> {
    match id {
        "networking_io_spine" => Some(TopicContent {
            title: "Networking & I/O",
            description: "Placeholder for Networking & I/O",
            resources: vec![],
        }),
        _ => None,
    }
}
