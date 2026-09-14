# M10 — Pressure Evidence

Accepted runtime screenshots repeatedly show renderer-generated semantic scaffolding inside the authored interface.

Representative examples:

- density pressure screen: `controls` followed by semantic role `controls`, and `content` followed by `primary_content`;
- visual Reader: region ids such as `app_commands`, `library`, `reader`, `inspector`, and `transport` are visible together with ontology roles;
- commands appear twice because the renderer first paints `element.label` and then paints the same label again on the egui button.

The source does not author region ids or roles as user-facing prose. They exist for structure, addressing, ontology, layout, and agent inspection.

Likewise, a command's authored label is already consumed by the command widget itself. Rendering an additional text copy is backend invention.

M10 is therefore earned by canonical runtime evidence rather than speculative API design.