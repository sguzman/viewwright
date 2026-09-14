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
7. **Intent survives implementation.** Design hierarchy, dominance, density, composition, geometry, and explicit avoidances should remain machine-readable rather than disappearing into widget code.
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

M1 added an explicit composition root, recursive nested compositions, fixed vertical sizing, cycle diagnostics, and a semantic document surface.

M2 added semantic visual authoring:

- named color tokens
- compact type scale
- semantic region surface roles
- border and corner policy
- strict authored-field validation
- deterministic concept-specification projection
- egui consumption of resolved visual semantics
- scoped preview styling so authored screen visuals do not contaminate preview-host chrome

M3 added fixture-backed semantic preview content:

- collection items and authored selection
- ordered property-sheet values
- status text
- validation of fixture-content compatibility
- migration away from fixture-id substring heuristics for supported content families

The Project Browser, Reader Workspace, visual Reader Workspace, and Dependency Workbench now share the same semantic pipeline.

## Running the current slice

The repository is a Cargo workspace. Blueprints are parsed, validated, and resolved before the preview enters its render loop:

    cargo test
    cargo run -p viewwright-preview

## Current design work — M4 layout-slot fidelity

Runtime QA of both the Reader Workspace and Dependency Workbench exposed a repeated geometry failure: the semantic model allocates growing major areas, but egui still allows region surfaces to collapse to intrinsic content size and huddle at the upper-left.

M4 makes major composition geometry deterministic and backend-independent for a concrete viewport:

```text
resolved blueprint + viewport
        ↓
major layout-slot projection
        ↓
composition / region rectangles
        ↓
backend rendering inside exact slots
```

M4 also adds `composition.grow` so nested compositions can explicitly own remaining space when they are first-class children of another composition.

This is not a responsive-layout milestone and does not introduce breakpoints, percentages, min/max constraints, docking, or a general constraint solver.

See:

- [`docs/m4-layout-slots.md`](docs/m4-layout-slots.md)
- [`docs/m4-acceptance.md`](docs/m4-acceptance.md)
- [`docs/m4-schema-summary.md`](docs/m4-schema-summary.md)
- [`docs/m4-implementation-boundary.md`](docs/m4-implementation-boundary.md)
- [`docs/m4-stop-condition.md`](docs/m4-stop-condition.md)

## Status

- M0 — accepted
- M1 — accepted
- M2 — accepted
- M3 — accepted
- M4 — layout-slot pressure test formalized; implementation not yet started
