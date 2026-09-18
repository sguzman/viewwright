# M39 — Implementation Plan

## 1. Sync and establish baseline

Fetch/pull `main`.

Start from the committed M39 authority head.

Verify no local divergence before editing.

## 2. Reconcile current-facing documentation

Update README:

- M38 accepted;
- describe the ViewWright/ViewWitness comparison bridge as implemented;
- describe M39 as v0.1 release readiness;
- preserve the root heading exactly `# 🟩 ViewWright`;
- do not turn README into a milestone diary beyond what already exists.

Update `docs/architecture.md`:

- retain the one-way authoring pipeline;
- describe expectation export, observed Witness conversion, and exact comparison as current architecture;
- preserve the boundary that ViewWright remains normative and ViewWitness descriptive.

Update `docs/charter.md`:

- remove obsolete future/eventual bridge wording;
- preserve authority separation and anti-merger principle.

Update `docs/model.md` minimally:

- add a conspicuous historical/non-authoritative notice;
- point readers to current code/README/accepted milestone docs;
- preserve M0-era text as design provenance unless a small clarification is necessary.

Do not rewrite accepted milestone authority docs.

## 3. Add release surfaces

Add root `CHANGELOG.md` with a `0.1.0` entry.

Summarize capability families rather than listing all 39 milestones individually.

Add `docs/releases/v0.1.0.md` with:

- release identity;
- what 0.1 ships;
- verification boundary;
- known intentional deferrals;
- note that responsive authoring remains roadmap Issue #33;
- note that this repository release does not promise crates.io publication/MSRV;
- tag-finalization step after Director acceptance.

Add standard MIT `LICENSE` consistent with existing manifest metadata and repository authorship.

## 4. Release audit

Run:

```text
cargo fmt --all --check
cargo test --workspace
cargo check --workspace --all-targets
cargo clippy --workspace --all-targets -- -D warnings
RUSTDOCFLAGS="-D warnings" cargo doc --workspace --no-deps
cargo test -p viewwright-egui --test viewwitness_compat
cargo test -p viewwright-egui --test exact_verification -- --nocapture
cargo metadata --no-deps --format-version 1
cargo tree -p viewwright-egui -e features
cargo tree -p viewwright-compare -e features
git diff --check
```

Audit with ripgrep or equivalent for:

```text
todo!(
unimplemented!(
TODO
FIXME
XXX
HACK
```

Classify findings.

Do not mass-edit historical docs merely to remove words.

## 5. Handle check failures conservatively

If fmt/docs/clippy requires a trivial non-behavioral source correction, make the smallest change and report it.

If a failure exposes semantic, layout, renderer, accessibility, expectation, comparison, or ViewWitness behavior pressure, STOP and report instead of turning M39 into another product milestone.

## 6. README release-candidate bookkeeping

At implementation completion:

- M38 remains accepted;
- M39 becomes `implementation complete; Director audit pending`.

Do not mark M39 accepted yourself.

## 7. Issue evidence

Comment the M39 issue with:

- implementation commit;
- exact files changed;
- current-doc reconciliation summary;
- changelog/release-note/license additions;
- all release-check results;
- TODO/FIXME audit results;
- metadata/version/license audit;
- dependency-feature audit;
- confirmation Issue #33 remains open/unpromoted;
- confirmation no production behavior changed;
- confirmation no v0.1.0 tag yet;
- confirmation no M40.

Leave the issue open.

## 8. Publish candidate

Commit and push `main`.

Verify clean tree and `local main == origin/main`.

Do not create the v0.1.0 tag yet.

Stop for Director audit.
