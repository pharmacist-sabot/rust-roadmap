//! Main roadmap diagram component.

use crate::components::roadmap::edge::{ArrowheadMarker, EdgeData, RoadmapEdge};
use crate::components::roadmap::node::{NodeData, RoadmapNode};
use crate::layout::tree::{
    LayoutConfig, LayoutResult, TopicPosition, topic_bottom_edge, topic_left_edge,
    topic_right_edge, topic_top_edge,
};
use crate::models::roadmap::{Dependency, Section, Topic};
use leptos::*;

#[derive(Clone, Debug, PartialEq)]
pub struct SectionHeaderData {
    pub id: &'static str,
    pub title: &'static str,
    pub x: f64,
    pub y: f64,
    pub width: f64, // Added width for centering
}

#[component]
fn SectionHeader(props: SectionHeaderData) -> impl IntoView {
    // Center text in the section column
    let center_x = props.x + props.width / 2.0;
    view! {
        <text x=center_x y=props.y class="section-header" data-section-id=props.id>
            {props.title}
        </text>
    }
}

#[derive(Clone)]
pub struct DiagramData {
    pub sections: &'static [Section],
    pub topics: &'static [Topic],
    pub dependencies: &'static [Dependency],
    pub layout: LayoutResult,
    pub config: LayoutConfig,
    pub on_topic_click: Callback<&'static str>,
}

fn find_topic<'a>(topics: &'a [Topic], id: &str) -> Option<&'a Topic> {
    topics.iter().find(|t| t.id == id)
}

fn find_topic_position<'a>(positions: &'a [TopicPosition], id: &str) -> Option<&'a TopicPosition> {
    positions.iter().find(|p| p.topic_id == id)
}

#[component]
pub fn RoadmapDiagram(props: DiagramData) -> impl IntoView {
    let viewbox = format!(
        "0 0 {} {}",
        props.layout.total_width, props.layout.total_height
    );

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
                    width: sp.width,
                })
        })
        .collect();

    let node_props: Vec<_> = props
        .layout
        .topics
        .iter()
        .filter_map(|tp| {
            find_topic(props.topics, tp.topic_id).map(|topic| NodeData {
                id: topic.id,
                title: topic.title,
                level: topic.level,
                topic_type: topic.topic_type, // Pass TopicType
                x: tp.x,
                y: tp.y,
                width: props.config.node_width,
                height: props.config.node_height,
                on_click: props.on_topic_click,
            })
        })
        .collect();

    let edge_props: Vec<_> = props
        .dependencies
        .iter()
        .filter_map(|dep| {
            let from_pos = find_topic_position(&props.layout.topics, dep.from)?;
            let to_pos = find_topic_position(&props.layout.topics, dep.to)?;
            let from_topic = find_topic(props.topics, dep.from)?;
            let to_topic = find_topic(props.topics, dep.to)?;

            // Determine connection points based on Placement
            let (x1, y1, x2, y2) = match (from_topic.placement, to_topic.placement) {
                // Center → Right: exit from right edge, enter from left edge
                (
                    crate::models::roadmap::Placement::Center,
                    crate::models::roadmap::Placement::Right,
                ) => {
                    let (fx, fy) = topic_right_edge(from_pos, &props.config);
                    let (tx, ty) = topic_left_edge(to_pos, &props.config);
                    (fx, fy, tx, ty)
                }
                // Center → Left: exit from left edge, enter from right edge
                (
                    crate::models::roadmap::Placement::Center,
                    crate::models::roadmap::Placement::Left,
                ) => {
                    let (fx, fy) = topic_left_edge(from_pos, &props.config);
                    let (tx, ty) = topic_right_edge(to_pos, &props.config);
                    (fx, fy, tx, ty)
                }
                // Vertical spine connections (Center → Center): top/bottom
                _ => {
                    let (fx, fy) = topic_bottom_edge(from_pos, &props.config);
                    let (tx, ty) = topic_top_edge(to_pos, &props.config);
                    (fx, fy, tx, ty)
                }
            };

            // Dashed line: different Section OR different TopicType
            let is_cross_section = from_pos.section_id != to_pos.section_id
                || from_topic.topic_type != to_topic.topic_type;

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
        <svg class="roadmap-diagram" viewBox=viewbox xmlns="http://www.w3.org/2000/svg">
            <ArrowheadMarker />
            <g class="edges-layer">
                {edge_props.into_iter().map(|ep| view! { <RoadmapEdge props=ep /> }).collect_view()}
            </g>
            <g class="sections-layer">
                {section_headers.into_iter().map(|sh| view! { <SectionHeader props=sh /> }).collect_view()}
            </g>
            <g class="nodes-layer">
                {node_props.into_iter().map(|np| view! { <RoadmapNode props=np /> }).collect_view()}
            </g>
        </svg>
    }
}
