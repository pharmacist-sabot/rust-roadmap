//! Main roadmap diagram component that assembles nodes and edges.
//!
//! Receives precomputed layout data. No layout computation here.

use leptos::*;

use crate::components::roadmap::edge::{ArrowheadMarker, EdgeData, RoadmapEdge};
use crate::components::roadmap::node::{NodeData, RoadmapNode};
use crate::layout::tree::{
    LayoutConfig, LayoutResult, TopicPosition, topic_bottom_edge, topic_top_edge,
};
use crate::models::roadmap::{Dependency, Section, Topic};

/// Props for section header rendering.
#[derive(Clone, Debug, PartialEq)]
pub struct SectionHeaderData {
    /// Section ID.
    pub id: &'static str,
    /// Display title.
    pub title: &'static str,
    /// X position.
    pub x: f64,
    /// Y position.
    pub y: f64,
}

/// Render a section header in the diagram.
#[component]
fn SectionHeader(props: SectionHeaderData) -> impl IntoView {
    view! {
        <text
            x=props.x
            y=props.y
            class="section-header"
            data-section-id=props.id
        >
            {props.title}
        </text>
    }
}

/// Props for the full roadmap diagram.
#[derive(Clone)]
pub struct DiagramData {
    /// All sections.
    pub sections: &'static [Section],
    /// All topics.
    pub topics: &'static [Topic],
    /// All dependencies.
    pub dependencies: &'static [Dependency],
    /// Precomputed layout result.
    pub layout: LayoutResult,
    /// Layout configuration (for node dimensions).
    pub config: LayoutConfig,
}

/// Find a topic by ID.
fn find_topic<'a>(topics: &'a [Topic], id: &str) -> Option<&'a Topic> {
    topics.iter().find(|t| t.id == id)
}

/// Find a topic position by ID.
fn find_topic_position<'a>(positions: &'a [TopicPosition], id: &str) -> Option<&'a TopicPosition> {
    positions.iter().find(|p| p.topic_id == id)
}

/// Main roadmap diagram component.
///
/// Assembles all nodes, edges, and section headers into a complete SVG.
#[component]
pub fn RoadmapDiagram(props: DiagramData) -> impl IntoView {
    let viewbox = format!(
        "0 0 {} {}",
        props.layout.total_width, props.layout.total_height
    );

    // Build section headers
    let section_headers: Vec<_> = props
        .layout
        .sections
        .iter()
        .filter_map(|sp| {
            props
                .sections
                .iter()
                .find(|s| s.id == sp.section_id)
                .map(|s| SectionHeaderData {
                    id: s.id,
                    title: s.title,
                    x: sp.x,
                    y: sp.y,
                })
        })
        .collect();

    // Build node props
    let node_props: Vec<_> = props
        .layout
        .topics
        .iter()
        .filter_map(|tp| {
            find_topic(props.topics, tp.topic_id).map(|topic| NodeData {
                id: topic.id,
                title: topic.title,
                level: topic.level,
                x: tp.x,
                y: tp.y,
                width: props.config.node_width,
                height: props.config.node_height,
            })
        })
        .collect();

    // Build edge props
    let edge_props: Vec<_> = props
        .dependencies
        .iter()
        .filter_map(|dep| {
            let from_pos = find_topic_position(&props.layout.topics, dep.from)?;
            let to_pos = find_topic_position(&props.layout.topics, dep.to)?;

            // Compute connection points (bottom edge of from, top edge of to)
            let (x1, y1) = topic_bottom_edge(from_pos, &props.config);
            let (x2, y2) = topic_top_edge(to_pos, &props.config);

            // Check if cross-section
            let is_cross_section = from_pos.section_id != to_pos.section_id;

            Some(EdgeData {
                from_id: dep.from,
                to_id: dep.to,
                x1,
                y1,
                x2,
                y2,
                is_cross_section,
            })
        })
        .collect();

    view! {
        <svg
            class="roadmap-diagram"
            viewBox=viewbox
            xmlns="http://www.w3.org/2000/svg"
        >
            <ArrowheadMarker />

            // Render edges first (behind nodes)
            <g class="edges-layer">
                {edge_props.into_iter().map(|ep| view! { <RoadmapEdge props=ep /> }).collect_view()}
            </g>

            // Render section headers
            <g class="sections-layer">
                {section_headers.into_iter().map(|sh| view! { <SectionHeader props=sh /> }).collect_view()}
            </g>

            // Render nodes on top
            <g class="nodes-layer">
                {node_props.into_iter().map(|np| view! { <RoadmapNode props=np /> }).collect_view()}
            </g>
        </svg>
    }
}
