use crate::models::{ResourceLink, ResourceType, TopicContent};

pub fn get_introduction_data() -> TopicContent {
    TopicContent {
        id: "intro".to_string(),
        title: "What is Rust?".to_string(),
        description: "Rust is a modern system programming language focused on performance, safety, and concurrency. It accomplishes these goals without having a garbage collector, making it a useful language for a number of use cases other languages aren't good at. Its syntax is similar to C++, but Rust offers better memory safety while maintaining high performance.".to_string(),
        links: vec![
            ResourceLink {
                kind: ResourceType::Official,
                title: "Rust? What is it?".to_string(),
                url: "https://www.rust-lang.org/".to_string(),
            },
            ResourceLink {
                kind: ResourceType::Official,
                title: "The Rust Programming Language Book".to_string(),
                url: "https://doc.rust-lang.org/book/".to_string(),
            },
            ResourceLink {
                kind: ResourceType::Article,
                title: "What is Rust and why is it so popular?".to_string(),
                url: "https://stackoverflow.blog/2020/01/20/what-is-rust-and-why-is-it-so-popular/".to_string(),
            },
            ResourceLink {
                kind: ResourceType::Video,
                title: "Rust in 100 Seconds".to_string(),
                url: "https://www.youtube.com/watch?v=5C_HPTJg5ek".to_string(),
            },
        ],
    }
}
