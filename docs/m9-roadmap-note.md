# M9 — Roadmap Note

M9 is a fidelity milestone, not a vocabulary-expansion milestone.

The source already contains `screen.density`, and accepted canonical screens already use both `comfortable` and `dense`. The renderer currently ignores that distinction.

The smallest earned correction is therefore:

```text
raw density string
    ↓
typed resolved density
    ↓
small backend micro-layout policy
```

M9 deliberately stops before per-region density, responsive density, or authored micro-spacing controls.
