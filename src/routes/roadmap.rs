//! Roadmap page route.
//!
//! Loads roadmap data, computes layout, and renders the SVG diagram.
//! No business logic here - delegates to data, layout, and component modules.

use leptos::*;

use crate::components::roadmap::diagram::{DiagramData, RoadmapDiagram};
use crate::data::roadmap_data::{DEPENDENCIES, SECTIONS, TOPICS};
use crate::layout::tree::{LayoutConfig, compute_layout};

/// Roadmap page component.
///
/// This is the main entry point for the roadmap visualization.
/// It orchestrates data loading, layout computation, and rendering.
#[component]
pub fn RoadmapPage() -> impl IntoView {
    // Use default layout configuration
    let config = LayoutConfig::default();

    // Compute layout positions from static data
    let layout = compute_layout(SECTIONS, TOPICS, DEPENDENCIES, &config);

    // Build diagram props
    let diagram_props = DiagramData {
        sections: SECTIONS,
        topics: TOPICS,
        dependencies: DEPENDENCIES,
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
