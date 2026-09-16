# M28 — Implementation Boundary

Implement only local source-resolution validation for present status `text` payloads.

Preserve exactly:

- fixture payload-family exclusivity;
- status-element compatibility checks;
- resolved `Text { element, text }` shape;
- authored text bytes and ordering;
- omission of status content records;
- property-value behavior;
- document paragraph behavior;
- command-reason behavior;
- renderer, layout, concept, ASCII, and runtime behavior.

Do not create a shared generalized visible-copy validator merely to implement M28. Do not add localization, rich text, status enums, status IDs, or application-state semantics.