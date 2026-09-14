# M9 — Screen Density Fidelity

ViewWright already authors screen-level density intent, and accepted canonical screens already disagree intentionally about it:

- Project Browser: `comfortable`
- visual Reader Workspace: `comfortable`
- Dependency Workbench: `dense`

Today `screen.density` survives only as an unchecked source string carried inside the resolved blueprint. The egui backend does not consume it. Local rendering rhythm instead comes from backend constants and ordinary egui defaults.

M9 makes the existing density word semantically truthful.

## Goal

Preserve density intent through:

```text
source `screen.density`
    ↓
validated typed density
    ↓
resolved screen semantics
    ├── semantic/debug projection
    ├── concept projection
    └── backend-local micro-layout policy
```

M9 adds no new source field.

## Initial density vocabulary

Support exactly:

- `comfortable`
- `dense`

Omitted density resolves to `comfortable`.

These are semantic density modes, not serialized egui spacing structs.

## Meaning

Density governs local interaction/content rhythm **inside** the major rectangles already planned by M4.

A backend may map density to local policy such as:

- region content inset;
- vertical item spacing;
- button/control padding;
- minimum interaction height;
- small element-to-element gaps;
- comparable backend-local micro-spacing.

`dense` should fit more information and controls into the same already-allocated region while remaining legible.

`comfortable` should retain more breathing room.

## What density does not own

Density must not become a hidden second layout or visual system.

It does not change:

- M4 region/composition rectangles;
- authored region widths/heights/grow;
- authored composition gap/padding token values;
- M2 typography sizes;
- palette or surface roles;
- fixture content;
- command semantics;
- collection presentation semantics.

In particular, do not implement density by globally scaling the blueprint.

## Canonical pressure

Accepted source already establishes meaningful intent:

- `project_browser` is comfortable;
- `reader_workspace_visual` is comfortable;
- `dependency_workbench` is dense and describes itself as `dense_but_legible`.

The current renderer has hardcoded/local defaults instead of a density policy, so those authored values do not reliably affect projection.

## A/B pressure specimens

M9 should include a tiny pair of pressure specimens with identical structure, fixture content, visual semantics, and viewport assumptions, differing only in `screen.density`.

Suggested ids:

- `density_pressure_comfortable`
- `density_pressure_dense`

The pair exists only to isolate the semantic variable for human QA. It must not introduce new schema.

The dense member should visibly tighten local rhythm without changing major slot geometry or font sizes.

## Governing principle

> A screen-level semantic authored by canonical TOML must not survive merely as documentation. If a backend can honor it, projection should make the distinction observable without smuggling in a second layout system.
