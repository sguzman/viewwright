# M13 — Pressure Evidence

The current source and resolved models both carry composition topology as strings:

- `CompositionSource.kind: String`
- `CompositionSource.axis: Option<String>`
- `ResolvedComposition.kind: String`
- `ResolvedComposition.axis: String`

Validation currently recognizes `row`, `column`, `stack`, `split`, and `overlay`.

The M4 layout engine does not implement five distinct topology modes. It computes one linear axis and allocates children sequentially. `row` forces horizontal; `column` only receives a vertical default during resolution; `stack` and `overlay` have no distinct layout semantics.

Accepted canonical pressure demonstrates real semantics for:

- Reader root: `column` + `vertical`;
- Reader body: `split` + `horizontal`;
- Project Browser: `split` + `horizontal`;
- Dependency Workbench and density pressure specimens: linear split compositions.

Repository audit found no canonical authored use of `stack` or `overlay`.

Therefore the honest M13 contract is to type and validate the implemented linear topology and stop accepting unimplemented topology names.