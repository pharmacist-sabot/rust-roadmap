//! Domain models for the Rust learning roadmap.

/// Difficulty level for a topic.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Level {
    Beginner,
    Intermediate,
    Advanced,
}

/// Visual type of the topic box
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum TopicType {
    Main,
    #[default]
    Sub,
}

/// Explicit layout instruction relative to the central spine.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum Placement {
    /// Aligned to the central axis.
    #[default]
    Center,
    /// Branches out to the Left.
    Left,
    /// Branches out to the Right.
    Right,
}

/// A logical grouping of topics.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Section {
    pub id: &'static str,
    pub title: &'static str,
    pub order: u8,
}

/// A single node in the roadmap.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Topic {
    pub id: &'static str,
    pub title: &'static str,
    pub section_id: &'static str,
    pub level: Level,
    pub topic_type: TopicType,
    pub placement: Placement,
}

/// A directed edge between topics.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Dependency {
    pub from: &'static str,
    pub to: &'static str,
}

/// Type of resource badge.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BadgeKind {
    Official,
    OpenSource,
    Article,
    Video,
    Feed,
    Other(&'static str),
}

/// A resource link for a topic.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Resource {
    pub label: &'static str,
    pub url: &'static str,
    pub badge: BadgeKind,
}

/// Detailed content for a topic (Title, Description, Resources).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TopicContent {
    pub title: &'static str,
    pub description: &'static str,
    pub resources: Vec<Resource>,
}
