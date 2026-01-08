use crate::models::roadmap::{BadgeKind, Resource, TopicContent};

pub fn get_content(id: &str) -> Option<TopicContent> {
    match id {
        "basics" => Some(TopicContent {
            title: "Language Basics",
            description: "Fundamental concepts of Rust, including variables, types, and control flow.",
            resources: vec![Resource {
                label: "The Book - Ch 3",
                url: "https://doc.rust-lang.org/book/ch03-00-common-programming-concepts.html",
                badge: BadgeKind::Official,
            }],
        }),
        "syntax_group" => Some(TopicContent {
            title: "Syntax and Semantics",
            description: "Understanding Rust's syntax structure, keywords, and basic semantic rules.",
            resources: vec![Resource {
                label: "Rust Reference - Notation",
                url: "https://doc.rust-lang.org/reference/notation.html",
                badge: BadgeKind::Official,
            }],
        }),
        "vars" => Some(TopicContent {
            title: "Variables & Data Types",
            description: "Immutability by default, shadowing, basic scalar and compound types.",
            resources: vec![
                Resource {
                    label: "Variables and Mutability",
                    url: "https://doc.rust-lang.org/book/ch03-01-variables-and-mutability.html",
                    badge: BadgeKind::Official,
                },
                Resource {
                    label: "Data Types",
                    url: "https://doc.rust-lang.org/book/ch03-02-data-types.html",
                    badge: BadgeKind::Official,
                },
            ],
        }),
        "control_flow" => Some(TopicContent {
            title: "Control Flow",
            description: "if expressions, loops (loop, while, for), and matching patterns.",
            resources: vec![Resource {
                label: "Control Flow",
                url: "https://doc.rust-lang.org/book/ch03-05-control-flow.html",
                badge: BadgeKind::Official,
            }],
        }),
        _ => None,
    }
}
