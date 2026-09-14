# Architecture

## Pipeline

ViewWright should have a one-way authoring pipeline with explicit stages.

```text
Authoring source (TOML)
        ↓
      parse
        ↓
Parsed source model
        ↓
     validate
        ↓
Validated source model
        ↓
     resolve
        ↓
Resolved semantic blueprint
   ┌────┼──────────┬──────────────┐
   ↓    ↓          ↓              ↓
 tree  ASCII      egui       expectation export
```

Projection code must not become an alternate source of semantic truth.

## Source model

The source model mirrors what the author can express conveniently in TOML.

It may contain:

- token names rather than resolved values
- string IDs rather than direct references
- omitted values that inherit defaults
- friendly shorthand forms

The source model should remain close enough to the authored document that diagnostics can explain errors in author terms.

## Validation

Validation should detect at minimum:

- duplicate IDs
- references to missing regions/elements/tokens
- illegal composition cycles
- elements assigned to nonexistent regions
- invalid sizing combinations
- invalid enum values already not rejected during deserialization

Validation should report multiple useful diagnostics in one run where practical rather than fail at the first trivial problem.

## Resolution

Resolution converts authored convenience into explicit semantic structure.

Examples:

```text
"md" spacing token
        ↓
resolved logical spacing value

region = "projects"
        ↓
stable resolved RegionId/reference

importance = "primary"
        ↓
typed Importance::Primary
```

The resolved model is the contract consumed by projectors/backends.

## Projection contract

A projector:

1. accepts a resolved blueprint;
2. may derive backend-specific structures;
3. preserves stable semantic identity when feasible;
4. does not silently rewrite authorial semantics;
5. emits diagnostics when intent cannot be honored.

## ASCII

The ASCII projector is a spatial/textual interpretation for humans.

It should optimize for:

- major-region legibility
- hierarchy
- rough proportion
- recognizable representative elements
- stable output useful in diffs and chat

It should not pretend to encode pixel-perfect layout.

## egui

The egui backend is the first executable projection.

The renderer should translate semantic structures into a limited set of opinionated composition primitives. It should not require each blueprint to smuggle arbitrary egui code through escape hatches.

Application-specific behavior should enter through explicit bindings/adapters rather than through the layout schema.

## Preview host

The preview host loads a blueprint and fixture, resolves them outside recurring render work, and exposes the resolved state to the egui renderer.

At steady state, the render thread should primarily:

- render the current resolved blueprint
- consume already-prepared fixture state
- detect interactions
- enqueue meaningful work

Parsing and resolution should occur on load/reload, not every frame.

## Future ViewWitness bridge

The bridge should translate a subset of normative ViewWright intent into expectations that can be compared against ViewWitness observations.

Examples:

```text
ViewWright intent:
  dominant = project_collection
  inspector importance = secondary
  inspector preferred width = 320

ViewWitness observation:
  dominant = inspector
  inspector observed width = 487
```

The comparison layer can then report divergence without requiring ViewWitness to understand the complete ViewWright authoring language.

This bridge belongs at the boundary between projects, not inside their respective canonical models.
