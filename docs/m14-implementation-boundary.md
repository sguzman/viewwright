# M14 — Implementation Boundary

M14 is a validation/resolution correction, not a text-system redesign.

## In scope

- remove id-derived element-label fallback;
- validate label presence;
- reject empty and whitespace-only labels;
- preserve authored label bytes/text in the resolved model;
- update regression snippets that omitted labels only because the old fallback existed;
- add focused tests.

## Out of scope

Do not add:

- localization or translation keys;
- accessibility-name fields;
- placeholders;
- tooltips;
- icon-only command semantics;
- label display modes;
- title casing or formatting rules;
- per-kind alternate label fields;
- region titles;
- business behavior;
- renderer redesign;
- M15 work.

## Projection rule

Existing projections continue using `ResolvedElement.label` exactly as their accepted contracts require. M14 changes the provenance/invariant of that value, not its presentation.

## Diagnostics

Prefer validation errors tied to the element id and, when useful, its kind. Missing and blank authored labels must not be silently repaired.