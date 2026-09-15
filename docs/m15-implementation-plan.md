# M15 — Implementation Plan

Implementation should remain model-led and small.

1. Introduce `ResolvedDesign` or equivalent with cloned `character`/`avoid` fields and typed optional dominant target.
2. Introduce `DominantTarget::{Region, Element}` or equivalent.
3. Resolve `DesignSource.dominant` only after region and element ids are known.
4. Preserve existing missing-reference diagnostics or improve them without changing source syntax.
5. Change `ResolvedBlueprint.design` to the resolved design type.
6. Update concept projection to print the target id without leaking Rust enum/debug syntax.
7. Let compiler failures identify any additional downstream consumer; update only those consumers required by the new resolved contract.
8. Add focused model tests for region target, element target, omission, invalid target, and preservation of `character`/`avoid`.
9. Add/update concept tests proving canonical `dominant: reader` output remains stable.
10. Run the full regression suite.

No egui behavior change is expected.
