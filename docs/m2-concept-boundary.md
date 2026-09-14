# Concept Art Boundary

ViewWright should support concept art without making generated imagery authoritative.

## Authority flow

```text
canonical TOML
    ↓
resolved semantic + visual model
    ↓
concept specification
    ↓
optional external image generation
    ↓
human visual review
    ↓
authored changes return to TOML
```

The image is never the canonical design document.

If a generated concept suggests a better visual direction, that decision must be translated back into ViewWright source before implementation treats it as intent.

## Why this matters

Without this boundary, visual decisions would drift into screenshots and prompts that the renderer cannot inspect, validate, diff, or reproduce.

ViewWright should instead make image generation one projection/exploration path from the same source that drives egui.

## What the concept specification owns

The concept specification should summarize resolved information useful to a visual generator or human designer:

- purpose and dominant task
- composition and hierarchy
- visual character and avoidances
- palette
- type scale
- semantic surfaces
- important interface objects

It may also derive concise natural-language guidance, provided that the derivation does not invent requirements absent from the blueprint.

## What it does not own

The concept specification does not define:

- exact pixel geometry beyond authored constraints
- image-generation model parameters
- image service credentials or API calls
- final renderer behavior
- hidden design decisions that exist nowhere in TOML

## Round-trip rule

A visual exploration may influence ViewWright, but only by explicit authored change:

```text
concept image
    ↓ human/agent judgment
"make the reader surface warmer and quieter"
    ↓
edit palette / surface / design intent in TOML
    ↓
new resolved model
    ↓
new concept + egui projections
```

This preserves a shared collaboration surface for humans and agents instead of making screenshots the only place where visual knowledge lives.
