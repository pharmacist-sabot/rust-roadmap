use crate::models::roadmap::TopicContent;

pub fn get_content(id: &str) -> Option<TopicContent> {
    match id {
        "ffi_interop_spine" => Some(TopicContent {
            title: "FFI & Interop",
            description: "Placeholder for FFI & Interop",
            resources: vec![],
        }),
        _ => None,
    }
}
