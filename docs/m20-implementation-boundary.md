# M20 — Implementation Boundary

Keep M20 model-local and validation-led.

The expected implementation is a narrow nonblank check during resolution, analogous in spirit to existing authored label/ID validation but specific to `screen.purpose`.

Do not:

- create a new purpose type or hierarchy;
- generalize all strings into one validator;
- change semantic/debug output shape;
- change concept output shape;
- touch layout or egui;
- validate unrelated fixture/content strings;
- add localization or presentation semantics.

M20 should not alter valid resolved blueprints except that previously accepted blank-purpose sources now fail.
