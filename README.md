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

## What ViewWright owns

ViewWright owns the UI blueprint schema, semantic UI vocabulary, composition primitives, design intent, design tokens, fixture descriptions, projections, backend contracts, the initial egui renderer, and eventually a bridge for exporting expectations to ViewWitness.

It does **not** own application business logic, arbitrary 2D/3D graphics, runtime UI observation, GUI automation, or persistence for the host application.

## M0 — Blueprint embryo

The first milestone is intentionally narrow. It should support:

- screens
- regions
- elements
- composition
- importance / visual hierarchy
- basic sizing
- basic spacing
- design intent
- validation

And produce three useful results:

```text
TOML → validated semantic model
TOML → ASCII projection
TOML → simple egui projection
```

M0 succeeds when one real application screen can be described declaratively, understood quickly through its ASCII projection, and rendered recognizably in egui without hand-writing the entire layout.

See [`docs/charter.md`](docs/charter.md) for project boundaries and [`examples/project-browser.toml`](examples/project-browser.toml) for the first design specimen.

## Running the M0 slice

The repository is a Cargo workspace. The example is parsed, validated, and
resolved before the preview enters its render loop:

    cargo test
    cargo run -p viewwright-preview

The preview prints the resolved semantic tree and generated ASCII projection
to stdout, then opens an isolated egui window. Its fixture bar selects the
named many_projects, selected_project, and empty fixtures.

## Status

M0 vertical slice implemented. The egui projection is intentionally a small,
recognizable renderer for the specimen, not a production theme or a general
widget/layout language.
