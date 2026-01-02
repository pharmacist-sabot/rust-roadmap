use crate::components::roadmap::diagram::{DiagramData, RoadmapDiagram};
use crate::data::{SECTIONS, get_all_dependencies, get_all_topics};
use crate::layout::tree::{LayoutConfig, compute_layout};
use leptos::*;

#[component]
pub fn RoadmapPage() -> impl IntoView {
    let config = LayoutConfig::default();

    // Data Loading (Now aggregated from modules)
    // Note: Since these return Vec, we leak them to get &'static slice lifetime
    // or adjust the DiagramData to accept Vec/Cow.
    // For simplicity in this constraints, we will store them in local Signals or just Box::leak for the demo.
    // **Proper Rust Way:** Change DiagramData to accept Vec or generic slice.
    // But to minimize diffs, I will use a simple block here.

    let topics = get_all_topics();
    let dependencies = get_all_dependencies();

    // Box::leak to match the existing &'static lifetime requirement in components
    // (In a real app, you'd use Rc<Vec<>> or similar)
    let static_topics = Box::leak(topics.into_boxed_slice());
    let static_deps = Box::leak(dependencies.into_boxed_slice());

    let layout = compute_layout(SECTIONS, static_topics, static_deps, &config);

    let diagram_props = DiagramData {
        sections: SECTIONS,
        topics: static_topics,
        dependencies: static_deps,
        layout,
        config,
    };

    view! {
        <div class="roadmap-page">
            <header class="roadmap-header">
                <h1>"Rust Developer Roadmap"</h1>
                <p class="roadmap-subtitle">"Step by step guide to becoming a Rust developer"</p>
            </header>
            <main class="roadmap-content">
                <RoadmapDiagram props=diagram_props />
            </main>
        </div>
    }
}
