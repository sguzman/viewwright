# M6 — Visual Legibility Audit

M2 made visual semantics canonical. M5 then made the Reader representative enough for honest human visual QA. That QA exposed a new failure mode: ViewWright can faithfully render an authored palette that is internally coherent yet still feels oppressive or visually crushed.

The current visual Reader is a useful example. Its foreground text contrast is strong, but its structural surfaces are all extremely close to black and only weakly separated from one another. The result is technically readable text inside an overall workspace that feels too dark and visually flat.

M6 adds a small backend-independent **visual audit projection** over the resolved blueprint.

The audit exists to make this kind of failure inspectable before or alongside backend rendering. It does not mutate the palette and it is not a complete accessibility certification system.

## Goal

Given a resolved visual blueprint, derive deterministic visual metrics and warnings for:

- semantic foreground/background contrast
- structural surface relative luminance
- structural surface separation
- compressed very-dark surface ranges

The pipeline becomes:

```text
canonical TOML
    ↓
resolved visual blueprint
    ├── concept specification
    ├── egui projection
    └── visual audit
          ↓
      metrics + warnings
```

## No new authoring syntax

M6 should not add another visual configuration language.

The audit consumes the visual semantics already authored in M2:

- `canvas`
- `surface`
- `surface_raised`
- `text`
- `text_muted`
- `accent`
- `border`
- semantic region surface roles

A later milestone may earn explicit author-controlled audit policy if real pressure demonstrates the need. M6 does not.

## Relative luminance

Use the standard sRGB-relative-luminance calculation for deterministic palette analysis.

Expose relative luminance for each resolved palette role.

This is a backend-independent property of the authored color values.

## Foreground contrast

Compute contrast ratios between semantic foreground colors and the structural surfaces on which they may appear.

At minimum inspect:

- `text` against used canvas/panel/raised surfaces
- `text_muted` against used canvas/panel/raised surfaces
- `accent` against used canvas/panel/raised surfaces

A basic readability warning may use a 4.5:1 text-contrast guardrail.

This is a practical lint rule, not a claim of complete WCAG conformance. ViewWright does not yet model enough typography/accessibility context to make that claim.

## Surface separation

Text contrast alone does not catch the current Reader failure.

The audit should also report pairwise structural-surface separation for semantic surfaces actually used by the screen:

- canvas ↔ panel
- panel ↔ raised
- canvas ↔ raised

Report both:

- luminance difference
- contrast ratio

M6 may emit a non-fatal warning when adjacent semantic surface roles are so close that their hierarchy is likely to disappear. A ratio around 1.15:1 is an acceptable initial heuristic threshold for this milestone.

This threshold is intentionally a design-lint heuristic, not an accessibility standard.

## Compressed dark-range warning

The Reader pressure also demonstrates that an entire structural palette may cluster near black while still maintaining excellent text contrast.

M6 should detect this separately.

A reasonable initial heuristic is to warn when:

- all used structural surfaces remain in a very low luminance range, and
- the surface tone span is also narrow.

For the current Reader palette, the canvas/panel/raised luminances are approximately:

- canvas: 0.006
- panel: 0.010
- raised: 0.016

This is the kind of range the audit should identify as "very dark / compressed" without treating it as invalid.

The warning should explain the measurable reason rather than simply saying "too dark".

## Warnings are not validation errors

A screen may intentionally use a near-black palette.

Therefore M6 audit findings are advisory.

They must not:

- make TOML fail resolution
- silently alter tokens
- automatically brighten colors
- override renderer behavior

Human/agent authorship remains authoritative.

## Deterministic textual projection

Provide a deterministic textual visual-audit summary suitable for:

- tests
- code review
- preview-host diagnostics
- human/agent iteration

A useful shape is approximately:

```text
Visual audit — reader_workspace_visual
surface luminance:
  canvas          0.006
  panel           0.010
  raised          0.016
foreground contrast:
  text/canvas     15.59:1
  muted/panel      6.08:1
  accent/raised    7.39:1
surface separation:
  canvas/panel     1.08:1  WARN compressed separation
  panel/raised     1.09:1  WARN compressed separation
warnings:
  - structural palette is clustered in a very-dark luminance range
```

Exact formatting is implementation-defined but must be stable enough for testing.

## Preview integration

The isolated preview host should expose visual-audit warnings without contaminating the authored specimen.

A compact host-chrome summary is sufficient, for example:

- `Visual audit: 3 warnings`
- a concise expandable/listed text summary

Do not paint audit warnings inside the authored screen itself.

## Pressure palette

M6 includes a separate Reader pressure palette that remains dark but opens the structural tone range substantially.

It is a test direction, not automatically accepted canonical design.

The human should compare the resulting preview against the existing Reader and decide whether the visual character remains quiet/reading-first while becoming easier to perceive.

## Explicit non-goals

M6 is not:

- a complete accessibility ontology
- WCAG certification
- automatic palette repair
- automatic theme generation
- light/dark theme inheritance
- color-blind simulation
- typography accessibility policy
- per-widget contrast rules
- ambient-light adaptation
- HDR/color-management work
- new arbitrary styling syntax
- responsive design
- ViewWitness integration

## Stop condition

Stop M6 once visual metrics and warnings are deterministic/backend-independent, the preview can surface them, the current Reader's compressed-darkness problem is diagnosable, a less-crushed dark pressure palette can be compared, and accepted M0–M5 behavior remains intact.