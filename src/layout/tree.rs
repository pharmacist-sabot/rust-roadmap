//! Deterministic tree-style layout algorithm.
//! Now strictly Row-based to maintain visual relationships.

use crate::models::roadmap::{ColumnType, Dependency, LayoutZone, Section, Topic};
use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct LayoutConfig {
    pub column_width: f64,
    pub column_spacing: f64,
    pub section_spacing_y: f64,
    pub node_width: f64,
    pub node_height: f64,
    pub node_spacing_x: f64,
    pub node_spacing_y: f64,
    pub margin: f64,
    pub topics_per_row: usize,
}

impl Default for LayoutConfig {
    fn default() -> Self {
        Self {
            column_width: 250.0,     // Standard width
            column_spacing: 60.0,    // Tighter spacing
            section_spacing_y: 60.0, // More breathing room between rows
            node_width: 180.0,
            node_height: 40.0,
            node_spacing_x: 20.0,
            node_spacing_y: 20.0,
            margin: 50.0,
            topics_per_row: 1, // Default to single stack for Side columns
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct SectionPosition {
    pub section_id: &'static str,
    pub x: f64,
    pub y: f64,
    pub width: f64,
}

#[derive(Debug, Clone, PartialEq)]
pub struct TopicPosition {
    pub topic_id: &'static str,
    pub section_id: &'static str,
    pub x: f64,
    pub y: f64,
}

#[derive(Debug, Clone, PartialEq)]
pub struct LayoutResult {
    pub sections: Vec<SectionPosition>,
    pub topics: Vec<TopicPosition>,
    pub total_width: f64,
    pub total_height: f64,
}

fn topological_sort<'a>(topics: &[&'a Topic], dependencies: &[Dependency]) -> Vec<&'a Topic> {
    let mut id_map: HashMap<&str, &'a Topic> = HashMap::new();
    let mut topic_ids: HashSet<&str> = HashSet::new();
    for topic in topics {
        id_map.insert(topic.id, *topic);
        topic_ids.insert(topic.id);
    }

    let mut in_degree: HashMap<&str, usize> = HashMap::new();
    let mut adj: HashMap<&str, Vec<&str>> = HashMap::new();
    for topic in topics {
        in_degree.insert(topic.id, 0);
        adj.insert(topic.id, Vec::new());
    }

    for dep in dependencies {
        if topic_ids.contains(dep.from) && topic_ids.contains(dep.to) {
            adj.entry(dep.from).or_default().push(dep.to);
            *in_degree.entry(dep.to).or_default() += 1;
        }
    }

    let mut queue: Vec<&str> = in_degree
        .iter()
        .filter(|&(_, &d)| d == 0)
        .map(|(&id, _)| id)
        .collect();

    // Sort queue by original index to maintain stability
    let mut topic_order_map = HashMap::new();
    for (i, t) in topics.iter().enumerate() {
        topic_order_map.insert(t.id, i);
    }
    queue.sort_by_key(|id| topic_order_map.get(id).unwrap_or(&0));
    queue.reverse();

    let mut result = Vec::new();
    while let Some(u) = queue.pop() {
        if let Some(topic) = id_map.get(u) {
            result.push(*topic);
        }
        if let Some(neighbors) = adj.get(u) {
            let mut sorted_neighbors = neighbors.clone();
            sorted_neighbors.sort_by_key(|id| topic_order_map.get(id).unwrap_or(&0));
            // Stable sort

            for &v in neighbors {
                if let Some(d) = in_degree.get_mut(v) {
                    *d -= 1;
                    if *d == 0 {
                        queue.push(v);
                    }
                }
            }
            queue.sort_by_key(|id| std::cmp::Reverse(topic_order_map.get(id).unwrap_or(&0)));
        }
    }
    // Add remaining
    for topic in topics {
        if !result.contains(topic) {
            result.push(*topic);
        }
    }
    result
}

pub fn compute_layout(
    sections: &[Section],
    topics: &[Topic],
    dependencies: &[Dependency],
    config: &LayoutConfig,
) -> LayoutResult {
    let mut section_positions = Vec::with_capacity(sections.len());
    let mut topic_positions = Vec::with_capacity(topics.len());

    // Group sections by Order (Row)
    // Key: Order Index -> Value: List of Sections
    let mut rows: HashMap<i32, Vec<&Section>> = HashMap::new();
    for section in sections {
        // Footer is special, treat as Order 9999 or handle separately?
        // Let's treat standard Header/Body zones as tiered rows.
        let order_key: i32 = match section.zone {
            LayoutZone::Footer => 9999,
            _ => section.order as i32,
        };
        rows.entry(order_key).or_default().push(section);
    }

    let mut sorted_keys: Vec<i32> = rows.keys().cloned().collect();
    sorted_keys.sort();

    // Determine X Base Positions
    // Assuming symmetric 3-column layout
    // Left Start | Center Start | Right Start
    let center_col_width = config.node_width * 1.5; // Wider for main topics
    let side_col_width = config.column_width;

    let total_width_est = side_col_width * 2.0 + center_col_width + config.column_spacing * 2.0;

    let left_x = config.margin;
    let center_x = left_x + side_col_width + config.column_spacing;
    let right_x = center_x + center_col_width + config.column_spacing;

    let mut current_y = config.margin;

    for key in sorted_keys {
        let row_sections = rows.get(&key).unwrap();

        let mut row_max_height = 0.0;
        let start_y = current_y;

        // Sorting sections within the row by Zone usually helps: Left -> Center -> Right
        // But we iterate simply.

        for section in row_sections {
            let (target_x, target_w, layout_mode) = match section.zone {
                LayoutZone::Body(ColumnType::Left) => (left_x, side_col_width, ColumnType::Left),
                LayoutZone::Body(ColumnType::Right) => (right_x, side_col_width, ColumnType::Right),
                LayoutZone::Body(ColumnType::Center) | LayoutZone::Header => {
                    (center_x, center_col_width, ColumnType::Center)
                }
                LayoutZone::Footer => (config.margin, total_width_est, ColumnType::Center), // Full width
            };

            // Layout this section
            let raw_topics: Vec<&Topic> = topics
                .iter()
                .filter(|t| t.section_id == section.id)
                .collect();
            let section_topics = topological_sort(&raw_topics, dependencies);

            let header_h = 30.0;
            let is_wide_single = section.zone == LayoutZone::Footer; // Or specific logic
            let topics_per_row = if is_wide_single {
                4
            } else {
                match layout_mode {
                    ColumnType::Center => 1,
                    _ => config.topics_per_row, // 1 for sides usually
                }
            };

            let rows_count = section_topics.len().div_ceil(topics_per_row);

            section_positions.push(SectionPosition {
                section_id: section.id,
                x: target_x,
                y: start_y,
                width: target_w,
            });

            for (idx, topic) in section_topics.iter().enumerate() {
                let r = idx / topics_per_row;
                let c = idx % topics_per_row;

                let node_y =
                    start_y + header_h + (r as f64) * (config.node_height + config.node_spacing_y);

                // Calculate X
                let row_nodes_count =
                    if r == rows_count - 1 && section_topics.len() % topics_per_row != 0 {
                        section_topics.len() % topics_per_row
                    } else {
                        topics_per_row
                    };

                let row_width = (row_nodes_count as f64) * config.node_width
                    + ((row_nodes_count.saturating_sub(1)) as f64) * config.node_spacing_x;

                let node_x = match layout_mode {
                    ColumnType::Left => {
                        // Right align within Left Column to hug the spine
                        let empty = target_w - row_width;
                        target_x + empty + (c as f64) * (config.node_width + config.node_spacing_x)
                    }
                    ColumnType::Center => {
                        // Center align
                        target_x
                            + (target_w - row_width) / 2.0
                            + (c as f64) * (config.node_width + config.node_spacing_x)
                    }
                    ColumnType::Right => {
                        // Left align
                        target_x + (c as f64) * (config.node_width + config.node_spacing_x)
                    }
                };

                topic_positions.push(TopicPosition {
                    topic_id: topic.id,
                    section_id: section.id,
                    x: node_x,
                    y: node_y,
                });
            }

            let section_h = header_h
                + (rows_count as f64) * config.node_height
                + (rows_count.saturating_sub(1) as f64) * config.node_spacing_y;
            if section_h > row_max_height {
                row_max_height = section_h;
            }
        }

        current_y += row_max_height + config.section_spacing_y;
    }

    LayoutResult {
        sections: section_positions,
        topics: topic_positions,
        total_width: total_width_est + config.margin * 2.0,
        total_height: current_y + config.margin,
    }
}

pub fn topic_bottom_edge(pos: &TopicPosition, config: &LayoutConfig) -> (f64, f64) {
    (pos.x + config.node_width / 2.0, pos.y + config.node_height)
}
pub fn topic_top_edge(pos: &TopicPosition, config: &LayoutConfig) -> (f64, f64) {
    (pos.x + config.node_width / 2.0, pos.y)
}
