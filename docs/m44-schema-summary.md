# M44 — Schema Summary

## Source

Responsive authoring is optional.

Legacy sources without `[responsive]` behave exactly as before.

Example:

```toml
[responsive]
default = "wide"

[[responsive.variant]]
id = "narrow"
max_width = 960
root = "workspace_narrow"

[[responsive.variant.region]]
id = "reader"
furnishing = "reader_furnishing_narrow"

[[responsive.variant.region]]
id = "tts_player"
height = "320px"
furnishing = "tts_furnishing_narrow"

[[responsive.variant]]
id = "compact"
min_width = 960
max_width = 1260
root = "workspace_compact"

[[responsive.variant.region]]
id = "library"
width = "210px"

[[responsive.variant]]
id = "wide"
min_width = 1260
root = "workspace"
```

The nested `[[responsive.variant.region]]` tables belong to the immediately containing variant.

## Responsive source

```text
default: String
variant: Vec<ResponsiveVariantSource>
```

At least two variants are required when responsive authoring is present.

## Variant source

Fields:

- `id: String`;
- `root: String`;
- optional `min_width: u32`;
- optional `max_width: u32`;
- zero or more `region` override tables.

Rules:

- ID nonblank and unique;
- root resolves to a composition;
- omitted min = 0;
- omitted max = infinity;
- min < max when max exists;
- intervals disjoint and exhaustive over [0, infinity);
- source order carries no precedence;
- default references one variant;
- default variant root equals `screen.root`.

## Region override

Fields:

- `id: String`;
- optional `width = "<n>px"`;
- optional `height = "<n>px"`;
- optional `grow = finite nonnegative number`;
- optional `furnishing = "<id>"`.

At least one override field is required.

Rules:

- region exists;
- region is reachable in that variant root;
- no duplicate override for one region in one variant;
- dimensions use existing positive pixel syntax;
- furnishing resolves;
- active furnishing validates for that semantic region.

No clearing/unset syntax in M44.

## Resolved model

Add typed responsive structures conceptually equivalent to:

```rust
ResolvedResponsive {
    default: String,
    variants: Vec<ResolvedResponsiveVariant>,
}

ResolvedResponsiveVariant {
    id: String,
    min_width: u32,
    max_width: Option<u32>,
    root: String,
    regions: HashMap<String, ResolvedRegionOverride>,
}
```

Exact container types are implementation detail.

The resolved model must expose cheap deterministic selection by logical width.

## Selected viewport state

Add a typed selected-state/view concept containing at least:

- optional active variant ID;
- active root composition;
- effective region geometry/furnishing lookup.

Non-responsive blueprints behave as one implicit legacy state with `variant = None`.

Do not clone/parse TOML per frame.

## LayoutPlan

LayoutPlan gains:

- active responsive variant ID when present;
- active root;
- only active reachable composition/region/furnishing rectangles.

Provide stable query access such as:

```rust
plan.variant_id()
plan.active_root()
plan.region(id)
plan.furnishing(id)
```

Exact names are implementation detail except existing query behavior must remain.

## Expectation

Canonical expectation remains version 0.2.

No new field is required in M44.

For a requested viewport, expectation export uses the selected variant and exports only active reachable regions/elements.

M35 remains unchanged.

## Structural projections

Preserve existing viewport-free APIs by projecting the responsive `default` variant.

Add viewport-aware ASCII/concept projection APIs so responsive selection can be inspected explicitly.
