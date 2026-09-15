# M19 — Implementation Boundary

Implementation should remain model-local and validation-led.

Preferred shape:

1. Validate every authored `tokens.color` value once as source validity.
2. Make any new map-wide diagnostic ordering deterministic, preferably by sorted token name.
3. Preserve existing visual-role reference resolution and concrete `ResolvedVisual` palette behavior.
4. Avoid duplicate diagnostics for the same malformed referenced token where practical; do not broaden scope merely to perfect diagnostic deduplication.

Do not change:

- TOML fields;
- color token names or namespace rules;
- `ResolvedVisual` shape;
- visual role vocabulary;
- region surface semantics;
- concept specification format for valid input;
- layout;
- egui rendering;
- visual audit thresholds;
- canonical palettes.

Do not add a general token linter, unused-token warning system, CSS color parser, alpha channel, shared-theme inheritance, or M20 work.