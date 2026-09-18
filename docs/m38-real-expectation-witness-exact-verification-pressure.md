# M38 — Real Expectation/Witness Exact Verification Pressure

## Problem

M33 through M37 now establish every stage required for exact semantic verification, but only in isolation:

```text
M33: ResolvedBlueprint + logical viewport
    -> ViewWrightExpectation

M34: ViewWright semantic identity
    -> exact AccessKit author_id

M35: ViewWrightExpectation + ViewWitness Witness
    -> exact ComparisonReport

M37: real ViewWright egui::FullOutput
    -> ViewWitness Witness
```

M37 explicitly identified the next pressure: compose those accepted stages and prove that a real ViewWright rendering exactly satisfies its own M33 expectation under M35.

No current test traverses the complete chain.

## Goal

For representative canonical ViewWright screens, build the intended M33 expectation and independently render the same resolved blueprint through the real egui backend, convert that real output through the pinned ViewWitness adapter, and compare the two typed values with M35.

The required composition is:

```text
ResolvedBlueprint + logical viewport
        ├──> build_expectation(...)
        │        -> ViewWrightExpectation
        │
        └──> viewwright_egui::show(...)
                 -> egui::FullOutput
                 -> viewwitness::witness_from_egui_output(...)
                 -> ViewWitness Witness

ViewWrightExpectation + Witness
        -> viewwright_compare::compare(...)
        -> ComparisonReport
        -> exact match
```

An exact match means:

- zero mismatches;
- zero evidence gaps;
- `ComparisonReport::is_exact_match() == true`.

## Representative proof set

M38 must prove the real chain on three existing accepted surfaces at 1440×900 logical pixels:

1. Project Browser / `many_projects`;
2. M31 overlay pressure / `palette_open`;
3. M32 overflow pressure / `long_document`.

These cases exercise ordinary split geometry, overlay geometry, and authored vertical-overflow structure without inventing new source pressure.

## One viewport authority

The expectation branch and observation branch must use the same explicit logical width and height.

The ViewWitness capture context should use scale factor 1.0 for this proof because M35 intentionally compares logical width/height and ignores scale factor.

Do not compensate for mismatches by modifying one branch's viewport independently.

## Fixture boundary

M33 expectations remain fixture-independent.

The renderer still needs a fixture to produce representative visible content. Fixture-local collection/tree IDs and payload nodes may appear in the observed tree, but M35 remains expectation-driven and ignores unrelated observed identities.

Do not add fixture content to M33 merely to make M38 convenient.

## Failure semantics

If the real pipeline produces a mismatch or evidence gap, M38 must surface that fact.

Do not:

- loosen M35 exact comparison;
- add tolerances;
- fabricate missing observed bounds;
- walk ancestors when M35 requires immediate parent identity;
- rewrite authored IDs;
- normalize geometry;
- inject compatibility nodes solely to satisfy comparison;
- change M33 expectation semantics.

A real failure is architectural evidence and must be reported.

## Responsive-layout sequencing

The existing responsive-layout roadmap remains intentionally unnumbered. Its own promotion condition requires concrete intentionally different layouts that the current single-topology model cannot represent.

M38 does not promote that roadmap. The verification chain is the currently earned pressure.

## Non-goals

M38 does not add:

- new TOML syntax;
- new resolved-model fields;
- expectation schema changes;
- comparator semantics changes;
- tolerance or fuzzy matching;
- a public verification API;
- a new runtime verification crate;
- preview verification controls;
- recurring per-frame verification;
- ViewWitness observer/network/server integration;
- external process orchestration;
- paint or raster evidence;
- screenshots as machine evidence;
- CI gating;
- responsive authoring;
- ViewWitness repository changes;
- M39 work.
