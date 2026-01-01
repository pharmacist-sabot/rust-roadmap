# AI Operating Contract for `rust-roadmap`

This document defines **mandatory rules, architectural constraints, and quality
standards** for any AI agent contributing to this repository.

This is not documentation.
This is a **binding contract**.

The goal of this project is to build a **Rust-first, type-safe web application**
that visualizes a **Rust learning roadmap**, inspired by roadmap-style diagrams,
implemented using **Leptos + SVG**, with minimal to zero JavaScript.

This project is maintained at **senior Rust developer standards**.
AI agents must behave accordingly.

---

## 1. Core Philosophy (Non-Negotiable)

- Rust-first. Type safety is mandatory.
- Data-driven UI. No hardcoded visual logic.
- Deterministic behavior. Same input → same output.
- Separation of concerns at all times.
- Explicit > implicit.
- Readability > cleverness.
- Compile-time guarantees are preferred over runtime checks.

If a solution trades correctness, clarity, or maintainability for speed,
**it must be rejected**.

---

## 2. Technology Stack (MANDATORY)

AI agents MUST follow this stack unless explicitly instructed otherwise.

### Required
- Rust (stable toolchain)
- Leptos (frontend framework)
- SVG for roadmap rendering
- Trunk for WASM bundling

### Optional (Only with Justification)
- Axum (server-side / API)
- Serde (serialization / deserialization)
- WASM-bindgen (only if strictly required)

### Forbidden by Default
- React, Vue, Svelte, or other JS frameworks
- D3.js or JavaScript-based graph engines
- Inline JavaScript logic in HTML
- CSS-driven layout logic replacing Rust layout code

---

## 3. Project Intent

This repository aims to:

1. Model a **Rust learning roadmap** as structured, typed data
2. Represent **sections, topics, and dependencies** explicitly
3. Compute layout positions deterministically in Rust
4. Render the roadmap visually using SVG
5. Allow future extensions without architectural rewrites:
   - Multiple tracks (Beginner / Backend / Embedded / Systems)
   - Filtering and highlighting
   - Export to SVG / PDF
   - Versioned roadmap data

AI agents MUST design code that anticipates growth.

---

## 4. Authoritative Folder Structure

AI agents MUST respect and extend the following structure.

```text
src/
├── app.rs                 # Root Leptos component
├── main.rs                # Application entry point
├── lib.rs                 # Shared logic (client/server safe)
│   ├── routes/                # Page-level routing
│   │   ├── mod.rs
│   │   ├── home.rs
│   │   ├── roadmap.rs
│   │   └── about.rs
│   ├── components/            # UI components
│   │   ├── mod.rs
│   │   ├── ui/
│   │   │   ├── navbar.rs
│   │   │   └── footer.rs
│   │   └── roadmap/
│   │       ├── node.rs        # SVG node rendering
│   │       ├── edge.rs        # Dependency edges
│   │       └── diagram.rs     # Full roadmap renderer
│   ├── data/                  # Roadmap data definitions
│   │   ├── mod.rs
│   │   └── roadmap_data.rs
│   ├── models/                # Domain models (pure Rust)
│   │   ├── mod.rs
│   │   └── roadmap.rs
│   ├── layout/                # Layout & positioning algorithms
│   │   ├── mod.rs
│   │   └── tree.rs
│   ├── state/                 # Application & UI state
│   │   ├── mod.rs
│   │   └── roadmap_state.rs
│   ├── utils/                 # Shared helpers & utilities
│   │   ├── mod.rs
│   │   └── helpers.rs
```

AI agents MUST NOT:
- Mix layout logic with rendering
- Embed roadmap data inside UI components
- Collapse unrelated responsibilities into a single file

---

## 5. Roadmap Data Model Rules

The roadmap MUST be represented using **explicit Rust data structures**.

### Required Concepts
- Section (or Phase)
- Topic (node)
- Dependency (edge)
- Level or difficulty (enum)

### Conceptual Example

```rust
enum Level {
    Beginner,
    Intermediate,
    Advanced,
}

struct Section {
    id: &'static str,
    title: &'static str,
    order: u8,
}

struct Topic {
    id: &'static str,
    title: &'static str,
    section_id: &'static str,
    level: Level,
}

struct Dependency {
    from: &'static str,
    to: &'static str,
}
```

Rules:
- IDs must be stable and deterministic
- No stringly-typed logic inside UI components
- Prefer enums over free-form strings
- Data definitions MUST be independent from rendering

---

## 6. Layout Responsibilities

Layout computation MUST be:
- Deterministic
- Testable
- Independent from rendering

Rules:
- Layout logic lives in `src/layout/`
- SVG components receive precomputed positions
- No layout math inside Leptos view macros

If layout and rendering logic are mixed, the change must be rejected.

---

## 7. Rendering Rules (SVG)

- SVG is the primary rendering layer
- Nodes are rendered using `<rect>`, `<g>`, or `<foreignObject>`
- Edges use `<line>` or `<path>`
- Styling is done via CSS, not inline SVG attributes

Accessibility matters:
- Text must be readable
- Colors must have sufficient contrast
- Hover and focus states must be intentional

---

## 8. State Management

- Use Leptos signals (`create_signal`, `RwSignal`)
- Global roadmap state belongs in `state/`
- Components receive state via props

Do NOT:
- Use global mutable statics
- Store UI state inside domain models

---

## 9. Testing Expectations

AI agents SHOULD add tests when:
- Implementing layout algorithms
- Transforming roadmap data
- Writing non-trivial logic

Preferred:
- Unit tests for layout
- Deterministic tests for data transformations

---

## 10. Code Style & Discipline

- No unused imports
- No dead code
- No unexplained TODOs
- Clear naming over short naming
- Rustfmt-compliant code only

If the code would embarrass a senior Rust developer during review, do not commit it.

---

## 11. Automated Quality Gates (MANDATORY)

All AI-generated or AI-modified code MUST pass the following checks.
These checks define the meaning of “done”.

### 11.1 rustfmt
All Rust code MUST be formatted using `rustfmt`
Default stable formatting only

AI agents MUST assume:
`cargo fmt --check`
will be enforced.

### 11.2 clippy
All code MUST pass:
`cargo clippy --all-targets --all-features`
Clippy warnings are treated as errors

AI agents MUST proactively avoid:
- Needless clones
- Unnecessary allocations
- `unwrap()` / `expect()` in production code
- Blanket `#[allow(...)]` attributes

If clippy would complain, the code is unacceptable.

### 11.3 Build Integrity
Before suggesting changes, AI agents MUST ensure:
1. `cargo check`
2. `cargo fmt --check`
3. `cargo clippy --all-targets --all-features`

would succeed without warnings or errors.

### 11.4 Error Handling Discipline
- No `unwrap()` or `expect()` in production code
- Errors must be propagated using `Result`
- Or handled explicitly with meaningful context

Tests may use `unwrap()` if justified.

### 11.5 Unsafe Code Policy
`unsafe` is forbidden by default

Any use of unsafe requires:
1. Clear justification
2. A safety comment explaining invariants
3. Explicit approval

### 11.6 Lint Suppression Rules
`#[allow(...)]` is discouraged

If used, it MUST:
- Be narrowly scoped
- Include a justification comment

Global lint suppression is forbidden.

### 11.7 CI Awareness
AI agents must write code as if CI enforces:
- Formatting failures = build failures
- Clippy warnings = build failures
- Non-deterministic behavior = rejection

Failure to meet these quality gates indicates that the AI agent did not follow project rules and the output must be rejected.

---

## 12. Commit Discipline (For AI Suggestions)

When proposing commit messages, use this format:

- `feat(roadmap): add typed roadmap data model`
- `refactor(layout): extract deterministic tree positioning`
- `docs: update AGENTS.md with quality gates`

Vague messages are not acceptable.

---

## 13. Final Authority

This file is the source of truth.

If any instruction conflicts with:
- Convenience
- Speed
- External examples

This file wins.

We build this as Rust developers. Carefully. Correctly. Proudly.