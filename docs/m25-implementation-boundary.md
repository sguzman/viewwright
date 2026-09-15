# M25 — Implementation Boundary

Implementation should remain inside model validation plus focused tests and README milestone bookkeeping.

Expected implementation shape:

- validate `design.character` entries for trim-based blankness;
- validate `design.avoid` entries for trim-based blankness;
- preserve original strings untouched in `ResolvedDesign`;
- keep list order and duplicates unchanged;
- keep M15 dominant-target resolution unchanged.

Do not edit egui, layout, visual audit, ASCII, or concept projection unless an unexpected blocker proves necessary. Do not add shared text-policy machinery beyond a tiny local helper if useful.