<div align="center">

# 🦀 Rust Roadmap

[![Rust](https://img.shields.io/badge/Rust-Edition_2024-000000?style=for-the-badge&logo=rust&logoColor=white)](https://www.rust-lang.org/)
[![Leptos](https://img.shields.io/badge/Leptos-0.6-FF8937?style=for-the-badge)](https://leptos.dev/)
[![WebAssembly](https://img.shields.io/badge/WebAssembly-654FF0?style=for-the-badge&logo=webassembly&logoColor=white)](https://webassembly.org/)
[![License](https://img.shields.io/badge/License-MIT-blue?style=for-the-badge)](LICENSE)
[![Deploy](https://img.shields.io/github/actions/workflow/status/pharmacist-sabot/rust-roadmap/deploy.yml?style=for-the-badge&label=Deploy)](https://github.com/pharmacist-sabot/rust-roadmap/actions)

**An interactive, type-safe visualization of the Rust learning path.**

[**View Live Demo →**](https://pharmacist-sabot.github.io/rust-roadmap/)

</div>

---

## ✨ Features

- **📖 Comprehensive Learning Path** — 27 curated sections covering Rust from fundamentals to advanced domains including WebAssembly, embedded systems, and game development.
- **🔍 Interactive Detail View** — Click any topic to reveal a slide-in drawer with descriptions and curated learning resources (Official docs, Books, Videos, Articles, and more).
- **🎯 Deterministic Layout** — Custom "Fishbone" positioning algorithm ensures pixel-perfect, consistent visualization across all devices.
- **⚡ Compile-Time Validation** — All topics, dependencies, and content are Rust structs verified at compile time. Invalid links or missing data break the build.
- **🎨 Premium Dark Theme** — Carefully designed CSS token system with orange/red accents inspired by Rust's brand identity.

---

## 🏗️ Architecture

This project enforces strict **separation of concerns** between content, layout logic, and rendering.

```text
src/
├── data/               # Content definitions (Source of Truth)
│   ├── mod.rs          # Aggregates all sections, topics, dependencies
│   └── sections/       # 27 modular section directories
│       └── s01_introduction/
│           ├── mod.rs      # Topics & Dependencies
│           └── content.rs  # Descriptions & Resources
├── models/             # Domain types (Topic, Section, Resource, etc.)
├── layout/             # Deterministic coordinate calculation
│   └── tree.rs         # "Fishbone" layout algorithm
├── components/         # Leptos UI components
│   ├── roadmap/        # Diagram, nodes, edges, detail drawer
│   └── ui/             # Header, footer, hero
├── state/              # Global reactive state (Leptos signals)
└── styles/             # CSS design system (theme tokens, components)
```

### Design Decisions

| Decision | Rationale |
|----------|-----------|
| **Explicit `Placement` enums** | Node coordinates are derived from `Center`, `Left`, or `Right` placement types rather than force-directed algorithms. This guarantees visual fidelity to the intended design and eliminates layout randomness. |
| **Data in Rust, not JSON** | Topics and dependencies are Rust `const` structs. The compiler validates references, preventing broken links or orphan topics that would silently fail with external data files. |
| **Section-based modularity** | Each learning section is an isolated module (`s01_introduction`, `s02_setup`, etc.) with its own topics, dependencies, and content. Adding new content cannot break existing sections. |
| **Fine-grained reactivity** | Leptos signals provide precise DOM updates. Only the changed nodes re-render, not the entire tree. |

---

## 🛠️ Tech Stack

| Technology | Purpose |
|------------|---------|
| **Rust** (Edition 2024) | Systems language with memory safety guarantees |
| **Leptos 0.6** | Fine-grained reactive framework for CSR |
| **WebAssembly** | Compile target for browser execution |
| **Trunk** | WASM bundler and development server |
| **Lightning CSS** | CSS transformation and minification |
| **GitHub Actions** | CI/CD for linting, testing, and GitHub Pages deployment |

---

## 🚀 Getting Started

### Prerequisites

Ensure you have the Rust toolchain installed. Then add the WebAssembly target and install Trunk:

```bash
# Add WASM target
rustup target add wasm32-unknown-unknown

# Install Trunk bundler
cargo install trunk
```

### Development

```bash
# Clone the repository
git clone https://github.com/pharmacist-sabot/rust-roadmap.git
cd rust-roadmap

# Start development server with hot reload
trunk serve --open
```

The application will be available at `http://127.0.0.1:8080`.

### Production Build

```bash
trunk build --release
```

Optimized artifacts are generated in the `dist/` directory.

---

## 📚 Content Structure

Each section follows a consistent module pattern:

```rust
// src/data/sections/s01_introduction/mod.rs

pub const SECTION_ID: &str = "intro_sec";

pub fn get_topics() -> Vec<Topic> {
    vec![
        Topic {
            id: "intro",
            title: "Introduction",
            section_id: SECTION_ID,
            level: Level::Beginner,
            topic_type: TopicType::Main,    // Center spine
            placement: Placement::Center,
            row: None,
        },
        Topic {
            id: "what_is_rust",
            title: "What is Rust?",
            section_id: SECTION_ID,
            level: Level::Beginner,
            topic_type: TopicType::Sub,     // Branch node
            placement: Placement::Right,
            row: None,
        },
        // ...
    ]
}

pub fn get_dependencies() -> Vec<Dependency> {
    vec![
        Dependency { from: "intro", to: "what_is_rust" },
        // ...
    ]
}
```

### Resource Badges

Content resources are categorized with semantic badges:

| Badge | Use Case |
|-------|----------|
| `Official` | Rust Book, std docs, rust-lang.org |
| `Book` | Digital or physical books |
| `Article` | Blog posts and tutorials |
| `Video` | YouTube, conference talks |
| `Course` | Structured learning series |
| `Interactive` | Rustlings, exercism.io |
| `Crate` | crates.io, docs.rs links |
| `OpenSource` | GitHub repositories |
| `Community` | Reddit, Discord, forums |
| `Podcast` | Audio content |
| `Newsletter` | This Week in Rust, etc. |

---

## 🤝 Contributing

Contributions are welcome! Please follow these guidelines:

### Adding a New Topic

1. Locate or create the appropriate section in `src/data/sections/`.
2. Add the `Topic` struct to `mod.rs` with correct `Placement`:
   - `Placement::Center` — Main spine topics only
   - `Placement::Left` or `Placement::Right` — Branch topics
3. Add `Dependency` entries connecting the new topic to existing ones.
4. Add `TopicContent` in `content.rs` with description and resources.
5. Run `cargo build` to verify compile-time validity.

### Code Quality

This project enforces strict quality checks via CI:

```bash
# Format check
cargo fmt --all -- --check

# Linting
cargo clippy --all-targets -- -D warnings

# Tests
cargo test --verbose
```

### Pull Request Process

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes with clear messages
4. Ensure all CI checks pass
5. Open a Pull Request with a description of your changes

---

## 📄 License

This project is licensed under the **MIT License**. See [LICENSE](LICENSE) for details.

---

<div align="center">

**Built with 🦀 by the Rust community**

[Report Bug](https://github.com/pharmacist-sabot/rust-roadmap/issues) · [Request Feature](https://github.com/pharmacist-sabot/rust-roadmap/issues)

</div>
