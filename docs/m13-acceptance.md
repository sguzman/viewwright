# M13 — Acceptance

M13 is accepted when composition topology is typed and validated end-to-end without changing accepted M4 geometry.

## Required behavior

- add typed resolved composition-kind semantics;
- add typed resolved axis semantics;
- support exactly `split`, `row`, and `column`;
- reject `stack`, `overlay`, and other unknown kinds;
- `row` resolves horizontal;
- `column` resolves vertical;
- explicit contradictory `row`/`column` axes are rejected;
- preserve the existing split-axis default behavior unless repo evidence establishes otherwise;
- `ResolvedComposition.kind` and `.axis` are typed rather than arbitrary strings;
- semantic/debug and concept projections remain deterministic;
- M4 layout consumes typed topology rather than string comparisons;
- canonical M0–M12 specimens preserve accepted geometry and behavior.

## Regression pressure

- `reader_workspace_visual / reading` preserves vertical root + horizontal nested split geometry;
- `project_browser / many_projects` preserves horizontal split geometry;
- `dependency_workbench / healthy` remains unchanged;
- the M9 density pressure pair remains layout-identical.

## Tests

Add focused coverage for `split`, `row`, `column`, rejected `stack`/`overlay`, contradictory axes, projection output, and layout equivalence. Preserve all prior regressions.

Human visual QA is not inherently required because M13 is intended to produce no visible delta.