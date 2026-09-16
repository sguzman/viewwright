# M32 — Acceptance

M32 is accepted when all of the following hold:

- `region.overflow` is typed and validated;
- supported authored values are exactly `clip` and `scroll_y`;
- omitted overflow resolves to `clip`;
- unknown overflow values fail validation;
- `clip` confines painting and interaction to the planned region content viewport;
- `scroll_y` keeps the region frame/geometry fixed while content scrolls vertically inside it;
- vertical scrolling does not change LayoutPlan rectangles;
- horizontal scrolling is not introduced;
- region inset/density policy remains outside the scrollable content extent as existing geometry requires;
- ordinary Search, Collection, Tree, Document, PropertySheet, Command, Status, Text, and Preview rendering continues to work inside a scrollable region;
- M31 overlay ordering/interaction remains intact;
- existing canonical sources omit overflow and require no migration;
- semantic/debug and concept/ASCII inspection expose non-default `scroll_y` intent without inventing new geometry;
- the M32 Reader pressure specimen resolves and previews with a fixed Reader surface whose long document can be scrolled vertically;
- scrolling inside the Reader does not move app commands, library, inspector, or transport regions;
- existing canonical screens retain their geometry and visible output where content fits;
- no responsive, pagination, virtualization, rich-text, or persisted-scroll system is introduced;
- all M0–M31 regressions remain green.

M32 has an intended visible/interaction delta only for the new overflow pressure specimen and any future source that explicitly authors `scroll_y`.
