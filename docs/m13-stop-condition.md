# M13 — Stop Condition

Stop M13 once the existing linear composition topology is truthful end-to-end.

Completion means:

- typed `CompositionKind` exists for Split, Row, Column;
- typed `Axis` exists for Horizontal, Vertical;
- `ResolvedComposition.kind` and `.axis` are typed;
- unsupported `stack` / `overlay` are rejected;
- contradictory explicit row/column axes are rejected;
- split preserves existing omitted-axis compatibility;
- semantic/debug and concept output remain deterministic;
- M4 layout uses typed topology with accepted geometry unchanged;
- all M0–M12 regressions pass.

Do not continue into implementation of stack, overlay, alignment, grids, constraints, or other layout vocabulary.