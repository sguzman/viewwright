# M21 — Implementation Boundary

M21 is a narrow layout-projection correction.

Expected implementation area: `crates/viewwright-layout` tests and helper logic. Model parsing/resolution should not need structural changes because source and resolved semantics already carry both fixed size and grow.

## Required boundary

- preserve M4 fixed-size accounting;
- include positive region `grow` in growth-weight totals even when the same-axis fixed size exists;
- keep main allocation as fixed base plus proportional growth share;
- preserve composition grow behavior;
- preserve padding, gaps, cross-axis fill, and root-driven recursion;
- preserve `LayoutPlan` shape and semantic-ID addressing.

## Explicitly excluded

Do not add validation forbidding fixed+grow. Do not add percentages, shrink, min/max, intrinsic measurement, overflow handling, responsive behavior, element-level geometry, renderer-specific hacks, or M22 scope.

No heavy work may move onto the render thread.