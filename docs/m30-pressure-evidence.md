# M30 — Pressure Evidence

M3 defines fixture content as typed representative preview/test data supplied to semantic elements. Its purpose is to replace backend-invented placeholder content with authored state that can actually appear in isolated preview.

Current resolution already checks:

- referenced element exists;
- one payload family is authored;
- payload family is compatible with element kind;
- family-specific invariants such as selection, IDs, labels, titles, text, and command state.

M29 now computes the set of regions structurally reachable from the validated `screen.root`.

However, fixture resolution still accepts any existing element regardless of whether the element's owning region is in that root-reachable set. Therefore an otherwise-valid fixture record can resolve into `ResolvedFixtureContent` for an element that the screen's root composition never traverses.

That is authored intent loss: the source claims representative content for the preview, resolution accepts it, but the projection cannot display it because the target element has no structural place in the screen.

M30 closes only that contradiction. It does not make unused declarations invalid in general.