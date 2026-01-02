# AI Operating Contract for `rust-roadmap`

This document defines **mandatory rules, architectural constraints, and quality
standards** for any AI agent contributing to this repository.

This is not documentation.
This is a **binding contract**.

The goal of this project is to build a **Rust-first, type-safe web application**
that visualizes a **Rust learning roadmap**, matching the visual structure of the official Rust roadmap PDF.

---

## 1. Core Philosophy (Non-Negotiable)

- **Visual Fidelity:** The output must visually match the reference PDF (Fishbone layout).
- **Explicit Layout Control:** Position (Left/Right/Center) is defined in Data, not guessed by algorithms.
- **Modular Content:** Content is split into atomic modules per section.
- **Type Safety:** Stringly-typed logic is forbidden in core structures.

---

## 2. Technology Stack (MANDATORY)

- **Rust** (stable toolchain)
- **Leptos** (frontend framework, CSR)
- **SVG** (rendering)
- **Trunk** (bundler)

---

## 3. Authoritative Folder Structure

AI agents MUST adhere to this modular data structure to manage the 23 roadmap sections.

```text
src/
├── components/            # UI components (Leptos)
│   ├── roadmap/
│   │   ├── node.rs        # Renders based on TopicType (Main/Sub)
│   │   ├── edge.rs
│   │   └── diagram.rs
├── data/                  # Data Layer
│   ├── mod.rs             # Aggregates all sections
│   └── sections/          # CONTENT MODULES
│       ├── mod.rs
│       ├── s01_introduction.rs
│       ├── s02_language_basics.rs
│       ├── ...
│       └── s23_debugging.rs
├── layout/                # Layout Engine
│   ├── mod.rs
│   └── tree.rs            # Logic must respect `Placement` enum
├── models/                # Domain models
│   ├── mod.rs
│   └── roadmap.rs
```

---

## 4. Roadmap Data Model Rules

The data model MUST support explicit positioning to replicate the "Fishbone" diagram.

### 4.1 Required Enums
AI agents must use these specific enums in `src/models/roadmap.rs`:

```rust
pub enum TopicType {
    Main, // Yellow box (Central Spine)
    Sub,  // Beige box (Branches)
}

pub enum Placement {
    Center, // Aligned to the central axis
    Left,   // Branches out to the Left
    Right,  // Branches out to the Right
}
```

### 4.2 Content Separation Rules
- **One File Per Section:** Each of the 23 yellow boxes in the PDF corresponds to **one** Rust file in `src/data/sections/`.
- **Naming Convention:** `s{order}_{snake_case_title}.rs` (e.g., `s05_testing.rs`).
- **Aggregator:** `src/data/mod.rs` must publicly expose a function `get_all_topics()` that combines vectors from all sub-modules.

---

## 5. Layout Engine Responsibilities

The layout algorithm in `src/layout/tree.rs` MUST NOT try to "smartly" guess positions based on topology.
It MUST obey the `Placement` field.

- **Center:** Render on the central X-axis.
- **Left:** Render to the left of the axis. If multiple items are Left, stack them or flow them outwards based on index.
- **Right:** Render to the right of the axis.

---

## 6. Rendering Rules (Visuals)

The UI must distinguish between Topic Types:

- **TopicType::Main (Cyber Neon):**
  - Fill: `#111111` (Dark Matter)
  - Stroke: `#FF4400` (Neon Rust) + Glow Effect
  - Text: White, Bold, Glowing
- **TopicType::Sub (Dark Glass):**
  - Fill: `#1A1A1A` (Dark Surface)
  - Stroke: `#FF8800` (Amber Neon)
  - Text: Silver White, Normal

---

## 7. Quality Gates

1.  **Rustfmt:** `cargo fmt --check` must pass.
2.  **Clippy:** `cargo clippy` must pass with no warnings.
3.  **Compilation:** Code must compile.
4.  **No Hallucinated Imports:** Do not import modules that do not exist. Create the module file first.

---

## 8. Commit Discipline

When generating code for sections, use granular commits:
- `feat(data): add s01_introduction data`
- `feat(layout): implement explicit placement logic`
- `style(roadmap): update node colors to match PDF`

This file is the absolute source of truth.