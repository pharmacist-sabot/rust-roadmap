//! Deterministic tree-style layout algorithm.

use crate::models::roadmap::{Column, Dependency, Section, Topic};
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
            column_width: 350.0,   // ลดความกว้างลงหน่อยเพื่อให้เกาะกลุ่ม
            column_spacing: 120.0, // เพิ่มระยะห่างระหว่างคอลัมน์
            section_spacing_y: 50.0,
            node_width: 180.0,
            node_height: 40.0,
            node_spacing_x: 15.0,
            node_spacing_y: 15.0,
            margin: 50.0,
            topics_per_row: 2, // ปกติ 2 กล่องต่อแถวสำหรับกิ่งก้าน
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct SectionPosition {
    pub section_id: &'static str,
    pub x: f64,
    pub y: f64,
    pub width: f64, // เก็บความกว้างของ Section ไว้ด้วย
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

// Override column width specifically
fn column_width(column: Column, config: &LayoutConfig) -> f64 {
    match column {
        Column::Center => config.node_width * 1.2, // Center is narrow (only 1 node usually)
        Column::Full => config.column_width * 3.0 + config.column_spacing * 2.0,
        _ => config.column_width,
    }
}

fn topological_sort<'a>(topics: &[&'a Topic], dependencies: &[Dependency]) -> Vec<&'a Topic> {
    // (Logic เดิม ใช้ได้เลย)
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

    // Stable sort logic to keep input order if possible
    let mut queue: Vec<&str> = in_degree
        .iter()
        .filter(|&(_, &d)| d == 0)
        .map(|(&id, _)| id)
        .collect();
    // Sort to make deterministic
    let mut topic_order_map = HashMap::new();
    for (i, t) in topics.iter().enumerate() {
        topic_order_map.insert(t.id, i);
    }
    queue.sort_by_key(|id| topic_order_map.get(id).unwrap_or(&0));
    queue.reverse(); // Pop from end

    let mut result = Vec::new();
    while let Some(u) = queue.pop() {
        if let Some(topic) = id_map.get(u) {
            result.push(*topic);
        }
        if let Some(neighbors) = adj.get(u) {
            let mut sorted_neighbors = neighbors.clone();
            sorted_neighbors.sort_by_key(|id| topic_order_map.get(id).unwrap_or(&0));
            // Reverse because we reverse-sort queue later? No, just keep stable.

            for &v in neighbors {
                if let Some(d) = in_degree.get_mut(v) {
                    *d -= 1;
                    if *d == 0 {
                        queue.push(v);
                    }
                }
            }
            // Re-sort queue to maintain order priority
            queue.sort_by_key(|id| std::cmp::Reverse(topic_order_map.get(id).unwrap_or(&0)));
        }
    }
    // Cycles check
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

    let mut left_sections: Vec<&Section> = sections
        .iter()
        .filter(|s| s.column == Column::Left)
        .collect();
    let mut center_sections: Vec<&Section> = sections
        .iter()
        .filter(|s| s.column == Column::Center)
        .collect();
    let mut right_sections: Vec<&Section> = sections
        .iter()
        .filter(|s| s.column == Column::Right)
        .collect();
    let mut full_sections: Vec<&Section> = sections
        .iter()
        .filter(|s| s.column == Column::Full)
        .collect();

    left_sections.sort_by_key(|s| s.order);
    center_sections.sort_by_key(|s| s.order);
    right_sections.sort_by_key(|s| s.order);
    full_sections.sort_by_key(|s| s.order);

    let mut left_y = config.margin;
    let mut center_y = config.margin;
    let mut right_y = config.margin;

    // Config adjustments for alignment
    // Center column starts further right to make room for left
    let center_x_base = config.margin + config.column_width + config.column_spacing;

    // Process columns
    let col_defs = [
        (
            Column::Center,
            &center_sections,
            &mut center_y,
            center_x_base,
        ),
        (Column::Left, &left_sections, &mut left_y, config.margin),
        (
            Column::Right,
            &right_sections,
            &mut right_y,
            center_x_base + config.node_width * 1.5 + config.column_spacing,
        ),
    ];

    // Note: We process them independently, but typically Center dictates the "height" of the spine.
    // For this simple tree, independent y-growth is okay, but visual alignment across rows is hard without a grid.
    // We will stick to independent stacking for now as it's robust.

    for (col_type, col_sections, current_y, start_x) in col_defs {
        let col_w = column_width(col_type, config);

        for section in col_sections.iter() {
            let raw_topics: Vec<&Topic> = topics
                .iter()
                .filter(|t| t.section_id == section.id)
                .collect();
            let section_topics = topological_sort(&raw_topics, dependencies);

            // Calculate section height
            // Center usually has 1 per row
            let effective_cols = if col_type == Column::Center {
                1
            } else {
                config.topics_per_row
            };
            let effective_rows = section_topics.len().div_ceil(effective_cols);

            let header_h = 30.0;
            section_positions.push(SectionPosition {
                section_id: section.id,
                x: start_x,
                y: *current_y,
                width: col_w,
            });

            for (idx, topic) in section_topics.iter().enumerate() {
                let (row, col) = if col_type == Column::Center {
                    (idx, 0)
                } else {
                    (idx / config.topics_per_row, idx % config.topics_per_row)
                };

                let mut topic_x = 0.0;
                let topic_y = *current_y
                    + header_h
                    + (row as f64) * (config.node_height + config.node_spacing_y);

                match col_type {
                    Column::Left => {
                        // Align RIGHT within the column (closest to center)
                        // x = end_of_col - node_width - (col_reversed * spacing)
                        // Actually, just standard grid but pushed right?
                        // Let's do: fill from right to left visually?
                        // Simple grid:
                        // To align right: start_x + col_width - node_width - offset_from_right_side
                        // Let's stick to standard left-aligned grid inside the column for simplicity,
                        // but the column ITSELF is to the left of center.

                        // BUT, to look like the PDF, the Left nodes should "hug" the center.
                        // So if we have 1 node, it should be on the right side of the Left Column.
                        let nodes_in_this_row = if row == effective_rows - 1
                            && section_topics.len() % effective_cols != 0
                        {
                            section_topics.len() % effective_cols
                        } else {
                            effective_cols
                        };

                        // Shift x so the group is right-aligned
                        let row_width = (nodes_in_this_row as f64) * config.node_width
                            + ((nodes_in_this_row.saturating_sub(1)) as f64)
                                * config.node_spacing_x;
                        let empty_space = col_w - row_width;
                        topic_x = start_x
                            + empty_space
                            + (col as f64) * (config.node_width + config.node_spacing_x);
                    }
                    Column::Center => {
                        // Center align
                        topic_x = start_x + (col_w - config.node_width) / 2.0;
                    }
                    Column::Right => {
                        // Left align (standard)
                        topic_x =
                            start_x + (col as f64) * (config.node_width + config.node_spacing_x);
                    }
                    _ => {}
                }

                topic_positions.push(TopicPosition {
                    topic_id: topic.id,
                    section_id: section.id,
                    x: topic_x,
                    y: topic_y,
                });
            }

            let content_h = (effective_rows as f64) * config.node_height
                + ((effective_rows.saturating_sub(1)) as f64) * config.node_spacing_y;
            *current_y += header_h + content_h + config.section_spacing_y;
        }
    }

    // Full Width Logic (Bottom)
    let max_y = left_y.max(center_y).max(right_y);
    let mut full_y = max_y + 40.0;
    let full_start_x = config.margin;
    let full_width = config.margin + config.column_width * 3.0 + config.column_spacing * 2.0; // Approximation

    for section in full_sections {
        let raw_topics: Vec<&Topic> = topics
            .iter()
            .filter(|t| t.section_id == section.id)
            .collect();
        let section_topics = topological_sort(&raw_topics, dependencies);

        section_positions.push(SectionPosition {
            section_id: section.id,
            x: full_start_x,
            y: full_y,
            width: full_width,
        });

        let topics_per_row = 5;
        let rows = section_topics.len().div_ceil(topics_per_row);
        let header_h = 30.0;

        for (idx, topic) in section_topics.iter().enumerate() {
            let row = idx / topics_per_row;
            let col = idx % topics_per_row;

            // Center the row
            let nodes_in_this_row = if row == rows - 1 && section_topics.len() % topics_per_row != 0
            {
                section_topics.len() % topics_per_row
            } else {
                topics_per_row
            };

            let row_w = (nodes_in_this_row as f64) * config.node_width
                + ((nodes_in_this_row.saturating_sub(1)) as f64) * config.node_spacing_x;
            let start_x_row = full_start_x + (full_width - row_w) / 2.0;

            let x = start_x_row + (col as f64) * (config.node_width + config.node_spacing_x);
            let y = full_y + header_h + (row as f64) * (config.node_height + config.node_spacing_y);

            topic_positions.push(TopicPosition {
                topic_id: topic.id,
                section_id: section.id,
                x,
                y,
            });
        }

        let content_h = (rows as f64) * config.node_height
            + ((rows.saturating_sub(1)) as f64) * config.node_spacing_y;
        full_y += header_h + content_h + config.section_spacing_y;
    }

    LayoutResult {
        sections: section_positions,
        topics: topic_positions,
        total_width: full_width + config.margin * 2.0,
        total_height: full_y + config.margin,
    }
}

pub fn topic_bottom_edge(pos: &TopicPosition, config: &LayoutConfig) -> (f64, f64) {
    (pos.x + config.node_width / 2.0, pos.y + config.node_height)
}
pub fn topic_top_edge(pos: &TopicPosition, config: &LayoutConfig) -> (f64, f64) {
    (pos.x + config.node_width / 2.0, pos.y)
}
