# M32 — Implementation Boundary

Keep M32 focused on a region-level vertical overflow contract.

Allowed implementation work:

- add optional authored `region.overflow`;
- add typed resolved `Clip` / `ScrollY` policy;
- preserve omitted overflow as `Clip`;
- validate unknown values;
- make clip behavior explicit at the planned region content viewport;
- render `ScrollY` regions with a vertical egui scroll area inside the existing region content rect;
- keep the region surface/frame fixed while only inner content moves;
- expose non-default scroll intent in semantic/debug, ASCII, and concept projections;
- add the Reader overflow pressure specimen to the preview selector;
- add focused model/backend/projection regression coverage;
- perform human scroll QA on the pressure specimen;
- update milestone bookkeeping.

Do not:

- change LayoutPlan geometry;
- add horizontal or two-axis scroll;
- add element-level overflow;
- add scroll offset to TOML or `ResolvedBlueprint`;
- add application persistence;
- add pagination;
- add virtualization;
- add auto-scroll or scroll-to-selection;
- add scrollbar theme/schema fields;
- add responsive conditions or variants;
- add rich-text/EPUB semantics;
- reinterpret fixture content;
- start M33.

Parsing/validation remains outside the hot render loop. The renderer consumes resolved overflow semantics and existing LayoutPlan rectangles.
