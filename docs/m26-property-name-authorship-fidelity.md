# M26 — Property Name Authorship Fidelity

## Problem

M3 defines property-sheet fixture content as ordered `{ name, value }` metadata pairs. The resolver currently copies both strings directly, and egui renders `property.name` as the strong visible field label beside `property.value`.

An explicitly authored property may therefore resolve with `name = ""` or whitespace-only text, producing an unlabeled metadata row.

## Principle

A property row may have an empty value, but an authored property name must actually name the property.

Every `PropertySource.name` must contain at least one non-whitespace character. Validation may use `name.trim().is_empty()` only to detect invalidity. Valid names are preserved exactly.

## Boundary

M26 does not require nonblank property values, unique names, schemas, editable forms, localization, normalization, taxonomy, renderer/layout changes, or M27 work.