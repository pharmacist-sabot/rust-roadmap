use crate::models::roadmap::{BadgeKind, Resource, TopicContent};

pub fn get_content(id: &str) -> Option<TopicContent> {
    match id {
        "intro" => Some(TopicContent {
            title: "Introduction",
            description: "Rust is a modern systems programming language developed by Graydon Hoare at
            Mozilla Research starting in 2010, achieving its first stable 1.0 release in 2015.
            It emphasizes performance matching C++ speeds, memory safety through a unique ownership
            model and borrow checker that prevents data races and null pointer issues at compile time,
            and concurrency without a garbage collector. With expressive features like pattern matching,
            traits, and zero-cost abstractions, Rust ensures reliability and efficiency, making it
            ideal for systems, web, embedded, and high-performance applications.",
            resources: vec![
                Resource {
                    label: "Rust Programming Language",
                    url: "https://www.rust-lang.org/",
                    badge: BadgeKind::Official,
                },
                Resource {
                    label: "Rust by Example",
                    url: "https://doc.rust-lang.org/rust-by-example/",
                    badge: BadgeKind::Official,
                },
                Resource {
                    label: "Rust Book",
                    url: "https://doc.rust-lang.org/book/",
                    badge: BadgeKind::Official,
                },
                Resource {
                    label: "Rust Book Interactive",
                    url: "https://rust-book.cs.brown.edu/",
                    badge: BadgeKind::OpenSource,
                },
            ],
        }),
        _ => None,
    }
}
