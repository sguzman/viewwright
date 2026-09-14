# M6 — Implementation Boundary

M6 is a diagnostic/projection milestone. It does not expand ViewWright's authoring language.

## In scope

- backend-independent sRGB relative-luminance calculation
- backend-independent contrast-ratio calculation
- detection of semantic surfaces actually used by the screen
- deterministic visual-audit metrics
- advisory findings for:
  - insufficient foreground/background contrast
  - weak semantic surface separation
  - compressed very-dark structural surface ranges
- deterministic textual audit summary
- small preview-host integration for audit visibility
- a separate less-compressed dark Reader pressure specimen
- tests preserving M0–M5 behavior

## Model/projection boundary

The canonical model continues to own authored palette roles and semantic region surface roles.

The visual audit owns only derived analysis.

It must not:

- mutate `ResolvedVisual`
- choose replacement colors
- feed corrected values into egui automatically
- become source authority

A dedicated crate such as `viewwright-visual-audit` is appropriate if that keeps the dependency direction clean, but exact crate organization is an implementation decision.

## Warning severity

M6 findings are advisory warnings, not blueprint validation errors.

The audit should be able to report zero or more findings while blueprint resolution still succeeds normally.

## Basic text guardrail

A 4.5:1 contrast warning threshold is acceptable as an initial general readability lint for semantic text roles.

Do not claim full accessibility/WCAG conformance from this threshold alone.

## Surface separation heuristic

M6 may use an initial approximate 1.15:1 contrast-ratio threshold to flag canvas/panel/raised roles that are nearly indistinguishable.

This is a ViewWright design heuristic, not an external accessibility standard.

The warning should identify which surface pair is compressed and report the measured ratio.

## Very-dark compressed range heuristic

The implementation may combine:

- low maximum luminance among used structural surfaces
- narrow luminance span among those surfaces

into one advisory finding.

The exact constants should be explicit, tested, and documented rather than hidden magic values.

Do not classify all dark themes as erroneous.

## Transparent surfaces

`transparent` does not introduce its own palette color.

For audit purposes, treat a transparent major region as inheriting the authored screen/root canvas unless a more precise existing semantic parent surface is trivially available.

Do not build a generalized compositing engine for M6.

## Preview host

Audit information belongs to preview-host chrome, not inside authored regions.

Keep host integration small and legible.

Do not let diagnostic controls contaminate screenshot judgments of the authored screen.

## Pressure palette

The M6 pressure palette should remain dark but materially raise structural luminance and separation compared with the accepted M5 Reader.

This is a comparison instrument, not automatic migration.

Do not change the canonical visual Reader palette until human QA explicitly accepts a new direction.

## Runtime discipline

Audit computation is pure and lightweight.

It may be calculated after resolution and when the selected specimen changes.

Do not add file I/O, network I/O, image analysis, or other heavy work to the UI/render thread.

## Stop rather than widen

If implementation seems to require:

- accessibility role ontology
- per-element foreground/background reconstruction
- color-vision simulation
- automatic theme repair
- shared theme inheritance
- image/screenshot analysis

stop and report that pressure instead of silently implementing those systems.