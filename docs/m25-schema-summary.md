# M25 — Schema Summary

No source or resolved field shape changes.

Existing source remains:

```text
DesignSource {
  character: Vec<String>,
  dominant: Option<String>,
  avoid: Vec<String>,
}
```

Existing resolved design remains:

```text
ResolvedDesign {
  character: Vec<String>,
  dominant: Option<DominantTarget>,
  avoid: Vec<String>,
}
```

M25 only adds validation that every explicitly authored `character` and `avoid` entry contains at least one non-whitespace character. Accepted strings are preserved exactly.