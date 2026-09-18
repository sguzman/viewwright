# M38 — Implementation Plan

## 1. Preserve the accepted baseline

Start from the committed M38 authority head.

Read all M38 authority documents before editing.

Do not alter M33, M34, M35, M37, or ViewWitness contracts merely to obtain a passing integration test.

## 2. Compose at the existing renderer test boundary

Prefer `crates/viewwright-egui/tests/exact_verification.rs`.

Add only the dev dependencies necessary for that test boundary:

- `viewwright-expectation`;
- `viewwright-compare`.

Continue using the existing pinned ViewWitness dev dependency with only the `egui` feature.

## 3. Build one reusable proof helper

The helper should accept at least:

- source text;
- fixture ID;
- logical width;
- logical height.

For each invocation:

1. parse and resolve the source;
2. call `build_expectation`;
3. create an egui `Context`;
4. enable AccessKit;
5. render with `Context::run_ui` using the exact logical screen rectangle;
6. call `viewwright_egui::show`;
7. pass the real `FullOutput` directly to `witness_from_egui_output`;
8. validate the Witness;
9. call `viewwright_compare::compare`;
10. assert no mismatches;
11. assert no evidence gaps;
12. assert `is_exact_match()`.

Failure output should print the actual comparison report clearly.

## 4. Required cases

Run the helper for:

- `examples/project-browser.toml` / `many_projects` / 1440×900;
- `specimens/overlay-command-palette-pressure.toml` / `palette_open` / 1440×900;
- `specimens/reader-overflow-pressure.toml` / `long_document` / 1440×900.

## 5. Do not compensate

If any case fails:

- inspect the real mismatch/gap;
- do not add tolerance;
- do not change expected geometry;
- do not rewrite observed geometry;
- do not add fake author IDs;
- do not change comparator ownership rules;
- do not enable extra ViewWitness features.

If the accepted layers genuinely disagree, report the blocker.

## 6. Regression and dependency audit

Run the full workspace test/check set used by M37 plus focused M38 integration tests.

Inspect dependency features to confirm:

- ViewWitness egui remains confined to the renderer dev/test graph;
- `viewwright-compare` still uses ViewWitness model-only.

## 7. Publication

Update README:

- M37 accepted;
- M38 implementation complete, Director audit pending.

Comment Issue #41 with implementation evidence.

Leave Issue #41 open for Director audit.

Commit, push, synchronize, clean the tree, and stop before M39.
