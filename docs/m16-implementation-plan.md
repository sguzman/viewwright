# M16 — Implementation Plan

1. Pull current `main` and read M16 authority plus M1/M4 composition authority.
2. Audit all canonical composition child relationships before editing.
3. Add one narrow ownership-validation pass in `viewwright-model` after root and child references are known.
4. Track child ownership separately for region/composition identities or otherwise preserve category-aware diagnostics.
5. Reject duplicate siblings, region multiple parents, composition multiple parents, and root-as-child.
6. Preserve existing cycle detection; do not replace it with ownership validation.
7. Add focused regression tests for every invalid ownership shape and canonical compatibility.
8. Keep layout/egui/concept/ASCII implementation unchanged unless a compiler-only call-site adjustment is genuinely required.
9. Run full checks and publish as implementation-complete / audit-pending.