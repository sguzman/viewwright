# M20 — Screen Purpose Authorship Fidelity

ViewWright explicitly owns screen identity and purpose. `screen.purpose` is already a required source field and is carried into semantic/debug and concept projections, but resolution currently accepts empty or whitespace-only purpose text.

That permits a syntactically present field to erase the screen's top-level authored intent.

## Principle

Required authorial intent must be semantically present, not merely syntactically present.

A valid `screen.purpose` must contain at least one non-whitespace character.

Valid purpose text is preserved exactly as authored. Validation uses `trim().is_empty()` only to detect blankness; it must not trim or normalize accepted text.

## Contract

Reject:

```toml
[screen]
id = "x"
purpose = ""
```

and:

```toml
purpose = "   "
```

Continue accepting any nonblank purpose text under the existing string model.

## Projection boundary

`ResolvedScreen.purpose` remains a definite `String`. Semantic/debug and concept projections continue to display the exact resolved purpose. No fallback purpose is generated from screen id, filename, region labels, or other metadata.

## Explicit non-goals

M20 does not add screen titles, subtitles, descriptions, localization keys, purpose grammar, punctuation rules, length limits, text transformation, fixture-state validation, design-character validation, or M21 work.
