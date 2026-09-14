# M4 — Layout-Slot Acceptance

M4 is accepted when ViewWright can project deterministic major layout slots from the resolved blueprint and a viewport, and egui visibly honors those slots.

## Required source/model changes

- `composition.grow` is supported as a non-negative finite growth weight.
- Region growth remains supported.
- Invalid negative/non-finite growth is diagnosed for both regions and compositions.
- Resolved compositions carry typed growth intent.

No other new sizing language is required.

## Required layout projection

Given a viewport width/height, a backend-independent layout projection must produce stable rectangles for:

- compositions
- regions

The plan must be addressable by semantic IDs.

The plan must account for:

- root viewport extent
- true four-edge composition padding
- gaps between siblings
- fixed main-axis region width/height
- proportional grow distribution
- nested composition growth
- cross-axis fill

The projection must not depend on egui content measurement or egui types.

## Pressure assertions at 1440 × 900

Exact implementation rounding may differ by a pixel, but these relationships must hold.

### Visual Reader Workspace

- `workspace` fills the preview specimen viewport.
- `app_commands` retains its fixed height.
- `transport` retains its fixed height and sits at the bottom of the root inner slot.
- `reading_body` receives the remaining vertical space via `grow = 1`.
- `library` retains its fixed width.
- `inspector` retains its fixed width.
- `reader` receives the remaining horizontal space via region `grow = 1`.
- library, reader, and inspector occupy the full body height.

### Dependency Workbench

- `workspace` fills the preview specimen viewport.
- `commands` retains its fixed height.
- `status` retains its fixed height and sits at the bottom of the root inner slot.
- nested `body` receives the remaining vertical space via `grow = 1`.
- `filters` retains its fixed width.
- `inspector` retains its fixed width.
- `packages` receives the remaining horizontal space via region `grow = 1`.
- filters, packages, and inspector occupy the full body height.

### Project Browser

- navigation and inspector retain fixed widths.
- projects receives remaining horizontal width.
- accepted M0–M3 semantic/fixture behavior remains intact.

## egui requirements

- egui consumes the layout plan rather than independently recomputing major slot geometry.
- region surfaces fill their assigned region rectangles rather than shrink to intrinsic content bounds.
- nested compositions render recursively inside their assigned composition rectangles.
- element rendering remains backend-local within regions.
- no application-specific IDs are used to repair layout.

## Tests

Add deterministic tests for at least:

- composition `grow` parsing/resolution
- invalid composition grow
- invalid region grow if not already covered
- root layout-plan bounds
- fixed + growing horizontal allocation
- fixed + growing vertical allocation
- nested composition growth
- gap accounting
- four-edge padding accounting
- Reader Workspace slot relationships
- Dependency Workbench slot relationships
- Project Browser slot relationships
- existing model/fixture/visual tests remain passing

## Runtime QA

At the default desktop preview size, human QA should visibly confirm:

- major content no longer huddles at the upper-left;
- Reader Workspace body fills the vertical workspace between command/transport strips;
- Dependency Workbench body fills the vertical workspace between commands/status;
- side panels fill body height;
- primary growing content owns the remaining center area;
- bottom command/status strips are actually bottom-aligned by geometry.

## Worker hygiene

Any native preview/process launched for QA must be closed before completion is reported unless explicitly requested otherwise.

## Acceptance boundary

M4 does not require responsive breakpoints, drag-resizing, docking, min/max constraints, percentages, element-level layout plans, scrolling, ViewWitness integration, or interaction semantics.