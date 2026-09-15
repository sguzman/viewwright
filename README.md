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

## M8 — collection presentation fidelity

The accepted Project Browser authors two different collection presentations: navigation is a `list`, while projects are `adaptive_cards`. M8 resolves that field into typed collection-presentation semantics, validates compatibility/defaulting, exposes the distinction in semantic/concept projections, and gives egui separate list and wrapping adaptive-card renderers.

```text
authored collection presentation
    ↓
typed resolved presentation
    ↓
semantic / concept inspection
    ↓
backend honors list vs adaptive cards
```

Human QA accepted the canonical Project Browser projection after correcting the non-visual fallback to inherit the active egui theme instead of rendering over a transparent root.

See:

- [`docs/m8-collection-presentation.md`](docs/m8-collection-presentation.md)
- [`docs/m8-acceptance.md`](docs/m8-acceptance.md)
- [`docs/m8-schema-summary.md`](docs/m8-schema-summary.md)
- [`docs/m8-implementation-boundary.md`](docs/m8-implementation-boundary.md)
- [`docs/m8-stop-condition.md`](docs/m8-stop-condition.md)
- [`docs/m8-pressure-evidence.md`](docs/m8-pressure-evidence.md)
- [`docs/m8-acceptance-matrix.md`](docs/m8-acceptance-matrix.md)

## M9 — screen density fidelity

Accepted canonical screens author both `comfortable` and `dense` screen density. M9 resolves that field into typed screen semantics and projects it through a centralized egui micro-layout policy without turning density into a second layout system.

```text
authored screen density
    ↓
typed resolved density
    ↓
semantic / concept inspection
    ↓
backend-local micro-layout policy
```

Density remains limited to local rhythm inside M4-planned regions: region inset, local item/element spacing, control padding/minimum interaction height, and card-local compactness. It does not rescale major geometry, authored spacing tokens, typography, palette, fixtures, actions, or collection presentation.

Human A/B QA accepted the isolated pressure pair: Comfortable visibly preserves more breathing room while Dense is noticeably tighter with the same major geometry, typography, content, and presentation.

Pressure pair:

- [`specimens/density-pressure-comfortable.toml`](specimens/density-pressure-comfortable.toml)
- [`specimens/density-pressure-dense.toml`](specimens/density-pressure-dense.toml)

See:

- [`docs/m9-screen-density.md`](docs/m9-screen-density.md)
- [`docs/m9-acceptance.md`](docs/m9-acceptance.md)
- [`docs/m9-schema-summary.md`](docs/m9-schema-summary.md)
- [`docs/m9-implementation-boundary.md`](docs/m9-implementation-boundary.md)
- [`docs/m9-stop-condition.md`](docs/m9-stop-condition.md)
- [`docs/m9-pressure-evidence.md`](docs/m9-pressure-evidence.md)
- [`docs/m9-pressure-pair.md`](docs/m9-pressure-pair.md)
- [`docs/m9-acceptance-matrix.md`](docs/m9-acceptance-matrix.md)

## M10 — authored chrome fidelity

M10 removes backend-invented product chrome from the egui projection. Region ids and ontology roles remain available to semantic/debug and concept inspection instead of being painted into authored UI, while element labels are rendered according to their kind so commands own their labels exactly once.

```text
internal semantic metadata ──→ semantic / concept inspection

authored element label ──→ kind-appropriate visible presentation
```

Human QA accepted `reader_workspace_visual / reading`: region scaffolding was absent, command labels were not duplicated, meaningful authored content labels remained, and the accepted Reader palette/layout stayed intact. If visible region titles are needed later, they will require explicit authoring rather than inference from technical identifiers.

See:

- [`docs/m10-authored-chrome-fidelity.md`](docs/m10-authored-chrome-fidelity.md)
- [`docs/m10-acceptance.md`](docs/m10-acceptance.md)
- [`docs/m10-implementation-boundary.md`](docs/m10-implementation-boundary.md)
- [`docs/m10-pressure-evidence.md`](docs/m10-pressure-evidence.md)
- [`docs/m10-stop-condition.md`](docs/m10-stop-condition.md)

## M11 — search input fidelity

Canonical screens already author Search controls, but the egui backend previously gave their editable value frame lifetime by constructing a fresh empty string during every render. M11 makes the existing Search affordance truthfully editable across redraws without adding search execution or application state.

```text
resolved Search affordance + backend-local ephemeral state
    ↓
egui text input that survives redraws
```

Mutable query text remains outside canonical TOML, fixtures, and `ResolvedBlueprint`. The preview host owns a small explicit renderer state and clears/rebinds it when specimen or fixture selection changes. M11 does not filter collections, emit SearchChanged application events, persist query data, or generalize into form/application state management.

Human QA accepted `project_browser / many_projects`: a multi-character query remained visible across redraws while all project cards stayed present and no filtering or other search execution occurred.

See:

- [`docs/m11-search-input-fidelity.md`](docs/m11-search-input-fidelity.md)
- [`docs/m11-acceptance.md`](docs/m11-acceptance.md)
- [`docs/m11-implementation-boundary.md`](docs/m11-implementation-boundary.md)
- [`docs/m11-pressure-evidence.md`](docs/m11-pressure-evidence.md)
- [`docs/m11-state-boundary.md`](docs/m11-state-boundary.md)
- [`docs/m11-stop-condition.md`](docs/m11-stop-condition.md)
- [`docs/m11-acceptance-matrix.md`](docs/m11-acceptance-matrix.md)

## M12 — region role fidelity

Accepted blueprints use a stable semantic region-role vocabulary. M12 makes the existing `region.role` field typed and validated so ontology and renderer behavior cannot silently drift through misspellings.

```text
authored region.role
    ↓
validated typed RegionRole
    ↓
semantic / concept inspection + backend consumption
```

M12 supports only the already-observed accepted roles: `commands`, `controls`, `navigation`, `primary_content`, `inspector`, and `status`. It adds no new TOML syntax and no intended visible behavior change. Director audit accepted the typed resolution, deterministic semantic/concept projection, and typed egui command-region handling without requiring human screenshot QA.

See:

- [`docs/m12-region-role-fidelity.md`](docs/m12-region-role-fidelity.md)
- [`docs/m12-acceptance.md`](docs/m12-acceptance.md)
- [`docs/m12-schema-summary.md`](docs/m12-schema-summary.md)
- [`docs/m12-implementation-boundary.md`](docs/m12-implementation-boundary.md)
- [`docs/m12-pressure-evidence.md`](docs/m12-pressure-evidence.md)
- [`docs/m12-stop-condition.md`](docs/m12-stop-condition.md)
- [`docs/m12-acceptance-matrix.md`](docs/m12-acceptance-matrix.md)

## Status

- M0 — accepted
- M1 — accepted
- M2 — accepted
- M3 — accepted
- M4 — accepted
- M5 — accepted
- M6 — accepted
- M7 — accepted
- M8 — accepted
- M9 — accepted
- M10 — accepted
- M11 — accepted
- M12 — accepted
