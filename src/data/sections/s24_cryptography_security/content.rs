use crate::models::roadmap::TopicContent;

pub fn get_content(id: &str) -> Option<TopicContent> {
    match id {
        "cryptography_security_spine" => Some(TopicContent {
            title: "Cryptography & Security",
            description: "Placeholder for Cryptography & Security",
            resources: vec![],
        }),
        _ => None,
    }
}
