# M6 — Visual Legibility Audit Acceptance

M6 is accepted when ViewWright can explain measurable visual-legibility pressure from the resolved visual model without turning those diagnostics into a new styling language or a hard validation gate.

## Required audit capability

Provide a backend-independent visual-audit projection over `ResolvedBlueprint` / `ResolvedVisual` that reports at least:

- relative luminance for resolved palette roles
- text contrast against semantic surfaces used by the screen
- muted-text contrast against semantic surfaces used by the screen
- accent contrast against semantic surfaces used by the screen
- pairwise canvas/panel/raised surface separation where those roles are used
- a non-fatal warning for very compressed surface separation
- a non-fatal warning for a structurally very-dark compressed surface range

The audit must not contain egui types.

## Determinism

For a given resolved blueprint, audit output must be deterministic.

Metrics should be stable enough for regression tests with reasonable floating-point tolerances.

A deterministic textual summary must also be available.

## Current Reader pressure

The current canonical visual Reader should produce an audit that makes the human-QA complaint legible in machine-readable terms.

In particular, the audit should distinguish:

- strong foreground text contrast
- extremely low structural-surface luminance
- weak canvas/panel/raised separation

The audit must not misdiagnose the current Reader as merely a text-contrast failure.

## Warning semantics

Visual-audit warnings are advisory.

They must not cause otherwise-valid TOML to fail parsing/resolution.

They must not silently mutate authored colors or renderer output.

## Preview-host behavior

The preview host should expose the audit outside the authored specimen.

Human QA must be able to see that the current visual Reader has audit warnings while the authored screen itself remains uncontaminated by diagnostic UI.

A compact summary plus readable details is sufficient.

## Pressure palette

Add a separate M6 visual Reader pressure specimen with a less-compressed dark palette.

The pressure specimen should:

- preserve the same semantic structure and M5 fixture content
- remain recognizably dark
- materially increase canvas/panel/raised luminance and separation
- preserve strong foreground readability
- produce fewer compressed-darkness/surface-separation warnings than the canonical M5 palette

Do not migrate the canonical visual Reader palette merely because the pressure specimen exists.

Human QA decides whether the pressure palette should later become canonical.

## Tests

Add meaningful tests covering at least:

- known sRGB relative-luminance values
- known contrast-ratio values
- audit determinism
- semantic surface-role usage detection
- current visual Reader produces compressed-surface warnings
- current visual Reader still reports strong primary text contrast
- M6 pressure Reader clears or materially reduces compressed-surface warnings
- audit warnings do not fail blueprint resolution
- Project Browser remains valid
- Dependency Workbench remains valid
- structural Reader remains valid
- M4 layout tests remain valid
- M5 fixture tests remain valid

## Runtime QA

Human QA should compare:

- canonical `reader_workspace_visual`
- M6 pressure visual Reader

and judge:

- absolute darkness
- surface separation
- text readability
- whether the screen still feels quiet / focused / reading-first
- whether the lighter pressure palette becomes washed-out or overly bright

M6 does not prescribe the human answer.

## Worker hygiene

Any preview/process launched during QA must be closed before completion is reported unless explicitly requested otherwise.

## Acceptance boundary

M6 does not require full accessibility conformance, automatic color correction, theme inheritance, responsive behavior, new interaction semantics, or ViewWitness integration.