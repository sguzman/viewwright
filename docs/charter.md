# Project Charter

## Mission

ViewWright is an authoring system for interface intent.

Its central question is:

> **What interface should exist?**

The project exists because jumping directly from requirements into low-level GUI toolkit calls destroys design intent too early. ViewWright inserts an explicit, inspectable design representation between product thinking and toolkit implementation.

## Authority

ViewWright is **normative**. Its artifacts represent desired interface structure and intent.

ViewWitness is **descriptive**. Its artifacts represent observed runtime interface evidence.

Neither project should absorb the other.

```text
ViewWright                  ViewWitness
──────────                  ───────────
desired                     observed
normative                   descriptive
should                      is
authoring                   auditing
before implementation       after/during runtime
```

The future bridge is comparison, not merger.

## Canonical truth

The canonical authoring representation is initially TOML.

TOML is parsed into a typed semantic model. Everything downstream consumes that model rather than treating the TOML syntax itself as the ontology.

```text
TOML source
    ↓
parse
    ↓
validate
    ↓
resolve
    ↓
semantic blueprint
    ├── ASCII projection
    ├── concept specification
    ├── preview
    ├── egui renderer
    └── ViewWitness expectations
```

ASCII diagrams are useful and encouraged, but they are generated projections. They must never become a second independent source of truth.

## Design level

ViewWright operates above primitive widgets.

Bad canonical vocabulary:

```text
horizontal
label
button
space(7)
separator
```

Preferred canonical vocabulary:

```text
workspace
navigation
primary-content
inspector
collection
property-sheet
command-bar
preview-pane
```

Low-level renderers may realize these concepts using toolkit primitives, but backend mechanics should not unnecessarily leak into the authoring model.

## Owned concerns

ViewWright owns:

- screen identity and purpose
- semantic regions and elements
- composition and adjacency
- hierarchy and visual dominance
- density and spacing intent
- design tokens
- semantic sizing constraints
- interaction affordance declarations
- screen fixtures and representative states
- design avoidances
- ASCII projection
- renderer contracts
- initial egui projection
- concept-generation specifications
- eventual ViewWitness expectation export

## Non-goals

ViewWright is not:

- an application programming language
- an application-state framework
- a business-logic runtime
- a replacement for egui
- HTML/CSS with different punctuation
- a screenshot-testing system
- runtime UI observation
- GUI automation
- a general graphics engine
- a Figma clone

## Business logic boundary

Blueprint objects may bind to stable application identifiers, but they do not implement the corresponding application behavior.

Example:

```toml
[[element]]
id = "delete_project"
kind = "command"
action = "project.delete"
```

ViewWright may know that the command exists, where it belongs, how important it is, and what state enables it. The host application owns what `project.delete` actually does.

## Renderer boundary

A renderer consumes a resolved semantic blueprint and maps it to a concrete toolkit.

The renderer may make backend-specific decisions that do not belong in the canonical model. Backend limitations should be surfaced explicitly rather than silently deforming design intent.

The initial backend is egui because it is immediately useful to the project's owner and provides a constrained environment in which to learn what the semantic model actually needs.

## Threading rule

Rendering must remain lightweight and responsive.

The render/UI thread may:

- consume already-resolved interface state
- lay out and paint
- detect lightweight interactions
- enqueue work

It must not become the default home for parsing, expensive resolution, asset processing, I/O, or other heavy work.

## Governing principle

> **The author manipulates interface concepts. The renderer manipulates widgets.**
