# M9 — Pressure Evidence

M9 is justified by accepted canonical source and current renderer behavior.

## Existing authored distinction

Accepted canonical screens already use different density values:

```text
project_browser            comfortable
reader_workspace_visual    comfortable
dependency_workbench       dense
```

Dependency Workbench also describes its design character as `dense_but_legible`.

## Current semantic loss

`ScreenSource` contains `density: String` with a default of `comfortable`, but the resolved blueprint still carries raw `ScreenSource` rather than a typed resolved screen density.

The egui renderer does not consume density.

Local rhythm currently comes from backend constants/defaults such as:

- fixed region content shrink;
- fixed element spacing;
- fixed paragraph spacing;
- ordinary egui control spacing/padding;
- fixed card-local dimensions/padding.

Therefore the accepted `dense` declaration does not have a defined projection contract.

## Why canonical screenshots alone are insufficient

Project Browser, Reader, and Dependency Workbench differ in content, palette, region structure, spacing tokens, and visual profile. Comparing those screens cannot isolate whether a perceived density difference comes from `screen.density`.

M9 therefore uses an A/B pressure pair with the same:

- region/composition structure;
- spacing tokens;
- elements;
- fixture content;
- collection presentation;
- visual/fallback behavior;
- viewport assumptions.

The only meaningful design variable is Comfortable versus Dense.

## Expected pressure result

At the same viewport:

- major region/composition rectangles are identical;
- text sizes are identical;
- content is identical;
- Dense has visibly tighter local rhythm and smaller control/chrome spacing;
- Comfortable has visibly more breathing room.

That is sufficient to prove the semantic distinction survives projection without becoming a second layout system.
