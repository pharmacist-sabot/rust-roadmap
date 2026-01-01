//! Domain models for the Rust learning roadmap.

/// Difficulty level for a topic.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Level {
    Beginner,
    Intermediate,
    Advanced,
}

/// Visual type of the topic box.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TopicType {
    /// Yellow box (Main Spine topics)
    Main,
    /// Beige/Tan box (Sub topics)
    Sub,
}

impl Default for TopicType {
    fn default() -> Self {
        Self::Sub
    }
}

/// Column position within the Body zone.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum ColumnType {
    Left,
    #[default]
    Center,
    Right,
}

/// Logical zone for the section.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LayoutZone {
    /// Top section, centered.
    Header,
    /// Main split section.
    Body(ColumnType),
    /// Bottom section, full width.
    Footer,
}

impl Default for LayoutZone {
    fn default() -> Self {
        Self::Body(ColumnType::Center)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Section {
    pub id: &'static str,
    pub title: &'static str,
    pub order: u8,
    pub zone: LayoutZone,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Topic {
    pub id: &'static str,
    pub title: &'static str,
    pub section_id: &'static str,
    pub level: Level,
    pub topic_type: TopicType,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Dependency {
    pub from: &'static str,
    pub to: &'static str,
}
