# M20 — Pressure Evidence

The project charter states that ViewWright owns **screen identity and purpose** and exists to author interface intent.

`screen.purpose` is required by the source schema, preserved in `ResolvedScreen`, emitted by semantic/debug inspection, and emitted by the concept specification.

Current validation nevertheless accepts:

```toml
[screen]
id = "x"
purpose = "   "
root = "root"
```

when the remaining blueprint is structurally valid.

The result is a successfully resolved screen whose required top-level intent is semantically empty. This is especially visible in projections that must carry purpose but can only emit an empty value.

M20 therefore closes an existing authorship-fidelity gap rather than adding a new content system.
