# M32 — Vertical Region Overflow Pressure

M32 introduces one deliberately narrow overflow contract so content that exceeds a planned region does not depend on accidental backend clipping or escape behavior.

The pressure source is `specimens/reader-overflow-pressure.toml`: a long Reader document whose `reader` region authors `overflow = "scroll_y"`.

## Goal

Add a typed region-level overflow policy with exactly two initial values:

- `clip` — content is confined to the region content viewport and excess content is not interactable outside it;
- `scroll_y` — the region content viewport remains fixed while its content can scroll vertically inside it.

Omitted `region.overflow` resolves to `clip` so existing canonical screens preserve their current intended geometry and require no source migration.

## Why region-level

Regions already own planned geometry and group semantic elements. Overflow belongs to that geometry/content boundary rather than to fixture payloads or individual widgets.

M32 does not add element-level overflow.

## Pressure

The canonical Reader has a real document surface but only a few representative paragraphs. M5 explicitly deferred scrolling architecture. The M32 pressure specimen authors enough plain document content to exceed the Reader region at the standard preview size.

Without an authored overflow contract, a backend can clip, escape, grow, or otherwise improvise. That is contrary to ViewWright's rule that important visual behavior should survive as explicit intent.

## Relationship to responsive layout

M32 is not responsive layout.

Responsive layout chooses which geometry/topology should apply at a given available surface size. M32 defines what content does when it exceeds the region selected by that layout.

The responsive roadmap remains separate in `docs/roadmap-responsive-layout-authoring.md`.

## Runtime boundary

Parsing and validation remain outside the render loop. LayoutPlan geometry remains backend-independent and unchanged. The backend consumes a resolved overflow policy while rendering already-resolved region content.

## Non-goals

M32 does not add:

- horizontal scrolling;
- two-axis scrolling;
- pagination;
- virtualization;
- scroll-position TOML;
- persisted application scroll state;
- automatic scroll-to-selection;
- anchors or named document positions;
- scrollbar visual authoring;
- responsive breakpoints;
- element-level overflow;
- rich text or EPUB behavior;
- M33 work.
