# Rust Roadmap

<div align="center">

![Rust](https://img.shields.io/badge/rust-%23000000.svg?style=for-the-badge&logo=rust&logoColor=white)
![Leptos](https://img.shields.io/badge/Leptos-CSR-orange?style=for-the-badge)
![WebAssembly](https://img.shields.io/badge/wasm-%23654FF0.svg?style=for-the-badge&logo=webassembly&logoColor=white)

**A high-fidelity, type-safe visualization of the official Rust Learning Roadmap, built with Rust and Leptos.**

[Features](#features) • [Tech Stack](#tech-stack) • [Getting Started](#getting-started) • [Architecture](#architecture)

</div>

---

## 📖 Introduction

**Rust Roadmap** is a modern web application designed to help developers navigate the vast ecosystem of the Rust programming language. Adhering to the visual structure of the official Rust roadmap PDF, this project brings that static resource to life as an interactive, breakdown-rich application.

Unlike generic roadmap tools, this project enforces **strict visual fidelity** (the "Fishbone" layout) and **explicit data modeling**, ensuring that the learning path is presented exactly as intended by the community, but with the power of a modern reactive web framework.

## ✨ Features

- **Fishbone Layout Engine**: A custom layout algorithm that respects the "spine and branch" structure of natural learning progressions.
- **Explicit Positioning**: Data-driven rendering where content placement (`Left`, `Right`, `Center`) is strictly defined, avoiding algorithmic guesswork.
- **Cyber-Neon Aesthetic**: A distinct visual theme featuring "Dark Matter" backgrounds and "Neon Rust" accents for a premium developer experience.
- **Atomic Content Modules**: Each section of the roadmap (e.g., "Introduction", "Async Rust") is an isolated, type-safe module, making the content easy to maintain and scale.
- **Pure Rust Frontend**: Built entirely in Rust using Leptos, compiling to highly optimized WebAssembly.

## 🛠 Tech Stack

- **Core**: [Rust](https://www.rust-lang.org/) (2024 Edition)
- **Frontend Framework**: [Leptos](https://leptos.dev/) (Client-Side Rendering)
- **Bundler & Dev Server**: [Trunk](https://trunkrs.dev/)
- **Styling**: SCSS / CSS Variables with a custom design system.
- **Rendering**: SVG-based dynamic components.

## 🚀 Getting Started

Follow these steps to get the project running locally on your machine.

### Prerequisites

Ensure you have the following installed:

1.  **Rust Toolchain**:
    ```bash
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
    ```
2.  **WebAssembly Target**:
    ```bash
    rustup target add wasm32-unknown-unknown
    ```
3.  **Trunk** (WASM bundler):
    ```bash
    cargo install trunk
    ```

### Installation

1.  **Clone the repository**:
    ```bash
    git clone https://github.com/your-username/rust-roadmap.git
    cd rust-roadmap
    ```

2.  **Start the development server**:
    ```bash
    trunk serve
    ```
    This will compile the project and open it at `http://127.0.0.1:8080`. The server supports hot-reloading (HMR).

## 🏗 Architecture

This project strictly follows a **Type-Safe Architecture** to ensure maintainability and correctness.

### Directory Structure

```text
src/
├── components/       # Reactive Leptos UI components (Nodes, Edges)
├── data/             # Content Layer (The "Source of Truth")
│   └── sections/     # Atomic modules for each roadmap section
├── layout/           # Layout Engine (Calculates coordinates)
├── models/           # Domain Entities (TopicType, Placement)
└── state/            # Global State Management
```

### Core Principles

1.  **Visual Fidelity**: The output must visually match the reference PDF.
2.  **Explicit Layout**: We do not use force-directed graphs. Positions are deterministic based on the `Placement` enum.
3.  **Type Safety**: Layout logic and content definitions are strongly typed, preventing runtime rendering errors.

## 🤝 Contributing

We welcome contributions! Please note that this project has strict architectural guidelines.

Before contributing, **you MUST read** standard [AGENTS.md](./AGENTS.md) file, which acts as the architectural contract for this repository. It defines the allowed patterns, color palettes, and data structures.

---

<div align="center">

Made with ❤️ and 🦀 by the Rust Community

</div>
