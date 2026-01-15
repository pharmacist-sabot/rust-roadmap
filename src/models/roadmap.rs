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

/// Layout strategy for the section's branches.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SectionLayout {
    List,
    Grid { cols: usize },
}

/// A logical grouping of topics.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Section {
    pub id: &'static str,
    pub title: &'static str,
    pub order: u8,
    pub layout: SectionLayout,
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
    /// Row within the section for grid layout (0-indexed).
    /// `None` means auto-assign based on array order (backward compatible).
    pub row: Option<usize>,
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
    /// Official documentation (e.g., The Rust Book, std docs).
    Official,
    /// Source code repository (e.g., GitHub, GitLab).
    OpenSource, // หรืออาจจะใช้คำว่า Repository
    /// Specific crate on crates.io or docs.rs.
    Crate,
    /// A standalone blog post or text tutorial.
    Article,
    /// A full book (digital or physical).
    Book,
    /// A standalone video or conference talk.
    Video,
    /// A structured course (series of videos/lessons).
    Course,
    /// Interactive learning, coding challenges (e.g., Rustlings).
    Interactive,
    /// Audio content.
    Podcast,
    /// Newsletters (e.g., This Week in Rust).
    Newsletter,
    /// Community discussions (e.g., Reddit, Discord, URL).
    Community,
    /// Fallback for anything else.
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
