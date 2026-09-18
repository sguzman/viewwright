# M39 — Pressure Evidence

## Accepted implementation is ahead of current-facing docs

M38 completes and proves the real normative-to-observed verification loop, but several current-facing documents still speak about that bridge as future work.

Examples:

- README: "In time, ViewWright expectations and ViewWitness observations should be comparable";
- README ownership text: "eventually a bridge for exporting expectations to ViewWitness";
- `docs/architecture.md`: section titled "Future ViewWitness bridge";
- `docs/charter.md`: "The future bridge is comparison, not merger" and "eventual ViewWitness expectation export".

Those statements were reasonable earlier but are stale for an audited first release.

## Historical model ambiguity

`docs/model.md` explicitly calls itself an "initial conceptual model" and contains M0-era candidate roles, composition vocabulary, and future possibilities.

Because the repository now has 39 accepted/defined milestone layers, readers can mistake this old design sketch for current schema documentation unless it is clearly marked historical.

M39 should preserve it as provenance rather than silently rewriting history.

## Metadata pressure

Workspace metadata already declares:

```toml
version = "0.1.0"
license = "MIT"
```

but the repository has no root LICENSE file and no v0.1 release note/changelog checkpoint.

## Backlog pressure

The only open product issue after M38 acceptance is the authored responsive-layout roadmap. That issue explicitly says it is not yet a numbered milestone and gives a promotion condition that has not been established.

Therefore release closure should not manufacture responsive scope merely to keep development moving.

## Verification pressure

M38 proves the semantic verification chain, but a first release should also survive:

- workspace-wide all-target compilation;
- clippy with warnings denied;
- rustdoc with warnings denied;
- package metadata inspection;
- exact verification and ViewWitness compatibility regression tests.

M39 earns those release-quality checks without changing product semantics.
