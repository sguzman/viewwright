# M32 — Stop Condition

Stop M32 when ViewWright can truthfully express and preview one fixed region whose content either clips or scrolls vertically according to an authored typed policy.

Stop once:

- `clip` and `scroll_y` are the only accepted policies;
- omission means `clip`;
- the Reader overflow pressure specimen scrolls vertically inside the Reader region;
- region geometry and surrounding Reader regions do not move when the document scrolls;
- non-scroll regions remain confined to their planned rectangles;
- existing canonical sources require no migration and remain visually unchanged where their content fits;
- projections expose non-default scroll intent;
- M0–M31 tests pass;
- human QA confirms the long Reader can scroll while surrounding regions stay fixed.

Do not continue into horizontal scrolling, pagination, virtualization, persisted scroll state, responsive layout, rich text, EPUB/TTS behavior, or generalized scrolling architecture after this condition is met.
