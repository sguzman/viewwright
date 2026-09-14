# M10 — Authored Chrome Fidelity

ViewWright's egui backend currently leaks internal semantic scaffolding into the authored interface.

Two concrete defects are already visible in accepted previews:

1. every region paints its internal `id` and semantic `role` as visible UI text;
2. every element receives a generic visible pre-label before kind-specific rendering, so commands render the same authored label twice: once as ordinary text and once on the button itself.

Examples from accepted screens include visible pairs such as `content / primary_content`, `reader / primary_content`, and duplicated command chrome such as `Open [Open]` or `Play / pause [Play / pause]`.

These strings are valuable semantic/debug information, but they are not automatically product-facing chrome.

## Principle

The authored renderer may only paint visible text whose presentation is justified by authored UI semantics.

Internal identifiers and ontology terms remain available to semantic/debug and concept projections, but do not become visible application text merely because the backend knows them.

## Region rule

Region `id` and `role` are structural semantics.

M10 removes automatic visible rendering of both from egui.

M10 does **not** add an authored region-title field. If future screens require visible region titles, that vocabulary must be earned separately rather than inferred from internal identifiers.

## Element-label rule

`element.label` remains authored visible text, but each element kind owns how that label participates in presentation.

At minimum:

- `command`: label belongs to the button/control itself; no duplicate pre-label;
- `search`: label may identify the field before the input;
- `collection`: label may identify the collection before its items;
- `tree`: label may identify the tree before nodes;
- `property_sheet`: label may identify the property group;
- `document`: label may identify the document surface before fixture title/body;
- `status`: label may identify the status value.

M10 does not redesign those presentations beyond eliminating backend-invented or duplicated chrome.

## Boundary

M10 changes projection purity, not ontology breadth.

It does not add region titles, generic components, new element kinds, toolbar/menu systems, typography changes, responsive behavior, or business logic.