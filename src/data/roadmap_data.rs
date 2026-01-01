//! Complete roadmap data matching the PDF structure.

// Internal references within the lib can still use `crate::`
use crate::models::roadmap::{
    ColumnType, Dependency, LayoutZone, Level, Section, Topic, TopicType,
};

// Helper macro for cleaner data entry
macro_rules! topic {
    ($id:expr, $title:expr, $sec:expr, $lvl:expr, $type:expr) => {
        Topic {
            id: $id,
            title: $title,
            section_id: $sec,
            level: $lvl,
            topic_type: $type,
        }
    };
}

pub const SECTIONS: &[Section] = &[
    // --- CENTER SPINE ---
    Section {
        id: "intro_sec",
        title: "",
        order: 1,
        zone: LayoutZone::Header,
    },
    Section {
        id: "basics_sec",
        title: "",
        order: 2,
        zone: LayoutZone::Header,
    },
    Section {
        id: "error_sec",
        title: "",
        order: 3,
        zone: LayoutZone::Header,
    },
    Section {
        id: "modules_sec",
        title: "",
        order: 4,
        zone: LayoutZone::Header,
    },
    Section {
        id: "testing_sec",
        title: "",
        order: 5,
        zone: LayoutZone::Header,
    },
    Section {
        id: "traits_sec",
        title: "",
        order: 6,
        zone: LayoutZone::Header,
    },
    Section {
        id: "macros_sec",
        title: "",
        order: 7,
        zone: LayoutZone::Header,
    },
    Section {
        id: "web_sec",
        title: "",
        order: 8,
        zone: LayoutZone::Header,
    },
    Section {
        id: "async_sec",
        title: "Asynchronous Programming",
        order: 9,
        zone: LayoutZone::Header,
    }, // Yellow box, but visually wider
    // --- LEFT COLUMN ---
    Section {
        id: "syntax_group",
        title: "Syntax and Semantics",
        order: 1,
        zone: LayoutZone::Body(ColumnType::Left),
    },
    Section {
        id: "constructs_group",
        title: "Constructs",
        order: 2,
        zone: LayoutZone::Body(ColumnType::Left),
    },
    Section {
        id: "datastruct_group",
        title: "Data Structures",
        order: 3,
        zone: LayoutZone::Body(ColumnType::Left),
    },
    Section {
        id: "ownership_group",
        title: "Ownership System",
        order: 4,
        zone: LayoutZone::Body(ColumnType::Left),
    },
    Section {
        id: "advanced_group",
        title: "Advanced Topics",
        order: 5,
        zone: LayoutZone::Body(ColumnType::Left),
    },
    // --- RIGHT COLUMN ---
    Section {
        id: "intro_side",
        title: "",
        order: 1,
        zone: LayoutZone::Body(ColumnType::Right),
    },
    Section {
        id: "basics_side",
        title: "Environment Setup",
        order: 2,
        zone: LayoutZone::Body(ColumnType::Right),
    },
    Section {
        id: "error_side",
        title: "",
        order: 3,
        zone: LayoutZone::Body(ColumnType::Right),
    },
    Section {
        id: "modules_side",
        title: "",
        order: 4,
        zone: LayoutZone::Body(ColumnType::Right),
    },
    Section {
        id: "concurrency_group",
        title: "",
        order: 6,
        zone: LayoutZone::Body(ColumnType::Right),
    }, // Concurrency is yellow but on right
    Section {
        id: "traits_side",
        title: "",
        order: 7,
        zone: LayoutZone::Body(ColumnType::Right),
    },
    Section {
        id: "lifetimes_group",
        title: "Lifetimes & Borrow Checker",
        order: 8,
        zone: LayoutZone::Body(ColumnType::Right),
    }, // Yellow header/box
    Section {
        id: "eco_side",
        title: "",
        order: 9,
        zone: LayoutZone::Body(ColumnType::Right),
    },
    // --- BOTTOM / FULL WIDTH ---
    Section {
        id: "networking_group",
        title: "",
        order: 10,
        zone: LayoutZone::Footer,
    },
];

pub const TOPICS: &[Topic] = &[
    // === 1. INTRODUCTION ===
    topic!(
        "intro",
        "Introduction",
        "intro_sec",
        Level::Beginner,
        TopicType::Main
    ),
    // Right
    topic!(
        "what_is",
        "What is Rust?",
        "intro_side",
        Level::Beginner,
        TopicType::Sub
    ),
    topic!(
        "why_use",
        "Why use Rust?",
        "intro_side",
        Level::Beginner,
        TopicType::Sub
    ),
    // === 2. LANGUAGE BASICS ===
    topic!(
        "basics",
        "Language Basics",
        "basics_sec",
        Level::Beginner,
        TopicType::Main
    ),
    // Right (Setup)
    topic!(
        "install",
        "Installing Rust and Cargo",
        "basics_side",
        Level::Beginner,
        TopicType::Sub
    ),
    topic!(
        "ides",
        "IDEs and Rust Toolchains",
        "basics_side",
        Level::Beginner,
        TopicType::Sub
    ),
    topic!(
        "repl",
        "Rust REPL (Playground)",
        "basics_side",
        Level::Beginner,
        TopicType::Sub
    ),
    // Left (Syntax)
    topic!(
        "vars",
        "Variables, DataTypes",
        "syntax_group",
        Level::Beginner,
        TopicType::Sub
    ),
    topic!(
        "control",
        "Control Flow",
        "syntax_group",
        Level::Beginner,
        TopicType::Sub
    ),
    topic!(
        "funcs",
        "Functions & Methods",
        "syntax_group",
        Level::Beginner,
        TopicType::Sub
    ),
    topic!(
        "pattern",
        "Pattern Matching",
        "syntax_group",
        Level::Beginner,
        TopicType::Sub
    ),
    // === 3. ERROR HANDLING ===
    topic!(
        "error_main",
        "Error Handling",
        "error_sec",
        Level::Intermediate,
        TopicType::Main
    ),
    // Right
    topic!(
        "opt_res",
        "Option and Result",
        "error_side",
        Level::Intermediate,
        TopicType::Sub
    ),
    topic!(
        "prop_err",
        "Propagating Errors (?)",
        "error_side",
        Level::Intermediate,
        TopicType::Sub
    ),
    topic!(
        "cust_err",
        "Custom Error Types",
        "error_side",
        Level::Intermediate,
        TopicType::Sub
    ),
    // Left (Constructs)
    topic!(
        "enums",
        "Enums",
        "constructs_group",
        Level::Beginner,
        TopicType::Sub
    ),
    topic!(
        "traits",
        "Traits",
        "constructs_group",
        Level::Beginner,
        TopicType::Sub
    ),
    topic!(
        "structs",
        "Structs",
        "constructs_group",
        Level::Beginner,
        TopicType::Sub
    ),
    topic!(
        "impl_blk",
        "Impl Blocks",
        "constructs_group",
        Level::Beginner,
        TopicType::Sub
    ),
    // === 4. MODULES & CRATES ===
    topic!(
        "modules",
        "Modules & Crates",
        "modules_sec",
        Level::Intermediate,
        TopicType::Main
    ),
    // Right
    topic!(
        "code_org",
        "Code Organization",
        "modules_side",
        Level::Intermediate,
        TopicType::Sub
    ),
    topic!(
        "dep_mgmt",
        "Cargo Dependencies",
        "modules_side",
        Level::Intermediate,
        TopicType::Sub
    ),
    topic!(
        "pub_crate",
        "Publishing on Crates.io",
        "modules_side",
        Level::Intermediate,
        TopicType::Sub
    ),
    // Left (Data Structures - Partial List for space)
    topic!(
        "integers",
        "Integers",
        "datastruct_group",
        Level::Beginner,
        TopicType::Sub
    ),
    topic!(
        "floats",
        "Floats",
        "datastruct_group",
        Level::Beginner,
        TopicType::Sub
    ),
    topic!(
        "bool",
        "Boolean",
        "datastruct_group",
        Level::Beginner,
        TopicType::Sub
    ),
    topic!(
        "char",
        "Character",
        "datastruct_group",
        Level::Beginner,
        TopicType::Sub
    ),
    topic!(
        "string",
        "String",
        "datastruct_group",
        Level::Beginner,
        TopicType::Sub
    ),
    topic!(
        "tuple",
        "Tuple",
        "datastruct_group",
        Level::Beginner,
        TopicType::Sub
    ),
    topic!(
        "array",
        "Array",
        "datastruct_group",
        Level::Beginner,
        TopicType::Sub
    ),
    topic!(
        "vector",
        "Vector",
        "datastruct_group",
        Level::Beginner,
        TopicType::Sub
    ),
    topic!(
        "hashmap",
        "Hashmap",
        "datastruct_group",
        Level::Beginner,
        TopicType::Sub
    ),
    topic!(
        "hashset",
        "Hashset",
        "datastruct_group",
        Level::Beginner,
        TopicType::Sub
    ),
    // === 5. TESTING ===
    topic!(
        "testing",
        "Testing",
        "testing_sec",
        Level::Intermediate,
        TopicType::Main
    ),
    // Left (Ownership)
    topic!(
        "own_rule",
        "Ownership Rules",
        "ownership_group",
        Level::Intermediate,
        TopicType::Sub
    ),
    topic!(
        "borrow",
        "Borrowing & Slices",
        "ownership_group",
        Level::Intermediate,
        TopicType::Sub
    ),
    topic!(
        "stack_heap",
        "Stack vs Heap",
        "ownership_group",
        Level::Intermediate,
        TopicType::Sub
    ),
    // === 6. TRAITS & GENERICS ===
    topic!(
        "traits_gen",
        "Traits & Generics",
        "traits_sec",
        Level::Intermediate,
        TopicType::Main
    ),
    // Right (Concurrency - Yellow Box but on Right)
    topic!(
        "concurrency",
        "Concurrency",
        "concurrency_group",
        Level::Advanced,
        TopicType::Main
    ),
    topic!(
        "threads",
        "Threads & Channels",
        "concurrency_group",
        Level::Advanced,
        TopicType::Sub
    ),
    topic!(
        "atomic",
        "Atomic Ops",
        "concurrency_group",
        Level::Advanced,
        TopicType::Sub
    ),
    topic!(
        "futures",
        "Futures & Async",
        "concurrency_group",
        Level::Advanced,
        TopicType::Sub
    ),
    // Left (Advanced Topics)
    topic!(
        "btreemap",
        "BTreeMap",
        "advanced_group",
        Level::Advanced,
        TopicType::Sub
    ),
    topic!(
        "arc",
        "Arc",
        "advanced_group",
        Level::Advanced,
        TopicType::Sub
    ),
    topic!(
        "mutex",
        "Mutex",
        "advanced_group",
        Level::Advanced,
        TopicType::Sub
    ),
    // === 7. MACROS ===
    topic!(
        "macros",
        "Macros & Metaprogramming",
        "macros_sec",
        Level::Advanced,
        TopicType::Main
    ),
    // Right (Lifetimes - Yellow Box)
    topic!(
        "lifetimes",
        "Lifetimes",
        "lifetimes_group",
        Level::Advanced,
        TopicType::Main
    ),
    topic!(
        "explicit_life",
        "Explicit Annotations",
        "lifetimes_group",
        Level::Advanced,
        TopicType::Sub
    ),
    topic!(
        "elision",
        "Lifetime Elision",
        "lifetimes_group",
        Level::Advanced,
        TopicType::Sub
    ),
    // === 8. WEB DEV ===
    topic!(
        "web_dev",
        "Web Development",
        "web_sec",
        Level::Advanced,
        TopicType::Main
    ),
    // Right side of Web Dev
    topic!("axum", "Axum", "eco_side", Level::Advanced, TopicType::Sub),
    topic!(
        "actix",
        "Actix",
        "eco_side",
        Level::Advanced,
        TopicType::Sub
    ),
    // === BOTTOM / ECOSYSTEM ===
    topic!(
        "async_prog",
        "Tokio / Async-std",
        "async_sec",
        Level::Advanced,
        TopicType::Sub
    ),
    topic!(
        "networking",
        "Networking",
        "networking_group",
        Level::Advanced,
        TopicType::Main
    ),
    topic!(
        "reqwest",
        "reqwest",
        "networking_group",
        Level::Advanced,
        TopicType::Sub
    ),
    topic!(
        "serde",
        "Serialization (Serde)",
        "networking_group",
        Level::Advanced,
        TopicType::Main
    ),
];

pub const DEPENDENCIES: &[Dependency] = &[
    // SPINE
    Dependency {
        from: "intro",
        to: "basics",
    },
    Dependency {
        from: "basics",
        to: "error_main",
    },
    Dependency {
        from: "error_main",
        to: "modules",
    },
    Dependency {
        from: "modules",
        to: "testing",
    },
    Dependency {
        from: "testing",
        to: "traits_gen",
    },
    Dependency {
        from: "traits_gen",
        to: "macros",
    },
    Dependency {
        from: "macros",
        to: "web_dev",
    },
    // BRANCHES
    Dependency {
        from: "intro",
        to: "what_is",
    },
    Dependency {
        from: "basics",
        to: "install",
    },
    Dependency {
        from: "basics",
        to: "vars",
    },
    Dependency {
        from: "error_main",
        to: "opt_res",
    },
    Dependency {
        from: "error_main",
        to: "enums",
    },
    Dependency {
        from: "modules",
        to: "code_org",
    },
    Dependency {
        from: "modules",
        to: "integers",
    },
    Dependency {
        from: "testing",
        to: "own_rule",
    },
    Dependency {
        from: "traits_gen",
        to: "concurrency",
    },
    Dependency {
        from: "macros",
        to: "lifetimes",
    },
];
