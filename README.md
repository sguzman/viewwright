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

M3 added fixture-backed semantic preview content for collections, property sheets, and status text, including migration away from fixture-id substring heuristics for those supported families.

M4 added deterministic backend-independent major layout slots:

- composition `grow`
- true four-edge composition padding
- sibling gap geometry
- fixed + proportional growing slots
- nested composition growth
- cross-axis fill
- egui rendering inside planned composition/region rectangles

M5 made the Reader fixture state representative and canonical:

- hierarchical tree nodes with parent relationships and authored selection
- plain document title + ordered paragraphs
- existing property/status fixture families for settings and reading state
- canonical loaded and empty Reader states
- removal of the remaining Reader document fixture-name heuristic
- semantic/debug summaries for the new fixture families

M6 added a backend-independent visual audit for palette luminance, foreground contrast, structural-surface separation, and compressed-darkness warnings. Human QA selected the less-compressed dark Reader palette, which is now canonical; the old crushed palette remains only as audit regression evidence.

The Project Browser, Reader Workspace, visual Reader Workspace, and Dependency Workbench now share the same semantic pipeline and layout projection.

## Running the current slice

The repository is a Cargo workspace. Blueprints are parsed, validated, and resolved before the preview enters its render loop:

    cargo test
    cargo run -p viewwright-preview

## M7 — command affordance contracts

Accepted screens now contain real command affordances such as Open, Play / pause, Voice, Speed, and Refresh. Command elements carry validated namespaced action identifiers, and the egui backend reports enabled activations to the preview host without executing application behavior.

M7 closes that semantic gap without making ViewWright an application runtime:

```text
command element
    ↓
stable action id
    ↓
resolved affordance contract
    ↓
backend reports activation
    ↓
host application may decide what to do
```

M7 also adds a tiny fixture-backed command enabled/disabled state for honest isolated previews. It does not add callbacks, action execution, condition expressions, event routing, navigation, or application state graphs.

The M7 pressure specimen remains separate from the ordinary selector as reference coverage for fixture-backed disabled commands:

- [`specimens/reader-workspace-visual-m7.toml`](specimens/reader-workspace-visual-m7.toml)

See:

- [`docs/m7-command-affordances.md`](docs/m7-command-affordances.md)
- [`docs/m7-acceptance.md`](docs/m7-acceptance.md)
- [`docs/m7-schema-summary.md`](docs/m7-schema-summary.md)
- [`docs/m7-implementation-boundary.md`](docs/m7-implementation-boundary.md)
- [`docs/m7-stop-condition.md`](docs/m7-stop-condition.md)

## Status

- M0 — accepted
- M1 — accepted
- M2 — accepted
- M3 — accepted
- M4 — accepted
- M5 — accepted
- M6 — accepted
- M7 — accepted
