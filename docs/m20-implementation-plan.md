# M20 — Implementation Plan

1. Add a focused `screen.purpose` nonblank validation during model resolution.
2. Use `trim().is_empty()` only for invalidity detection; preserve accepted purpose text exactly.
3. Add tests for empty and whitespace-only rejection.
4. Add a preservation test for unusual but nonblank purpose text, including leading/trailing whitespace if practical.
5. Preserve existing missing-field parse behavior.
6. Keep `ResolvedScreen.purpose: String` and all projection shapes unchanged.
7. Run canonical compatibility and full workspace checks.
8. Update README bookkeeping: M19 is accepted; M20 implementation is audit-pending.
9. Comment the M20 issue with implementation evidence and leave it open for director audit.

No screenshot QA is expected because M20 has no visible delta for valid canonical sources.
