# M41 — Acceptance

M41 is accepted only when region-local furnishing solves the concrete M40 structural pressure without weakening the accepted verification architecture.

## Model acceptance

- `region.furnishing` is optional and backward compatible.
- furnishings are typed row/column structures.
- gap/padding resolve through existing spacing tokens.
- fixed width/height and non-negative grow validate deterministically.
- clip/scroll_y reuse the existing overflow vocabulary.
- cycles, duplicate children, multiple parents, missing references, cross-region element ownership, mixed direct child classes, and incomplete furnished-region element coverage are rejected.
- all furnishing declarations are reachable from exactly one furnished region in M41.
- existing unfurnished sources resolve unchanged.

## Layout acceptance

- LayoutPlan records furnishing rectangles.
- furnishing root == owning region rectangle.
- branch furnishing allocation is backend-independent fixed-plus-grow.
- cross-axis fill is deterministic.
- M21 fixed-plus-grow behavior is preserved locally.
- no element bounds are introduced.

## Projection acceptance

Semantic/debug, ASCII, and concept projection expose the furnishing hierarchy sufficiently to inspect:

- region → furnishing root;
- row/column structure;
- structural child order;
- fixed/grow local slot intent;
- local overflow.

The projection must not pretend furnishings are semantic controls.

## Renderer acceptance

- legacy unfurnished regions preserve existing rendering behavior.
- furnished regions render exactly one furnishing tree.
- leaf row/column flow follows authored order and gap.
- branch groups render in their LayoutPlan slots.
- furnishing scroll_y performs vertical inner scrolling.
- no furnishing-specific visible frame/chrome is invented.
- fixture behavior remains element-based.

## Identity acceptance

- furnishing IDs do not appear as M34 AccessKit `author_id`;
- semantic elements remain immediate authored-identity children of their owning region;
- M33 expectation format remains 0.1;
- M35 comparator remains unchanged;
- ViewWitness pin/repository remains unchanged;
- real expectation/Witness exact verification remains green for all prior cases and the new M41 specimen.

## Lantern Leaf acceptance

Add a new furnished Lantern Leaf specimen that demonstrates the capability rather than mutating the M40 baseline.

At 1440×900 human QA should establish:

- major shell remains recognizable;
- fake reader-toolbar region is gone;
- reader toolbar is local structure within the reader region;
- document remains the dominant growing/scrolling reader content;
- chapter status no longer suffers the M40 top-right squeeze/clipping;
- the TTS strip has deliberate local grouping;
- `Voice` no longer collapses into a vertical one-character column;
- no new control semantics are falsely claimed.

The near-black visual compression may remain; richer visual roles are later v0.2 work.

## Regression acceptance

All M0–M40 checks remain green.

No responsive implementation and no M42 work.
