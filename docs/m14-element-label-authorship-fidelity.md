# M14 — Element Label Authorship Fidelity

M10 established a projection-purity rule: internal identifiers and ontology terms must not become visible product chrome merely because the backend knows them. Visible element text is authored through `element.label` and rendered according to element kind.

The resolved model still violates that rule one stage earlier. When an authored element omits `label`, resolution currently synthesizes visible text from the internal element id by replacing underscores with spaces.

Conceptually:

```text
id = "project_search"
label omitted
    ↓
resolved label = "project search"
    ↓
visible product text
```

That is internal identity leaking into product-facing copy.

M14 removes that fallback and makes current element labels explicitly authored.

## Principle

Stable identity and visible copy are separate semantics.

- `element.id` identifies an element to the model, fixtures, interactions, tests, and future bridges.
- `element.label` is authored human-facing text used by current projections.

One must not silently stand in for the other.

## Contract

Every currently supported element kind requires an explicit non-empty authored `label`:

- `text`
- `command`
- `search`
- `collection`
- `property_sheet`
- `tree`
- `preview`
- `status`
- `document`

The source model may retain `Option<String>` so validation can produce source-oriented diagnostics. A successfully resolved `ResolvedElement.label` remains `String`, but its value must come from authored `label` rather than an id-derived fallback.

Whitespace-only labels are invalid.

## Compatibility

Accepted canonical specimens already author explicit labels. M14 is therefore intended to produce no visible delta in canonical previews.

Legacy test snippets that relied on id-to-label synthesis should be corrected to author labels explicitly rather than preserving the fallback.

## Boundary

M14 does not add:

- placeholder text;
- region titles;
- localization/i18n;
- accessibility-name vocabulary;
- icon-only command semantics;
- label visibility flags;
- automatic title-casing;
- text transformation;
- new element kinds;
- M15 work.

If future pressure requires an intentionally unlabeled visual control, that needs explicit semantics rather than reuse of internal identifiers.