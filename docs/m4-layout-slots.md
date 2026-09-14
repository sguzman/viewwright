# M4 — Layout-Slot Fidelity

M0–M3 established semantic structure, nested composition, visual intent, and fixture-backed representative content. Runtime QA exposed the next place where authored intent can disappear: **allocated geometry**.

Both the Reader Workspace and Dependency Workbench currently calculate large growing areas but render most visible content in an intrinsic-sized cluster at the upper-left. The egui backend uses requested sizes as loose UI maxima rather than committed semantic layout slots, so `grow = 1`, fixed sidebars, fixed command/status strips, gaps, and padding do not survive projection reliably.

M4 makes major composition geometry explicit and testable without creating a general constraint system.

## Goal

Given a resolved blueprint and a concrete viewport size, produce a deterministic backend-independent layout plan for compositions and regions, then make egui honor that plan.

```text
resolved blueprint + viewport
        ↓
major layout-slot projection
        ↓
composition / region rectangles
        ↓
egui renders inside those exact slots
```

Elements remain semantic objects laid out by the backend **inside** their owning region. M4 does not calculate rectangles for every button, label, or collection item.

## Pressure from accepted specimens

### Reader Workspace

The root column contains:

- fixed-height app commands
- nested `reading_body`
- fixed-height transport

`reading_body` should own the remaining vertical space. Inside it, fixed-width library and inspector regions flank a growing reader region.

### Dependency Workbench

The root column contains:

- fixed-height commands
- nested `body`
- fixed-height status

`body` should own the remaining vertical space. Inside it, fixed filters and inspector regions flank a growing package region. The status slot should remain at the bottom of the authored workspace rather than following intrinsic content height.

The same structural failure appearing in both specimens is the schema/backend pressure that earns M4.

## Composition growth

Nested compositions are first-class composition children but currently cannot express growth. M4 adds an optional `grow` value to compositions:

```toml
[[composition]]
id = "body"
kind = "split"
axis = "horizontal"
children = ["filters", "packages", "inspector"]
gap = "md"
grow = 1
```

This is the only new source-level sizing capability required by the current pressure.

Do not add responsive breakpoints, percentages, min/max expressions, alignment languages, or general constraints.

## Layout-slot projection

M4 should introduce a small backend-independent layout projection, conceptually:

```text
layout(ResolvedBlueprint, viewport_width, viewport_height)
    → LayoutPlan
```

The plan should identify at least composition and region rectangles by stable semantic ID.

A rectangle is simple logical geometry:

- x
- y
- width
- height

No egui types belong in the layout model.

A dedicated crate such as `viewwright-layout` is appropriate if it keeps the model/backend boundary clean. Exact crate naming is an implementation choice.

## Allocation rules

For M4, allocation stays deliberately small.

For a composition's main axis:

1. inset the composition slot by authored padding on all four edges;
2. reserve authored gaps between siblings;
3. reserve fixed main-axis region sizes (`width` in horizontal composition, `height` in vertical composition);
4. distribute remaining main-axis space among children with positive `grow`, proportional to their grow weights;
5. a nested composition participates using its own `grow`;
6. children fill the composition's cross axis for M4.

A child with neither fixed main-axis size nor positive grow may use zero/intrinsic allocation only where the accepted specimens permit it; canonical pressure specimens should author growth explicitly when remaining-space ownership matters.

The layout plan must never depend on egui content measurement.

## Padding and gaps

Current `padding` semantics should become true geometric inset, not a single `ui.add_space` approximation.

`gap` is geometric space between sibling slots.

Padding and gaps are part of the backend-independent slot calculation.

## egui fidelity

The egui backend should consume the layout plan and render each major region/composition inside its assigned rectangle.

A region surface should visually occupy its slot rather than shrink to the intrinsic size of its labels/content.

The backend may still use normal immediate-mode layout for elements within the region.

Do not hardcode Reader Workspace or Dependency Workbench IDs.

## Runtime boundary

Layout-slot calculation is lightweight pure computation. It may occur when the preview viewport size changes or before rendering a frame as needed, but it must not trigger parsing, file I/O, network I/O, or other heavy work.

No heavy/blocking work moves onto the render thread.

## Explicit non-goals

M4 is not:

- a CSS box model
- a general constraint solver
- responsive breakpoints
- media queries
- arbitrary percentages
- min/max/clamp sizing language
- docking
- drag-resizing
- scroll layout
- element-level rectangle planning
- text measurement
- intrinsic widget measurement across backends
- animation
- runtime application layout state
- ViewWitness integration

## Stop condition

Stop M4 once major region/composition slots are deterministic and backend-independent for the accepted specimens, nested body compositions can explicitly grow, padding/gaps are geometrically honored, and egui visibly fills the allocated workspace rather than collapsing major regions to intrinsic content size.