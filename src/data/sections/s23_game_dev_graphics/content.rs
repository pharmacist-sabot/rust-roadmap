use crate::models::roadmap::TopicContent;

pub fn get_content(id: &str) -> Option<TopicContent> {
    match id {
        "game_dev_graphics_spine" => Some(TopicContent {
            title: "Game Dev & Graphics",
            description: "Placeholder for Game Dev & Graphics",
            resources: vec![],
        }),
        _ => None,
    }
}
