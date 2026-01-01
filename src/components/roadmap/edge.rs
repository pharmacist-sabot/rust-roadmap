//! SVG edge component for rendering dependency connections.
//!
//! Receives precomputed positions via props. No layout logic here.

use leptos::*;

/// Props for the RoadmapEdge component.
#[derive(Clone, Debug, PartialEq)]
pub struct EdgeData {
    /// Source topic ID.
    pub from_id: &'static str,
    /// Target topic ID.
    pub to_id: &'static str,
    /// Start X position.
    pub x1: f64,
    /// Start Y position.
    pub y1: f64,
    /// End X position.
    pub x2: f64,
    /// End Y position.
    pub y2: f64,
    /// Whether this is a cross-section edge.
    pub is_cross_section: bool,
}

/// Render a dependency edge between two topics.
///
/// # Styling
/// CSS classes:
/// - `.roadmap-edge` - Base class for all edges
/// - `.edge-cross-section` - Applied to cross-section dependencies
#[component]
pub fn RoadmapEdge(props: EdgeData) -> impl IntoView {
    let class_attr = if props.is_cross_section {
        "roadmap-edge edge-cross-section"
    } else {
        "roadmap-edge"
    };

    // Compute orthogonal path (Manhattan routing)
    // Down -> Horizontal -> Down
    // IMPROVEMENT: Use a fixed offset above the TARGET node for the horizontal segment.
    // This prevents the line from cutting through intermediate nodes.
    let target_offset = 20.0;
    let mid_y = props.y2 - target_offset;

    // Safety check: if y1 is too close to y2 (shouldn't happen in top-down tree much),
    // fallback to midpoint.
    let mid_y = if mid_y < props.y1 + 10.0 {
        (props.y1 + props.y2) / 2.0
    } else {
        mid_y
    };

    let path_d = format!(
        "M {} {} L {} {} L {} {} L {} {}",
        props.x1, props.y1, props.x1, mid_y, props.x2, mid_y, props.x2, props.y2
    );

    view! {
        <path
            class=class_attr
            d=path_d
            data-from=props.from_id
            data-to=props.to_id
            fill="none"
            marker-end="url(#arrowhead)"
        />
    }
}

/// Render the arrowhead marker definition for edges.
///
/// This should be included once in the SVG defs section.
#[component]
pub fn ArrowheadMarker() -> impl IntoView {
    view! {
        <defs>
            <marker
                id="arrowhead"
                markerWidth="6"
                markerHeight="4"
                refX="5"
                refY="2"
                orient="auto"
                markerUnits="strokeWidth"
            >
                <polygon points="0 0, 6 2, 0 4" class="arrowhead-fill" />
            </marker>
        </defs>
    }
}
