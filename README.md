# ViewWright

**Design the interface before implementing the widgets.**

ViewWright is a declarative UI-authoring system for expressing what an interface **should be** before committing that design to a concrete GUI toolkit.

It exists to close the gap between product intent and implementation:

```text
requirements
    ↓
screen skeleton
    ↓
semantic UI blueprint
    ↓
visual intent
    ↓
projections (ASCII / concept specification / preview)
    ↓
renderer backend (initially egui)
    ↓
running interface
```

ViewWright is the normative counterpart to [ViewWitness](https://github.com/sguzman/viewwitness):

- **ViewWright specifies.** It describes the interface that should exist.
- **ViewWitness observes.** It describes the interface that actually exists.

They are separate projects with deliberately opposite authority. In time, ViewWright expectations and ViewWitness observations should be comparable without collapsing either model into the other.

## Core principles

1. **Author above the widget level.** ViewWright must describe interface concepts, composition, hierarchy, and intent—not merely serialize egui calls.
2. **TOML is canonical.** Human-readable declarative TOML is the initial source format.
3. **ASCII is a projection, not authority.** ASCII wireframes should be generated from the semantic model for rapid human comprehension.
4. **Visual exploration is first-class.** The model should be usable to derive concept-art specifications and visual previews, while keeping generated imagery non-authoritative.
5. **Backends are projections.** egui is the first renderer, not the ontology of the project.
6. **Screens should be previewable in isolation.** Real application startup must not be required merely to inspect a screen design.
7. **Intent survives implementation.** Design hierarchy, dominance, density, composition, and explicit avoidances should remain machine-readable rather than disappearing into widget code.
8. **No heavy work on the render thread.** Backends must keep rendering responsive; parsing, resolution, asset work, and other heavy operations belong off the UI/render thread.
9. **Preview tools clean up after themselves.** QA may launch windows or processes, but automated workers should close what they launch before declaring completion unless explicitly asked to leave it running.

## What ViewWright owns

ViewWright owns the UI blueprint schema, semantic UI vocabulary, composition primitives, design intent, design tokens, fixture descriptions, projections, backend contracts, the initial egui renderer, and eventually a bridge for exporting expectations to ViewWitness.

It does **not** own application business logic, arbitrary 2D/3D graphics, runtime UI observation, GUI automation, or persistence for the host application.

## Implemented foundation

M0 established the first executable vertical slice:

```text
TOML → typed / validated / resolved blueprint
     ├── semantic tree
     ├── ASCII
     └── egui preview
```

M1 added an explicit composition root, recursive nested compositions, fixed vertical sizing, cycle diagnostics, and a semantic document surface. The Project Browser and Reader Workspace resolve through the same backend-independent composition model.

M2 added semantic visual authoring:

- named color tokens
- compact type scale
- semantic region surface roles
- border and corner policy
- strict authored-field validation
- deterministic concept-specification projection
- egui consumption of resolved visual semantics
- scoped preview styling so authored screen visuals do not contaminate preview-host chrome

The visual Reader Workspace pressure test is now accepted. It demonstrated that a coherent authored visual profile can survive canonical TOML → resolution → concept specification + egui without becoming a CSS-like styling system.

## Running the current slice

The repository is a Cargo workspace. Blueprints are parsed, validated, and resolved before the preview enters its render loop:

    cargo test
    cargo run -p viewwright-preview

The preview currently includes the Project Browser, structural Reader Workspace, and visual Reader Workspace specimens.

## Current design work — M3 semantic element realization

The next pressure test focuses on the semantic element layer.

ViewWright can already place and style regions, but higher-level element kinds such as collections, property sheets, trees, commands, status surfaces, and document surfaces are still rendered with placeholder-like backend behavior. M3 will test whether a small amount of backend-independent element semantics can produce recognizable interface objects without collapsing into serialized widgets or application business logic.

M3 should be earned by a new specimen rather than by polishing the existing Reader Workspace indefinitely.

## Status

- M0 — accepted
- M1 — accepted
- M2 — accepted
- M3 — design pressure test in progress
