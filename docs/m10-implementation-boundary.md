# M10 — Implementation Boundary

M10 is a renderer-purity milestone.

## In scope

- remove automatic egui rendering of region ids;
- remove automatic egui rendering of region roles;
- make element-label rendering kind-aware instead of unconditional;
- ensure command labels are rendered once by the command control;
- retain authored labels for content-bearing non-command element kinds;
- retain semantic/debug and concept visibility of internal region semantics;
- preserve M0–M9 behavior.

## No new region-title syntax

Do not add `region.label`, `region.title`, or similar fields in M10.

A backend must not infer product-facing titles from technical ids or ontology roles. If visible region headings become necessary, earn an explicit authoring capability later.

## No generic chrome framework

Do not introduce a generic component/chrome DSL, slot system, title bars, toolbars, menus, breadcrumbs, or panel-header framework.

## Layout boundary

M4 geometry remains unchanged. Removing semantic labels may change the local occupied content inside a region, but must not change LayoutPlan rectangles, composition sizing, region sizing, or grow behavior.

## Visual/density boundary

Preserve M2/M6 visuals and M9 density policy. Do not compensate for removed labels with palette, typography, or density changes.

## Interaction/content boundary

Preserve M3/M5 fixtures, M7 command actions/states, and M8 collection presentation. Commands remain semantically identical; only duplicate chrome disappears.

## Unsupported element kinds

M10 does not need to solve dormant `Text` or `Preview` placeholder rendering unless required by an accepted canonical specimen. Do not widen scope to them opportunistically.

## Render-thread rule

This is lightweight rendering structure only. No parsing, I/O, business logic, or heavy work belongs in the frame path.