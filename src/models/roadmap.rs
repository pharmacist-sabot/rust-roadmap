//! Domain models for the Rust learning roadmap.
//!
//! This module defines pure, type-safe data structures representing the roadmap.
//! No rendering logic, layout logic, or framework dependencies belong here.

/// Difficulty level for a topic.
///
/// Represents the skill level required to understand a topic,
/// enabling filtering and visual differentiation in the UI.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Level {
    Beginner,
    Intermediate,
    Advanced,
}

/// Column position for sections in the tree-style layout.
///
/// Determines which column a section is placed in for the
/// multi-column vertical roadmap visualization.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum Column {
    /// Left column (e.g., Syntax, Data Structures, Ownership)
    Left,
    /// Center column (e.g., Introduction, Language Basics, Error Handling)
    #[default]
    Center,
    /// Right column (e.g., Concurrency, Lifetimes)
    Right,
    /// Full width section spanning all columns (e.g., Ecosystem)
    Full,
}

/// A logical grouping of related topics in the roadmap.
///
/// Sections represent major phases or categories (e.g., "Fundamentals", "Ownership").
/// Topics belong to exactly one section.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Section {
    /// Unique, stable identifier for the section.
    pub id: &'static str,
    /// Human-readable title displayed in the UI.
    pub title: &'static str,
    /// Display order (lower values appear first).
    pub order: u8,
    /// Column position in the tree layout.
    pub column: Column,
}

/// A single learning topic (node) in the roadmap.
///
/// Topics are the atomic units of the roadmap, representing individual
/// concepts or skills that learners should master.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Topic {
    /// Unique, stable identifier for the topic.
    pub id: &'static str,
    /// Human-readable title displayed in the UI.
    pub title: &'static str,
    /// ID of the section this topic belongs to.
    pub section_id: &'static str,
    /// Difficulty level of this topic.
    pub level: Level,
}

/// A directed dependency between two topics.
///
/// Represents that `from` should be learned before `to`.
/// Used to draw edges in the roadmap visualization.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Dependency {
    /// ID of the prerequisite topic.
    pub from: &'static str,
    /// ID of the dependent topic.
    pub to: &'static str,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn level_is_copy_and_eq() {
        let level = Level::Beginner;
        let copied = level;
        assert_eq!(level, copied);
    }

    #[test]
    fn section_fields_are_accessible() {
        let section = Section {
            id: "fundamentals",
            title: "Fundamentals",
            order: 1,
            column: Column::Center,
        };
        assert_eq!(section.id, "fundamentals");
        assert_eq!(section.title, "Fundamentals");
        assert_eq!(section.order, 1);
        assert_eq!(section.column, Column::Center);
    }

    #[test]
    fn topic_fields_are_accessible() {
        let topic = Topic {
            id: "variables",
            title: "Variables & Mutability",
            section_id: "fundamentals",
            level: Level::Beginner,
        };
        assert_eq!(topic.id, "variables");
        assert_eq!(topic.section_id, "fundamentals");
        assert_eq!(topic.level, Level::Beginner);
    }

    #[test]
    fn dependency_represents_directed_edge() {
        let dep = Dependency {
            from: "variables",
            to: "ownership",
        };
        assert_eq!(dep.from, "variables");
        assert_eq!(dep.to, "ownership");
    }
}
