# M29 — Implementation Plan

1. Re-read M15 dominant-target authority and M16 composition-tree ownership authority.
2. Reuse the already validated resolved composition tree to determine which region ids are reachable from `screen.root`.
3. Keep M15's existing missing-reference typing behavior for `design.dominant`.
4. After a dominant reference resolves to `DominantTarget`:
   - reject `Region(id)` when `id` is not in the root-reachable region set;
   - for `Element(id)`, find the resolved element and reject when its owning `region` is not in the root-reachable region set.
5. Keep exact target strings and `DominantTarget` variants unchanged.
6. Add focused tests for reachable/unreachable region and element targets, nested reachability, optional dominant, and unused non-dominant declarations.
7. Verify canonical specimens resolve unchanged.
8. Do not edit renderer/layout/projection behavior.
9. Run formatting, tests, preview check, and diff check.
10. Publish implementation, update README status, comment the issue, and leave it open for director audit.