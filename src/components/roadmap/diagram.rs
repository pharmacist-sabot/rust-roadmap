//! Main roadmap diagram component.

use crate::components::roadmap::edge::{ArrowheadMarker, EdgeData, RoadmapEdge};
use crate::components::roadmap::node::{NodeData, RoadmapNode};
use crate::layout::tree::{
    LayoutConfig, LayoutResult, TopicPosition, topic_bottom_edge, topic_left_edge,
    topic_right_edge, topic_top_edge,
};
use crate::models::roadmap::{Dependency, Section, Topic};
use leptos::*;

#[derive(Clone)]
pub struct DiagramData {
    pub sections: &'static [Section],
    pub topics: &'static [Topic],
    pub dependencies: &'static [Dependency],
    pub layout: LayoutResult,
    pub config: LayoutConfig,
    pub on_topic_click: Callback<&'static str>,
    pub search_term: ReadSignal<String>,
}

fn find_topic<'a>(topics: &'a [Topic], id: &str) -> Option<&'a Topic> {
    topics.iter().find(|t| t.id == id)
}

fn find_topic_position<'a>(positions: &'a [TopicPosition], id: &str) -> Option<&'a TopicPosition> {
    positions.iter().find(|p| p.topic_id == id)
}

/// Check if a topic title matches the search term (case-insensitive)
fn topic_matches_search(topic: &Topic, term: &str) -> bool {
    if term.is_empty() {
        return false; // No highlight when search is empty
    }
    topic.title.to_lowercase().contains(&term.to_lowercase())
}

/// Scroll to the first matching node in the viewport using web-sys (Pure Rust)
fn scroll_to_first_match(topic_id: &str) {
    let window = match web_sys::window() {
        Some(w) => w,
        None => return,
    };

    let document = match window.document() {
        Some(d) => d,
        None => return,
    };

    let selector = format!("[data-topic-id=\"{}\"]", topic_id);
    let element = match document.query_selector(&selector) {
        Ok(Some(el)) => el,
        _ => return,
    };

    // Create ScrollToOptions with smooth behavior
    let scroll_options = web_sys::ScrollToOptions::new();
    scroll_options.set_behavior(web_sys::ScrollBehavior::Smooth);

    // Get element position and scroll with offset
    let rect = element.get_bounding_client_rect();
    let current_scroll = window.scroll_y().unwrap_or(0.0);
    let target_y = current_scroll + rect.top() - 150.0; // 150px offset from top

    scroll_options.set_top(target_y);
    window.scroll_to_with_scroll_to_options(&scroll_options);
}

#[component]
pub fn RoadmapDiagram(props: DiagramData) -> impl IntoView {
    let viewbox = format!(
        "{} 0 {} {}",
        props.layout.min_x, props.layout.total_width, props.layout.total_height
    );

    // Clone props for use in closures
    let search_term = props.search_term;
    let topics = props.topics;
    let layout_topics = props.layout.topics.clone();
    let config = props.config;
    let on_topic_click = props.on_topic_click;

    // For scroll effect
    let topics_for_scroll = props.topics;
    let search_term_for_scroll = search_term;

    // Effect to scroll to first match when search term changes
    create_effect(move |_| {
        let term = search_term_for_scroll.get();
        if !term.is_empty() {
            // Find first matching topic
            for topic in topics_for_scroll.iter() {
                if topic_matches_search(topic, &term) {
                    scroll_to_first_match(topic.id);
                    break;
                }
            }
        }
    });

    // Pre-compute edge data (edges don't need to be reactive for highlight mode)
    let edge_props: Vec<_> = props
        .dependencies
        .iter()
        .filter_map(|dep| {
            let from_pos = find_topic_position(&props.layout.topics, dep.from)?;
            let to_pos = find_topic_position(&props.layout.topics, dep.to)?;
            let from_topic = find_topic(props.topics, dep.from)?;
            let to_topic = find_topic(props.topics, dep.to)?;

            let (x1, y1, x2, y2) = match (from_topic.placement, to_topic.placement) {
                (
                    crate::models::roadmap::Placement::Center,
                    crate::models::roadmap::Placement::Right,
                ) => {
                    let (fx, fy) = topic_right_edge(from_pos, &props.config);
                    let (tx, ty) = topic_left_edge(to_pos, &props.config);
                    (fx, fy, tx, ty)
                }
                (
                    crate::models::roadmap::Placement::Center,
                    crate::models::roadmap::Placement::Left,
                ) => {
                    let (fx, fy) = topic_left_edge(from_pos, &props.config);
                    let (tx, ty) = topic_right_edge(to_pos, &props.config);
                    (fx, fy, tx, ty)
                }
                (
                    crate::models::roadmap::Placement::Right,
                    crate::models::roadmap::Placement::Right,
                ) => {
                    let (fx, fy) = topic_right_edge(from_pos, &props.config);
                    let (tx, ty) = topic_left_edge(to_pos, &props.config);
                    (fx, fy, tx, ty)
                }
                (
                    crate::models::roadmap::Placement::Left,
                    crate::models::roadmap::Placement::Left,
                ) => {
                    let (fx, fy) = topic_left_edge(from_pos, &props.config);
                    let (tx, ty) = topic_right_edge(to_pos, &props.config);
                    (fx, fy, tx, ty)
                }
                _ => {
                    let (fx, fy) = topic_bottom_edge(from_pos, &props.config);
                    let (tx, ty) = topic_top_edge(to_pos, &props.config);
                    (fx, fy, tx, ty)
                }
            };

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
            <g class="nodes-layer">
                {move || {
                    let term = search_term.get();

                    // Show ALL nodes, but highlight matches
                    layout_topics
                        .iter()
                        .filter_map(|tp| {
                            let topic = find_topic(topics, tp.topic_id)?;
                            let is_highlighted = topic_matches_search(topic, &term);

                            Some(NodeData {
                                id: topic.id,
                                x: tp.x,
                                y: tp.y,
                                width: tp.width,
                                height: config.node_height,
                                title: topic.title,
                                level: topic.level,
                                topic_type: topic.topic_type,
                                on_click: on_topic_click,
                                is_highlighted,
                            })
                        })
                        .map(|np| view! { <RoadmapNode props=np /> })
                        .collect_view()
                }}
            </g>
        </svg>
    }
}
