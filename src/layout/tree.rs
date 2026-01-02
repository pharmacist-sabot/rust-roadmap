//! Deterministic layout algorithm respecting explicit Placement.

use crate::models::roadmap::{Dependency, Placement, Section, Topic};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct LayoutConfig {
    pub node_width: f64,
    pub node_height: f64,
    pub center_x: f64,        // Absolute center axis
    pub col_spacing: f64,     // Distance from center to Left/Right columns
    pub node_spacing_y: f64,  // Vertical gap between nodes
    pub section_spacing: f64, // Vertical gap between sections
}

impl Default for LayoutConfig {
    fn default() -> Self {
        Self {
            node_width: 180.0,
            node_height: 40.0,
            center_x: 600.0,    // Wide canvas center
            col_spacing: 150.0, // Gap between Spine and Branches
            node_spacing_y: 20.0,
            section_spacing: 60.0,
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

pub fn compute_layout(
    sections: &[Section],
    topics: &[Topic],
    _dependencies: &[Dependency], // Used for fine-tuning sort if needed, but we rely on order mostly
    config: &LayoutConfig,
) -> LayoutResult {
    let mut section_positions = Vec::new();
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

        // 1. Identify the Anchor (Main/Center topic) to determine this section's Y baseline
        // Usually the first Center topic found.

        // Store Section Header (Optional visual aid)
        section_positions.push(SectionPosition {
            section_id: section.id,
            x: config.center_x - (config.node_width / 2.0),
            y: current_y,
            width: config.node_width,
        });

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
        // Usually just one main spine node, but could be stacked
        let start_y_for_section = current_y;
        let mut max_y_in_section = current_y;

        for (i, t) in center_topics.iter().enumerate() {
            let y = start_y_for_section + (i as f64) * (config.node_height + config.node_spacing_y);
            let x = config.center_x - (config.node_width / 2.0);

            topic_positions.push(TopicPosition {
                topic_id: t.id,
                section_id: section.id,
                x,
                y,
            });
            max_y_in_section = max_y_in_section.max(y + config.node_height);
        }

        // 4. Layout Lefts (Stacking downwards)
        // Logic: Align Right-side of the node to (center_x - col_spacing)
        if !left_topics.is_empty() {
            let start_x = config.center_x - config.col_spacing - config.node_width;
            for (i, t) in left_topics.iter().enumerate() {
                let y =
                    start_y_for_section + (i as f64) * (config.node_height + config.node_spacing_y);

                topic_positions.push(TopicPosition {
                    topic_id: t.id,
                    section_id: section.id,
                    x: start_x,
                    y,
                });
                max_y_in_section = max_y_in_section.max(y + config.node_height);
            }
        }

        // 5. Layout Rights (Stacking downwards)
        // Logic: Align Left-side of the node to (center_x + col_spacing)
        if !right_topics.is_empty() {
            let start_x = config.center_x + config.col_spacing;
            for (i, t) in right_topics.iter().enumerate() {
                let y =
                    start_y_for_section + (i as f64) * (config.node_height + config.node_spacing_y);

                topic_positions.push(TopicPosition {
                    topic_id: t.id,
                    section_id: section.id,
                    x: start_x,
                    y,
                });
                max_y_in_section = max_y_in_section.max(y + config.node_height);
            }
        }

        // Move Y cursor for next section
        current_y = max_y_in_section + config.section_spacing;
    }

    LayoutResult {
        sections: section_positions,
        topics: topic_positions,
        total_width: config.center_x * 2.0, // Symmetric canvas
        total_height: current_y + 100.0,
    }
}

pub fn topic_bottom_edge(pos: &TopicPosition, config: &LayoutConfig) -> (f64, f64) {
    (pos.x + config.node_width / 2.0, pos.y + config.node_height)
}
pub fn topic_top_edge(pos: &TopicPosition, config: &LayoutConfig) -> (f64, f64) {
    (pos.x + config.node_width / 2.0, pos.y)
}
// Helpers for side connections
pub fn topic_left_edge(pos: &TopicPosition, config: &LayoutConfig) -> (f64, f64) {
    (pos.x, pos.y + config.node_height / 2.0)
}
pub fn topic_right_edge(pos: &TopicPosition, config: &LayoutConfig) -> (f64, f64) {
    (pos.x + config.node_width, pos.y + config.node_height / 2.0)
}
