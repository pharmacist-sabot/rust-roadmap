//! Horizontal group layout engine.
//! Sections flow left-to-right as labeled group boxes.
//! Topics within each group flow top-to-bottom.

use crate::models::roadmap::{Dependency, Section, Topic, TopicType};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct LayoutConfig {
  /// Width of each topic node.
  pub node_width: f64,
  /// Height of each topic node.
  pub node_height: f64,
  /// Height of the section group header bar.
  pub header_height: f64,
  /// Horizontal padding inside a group box.
  pub group_padding_x: f64,
  /// Vertical padding inside a group box (top/bottom of topic area).
  pub group_padding_y: f64,
  /// Vertical gap between topics within a group.
  pub topic_gap_y: f64,
  /// Horizontal gap between consecutive group boxes.
  pub group_gap_x: f64,
  /// Starting X offset for the first group.
  pub start_x: f64,
  /// Starting Y offset for all groups.
  pub start_y: f64,
}

impl Default for LayoutConfig {
  fn default() -> Self {
    Self {
      node_width: 200.0,
      node_height: 44.0,
      header_height: 36.0,
      group_padding_x: 14.0,
      group_padding_y: 12.0,
      topic_gap_y: 8.0,
      group_gap_x: 52.0,
      start_x: 40.0,
      start_y: 40.0,
    }
  }
}

/// A labeled bounding box for a section group.
#[derive(Debug, Clone, PartialEq)]
pub struct GroupBox {
  pub section_id: &'static str,
  pub x: f64,
  pub y: f64,
  pub width: f64,
  pub height: f64,
  /// Display label (the section title).
  pub label: &'static str,
}

/// Computed screen position for a single topic node.
#[derive(Debug, Clone, PartialEq)]
pub struct TopicPosition {
  pub topic_id: &'static str,
  pub section_id: &'static str,
  pub x: f64,
  pub y: f64,
  pub width: f64,
}

/// Full result of the layout pass.
#[derive(Debug, Clone, PartialEq)]
pub struct LayoutResult {
  pub groups: Vec<GroupBox>,
  pub topics: Vec<TopicPosition>,
  pub min_x: f64,
  pub total_width: f64,
  pub total_height: f64,
}

/// Compute the horizontal group layout.
///
/// Sections are sorted by `order` and laid out left to right.
/// Topics within each section are sorted: `Main` types first, then `Sub`.
/// Within a type, they're ordered by `row` hint then array position.
pub fn compute_layout(
  sections: &[Section],
  topics: &[Topic],
  _dependencies: &[Dependency],
  config: &LayoutConfig,
) -> LayoutResult {
  let mut sorted_sections = sections.to_vec();
  sorted_sections.sort_by_key(|s| s.order);

  let mut groups: Vec<GroupBox> = Vec::new();
  let mut topic_positions: Vec<TopicPosition> = Vec::new();

  let mut current_x = config.start_x;
  let mut max_group_height: f64 = 0.0;

  for section in sorted_sections.iter() {
    // Collect topics for this section
    let mut section_topics: Vec<&Topic> = topics
      .iter()
      .filter(|t| t.section_id == section.id)
      .collect();

    if section_topics.is_empty() {
      continue;
    }

    // Sort: Main first, then Sub; within each, by row hint then stable array order.
    section_topics.sort_by(|a, b| {
      let type_ord = |t: &&Topic| match t.topic_type {
        TopicType::Main => 0u8,
        TopicType::Sub => 1u8,
      };
      let ta = type_ord(a);
      let tb = type_ord(b);
      if ta != tb {
        return ta.cmp(&tb);
      }
      let ra = a.row.unwrap_or(usize::MAX);
      let rb = b.row.unwrap_or(usize::MAX);
      ra.cmp(&rb)
    });

    let n = section_topics.len();
    let group_inner_width = config.node_width;
    let group_width = group_inner_width + 2.0 * config.group_padding_x;
    let topics_height =
      n as f64 * config.node_height + n.saturating_sub(1) as f64 * config.topic_gap_y;
    let group_height =
      config.header_height + config.group_padding_y + topics_height + config.group_padding_y;

    let group_x = current_x;
    let group_y = config.start_y;

    groups.push(GroupBox {
      section_id: section.id,
      x: group_x,
      y: group_y,
      width: group_width,
      height: group_height,
      label: section.title,
    });

    // Position each topic within the group
    for (i, topic) in section_topics.iter().enumerate() {
      let x = group_x + config.group_padding_x;
      let y = group_y
        + config.header_height
        + config.group_padding_y
        + i as f64 * (config.node_height + config.topic_gap_y);

      topic_positions.push(TopicPosition {
        topic_id: topic.id,
        section_id: section.id,
        x,
        y,
        width: config.node_width,
      });
    }

    max_group_height = max_group_height.max(group_height);
    current_x += group_width + config.group_gap_x;
  }

  // Total canvas dimensions (add right/bottom margin)
  let total_width = (current_x - config.group_gap_x + config.start_x).max(0.0);
  let total_height = config.start_y + max_group_height + config.start_y;

  LayoutResult {
    groups,
    topics: topic_positions,
    min_x: 0.0,
    total_width,
    total_height,
  }
}

// ---------------------------------------------------------------------------
// Edge routing helpers
// ---------------------------------------------------------------------------

/// Right-center point of a group box (exit point for cross-section edges).
pub fn group_right_center(group: &GroupBox) -> (f64, f64) {
  (group.x + group.width, group.y + group.height / 2.0)
}

/// Left-center point of a group box (entry point for cross-section edges).
pub fn group_left_center(group: &GroupBox) -> (f64, f64) {
  (group.x, group.y + group.height / 2.0)
}

/// Bottom-center of a topic node.
pub fn topic_bottom_center(pos: &TopicPosition, config: &LayoutConfig) -> (f64, f64) {
  (pos.x + pos.width / 2.0, pos.y + config.node_height)
}

/// Top-center of a topic node.
pub fn topic_top_center(pos: &TopicPosition) -> (f64, f64) {
  (pos.x + pos.width / 2.0, pos.y)
}

/// Right-center of a topic node.
pub fn topic_right_center(pos: &TopicPosition, config: &LayoutConfig) -> (f64, f64) {
  (pos.x + pos.width, pos.y + config.node_height / 2.0)
}

/// Left-center of a topic node.
pub fn topic_left_center(pos: &TopicPosition, config: &LayoutConfig) -> (f64, f64) {
  (pos.x, pos.y + config.node_height / 2.0)
}
