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

/// Column position for sections in the tree-style layout.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum Column {
    /// Left column (Aligned to the Right side of the column)
    Left,
    /// Center column (Centered)
    #[default]
    Center,
    /// Right column (Aligned to the Left side of the column)
    Right,
    /// Full width section spanning all columns
    Full,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Section {
    pub id: &'static str,
    pub title: &'static str,
    pub order: u8,
    pub column: Column,
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
