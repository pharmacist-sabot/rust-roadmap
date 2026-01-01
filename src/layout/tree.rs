//! Deterministic tree-style layout algorithm for roadmap visualization.
//!
//! This module computes (x, y) positions for sections and topics
//! in a multi-column vertical tree layout.
//! No rendering logic, SVG, or framework dependencies belong here.

use crate::models::roadmap::{Column, Dependency, Section, Topic};
use std::collections::{HashMap, HashSet};

/// Layout configuration parameters for tree-style roadmap.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct LayoutConfig {
    /// Width of each column.
    pub column_width: f64,
    /// Horizontal spacing between columns.
    pub column_spacing: f64,
    /// Vertical spacing between sections within a column.
    pub section_spacing_y: f64,
    /// Width of a single topic node.
    pub node_width: f64,
    /// Height of a single topic node.
    pub node_height: f64,
    /// Horizontal spacing between topics in the same section.
    pub node_spacing_x: f64,
    /// Vertical spacing between topic rows in the same section.
    pub node_spacing_y: f64,
    /// Overall margin around the layout.
    pub margin: f64,
    /// Maximum topics per row within a section.
    pub topics_per_row: usize,
}

impl Default for LayoutConfig {
    fn default() -> Self {
        Self {
            column_width: 440.0,
            column_spacing: 80.0,
            section_spacing_y: 40.0,
            node_width: 200.0,
            node_height: 36.0,
            node_spacing_x: 10.0,
            node_spacing_y: 8.0,
            margin: 40.0,
            topics_per_row: 2,
        }
    }
}

/// Computed position for a section header.
#[derive(Debug, Clone, PartialEq)]
pub struct SectionPosition {
    /// Section ID (references `Section.id`).
    pub section_id: &'static str,
    /// X coordinate of the section header.
    pub x: f64,
    /// Y coordinate of the section header.
    pub y: f64,
}

/// Computed position for a topic node.
#[derive(Debug, Clone, PartialEq)]
pub struct TopicPosition {
    /// Topic ID (references `Topic.id`).
    pub topic_id: &'static str,
    /// Section ID this topic belongs to.
    pub section_id: &'static str,
    /// X coordinate of the topic node (top-left corner).
    pub x: f64,
    /// Y coordinate of the topic node (top-left corner).
    pub y: f64,
}

/// Complete layout result containing all computed positions.
#[derive(Debug, Clone, PartialEq)]
pub struct LayoutResult {
    /// Positions of all section headers.
    pub sections: Vec<SectionPosition>,
    /// Positions of all topic nodes.
    pub topics: Vec<TopicPosition>,
    /// Total width of the layout (for SVG viewBox).
    pub total_width: f64,
    /// Total height of the layout (for SVG viewBox).
    pub total_height: f64,
}

/// Compute the X offset for a given column.
fn column_x_offset(column: Column, config: &LayoutConfig) -> f64 {
    match column {
        Column::Left => config.margin,
        Column::Center => config.margin + config.column_width + config.column_spacing,
        Column::Right => config.margin + 2.0 * (config.column_width + config.column_spacing),
        Column::Full => config.margin,
    }
}

/// Compute the width for a given column.
fn column_width(column: Column, config: &LayoutConfig) -> f64 {
    match column {
        Column::Full => 3.0 * config.column_width + 2.0 * config.column_spacing,
        _ => config.column_width,
    }
}

/// Calculate the height of a section based on its topics.
fn section_height(topic_count: usize, config: &LayoutConfig) -> f64 {
    if topic_count == 0 {
        30.0 // Header only
    } else {
        let rows = (topic_count + config.topics_per_row - 1) / config.topics_per_row;
        let header_height = 30.0;
        let topics_height = (rows as f64) * config.node_height
            + ((rows.saturating_sub(1)) as f64) * config.node_spacing_y;
        header_height + topics_height
    }
}

/// Sort topics topologically based on dependencies within the section.
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

    // Build graph for topics in this section only
    for dep in dependencies {
        if topic_ids.contains(dep.from) && topic_ids.contains(dep.to) {
            let neighbors = adj.entry(dep.from).or_default();
            neighbors.push(dep.to);
            *in_degree.entry(dep.to).or_default() += 1;
        }
    }

    // Kahn's Algorithm
    // Stable sort initial queue by ID logic (here relying on HashMap order which is random-ish, so explicit sorting is needed)
    let mut queue: Vec<&str> = in_degree
        .iter()
        .filter(|&(_, &deg)| deg == 0)
        .map(|(&id, _)| id)
        .collect();

    // Sort reverse so pop() gets elements in order
    // Order: Alphabetical? Or original input order?
    // Ideally we preserve input order if no dependency.
    // Let's use alphabetic stability for now.
    queue.sort_by(|a, b| b.cmp(a));

    let mut result = Vec::new();
    while let Some(u) = queue.pop() {
        if let Some(topic) = id_map.get(u) {
            result.push(*topic);
        }

        if let Some(neighbors) = adj.get(u) {
            // Sort neighbors to ensure deterministic processing order
            let mut sorted_neighbors = neighbors.clone();
            sorted_neighbors.sort_by(|a, b| b.cmp(a)); // Reverse sort for same reason? No, just sort.

            for &v in neighbors {
                if let Some(deg) = in_degree.get_mut(v) {
                    *deg -= 1;
                    if *deg == 0 {
                        queue.push(v);
                    }
                }
            }
            // Re-sort queue
            queue.sort_by(|a, b| b.cmp(a));
        }
    }

    // If cycles or remaining nodes (not expected in roadmap), append them to ensure they appear
    for topic in topics {
        if !result.contains(topic) {
            result.push(*topic);
        }
    }

    result
}

/// Compute deterministic tree-style layout positions for the roadmap.
///
/// # Algorithm
/// 1. Group sections by column (Left, Center, Right, Full)
/// 2. For each column, stack sections vertically
/// 3. For each section, sort topics Topologically based on Dependencies
/// 4. Arrange topics in a grid (rows of N topics)
/// 5. Full-width sections are placed below all columns
///
/// # Arguments
/// * `sections` - Slice of sections to layout
/// * `topics` - Slice of topics to layout
/// * `dependencies` - Slice of dependencies for topological sorting
/// * `config` - Layout configuration parameters
///
/// # Returns
/// `LayoutResult` with computed positions for all sections and topics.
pub fn compute_layout(
    sections: &[Section],
    topics: &[Topic],
    dependencies: &[Dependency],
    config: &LayoutConfig,
) -> LayoutResult {
    let mut section_positions = Vec::with_capacity(sections.len());
    let mut topic_positions = Vec::with_capacity(topics.len());

    // Group sections by column, sorted by order
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

    // Track column heights
    let mut left_y = config.margin;
    let mut center_y = config.margin;
    let mut right_y = config.margin;

    // Process each column
    let column_configs = [
        (Column::Left, &left_sections, &mut left_y),
        (Column::Center, &center_sections, &mut center_y),
        (Column::Right, &right_sections, &mut right_y),
    ];

    for (column, column_sections, current_y) in column_configs {
        let col_x = column_x_offset(column, config);

        for section in column_sections.iter() {
            // Get topics and sort topologically
            let raw_topics: Vec<&Topic> = topics
                .iter()
                .filter(|t| t.section_id == section.id)
                .collect();

            let section_topics = topological_sort(&raw_topics, dependencies);

            // Add section header position
            section_positions.push(SectionPosition {
                section_id: section.id,
                x: col_x,
                y: *current_y,
            });

            // Layout topics in rows
            let header_offset = 30.0;
            for (idx, topic) in section_topics.iter().enumerate() {
                let row = idx / config.topics_per_row;
                let col = idx % config.topics_per_row;

                let topic_x = col_x + (col as f64) * (config.node_width + config.node_spacing_x);
                let topic_y = *current_y
                    + header_offset
                    + (row as f64) * (config.node_height + config.node_spacing_y);

                topic_positions.push(TopicPosition {
                    topic_id: topic.id,
                    section_id: section.id,
                    x: topic_x,
                    y: topic_y,
                });
            }

            // Advance Y position
            *current_y += section_height(section_topics.len(), config) + config.section_spacing_y;
        }
    }

    // Compute the maximum column height (for full-width sections)
    let max_column_y = left_y.max(center_y).max(right_y);

    // Process full-width sections
    let mut full_y = max_column_y;
    let full_x = column_x_offset(Column::Full, config);
    let full_width = column_width(Column::Full, config);

    for section in &full_sections {
        let raw_topics: Vec<&Topic> = topics
            .iter()
            .filter(|t| t.section_id == section.id)
            .collect();
        let section_topics = topological_sort(&raw_topics, dependencies);

        // Add section header
        section_positions.push(SectionPosition {
            section_id: section.id,
            x: full_x,
            y: full_y,
        });

        // For full-width sections, spread topics horizontally
        let header_offset = 30.0;
        let topics_per_row = 5; // More topics per row for full-width
        for (idx, topic) in section_topics.iter().enumerate() {
            let row = idx / topics_per_row;
            let col = idx % topics_per_row;

            let topic_spacing = (full_width - (topics_per_row as f64) * config.node_width)
                / ((topics_per_row - 1) as f64).max(1.0);
            let topic_x = full_x
                + (col as f64)
                    * (config.node_width + topic_spacing.min(config.node_spacing_x * 2.0));
            let topic_y = full_y
                + header_offset
                + (row as f64) * (config.node_height + config.node_spacing_y);

            topic_positions.push(TopicPosition {
                topic_id: topic.id,
                section_id: section.id,
                x: topic_x,
                y: topic_y,
            });
        }

        let effective_rows = (section_topics.len() + topics_per_row - 1) / topics_per_row;
        let section_h = if section_topics.is_empty() {
            30.0
        } else {
            30.0 + (effective_rows as f64) * config.node_height
                + ((effective_rows.saturating_sub(1)) as f64) * config.node_spacing_y
        };
        full_y += section_h + config.section_spacing_y;
    }

    // Compute total dimensions
    let total_width =
        config.margin + 3.0 * config.column_width + 2.0 * config.column_spacing + config.margin;
    let total_height = full_y + config.margin;

    LayoutResult {
        sections: section_positions,
        topics: topic_positions,
        total_width,
        total_height,
    }
}

/// Get the connection point on the bottom edge of a topic node.
pub fn topic_bottom_edge(pos: &TopicPosition, config: &LayoutConfig) -> (f64, f64) {
    (pos.x + config.node_width / 2.0, pos.y + config.node_height)
}

/// Get the connection point on the top edge of a topic node.
pub fn topic_top_edge(pos: &TopicPosition, config: &LayoutConfig) -> (f64, f64) {
    (pos.x + config.node_width / 2.0, pos.y)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::roadmap::Level;

    fn test_sections() -> Vec<Section> {
        vec![
            Section {
                id: "sec_b",
                title: "Second",
                order: 2,
                column: Column::Center,
            },
            Section {
                id: "sec_a",
                title: "First",
                order: 1,
                column: Column::Left,
            },
        ]
    }

    fn test_topics() -> Vec<Topic> {
        vec![
            Topic {
                id: "topic_1",
                title: "Topic 1",
                section_id: "sec_a",
                level: Level::Beginner,
            },
            Topic {
                id: "topic_2",
                title: "Topic 2",
                section_id: "sec_a",
                level: Level::Beginner,
            },
        ]
    }

    fn test_dependencies() -> Vec<Dependency> {
        vec![Dependency {
            from: "topic_1",
            to: "topic_2",
        }]
    }

    #[test]
    fn layout_is_deterministic() {
        let sections = test_sections();
        let topics = test_topics();
        let deps = test_dependencies();
        let config = LayoutConfig::default();

        let result1 = compute_layout(&sections, &topics, &deps, &config);
        let result2 = compute_layout(&sections, &topics, &deps, &config);

        assert_eq!(result1, result2, "Layout must be deterministic");
    }

    #[test]
    fn topological_sort_respects_dependencies() {
        let sections = test_sections();
        let topics = vec![
            Topic {
                id: "topic_2", // Depends on topic_1
                title: "Child",
                section_id: "sec_a",
                level: Level::Beginner,
            },
            Topic {
                id: "topic_1", // Parent
                title: "Parent",
                section_id: "sec_a",
                level: Level::Beginner,
            },
        ];
        let deps = vec![Dependency {
            from: "topic_1",
            to: "topic_2",
        }];
        let config = LayoutConfig::default();

        let result = compute_layout(&sections, &topics, &deps, &config);

        let t1 = result
            .topics
            .iter()
            .find(|t| t.topic_id == "topic_1")
            .unwrap();
        let t2 = result
            .topics
            .iter()
            .find(|t| t.topic_id == "topic_2")
            .unwrap();

        // In a grid 2 items wide (Topic 1, Topic 2):
        // If sorting worked, Topic 1 should be index 0, Topic 2 index 1.
        // Index 0: (col 0, row 0)
        // Index 1: (col 1, row 0)
        // Or if wrapping, row 1.

        // Assert t1 is BEFORE t2 visually (either strictly above, or same row and left)
        if t1.y == t2.y {
            assert!(t1.x < t2.x, "Parent should be left of child on same row");
        } else {
            assert!(t1.y < t2.y, "Parent should be above child");
        }
    }
}
