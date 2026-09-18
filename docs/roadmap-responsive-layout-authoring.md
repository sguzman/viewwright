# Roadmap — Responsive Layout Authoring

ViewWright should eventually make resize behavior an authored part of the UI contract rather than allowing backend compression, widget defaults, or accidental geometry to decide what happens when available space changes.

This roadmap item is intentionally **not yet a numbered milestone**. It records product intent and design constraints so future implementation can be pressure-driven rather than improvised.

## v0.2 north-star pressure

Lantern Leaf's archived EPUB + TTS reader concept is now the explicit ViewWright v0.2 north star. Responsive authoring is therefore no longer merely an indefinite possibility: it is a required v0.2 capability.

The north-star concept currently supplies a strong wide desktop composition and the product requirement that controls, panes, and reading surfaces behave deliberately as space shrinks or expands. The next pressure work must author concrete `wide`, `compact`, and `narrow` outcomes rather than infer them from backend compression.

This does **not** make the illustrative syntax below canonical. The exact responsive grammar must still be earned from the Lantern Leaf pressure specimen and validated against the existing ViewWright identity, reachability, layout, expectation, and observation contracts.

## Motivation

Current specimens are often designed at generous desktop widths. That is useful for proving semantic structure, but window resizing is a first-class UI behavior: the same screen may need different geometry, ordering, density, or presentation when the available surface becomes narrower or shorter.

A ViewWright blueprint should eventually be able to say, explicitly and declaratively:

- what layout applies when enough space is available;
- what changes at narrower or shorter sizes;
- which dimensions become fixed, grow, collapse, move, or change topology;
- whether a region changes presentation at a breakpoint;
- what happens exactly at breakpoint boundaries;
- which behavior is intentional versus merely a backend fallback.

The goal is to avoid "physics or chaos" as resize policy.

## Product principle

**Responsive behavior is authored intent.**

The renderer should not invent a responsive strategy because a window became smaller. Backend natural compression may still occur inside an explicitly selected layout state, but major responsive changes should come from the blueprint.

Conceptually:

```text
resolved semantic screen
        +
available surface
        ↓
authored responsive rule selection
        ↓
resolved layout state / variant
        ↓
LayoutPlan
        ↓
backend projection
```

## Likely authoring model

The preferred direction is a small ViewWright-native responsive rule system rather than cloning the full CSS cascade.

The source should probably support:

1. **Named layout states / variants**
   - e.g. `wide`, `compact`, `narrow`;
   - semantic region/element identity should survive across variants where the object still exists;
   - variants should express deliberate geometry/topology differences without duplicating an entire screen unnecessarily.

2. **Conditions based on available space**
   - minimum / maximum width;
   - minimum / maximum height;
   - possibly aspect or container-local conditions later, but not until earned by pressure.

3. **Deterministic rule selection**
   - no ambiguous CSS-like cascade;
   - overlapping conditions must either have explicit precedence or be rejected;
   - exact boundary behavior must be testable.

4. **Constrained layout overrides**
   Candidate override surfaces include:
   - composition kind / axis where valid;
   - composition child ordering or alternate composition selection;
   - region width / height / grow;
   - density or collection presentation if future pressure proves those need responsive variation;
   - deliberate collapse/hide/move behavior only after reachability and fixture semantics are defined.

## Illustrative TOML only

The following is a design sketch, **not committed syntax**:

```toml
[responsive]
default = "wide"

[[responsive.rule]]
max_width = 899
use = "compact"

[[responsive.rule]]
min_width = 900
use = "wide"

[responsive.variant.compact.region.navigation]
width = "72px"

[responsive.variant.compact.region.inspector]
width = "220px"
```

A future milestone may choose a different concrete representation if it produces a cleaner typed model.

## Required semantic properties

Any eventual implementation should preserve these principles:

- TOML remains authoritative.
- Responsive selection happens before backend rendering, not as ad hoc egui logic.
- LayoutPlan remains backend-independent.
- All authored variants are validated, not only the one active at startup.
- Resize behavior is deterministic for the same available dimensions.
- Existing semantic IDs should remain stable across variants unless the author explicitly changes structure.
- Breakpoint boundaries are regression-testable.
- Existing M16/M18 composition ownership/cycle rules continue to apply to every authored topology.
- M29 dominant-target and M30 fixture-content reachability need explicit semantics for variants that omit or relocate regions.
- M31 overlay topology must remain valid under responsive planning rather than being special-cased in the renderer.
- No heavy parsing or source resolution occurs on the render thread; runtime selection among already-resolved responsive states should be cheap.

## Important unresolved design questions

These questions must be answered by future pressure before implementation is considered complete:

- Are responsive conditions evaluated only against the root viewport, or can nested compositions become responsive containers later?
- Is a responsive state a sparse override of a base layout or a separately resolved topology referencing the same semantic objects?
- May a region disappear entirely at a breakpoint, or must it move/collapse into another authored surface?
- If a fixture targets content in a region absent from the active responsive state, is that fixture invalid, dormant for that state, or required to author per-state content?
- Can `design.dominant` vary by responsive state?
- Can screen density or collection presentation vary responsively, or should this roadmap initially stay geometry-only?
- What is the exact conflict rule when multiple conditions match?
- How should min/max dimensions interact with impossible physical sizes and oversized fixed surfaces such as M31 overlays?

Do not answer these questions by accident in backend code.

## Pressure specimens to create before implementation

A future milestone should be driven by at least one narrow/wide pair for the same semantic screen. Useful candidates:

- Project Browser wide: navigation + project canvas + inspector in one horizontal workspace;
- Project Browser narrow: intentionally authored alternate arrangement rather than squeezed columns;
- Reader wide/narrow pair showing how navigation, document, and settings should rearrange;
- M31 overlay at reduced viewport width to establish what should happen when the authored floating surface approaches the available width.

Human QA should include live window resizing across exact breakpoint boundaries.

## Non-goals for the first responsive milestone

The first implementation should not become:

- a full CSS clone;
- arbitrary media-query expressions;
- DOM-style cascade/specificity;
- animation system;
- device sniffing;
- platform-specific layout profiles;
- arbitrary script callbacks;
- runtime business logic;
- general constraint solver;
- automatic AI-generated responsive behavior.

Start with explicit available-space conditions and a small deterministic set of layout changes.

## Promotion condition

Promote this roadmap item into a numbered milestone when a concrete screen has at least two intentionally different layouts whose transition cannot be represented honestly by the current single-topology model.

At that point the milestone should define exact source syntax, typed resolution, conflict rules, breakpoint semantics, LayoutPlan integration, projection behavior, preview tooling, regression boundaries, and human resize QA.