//! SVG node component for rendering roadmap topics.
//!
//! Receives precomputed positions via props. No layout logic here.

use leptos::*;

use crate::models::roadmap::Level;

/// Props for the RoadmapNode component.
#[derive(Clone, Debug, PartialEq)]
pub struct NodeData {
    /// Topic ID for identification.
    pub id: &'static str,
    /// Display title.
    pub title: &'static str,
    /// Difficulty level (for styling).
    pub level: Level,
    /// X position (precomputed).
    pub x: f64,
    /// Y position (precomputed).
    pub y: f64,
    /// Node width.
    pub width: f64,
    /// Node height.
    pub height: f64,
}

/// Render a single roadmap topic node as SVG.
///
/// # Styling
/// CSS classes are applied based on difficulty level:
/// - `.roadmap-node` - Base class for all nodes
/// - `.level-beginner` / `.level-intermediate` / `.level-advanced`
#[component]
pub fn RoadmapNode(props: NodeData) -> impl IntoView {
    let level_class = match props.level {
        Level::Beginner => "level-beginner",
        Level::Intermediate => "level-intermediate",
        Level::Advanced => "level-advanced",
    };

    let class_attr = format!("roadmap-node {}", level_class);

    view! {
        <g class=class_attr data-topic-id=props.id>
            <rect
                x=props.x
                y=props.y
                width=props.width
                height=props.height
                rx="8"
                ry="8"
                class="node-rect"
            />
            <text
                x=props.x + props.width / 2.0
                y=props.y + props.height / 2.0
                class="node-text"
                text-anchor="middle"
                dominant-baseline="central"
            >
                {props.title}
            </text>
        </g>
    }
}
