# M2 — Visual Authoring Acceptance

M2 implements the smallest visual-authoring layer justified by the visual Reader Workspace specimen.

The milestone is intentionally narrower than a full styling system.

## Goal

Make visual intent survive the same pipeline as structural intent:

```text
TOML
  ↓
validated / resolved blueprint
  ├── ASCII structure
  ├── concept specification
  └── egui preview with authored visual hierarchy
```

## Required source concepts

M2 should support:

- named color tokens using hex literals
- a compact named type scale
- one screen-local visual profile
- region `surface` role
- border policy
- corner token reference
- existing spacing, importance, design character, and avoidances

## Required resolved concepts

The resolved visual model should contain concrete values for:

- palette roles
- type-scale values
- corner radius
- border policy
- each region's semantic surface role

Renderers should not repeatedly parse hex strings or token names.

## Required validation

M2 should diagnose at least:

- malformed color literals
- missing referenced visual color tokens
- missing referenced corner token
- invalid surface role
- invalid border policy
- unknown authored fields rather than silently ignoring them

Existing structural validation must remain intact.

## Concept specification

Add a deterministic textual concept-specification projection derived from the resolved blueprint.

It must include at least:

- screen purpose
- structure / major composition
- design character
- avoidances
- dominant and supporting hierarchy
- resolved palette
- type scale
- region surface treatments
- important semantic elements

`specimens/reader-workspace-visual.concept.txt` is an illustrative target, not a byte-for-byte golden file unless implementation finds that useful.

The concept projector must not call an image-generation service.

## egui visual projection

The egui backend should materially consume the resolved visual model.

For the visual Reader Workspace, it should visibly distinguish:

- primary canvas
- supporting panels
- raised command/transport surface
- primary vs secondary vs tertiary hierarchy
- text vs muted text
- accent usage
- restrained border policy
- authored corner radius
- compact type hierarchy

The implementation should stop wrapping every semantic object in visually equal default `group` containers.

M2 does not require production polish, but it must demonstrate that changing authored visual tokens or semantic surface roles changes the resulting preview coherently.

## Backward compatibility

The structural Project Browser and Reader Workspace should remain usable.

A small explicit migration to add canonical visual defaults/profile data is acceptable if needed. Prefer explicit canonical source over hidden renderer defaults that erase authorship.

## Runtime rule

Parsing, validation, resolution, and concept-spec preparation must not become recurring heavy work on the egui render thread.

## Tests

Tests should cover at least:

- valid visual Reader Workspace resolution
- malformed color rejection
- missing visual token rejection
- invalid surface rejection
- invalid border policy rejection
- unknown field rejection
- concept specification contains resolved palette and hierarchy
- existing structural specimens still resolve

## Explicitly outside M2

- arbitrary per-widget style declarations
- selector systems
- reusable theme inheritance
- font file/family loading
- gradients and shadows
- animation
- responsive breakpoints
- generalized design systems
- image generation APIs
- image analysis / round-trip import
- ViewWitness integration

## Acceptance

M2 is accepted when the visual Reader Workspace is canonical TOML that resolves without ignored intent, produces a useful concept specification, and renders in egui with visibly authored hierarchy from the same resolved visual model.
