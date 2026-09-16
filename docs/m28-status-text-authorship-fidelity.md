# M28 — Status Text Authorship Fidelity

M3 introduced fixture-backed `text` content for representative status surfaces. For a status content record, the authored string is the entire payload: resolution carries it directly and egui renders it directly.

M28 requires every explicitly authored status `text` payload to contain at least one non-whitespace character.

Validation uses trim-based blankness detection only. Valid text is preserved exactly, including case, punctuation, Unicode, and intentional surrounding whitespace.

This rule applies only when a status text payload is present. A fixture may omit a status content record entirely. M28 does not make status content mandatory.

M28 does not apply to property values, document paragraphs, command reasons, element labels already governed by M14, or any future textual family.

No renderer, layout, concept, ASCII, runtime-state, localization, rich-text, or generalized copy-validation work is introduced.