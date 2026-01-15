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
        "what_is_rust" => Some(TopicContent {
            title: "What is Rust?",
            description: "Rust is a modern system programming language focused on performance, safety, and concurrency. It accomplishes these goals without having a garbage collector, making it a useful language for a number of use cases other languages aren’t good at. Its syntax is similar to C++, but Rust offers better memory safety while maintaining high performance.",
            resources: vec![
                Resource {
                    label: "Rust? What is it?",
                    url: "https://www.rust-lang.org/learn/get-started",
                    badge: BadgeKind::Official,
                },
                Resource {
                    label: "Rust Programming Language",
                    url: "https://www.rust-lang.org/",
                    badge: BadgeKind::Official,
                },
                Resource {
                    label: "What is Rust and why is it so popular?",
                    url: "https://stackoverflow.blog/2020/01/20/what-is-rust-and-why-is-it-so-popular/",
                    badge: BadgeKind::Article,
                },
                Resource {
                    label: "What is Rust?",
                    url: "https://www.youtube.com/watch?v=zF34dRivLOw", // Fireship: What is Rust?
                    badge: BadgeKind::Video,
                },
                Resource {
                    label: "Explore top posts about Rust",
                    url: "https://www.reddit.com/r/rust/top/?t=all",
                    badge: BadgeKind::Community,
                },
            ],
        }),
        "why_rust" => Some(TopicContent {
            title: "Why use Rust?",
            description: "Rust solves pain points present in many other languages, offering memory safety without garbage collection and explicit concurrency handling.",
            resources: vec![
                Resource {
                    label: "Why Rust?",
                    url: "https://www.rust-lang.org/",
                    badge: BadgeKind::Official,
                },
            ],
        }),
        "ecosystem" => Some(TopicContent {
            title: "Ecosystem & Use Cases",
            description: "Rust is used in WebAssembly, Systems Programming, CLI tools, Embedded devices, and much more.",
            resources: vec![
                Resource {
                    label: "Is Rust the Future of Programming?",
                    url: "https://blog.jetbrains.com/rust/2025/05/13/is-rust-the-future-of-programming/",
                    badge: BadgeKind::Article,
                },
                Resource {
                    label: "Seeking Rust Use Cases",
                    url: "https://www.reddit.com/r/rust/comments/1ckgj4t/seeking_rust_use_cases/",
                    badge: BadgeKind::Community,
                },
                Resource {
                    label: "Rust Programming Language Adoption and Usage Trends",
                    url: "https://yalantis.com/blog/rust-market-overview/",
                    badge: BadgeKind::Article,
                },
                Resource {
                    label: "Practical uses of Rust",
                    url: "https://users.rust-lang.org/t/practical-uses-of-rust/12734",
                    badge: BadgeKind::Community,
                },
                Resource {
                    label: "Top 12 Real-World Business Use Cases for Rust",
                    url: "https://serokell.io/blog/best-rust-in-use-cases",
                    badge: BadgeKind::Article,
                },
            ],
        }),
        "community" => Some(TopicContent {
            title: "Rust Community",
            description: "The Rust community is known for being welcoming and helpful.",
            resources: vec![
                Resource {
                    label: "Rust Community",
                    url: "https://www.rust-lang.org/community",
                    badge: BadgeKind::Official,
                },
            ],
        }),
        _ => None,
    }
}
