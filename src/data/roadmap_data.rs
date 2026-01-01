//! Complete roadmap data for the Rust learning roadmap.
//!
//! This module provides static, structured data representing the roadmap content.
//! Data here references IDs and types defined in the `models` module.
//! No UI, layout, or rendering logic belongs here.

use crate::models::roadmap::{Column, Dependency, Level, Section, Topic};

// =============================================================================
// SECTIONS
// =============================================================================

/// All sections in the roadmap, ordered by display sequence.
pub const SECTIONS: &[Section] = &[
    // Center Column - Core Learning Path
    Section {
        id: "intro",
        title: "Introduction",
        order: 1,
        column: Column::Center,
    },
    Section {
        id: "language_basics",
        title: "Language Basics",
        order: 2,
        column: Column::Center,
    },
    Section {
        id: "error_handling",
        title: "Error Handling",
        order: 3,
        column: Column::Center,
    },
    Section {
        id: "modules_crates",
        title: "Modules & Crates",
        order: 4,
        column: Column::Center,
    },
    Section {
        id: "testing",
        title: "Testing",
        order: 5,
        column: Column::Center,
    },
    Section {
        id: "traits_generics",
        title: "Traits & Generics",
        order: 6,
        column: Column::Center,
    },
    // Left Column - Language Deep Dive
    Section {
        id: "syntax_semantics",
        title: "Syntax and Semantics",
        order: 7,
        column: Column::Left,
    },
    Section {
        id: "constructs",
        title: "Constructs",
        order: 8,
        column: Column::Left,
    },
    Section {
        id: "data_structures",
        title: "Data Structures",
        order: 9,
        column: Column::Left,
    },
    Section {
        id: "ownership_system",
        title: "Ownership System",
        order: 10,
        column: Column::Left,
    },
    Section {
        id: "advanced_topics",
        title: "Advanced Topics",
        order: 11,
        column: Column::Left,
    },
    Section {
        id: "macros",
        title: "Macros & Metaprogramming",
        order: 12,
        column: Column::Left,
    },
    // Right Column - Advanced Concepts
    Section {
        id: "concurrency",
        title: "Concurrency & Parallelism",
        order: 13,
        column: Column::Right,
    },
    Section {
        id: "lifetimes",
        title: "Lifetimes & Borrow Checker",
        order: 14,
        column: Column::Right,
    },
    // Full Width - Ecosystem
    Section {
        id: "ecosystem",
        title: "Ecosystem and Libraries",
        order: 15,
        column: Column::Full,
    },
    Section {
        id: "web_dev",
        title: "Web Development",
        order: 16,
        column: Column::Full,
    },
];

// =============================================================================
// TOPICS
// =============================================================================

/// All topics in the roadmap.
///
/// Each topic belongs to exactly one section via `section_id`.
pub const TOPICS: &[Topic] = &[
    // === Introduction ===
    Topic {
        id: "what_is_rust",
        title: "What is Rust?",
        section_id: "intro",
        level: Level::Beginner,
    },
    Topic {
        id: "why_use_rust",
        title: "Why use Rust?",
        section_id: "intro",
        level: Level::Beginner,
    },
    // === Language Basics ===
    Topic {
        id: "env_setup",
        title: "Environment Setup",
        section_id: "language_basics",
        level: Level::Beginner,
    },
    Topic {
        id: "install_rust_cargo",
        title: "Installing Rust and Cargo",
        section_id: "language_basics",
        level: Level::Beginner,
    },
    Topic {
        id: "ides_toolchains",
        title: "IDEs and Rust Toolchains",
        section_id: "language_basics",
        level: Level::Beginner,
    },
    Topic {
        id: "rust_repl",
        title: "Rust REPL (Rust Playground)",
        section_id: "language_basics",
        level: Level::Beginner,
    },
    // === Error Handling ===
    Topic {
        id: "option_result",
        title: "Option and Result Enumerations",
        section_id: "error_handling",
        level: Level::Intermediate,
    },
    Topic {
        id: "propagating_errors",
        title: "Propagating Errors and '?' Operator",
        section_id: "error_handling",
        level: Level::Intermediate,
    },
    Topic {
        id: "custom_errors",
        title: "Custom Error Types and Traits",
        section_id: "error_handling",
        level: Level::Intermediate,
    },
    // === Modules & Crates ===
    Topic {
        id: "code_organization",
        title: "Code Organization & Namespacing",
        section_id: "modules_crates",
        level: Level::Intermediate,
    },
    Topic {
        id: "dependency_cargo",
        title: "Dependency Management with Cargo",
        section_id: "modules_crates",
        level: Level::Intermediate,
    },
    Topic {
        id: "publishing_crates",
        title: "Publishing on Crates.io",
        section_id: "modules_crates",
        level: Level::Intermediate,
    },
    // === Testing ===
    Topic {
        id: "unit_integration_testing",
        title: "Unit & Integration Testing",
        section_id: "testing",
        level: Level::Intermediate,
    },
    Topic {
        id: "mocking_property_testing",
        title: "Mocking & Property Based Testing",
        section_id: "testing",
        level: Level::Intermediate,
    },
    // === Traits & Generics ===
    Topic {
        id: "trait_definitions",
        title: "Trait Definitions & Implementations",
        section_id: "traits_generics",
        level: Level::Intermediate,
    },
    Topic {
        id: "trait_bounds",
        title: "Trait Bounds and Associated Types",
        section_id: "traits_generics",
        level: Level::Intermediate,
    },
    Topic {
        id: "generics_types",
        title: "Generics & Type-Level Programming",
        section_id: "traits_generics",
        level: Level::Advanced,
    },
    // === Syntax and Semantics ===
    Topic {
        id: "variables_datatypes",
        title: "Variables, DataTypes and Constants",
        section_id: "syntax_semantics",
        level: Level::Beginner,
    },
    Topic {
        id: "control_flow",
        title: "Control Flow and Constructs",
        section_id: "syntax_semantics",
        level: Level::Beginner,
    },
    Topic {
        id: "functions_syntax",
        title: "Functions and Method Syntax",
        section_id: "syntax_semantics",
        level: Level::Beginner,
    },
    Topic {
        id: "pattern_matching",
        title: "Pattern Matching & Destructuring",
        section_id: "syntax_semantics",
        level: Level::Beginner,
    },
    // === Constructs ===
    Topic {
        id: "enums",
        title: "Enums",
        section_id: "constructs",
        level: Level::Beginner,
    },
    Topic {
        id: "traits_construct",
        title: "Traits",
        section_id: "constructs",
        level: Level::Beginner,
    },
    Topic {
        id: "structs",
        title: "Structs",
        section_id: "constructs",
        level: Level::Beginner,
    },
    Topic {
        id: "impl_blocks",
        title: "Impl Blocks",
        section_id: "constructs",
        level: Level::Beginner,
    },
    // === Data Structures ===
    Topic {
        id: "integers",
        title: "Integers",
        section_id: "data_structures",
        level: Level::Beginner,
    },
    Topic {
        id: "floats",
        title: "Floats",
        section_id: "data_structures",
        level: Level::Beginner,
    },
    Topic {
        id: "boolean",
        title: "Boolean",
        section_id: "data_structures",
        level: Level::Beginner,
    },
    Topic {
        id: "character",
        title: "Character",
        section_id: "data_structures",
        level: Level::Beginner,
    },
    Topic {
        id: "string",
        title: "String",
        section_id: "data_structures",
        level: Level::Beginner,
    },
    Topic {
        id: "tuple",
        title: "Tuple",
        section_id: "data_structures",
        level: Level::Beginner,
    },
    Topic {
        id: "array",
        title: "Array",
        section_id: "data_structures",
        level: Level::Beginner,
    },
    Topic {
        id: "vector",
        title: "Vector",
        section_id: "data_structures",
        level: Level::Beginner,
    },
    Topic {
        id: "hashmap",
        title: "HashMap",
        section_id: "data_structures",
        level: Level::Beginner,
    },
    Topic {
        id: "hashset",
        title: "HashSet",
        section_id: "data_structures",
        level: Level::Intermediate,
    },
    Topic {
        id: "linkedlist",
        title: "LinkedList",
        section_id: "data_structures",
        level: Level::Intermediate,
    },
    Topic {
        id: "stack",
        title: "Stack",
        section_id: "data_structures",
        level: Level::Intermediate,
    },
    Topic {
        id: "queue",
        title: "Queue",
        section_id: "data_structures",
        level: Level::Intermediate,
    },
    Topic {
        id: "binary_heap",
        title: "Binary Heap",
        section_id: "data_structures",
        level: Level::Intermediate,
    },
    // === Ownership System ===
    Topic {
        id: "ownership_rules",
        title: "Ownership Rules & Memory Safety",
        section_id: "ownership_system",
        level: Level::Intermediate,
    },
    Topic {
        id: "borrowing_refs_slices",
        title: "Borrowing, References and Slices",
        section_id: "ownership_system",
        level: Level::Intermediate,
    },
    Topic {
        id: "stack_vs_heap",
        title: "Deep Dive: Stack vs Heap",
        section_id: "ownership_system",
        level: Level::Intermediate,
    },
    // === Advanced Topics ===
    Topic {
        id: "btreemap",
        title: "BTreeMap",
        section_id: "advanced_topics",
        level: Level::Intermediate,
    },
    Topic {
        id: "btreeset",
        title: "BTreeSet",
        section_id: "advanced_topics",
        level: Level::Intermediate,
    },
    Topic {
        id: "rc",
        title: "Rc",
        section_id: "advanced_topics",
        level: Level::Intermediate,
    },
    Topic {
        id: "arc",
        title: "Arc",
        section_id: "advanced_topics",
        level: Level::Advanced,
    },
    Topic {
        id: "mutex",
        title: "Mutex",
        section_id: "advanced_topics",
        level: Level::Advanced,
    },
    Topic {
        id: "rwlock",
        title: "RwLock",
        section_id: "advanced_topics",
        level: Level::Advanced,
    },
    Topic {
        id: "channels",
        title: "Channels",
        section_id: "advanced_topics",
        level: Level::Advanced,
    },
    // === Macros & Metaprogramming ===
    Topic {
        id: "declarative_macros",
        title: "Declarative Macros with macro_rules!",
        section_id: "macros",
        level: Level::Advanced,
    },
    Topic {
        id: "procedural_macros",
        title: "Procedural Macros & Custom Derive",
        section_id: "macros",
        level: Level::Advanced,
    },
    Topic {
        id: "dsls",
        title: "Domain Specific Languages (DSLs)",
        section_id: "macros",
        level: Level::Advanced,
    },
    // === Concurrency & Parallelism ===
    Topic {
        id: "threads_channels",
        title: "Threads, Channels and Message Passing",
        section_id: "concurrency",
        level: Level::Advanced,
    },
    Topic {
        id: "atomic_ops",
        title: "Atomic Operations & Memory Barriers",
        section_id: "concurrency",
        level: Level::Advanced,
    },
    Topic {
        id: "futures_async",
        title: "Futures and Async/Await Paradigm",
        section_id: "concurrency",
        level: Level::Advanced,
    },
    // === Lifetimes & Borrow Checker ===
    Topic {
        id: "lifetime_annotations",
        title: "Explicit Lifetime Annotations",
        section_id: "lifetimes",
        level: Level::Advanced,
    },
    Topic {
        id: "lifetime_elision",
        title: "Lifetime Elision Rules",
        section_id: "lifetimes",
        level: Level::Advanced,
    },
    Topic {
        id: "variance",
        title: "Covariant & Contravariant Lifetimes",
        section_id: "lifetimes",
        level: Level::Advanced,
    },
    // === Web Development ===
    Topic {
        id: "axum",
        title: "Axum",
        section_id: "web_dev",
        level: Level::Intermediate,
    },
    Topic {
        id: "actix",
        title: "Actix",
        section_id: "web_dev",
        level: Level::Intermediate,
    },
    Topic {
        id: "leptos",
        title: "Leptos",
        section_id: "web_dev",
        level: Level::Intermediate,
    },
    Topic {
        id: "loco",
        title: "Loco",
        section_id: "web_dev",
        level: Level::Intermediate,
    },
    Topic {
        id: "tonic",
        title: "tonic-rust",
        section_id: "web_dev",
        level: Level::Advanced,
    },
];

// =============================================================================
// DEPENDENCIES
// =============================================================================

/// Dependencies between topics (directed edges).
///
/// Represents prerequisite relationships: `from` should be learned before `to`.
pub const DEPENDENCIES: &[Dependency] = &[
    // Introduction → Language Basics
    Dependency {
        from: "what_is_rust",
        to: "why_use_rust",
    },
    Dependency {
        from: "why_use_rust",
        to: "env_setup",
    },
    // Language Basics internal
    Dependency {
        from: "env_setup",
        to: "install_rust_cargo",
    },
    Dependency {
        from: "install_rust_cargo",
        to: "ides_toolchains",
    },
    Dependency {
        from: "ides_toolchains",
        to: "rust_repl",
    },
    // Language Basics → Syntax and Semantics
    Dependency {
        from: "install_rust_cargo",
        to: "variables_datatypes",
    },
    // Syntax and Semantics internal
    Dependency {
        from: "variables_datatypes",
        to: "control_flow",
    },
    Dependency {
        from: "control_flow",
        to: "functions_syntax",
    },
    Dependency {
        from: "functions_syntax",
        to: "pattern_matching",
    },
    // Syntax → Constructs
    Dependency {
        from: "variables_datatypes",
        to: "enums",
    },
    Dependency {
        from: "variables_datatypes",
        to: "structs",
    },
    // Constructs internal
    Dependency {
        from: "structs",
        to: "impl_blocks",
    },
    Dependency {
        from: "enums",
        to: "traits_construct",
    },
    // Constructs → Data Structures
    Dependency {
        from: "structs",
        to: "integers",
    },
    // Data Structures internal
    Dependency {
        from: "integers",
        to: "floats",
    },
    Dependency {
        from: "floats",
        to: "boolean",
    },
    Dependency {
        from: "character",
        to: "string",
    },
    Dependency {
        from: "string",
        to: "tuple",
    },
    Dependency {
        from: "array",
        to: "vector",
    },
    Dependency {
        from: "vector",
        to: "hashmap",
    },
    // Data Structures → Ownership
    Dependency {
        from: "string",
        to: "ownership_rules",
    },
    // Ownership internal
    Dependency {
        from: "ownership_rules",
        to: "borrowing_refs_slices",
    },
    Dependency {
        from: "borrowing_refs_slices",
        to: "stack_vs_heap",
    },
    // Ownership → Error Handling
    Dependency {
        from: "ownership_rules",
        to: "option_result",
    },
    // Error Handling internal
    Dependency {
        from: "option_result",
        to: "propagating_errors",
    },
    Dependency {
        from: "propagating_errors",
        to: "custom_errors",
    },
    // Error Handling → Modules & Crates
    Dependency {
        from: "custom_errors",
        to: "code_organization",
    },
    // Modules & Crates internal
    Dependency {
        from: "code_organization",
        to: "dependency_cargo",
    },
    Dependency {
        from: "dependency_cargo",
        to: "publishing_crates",
    },
    // Modules & Crates → Testing
    Dependency {
        from: "code_organization",
        to: "unit_integration_testing",
    },
    // Testing internal
    Dependency {
        from: "unit_integration_testing",
        to: "mocking_property_testing",
    },
    // Traits & Generics internal
    Dependency {
        from: "trait_definitions",
        to: "trait_bounds",
    },
    Dependency {
        from: "trait_bounds",
        to: "generics_types",
    },
    // Constructs → Traits & Generics
    Dependency {
        from: "traits_construct",
        to: "trait_definitions",
    },
    // Advanced Topics internal
    Dependency {
        from: "btreemap",
        to: "btreeset",
    },
    Dependency {
        from: "rc",
        to: "arc",
    },
    Dependency {
        from: "arc",
        to: "mutex",
    },
    Dependency {
        from: "mutex",
        to: "rwlock",
    },
    Dependency {
        from: "rwlock",
        to: "channels",
    },
    // Data Structures → Advanced Topics
    Dependency {
        from: "hashmap",
        to: "btreemap",
    },
    // Ownership → Advanced Topics
    Dependency {
        from: "borrowing_refs_slices",
        to: "rc",
    },
    // Advanced Topics → Concurrency
    Dependency {
        from: "arc",
        to: "threads_channels",
    },
    // Concurrency internal
    Dependency {
        from: "threads_channels",
        to: "atomic_ops",
    },
    Dependency {
        from: "atomic_ops",
        to: "futures_async",
    },
    // Traits & Generics → Macros
    Dependency {
        from: "generics_types",
        to: "declarative_macros",
    },
    // Macros internal
    Dependency {
        from: "declarative_macros",
        to: "procedural_macros",
    },
    Dependency {
        from: "procedural_macros",
        to: "dsls",
    },
    // Macros → Lifetimes
    Dependency {
        from: "declarative_macros",
        to: "lifetime_annotations",
    },
    // Lifetimes internal
    Dependency {
        from: "lifetime_annotations",
        to: "lifetime_elision",
    },
    Dependency {
        from: "lifetime_elision",
        to: "variance",
    },
    // Ownership → Lifetimes
    Dependency {
        from: "borrowing_refs_slices",
        to: "lifetime_annotations",
    },
    // Web Development internal
    Dependency {
        from: "axum",
        to: "actix",
    },
    Dependency {
        from: "actix",
        to: "leptos",
    },
    Dependency {
        from: "leptos",
        to: "loco",
    },
    Dependency {
        from: "loco",
        to: "tonic",
    },
];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sections_have_unique_ids() {
        let mut ids: Vec<&str> = SECTIONS.iter().map(|s| s.id).collect();
        ids.sort();
        ids.dedup();
        assert_eq!(ids.len(), SECTIONS.len(), "Section IDs must be unique");
    }

    #[test]
    fn topics_have_unique_ids() {
        let mut ids: Vec<&str> = TOPICS.iter().map(|t| t.id).collect();
        ids.sort();
        ids.dedup();
        assert_eq!(ids.len(), TOPICS.len(), "Topic IDs must be unique");
    }

    #[test]
    fn topics_reference_valid_sections() {
        let section_ids: Vec<&str> = SECTIONS.iter().map(|s| s.id).collect();
        for topic in TOPICS {
            assert!(
                section_ids.contains(&topic.section_id),
                "Topic '{}' references invalid section '{}'",
                topic.id,
                topic.section_id
            );
        }
    }

    #[test]
    fn dependencies_reference_valid_topics() {
        let topic_ids: Vec<&str> = TOPICS.iter().map(|t| t.id).collect();
        for dep in DEPENDENCIES {
            assert!(
                topic_ids.contains(&dep.from),
                "Dependency 'from' references invalid topic '{}'",
                dep.from
            );
            assert!(
                topic_ids.contains(&dep.to),
                "Dependency 'to' references invalid topic '{}'",
                dep.to
            );
        }
    }

    #[test]
    fn has_cross_section_dependencies() {
        let has_cross = DEPENDENCIES.iter().any(|dep| {
            let from_section = TOPICS
                .iter()
                .find(|t| t.id == dep.from)
                .map(|t| t.section_id);
            let to_section = TOPICS.iter().find(|t| t.id == dep.to).map(|t| t.section_id);
            from_section != to_section
        });
        assert!(has_cross, "Must have at least one cross-section dependency");
    }

    #[test]
    fn all_columns_represented() {
        let has_left = SECTIONS.iter().any(|s| s.column == Column::Left);
        let has_center = SECTIONS.iter().any(|s| s.column == Column::Center);
        let has_right = SECTIONS.iter().any(|s| s.column == Column::Right);
        let has_full = SECTIONS.iter().any(|s| s.column == Column::Full);

        assert!(has_left, "Must have Left column sections");
        assert!(has_center, "Must have Center column sections");
        assert!(has_right, "Must have Right column sections");
        assert!(has_full, "Must have Full width sections");
    }

    #[test]
    fn sections_count() {
        assert_eq!(SECTIONS.len(), 16, "Should have 16 sections");
    }

    #[test]
    fn topics_count() {
        assert!(TOPICS.len() >= 55, "Should have at least 55 topics");
    }
}
