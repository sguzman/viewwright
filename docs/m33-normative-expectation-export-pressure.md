# M33 — Normative Expectation Export Pressure

ViewWright has always reserved an expectation-export branch in its architecture, but until now every implemented projection remained human-facing (semantic/debug, ASCII, concept, preview) or backend-facing (egui).

M33 earns the first machine-readable normative projection intended for later comparison with ViewWitness observations.

## Goal

Export a deterministic **ViewWright expectation document** from a resolved blueprint plus an explicit logical viewport size.

The document represents **intended** UI facts. It is not a ViewWitness `Witness`, not an observation, and not a comparison result.

## Epistemic boundary

ViewWitness explicitly distinguishes `Intended` source/specification claims from observed witnesses. A basic ViewWitness witness is testimony about what appeared, not what source intended.

Therefore M33 MUST NOT serialize ViewWright intent into the ViewWitness witness format or label intended geometry as observed.

The first bridge is one-way:

```text
ResolvedBlueprint + LayoutPlan
        ↓
ViewWrightExpectation
        ↓
deterministic YAML
```

A later milestone may compare that document against ViewWitness evidence.

## Initial exported scope

Top-level metadata:

- expectation format version;
- epistemic category = `intended`;
- screen authored ID;
- explicit logical viewport width/height;
- optional typed dominant target.

Reachable region expectations:

- authored region ID (`author_id`);
- role;
- importance;
- exact planned logical bounds from LayoutPlan;
- overflow policy.

Reachable element expectations:

- authored element ID (`author_id`);
- owning region authored ID;
- kind;
- importance;
- exact authored label;
- command action ID when one exists.

## Reachability

Only the screen rooted at `screen.root` belongs in the expectation document.

Unused declarations remain legal in ViewWright but MUST NOT be exported as things expected to appear on the rendered screen.

## Identity

Use ViewWright-authored IDs as explicit `author_id` identity.

Do not claim they are observed node IDs. Do not invent ViewWitness-local IDs. Future comparison may use author identity evidence when both sides honestly expose it.

## Geometry

Region geometry comes only from the backend-independent LayoutPlan at the caller-supplied logical viewport size.

M33 does not export element bounds because ViewWright does not currently plan element rectangles backend-independently.

Do not invent element geometry from egui or text measurement.

## Fixture boundary

M33 is structural and geometry-oriented. It does not export fixture payload content, selection, mutable search state, scroll offsets, or runtime command activation state.

Fixture-aware expectations may be added only under later pressure.

## Serialization

Provide deterministic YAML suitable for files, diffs, agents, and future cross-project tooling.

A dedicated ViewWright expectation Rust model should remain canonical; YAML is its serialization projection.

## Non-goals

M33 does not add:

- ViewWitness capture parsing;
- ViewWitness comparison/diffing;
- runtime observation;
- AccessKit instrumentation;
- author-ID injection into egui widgets;
- screenshot/raster expectations;
- fixture-content expectations;
- element geometry;
- tolerances;
- responsive variants;
- mutation/repair loops;
- changes to the ViewWitness repository;
- M34 work.
