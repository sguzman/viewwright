# M24 — Pressure Evidence

M0 requires spacing-token references. M2 establishes named color tokens and named corner references. M19 reinforces that authored token vocabulary must remain valid even when unused.

Current `TokensSource` stores spacing and corners in `HashMap<String, u32>` and color tokens in a flattened `HashMap<String, String>`, but no validation rejects empty or whitespace-only authored keys. Such keys can therefore survive as supposedly named tokens.

M24 repairs token identity without adding a broader naming policy.