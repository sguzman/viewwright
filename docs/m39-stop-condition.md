# M39 — Stop Condition

Stop M39 when the repository is a coherent v0.1.0 release candidate and no new product capability has been introduced.

Required stop state:

- M0–M38 are represented as accepted in current-facing status docs;
- README describes the implemented ViewWright/ViewWitness verification relationship accurately;
- architecture and charter no longer misdescribe implemented bridge capabilities as future;
- historical `docs/model.md` cannot be mistaken for current schema authority;
- `CHANGELOG.md` contains a v0.1.0 entry;
- `docs/releases/v0.1.0.md` records release scope and known deferrals;
- root MIT LICENSE exists consistent with manifest metadata;
- release-quality fmt/test/check/clippy/rustdoc commands pass;
- M37 ViewWitness compatibility regression passes;
- M38 exact verification regression passes;
- package metadata is internally consistent;
- relevant dependency feature boundaries remain intact;
- release-blocking TODO/unimplemented audit has no unresolved blocker;
- Issue #33 remains future roadmap, not silently implemented or closed;
- no production behavior change is smuggled into release cleanup;
- working tree clean and local/remote main synchronized;
- no `v0.1.0` tag yet;
- no M40 work.

After Director audit accepts M39, the next action should be release finalization, not another numbered capability milestone.

Release finalization may create and push annotated tag `v0.1.0` at the accepted M39 commit.
