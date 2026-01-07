use crate::models::roadmap::TopicContent;

pub fn get_content(id: &str) -> Option<TopicContent> {
    match id {
        "webassembly_wasm_spine" => Some(TopicContent {
            title: "WebAssembly (WASM)",
            description: "Placeholder for WebAssembly (WASM)",
            resources: vec![],
        }),
        _ => None,
    }
}
