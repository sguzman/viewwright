# ViewWright project demo gallery

## Current direction

ViewWright is a filesystem-first UI design laboratory for multiple projects, not a single
application's prototype. The historic Lantern Leaf specimens remain available as regression
evidence; they are not the active product-design north star.

A demo is a regular ViewWright TOML blueprint with one or more representative fixture states.
A demo is **not** an emulator, chat service, calendar engine, or live business-logic integration.
Command affordances report actions to a host; they do not execute them inside ViewWright.

## Run

From the repository root:

    cargo run -p viewwright-preview

This discovers all `.toml` files recursively under `./demos`.

Point at files or directories from other projects without copying them into ViewWright:

    cargo run -p viewwright-preview -- --demos /path/to/ephemeris/ui
    cargo run -p viewwright-preview -- --demos /path/to/chatarium/ui --demos /path/to/starbyte/ui

Each path is scanned at startup. Restart the preview after editing or adding TOML files.
The preview shows the source path and exposes fixture-state selection for each loaded demo.
Invalid files are reported in stderr and in the gallery instead of preventing valid demos
from loading. Duplicate `screen.id` values across loaded roots are rejected.

The built-in demo library now deliberately uses three different application
compositions rather than one generic three-column dashboard shell:

- `demos/chatarium/session-observability.toml`: two-column conversation workspace,
  with an inline lower diagnostics zone for transport metrics and connection state.
  The transcript is the dominant surface; healthy/degraded fixture states are separate.
- `demos/ephemeris/calendar-workbench.toml`: horizontal five-day week board below
  date/calendar controls, plus a full-width selected-event details strip.
  Wednesday is visually emphasized; the busy/sparse week fixtures are illustrative.
  **This is not yet a true hour-scaled calendar grid**: that requires a new semantic
  temporal-layout primitive and real event geometry.
- `demos/starbyte/emulation-library.toml`: full-width game shelf with a system
  selector ribbon and a bottom details/cheats drawer. Ready/missing-cheats
  fixtures are separate. **This is not yet a cover-art gallery or emulator view**:
  the existing `adaptive_cards` presentation does not own game-cover/media semantics.

These separate shapes are regression expectations, not three interchangeable skins.
The gallery's Rust tests check their top-level and nested composition structures,
while headless ViewWitness tests cover each demo's rendered semantic identity.

Historical specimens remain available with `--demos specimens` or
`--demos examples`. They remain source and test fixtures, not default gallery content.

## Author a new project GUI

1. Create `demos/<project>/<screen>.toml` or an equivalent directory in the project's own repo.
2. Give `[screen].id` a stable, globally distinct identifier across selected demo roots.
3. Describe semantic regions, composition, elements, visual tokens, and design intent.
4. Add `[[fixture]]` states for realistic and adverse scenarios (empty, failed, loading,
   disconnected, dense, etc.).
5. Launch the gallery with `--demos` pointing to that directory. Select the demo and
   fixture in the gallery.

Use the existing `specimens/dependency-workbench.toml` or built-in demo files as
schema references. A gallery demo is self-contained; no host process must be running.

## Quality assurance

GitHub Actions runs repository checks on pushes, pull requests, and explicit manual dispatch:
formatting, Rust workspace tests (including real headless egui → ViewWitness exact checks),
all-targets check, Clippy, and rustdoc. Reports are uploaded as build artifacts.

This is baseline CI only. Full synthetic interaction/resizing/clipping exploration and an
offscreen visual regression harness must be added as separate automated tests before M44 is
accepted. A green baseline CI run is not proof that the full M44 stop condition passed.

Tests must remain headless and must not control an operator's desktop. The action runner
can test behavior and compare visual references; it cannot decide subjective design quality.

## Current language limits and next development priorities

The existing TOML schema represents collections, controls, documents, settings, statuses,
major and nested layout, and authored responsive variants. A calendar time grid, data plot,
timeline visualization, and rich media/gallery cards are **not** first-class widgets yet.
The three examples illustrate the existing primitives instead of pretending charts or
real runtime functionality exist.

Future capability should be promoted from pressure supplied by several actual project
demo packs, not by one old north-star application. Priorities include:

- a true project/screen/fixture catalog (beyond a flat searchable strip);
- canonical information-dashboard primitives such as metrics, charts, time grids, tables,
  and health indicators where existing collection/property semantics are inadequate;
- offscreen raster evidence and interaction-driven CI tests;
- responsive and reusable layout designs grounded in multiple project samples;
- richer assets, iconography, typography and readable design dossiers.

Do not substitute generated imagery for executable GUI structure or validation.
