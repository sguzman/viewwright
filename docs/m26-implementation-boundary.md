# M26 — Implementation Boundary

Implementation belongs in model validation/resolution only.

Allowed:
- inspect each authored property name before constructing `ResolvedProperty`;
- emit fixture/element/property-index-aware diagnostics where practical;
- add focused model regressions;
- update README milestone bookkeeping.

Forbidden:
- trimming or rewriting accepted names;
- validating property values;
- requiring unique property names;
- introducing property IDs;
- form/editability/schema systems;
- localization/accessibility redesign;
- egui/layout/concept changes for valid sources;
- M27 work.

No screenshot QA is required if valid renderer/layout output is unchanged.