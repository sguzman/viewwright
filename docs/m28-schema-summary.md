# M28 — Schema Summary

M28 adds no TOML field and changes no Rust source shape.

Existing authored shape remains:

```text
FixtureContentSource {
    element,
    ...,
    text: Option<String>,
    ...
}
```

For records resolved as status text content, a present `text` string must contain at least one non-whitespace character.

`None` remains legal because fixture content for a status element is optional at the fixture level.

Valid strings remain exact. No enum, newtype, localization key, markup wrapper, or normalized copy field is introduced.