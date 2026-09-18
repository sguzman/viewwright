# M39 — Acceptance

M39 is accepted when ViewWright has a coherent, tested, auditable v0.1.0 release candidate and no release-blocking contradiction remains between documentation, metadata, and accepted behavior.

## Documentation acceptance

Required:

- README states M0–M38 accepted and M39 release-readiness status accurately;
- README no longer describes the implemented ViewWright/ViewWitness expectation-comparison bridge as merely future work;
- `docs/architecture.md` describes the current expectation/observation/comparison architecture rather than a future hypothetical bridge;
- `docs/charter.md` replaces obsolete "future/eventual bridge" wording while preserving the normative/descriptive project boundary;
- `docs/model.md` is clearly marked as historical M0-era design exploration, or is otherwise made impossible to mistake for current schema authority;
- the responsive roadmap remains explicitly future/unpromoted;
- no accepted milestone authority history is rewritten merely to sound current.

## Release-document acceptance

Required:

- root `CHANGELOG.md` exists with a concise `0.1.0` entry summarizing the shipped capability families and known deferrals;
- `docs/releases/v0.1.0.md` exists as the human-readable release checkpoint, including scope, verification status, known deferrals, and release-finalization note;
- root `LICENSE` exists and is consistent with the already-authored workspace `license = "MIT"` metadata;
- no unsupported crates.io/MSRV promise is invented.

## Build / quality acceptance

Required release checks:

```text
cargo fmt --all --check
cargo test --workspace
cargo check --workspace --all-targets
cargo clippy --workspace --all-targets -- -D warnings
RUSTDOCFLAGS="-D warnings" cargo doc --workspace --no-deps
cargo test -p viewwright-egui --test viewwitness_compat
cargo test -p viewwright-egui --test exact_verification -- --nocapture
git diff --check
```

Also audit:

- `cargo metadata --no-deps --format-version 1` for workspace package version/license consistency;
- `cargo tree -p viewwright-egui -e features`;
- `cargo tree -p viewwright-compare -e features`;
- source tree for `todo!()`, `unimplemented!()`, and obvious release-blocking TODO/FIXME markers.

A TODO/FIXME mention is not automatically a failure if it clearly documents deferred roadmap work. Classify findings rather than deleting history blindly.

## Behavioral acceptance

If M39 changes no production Rust behavior, no new human visual QA is required.

If any production renderer/layout/accessibility behavior changes, M39 must stop for Director review and human QA as appropriate rather than claiming documentation-only closure.

## Repository acceptance

- Issue #33 remains open as future responsive-layout roadmap unless its promotion condition independently becomes true;
- M39 issue remains open until Director audit;
- working tree clean;
- local `main == origin/main`;
- no `v0.1.0` tag created before Director acceptance;
- no M40 work.
