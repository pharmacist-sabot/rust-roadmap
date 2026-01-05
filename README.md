<div align="center">

# Rust Roadmap

![Rust](https://img.shields.io/badge/rust-%23000000.svg?style=for-the-badge&logo=rust&logoColor=white)
![Leptos](https://img.shields.io/badge/Leptos-0.6-FF8937?style=for-the-badge&logo=leptos)
![WebAssembly](https://img.shields.io/badge/wasm-%23654FF0.svg?style=for-the-badge&logo=webassembly&logoColor=white)
![License](https://img.shields.io/badge/license-MIT-blue?style=for-the-badge)

**A high-fidelity, type-safe visualization of the Rust learning path, built entirely in Rust.**

</div>

---

## 🦀 Overview

**Rust Roadmap** is a modern, interactive implementation of the canonical Rust learning roadmap. Unlike static PDFs or generic diagramming tools, this project enforces a **strict architectural philosophy**: visual fidelity through explicit positioning and type safety through data-driven modeling.

Built with **Leptos (CSR)** and compiling to **WebAssembly**, it delivers a premium, reactive user experience directly in the browser while maintaining the rigor of systems programming.

---

## ✨ Features

- **Fishbone Layout Engine**: A custom deterministic layout algorithm that respects the "spine and branch" structure of the learning path. We avoid force-directed graph libraries in favor of explicit `Placement` logic (`Center`, `Left`, `Right`).
- **Terminal-Style Details View**: An immersive, CLI-inspired modal interface for browsing resources. Includes **Vim-style keybindings** (`j`, `k`, `Enter`) and simulated file system permissions.
- **Atomic Content Modules**: The roadmap content is decoupled from the UI. Sections (`s01_introduction`, `s02_language_basics`) are isolated, type-safe modules, making the data layer easy to extend and maintain.
- **High-Performance Rendering**: SVG-based rendering powered by Leptos's fine-grained reactivity. Zero hydration overhead.
- **Cyber-Neon Aesthetic**: A custom design system featuring "Dark Matter" backgrounds, "Neon Rust" accents, and the `JetBrains Mono` typeface for a premium developer experience.

---

## 🏗 Architecture

The project is architected around **Separation of Concerns** and **Type Safety**. The UI is a pure function of the state, and the layout is a pure function of the data.

### Directory Structure

```text
src/
├── components/
│   ├── roadmap/       # Core visualization logic (Diagram, Node, Edge)
│   │   └── detail_view.rs  # The interactive Terminal modal
│   └── ui/            # Shared UI components (Navbar, Footer)
├── data/              # The "Source of Truth"
│   └── sections/      # Modular content definitions (Introduction, Basics, etc.)
├── layout/            # Layout Engine (Deterministic coordinate calculation)
├── models/            # Domain Entities (TopicType, Placement, BadgeKind)
├── routes/            # Application routing
├── state/             # Global reactive state management
└── utils/             # Helper functions
```

### Core Principles

1.  **Explicit Data Modeling**: Nodes are not rendered by graph algorithms guessing positions. Coordinates are determined by explicit `Placement` enums, ensuring 1:1 fidelity with the intended roadmap design.
2.  **Content Agnostic**: The rendering engine (`RoadmapDiagram`) knows nothing about Rust. It renders generic `Topic` entities. The specific content is injected via the `data/` layer.
3.  **Reactive State**: Leptos Signals handle the UI state (e.g., which topic is currently open in the Terminal), ensuring the UI stays in sync without manual DOM manipulation.

---

## 🛠 Tech Stack

- **Core Language**: Rust (Edition 2024)
- **Frontend Framework**: [Leptos](https://leptos.dev/) (Client-Side Rendering)
- **Reactivity**: Fine-grained signals via `leptos_reactive`
- **Bundler**: [Trunk](https://trunkrs.dev/) (WASM build tool)
- **Styling**: Tailwind CSS (CDN) + Custom CSS Variables
- **Rendering**: SVG (Scalable Vector Graphics)

---

## 🚀 Getting Started

This project uses standard Rust tooling. To run the development environment, ensure you have Rust and Trunk installed.

### Prerequisites

1.  **Install Rust** (if you haven't already):
    ```bash
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
    ```

2.  **Add the WebAssembly Target**:
    ```bash
    rustup target add wasm32-unknown-unknown
    ```

3.  **Install Trunk** (the WASM bundler):
    ```bash
    cargo install trunk
    ```

### Installation & Run

1.  **Clone the repository**:
    ```bash
    git clone https://github.com/pharmacist-sabot/rust-roadmap.git
    cd rust-roadmap
    ```

2.  **Start the Development Server**:
    ```bash
    trunk serve --open
    ```
    
    Trunk will compile your Rust code to WASM, generate the necessary assets, and serve the application at `http://127.0.0.1:8080`. It supports Hot Reloading for rapid development.

### Building for Production

```bash
trunk build --release
```
The optimized artifacts will be placed in the `dist/` directory, ready to be deployed to any static hosting provider (Netlify, Vercel, GitHub Pages).

---

## 🤝 Contributing

We appreciate contributions! Because we value strict architectural consistency, please ensure the following:

1.  **Type Safety**: New topics must be defined in the `src/data/sections/` directory using the provided `Topic` and `Dependency` structs.
2.  **Visual Fidelity**: When adding new sections, ensure the `Placement` logic respects the "Fishbone" spine (Main topics in `Center`, sub-topics on `Left` or `Right`).
3.  **Content Quality**: Resources provided in the `TopicContent` should be high-quality, preferably official documentation or widely-recognized community resources.

---

<div align="center">

Made with ❤️ and 🦀 by the Rust Community

</div>
```
