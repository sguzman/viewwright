# M17 Stop Condition — Structural Identifier Fidelity

Stop M17 when:

- every required structural/local ID family rejects empty and whitespace-only identity;
- valid IDs are preserved exactly;
- root validation no longer uses an empty authored-compatible string as failure state;
- successful root output remains source-faithful;
- canonical sources resolve unchanged;
- existing duplicate/reference/tree/selection/cycle behavior remains green;
- layout and renderer code require no semantic change.

Do not continue into identifier grammar, generated IDs, typed ID wrappers, reachability, reusable components, or M18.