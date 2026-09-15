# M21 — Fixed-plus-Grow Allocation Fidelity

M4 established deterministic major layout slots from fixed region sizes and proportional growth. Its allocation contract reserves fixed main-axis sizes, then distributes the remaining main-axis space among children with positive `grow` weights.

The current layout implementation silently weakens that contract for regions that author both intents on the same axis. `fixed_size()` returns the authored width/height, but `growth_weight()` suppresses the region's grow weight whenever that same-axis fixed size is present. The resulting layout preserves the fixed base and discards authored growth.

## Principle

**Fixed main-axis size is a base allocation. `grow` is a weight for a share of remaining main-axis space. When both are authored, both must survive projection.**

For a horizontal composition, a region with `width = "100px"` and `grow = 1` receives its 100px base plus its proportional share of remaining width.

For a vertical composition, a region with `height = "50px"` and `grow = 1` receives its 50px base plus its proportional share of remaining height.

Cross-axis fixed sizing remains unrelated to main-axis growth.

## Existing M4 contract

For each composition main axis:

1. inset authored padding;
2. reserve gaps;
3. reserve fixed main-axis region sizes;
4. compute remaining space;
5. distribute that remaining space among every child with positive `grow`, proportional to its weight;
6. add each child's growth share to its fixed base, if any.

Nested compositions continue to participate through their existing `grow` value; they have no authored fixed-size field in the current model.

## Boundary

M21 adds no TOML syntax and no new sizing concepts. It does not add min/max/clamp, percentages, intrinsic measurement, overflow policy, breakpoints, shrink weights, flex semantics, or a general constraint system.

Canonical accepted specimens do not currently combine same-axis fixed size with positive grow, so they should retain identical geometry. M21 exists to make an already-valid authored combination project truthfully rather than silently discarding one of its intents.