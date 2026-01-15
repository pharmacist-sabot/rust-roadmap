//! Deterministic layout algorithm respecting explicit Placement.

use crate::models::roadmap::{Dependency, Placement, Section, Topic};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct LayoutConfig {
    pub node_width: f64,
    pub node_height: f64,
    pub center_x: f64,         // Absolute center axis
    pub col_spacing: f64,      // Distance from center to Left/Right columns
    pub node_spacing_y: f64,   // Vertical gap between nodes
    pub section_spacing: f64,  // Vertical gap between sections
    pub grid_col_spacing: f64, // Horizontal gap between grid columns
}

impl Default for LayoutConfig {
    fn default() -> Self {
        Self {
            node_width: 220.0,      // Reduced from 320.0 (Too big)
            node_height: 60.0,      // Reduced from 80.0
            center_x: 600.0,        // Adjusted center
            col_spacing: 180.0,     // Tighter columns
            node_spacing_y: 40.0,   // Tighter vertical flow
            section_spacing: 80.0,  // Reduced section gap
            grid_col_spacing: 20.0, // Compact grid
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
    pub width: f64, // Added dynamic width
}

fn estimate_width(title: &str) -> f64 {
    let base_padding = 40.0;
    let char_width = 11.0; // Approx width for monospace font
    let min_width = 140.0;
    let est = (title.len() as f64 * char_width) + base_padding;
    est.max(min_width)
}

#[derive(Debug, Clone, PartialEq)]
pub struct LayoutResult {
    pub sections: Vec<SectionPosition>,
    pub topics: Vec<TopicPosition>,
    pub min_x: f64,
    pub total_width: f64,
    pub total_height: f64,
}

pub fn compute_layout(
    sections: &[Section],
    topics: &[Topic],
    _dependencies: &[Dependency],
    config: &LayoutConfig,
) -> LayoutResult {
    let section_positions = Vec::new();
    let mut topic_positions = Vec::new();

    let mut current_y = 50.0; // Start margin

    // Sort sections by order explicitly
    let mut sorted_sections = sections.to_vec();
    sorted_sections.sort_by_key(|s| s.order);

    for section in sorted_sections {
        let section_topics: Vec<&Topic> = topics
            .iter()
            .filter(|t| t.section_id == section.id)
            .collect();

        if section_topics.is_empty() {
            continue;
        }

        let start_y_for_section = current_y;
        let mut max_y_in_section = start_y_for_section;

        // 2. Separate topics into columns
        let center_topics: Vec<&Topic> = section_topics
            .iter()
            .filter(|t| t.placement == Placement::Center)
            .copied()
            .collect();
        let left_topics: Vec<&Topic> = section_topics
            .iter()
            .filter(|t| t.placement == Placement::Left)
            .copied()
            .collect();
        let right_topics: Vec<&Topic> = section_topics
            .iter()
            .filter(|t| t.placement == Placement::Right)
            .copied()
            .collect();

        // 3. Layout Center(s)
        for (i, t) in center_topics.iter().enumerate() {
            let width = estimate_width(t.title);
            let y = start_y_for_section + (i as f64) * (config.node_height + config.node_spacing_y);
            let x = config.center_x - (width / 2.0);

            topic_positions.push(TopicPosition {
                topic_id: t.id,
                section_id: section.id,
                x,
                y,
                width,
            });
            max_y_in_section = max_y_in_section.max(y + config.node_height);
        }

        // 4. Layout Lefts (Dynamic Grid)
        if !left_topics.is_empty() {
            // Group by row
            let mut row_groups: std::collections::BTreeMap<usize, Vec<&Topic>> =
                std::collections::BTreeMap::new();
            let mut auto_row = 0usize;
            for t in left_topics.iter() {
                let row_idx = t.row.unwrap_or_else(|| {
                    let r = auto_row;
                    auto_row += 1;
                    r
                });
                row_groups.entry(row_idx).or_default().push(t);
            }

            // Layout each row
            for (row_idx, topics_in_row) in row_groups.iter() {
                let mut current_x = config.center_x - config.col_spacing;

                // For Left side, we flow backwards (Right to Left)
                // We need to calculate widths first to place them correctly?
                // Creating a standard flow: Center - Spacing - Node1 - Spacing - Node2
                // Since 'x' is the top-left corner, for Left side placement:
                // Node 1 (closest to center): x = center - col_spacing - node1_width
                // Node 2: x = node1_x - spacing - node2_width

                for t in topics_in_row.iter() {
                    let width = estimate_width(t.title);
                    current_x -= width; // Shift left by width

                    let y = start_y_for_section
                        + (*row_idx as f64) * (config.node_height + config.node_spacing_y);

                    topic_positions.push(TopicPosition {
                        topic_id: t.id,
                        section_id: section.id,
                        x: current_x,
                        y,
                        width,
                    });
                    max_y_in_section = max_y_in_section.max(y + config.node_height);

                    current_x -= config.grid_col_spacing; // Add spacing for next item
                }
            }
        }

        // 5. Layout Rights (Dynamic Grid)
        if !right_topics.is_empty() {
            let start_x = config.center_x + config.col_spacing;

            let mut row_groups: std::collections::BTreeMap<usize, Vec<&Topic>> =
                std::collections::BTreeMap::new();
            let mut auto_row = 0usize;
            for t in right_topics.iter() {
                let row_idx = t.row.unwrap_or_else(|| {
                    let r = auto_row;
                    auto_row += 1;
                    r
                });
                row_groups.entry(row_idx).or_default().push(t);
            }

            for (row_idx, topics_in_row) in row_groups.iter() {
                let mut current_x = start_x;

                for t in topics_in_row.iter() {
                    let width = estimate_width(t.title);
                    let y = start_y_for_section
                        + (*row_idx as f64) * (config.node_height + config.node_spacing_y);

                    topic_positions.push(TopicPosition {
                        topic_id: t.id,
                        section_id: section.id,
                        x: current_x,
                        y,
                        width,
                    });
                    max_y_in_section = max_y_in_section.max(y + config.node_height);

                    current_x += width + config.grid_col_spacing; // Move right for next item
                }
            }
        }

        // Move Y cursor for next section
        current_y = max_y_in_section + config.section_spacing;
    }

    // Calculate actual bounds from topic positions to avoid clipping
    let max_x = topic_positions
        .iter()
        .map(|p| p.x + p.width)
        .fold(0.0_f64, |a, b| a.max(b));

    let min_x = topic_positions
        .iter()
        .map(|p| p.x)
        .fold(0.0_f64, |a, b| a.min(b));

    // Calculate width accounting for both left and right overflow
    let symmetric_width = config.center_x * 2.0;
    let right_overflow_width = max_x + 50.0;
    let left_overflow_width = if min_x < 0.0 {
        (-min_x) + symmetric_width + 50.0
    } else {
        symmetric_width
    };
    let calculated_width = symmetric_width
        .max(right_overflow_width)
        .max(left_overflow_width);

    LayoutResult {
        sections: section_positions,
        topics: topic_positions,
        min_x: min_x.min(0.0), // Ensure min_x is 0 or negative
        total_width: calculated_width,
        total_height: current_y + 100.0,
    }
}

pub fn topic_bottom_edge(pos: &TopicPosition, config: &LayoutConfig) -> (f64, f64) {
    (pos.x + pos.width / 2.0, pos.y + config.node_height)
}
pub fn topic_top_edge(pos: &TopicPosition, _config: &LayoutConfig) -> (f64, f64) {
    (pos.x + pos.width / 2.0, pos.y)
}
// Helpers for side connections
pub fn topic_left_edge(pos: &TopicPosition, config: &LayoutConfig) -> (f64, f64) {
    (pos.x, pos.y + config.node_height / 2.0)
}
pub fn topic_right_edge(pos: &TopicPosition, config: &LayoutConfig) -> (f64, f64) {
    (pos.x + pos.width, pos.y + config.node_height / 2.0)
}
