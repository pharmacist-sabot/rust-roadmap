use crate::models::roadmap::{BadgeKind, Resource, TopicContent};

pub fn get_content(id: &str) -> Option<TopicContent> {
    match id {
        "intro" => Some(TopicContent {
            title: "Introduction",
            description: "Rust is a modern system programming language focused on performance, \
            safety, and concurrency. It accomplishes these goals without having a \
            garbage collector, making it a useful language for a number of use cases \
            other languages aren't good at. Its syntax is similar to C++, but Rust offers \
            better memory safety while maintaining high performance.",
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
                    badge: BadgeKind::OpenSource,
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
