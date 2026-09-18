# M39 — Implementation Boundary

M39 is a release-readiness and documentation-reconciliation milestone.

## In scope

- reconcile README with accepted M0–M38 reality;
- update current-facing architecture and charter wording where implemented capabilities are still described as future;
- clearly classify `docs/model.md` as historical M0-era design exploration rather than current schema authority;
- add `CHANGELOG.md`;
- add `docs/releases/v0.1.0.md`;
- add root `LICENSE` consistent with existing MIT package metadata;
- run stricter release checks, including clippy and rustdoc warnings;
- make narrow non-behavioral corrections only if required to satisfy release tooling;
- classify TODO/FIXME/unimplemented findings;
- update README M39 bookkeeping;
- publish evidence to the M39 issue.

## Out of scope

- source syntax changes;
- canonical specimen migration;
- new element/region/composition semantics;
- responsive layout;
- new renderer features;
- ViewWitness changes;
- comparator changes;
- expectation changes;
- public verification APIs;
- CI workflow creation;
- crates.io publication;
- GitHub Release creation;
- pre-audit `v0.1.0` tag creation;
- M40.

## Historical documentation boundary

Accepted milestone authority documents are historical records of what was authorized at each milestone.

Do not rewrite their future-tense/non-goal text after the fact merely because later milestones implemented those capabilities.

Current-facing docs may be reconciled; milestone authority history remains provenance.

## Code-change boundary

M39 is expected to be documentation/metadata-only.

If clippy/rustdoc exposes a trivial non-behavioral warning, a narrow correction may be acceptable if clearly reported.

If a check exposes a semantic defect or visible behavior issue, stop and report rather than widening M39.
