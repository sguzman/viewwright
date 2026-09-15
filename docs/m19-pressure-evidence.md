# M19 — Pressure Evidence

M2 authority explicitly requires malformed color literals to be diagnosed and says canonical visual TOML should resolve without ignored intent.

Current implementation does not fully satisfy that promise:

- `ColorTokensSource` stores arbitrary token strings;
- `parse_color` diagnoses malformed literals;
- but `parse_color` is reached only through `color_ref`;
- `color_ref` runs only for color tokens referenced by `[visual]`;
- therefore an unreferenced malformed token is never validated;
- if `[visual]` is absent, malformed authored color tokens are not inspected at all.

Example currently capable of escaping malformed-color validation:

```toml
[tokens.color]
future_accent = "definitely-not-a-color"
```

The defect is source-validation coverage, not rendering. A token does not need to be used, but if it is authored as a color token its literal must be valid.

M19 closes exactly that gap.