# M12 — Stop Condition

Stop M12 when:

- all accepted authored region roles resolve to a typed `RegionRole`;
- unsupported role strings fail validation;
- `ResolvedRegion.role` is no longer arbitrary text;
- semantic/debug and concept still expose role meaning;
- egui no longer string-matches resolved roles;
- all M0–M11 tests remain green;
- no visible behavior is intentionally changed.

Do not continue into new role semantics, region interaction, navigation, chrome, or M13 work.
