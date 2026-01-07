use crate::models::roadmap::TopicContent;

pub fn get_content(id: &str) -> Option<TopicContent> {
    match id {
        "gui_desktop_spine" => Some(TopicContent {
            title: "GUI & Desktop",
            description: "Placeholder for GUI & Desktop",
            resources: vec![],
        }),
        _ => None,
    }
}
