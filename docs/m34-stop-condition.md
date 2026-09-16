# M34 — Stop Condition

Stop implementation and report the contradiction if any of the following is discovered:

- egui 0.31 cannot set AccessKit `author_id` on the relevant nodes without replacing normal widget accessibility semantics;
- exact ViewWright IDs violate AccessKit sibling uniqueness in an accepted canonical screen and cannot be resolved through the existing semantic hierarchy;
- creating semantic anchors changes visible layout or interaction in accepted screens;
- region identity cannot be associated with the already-planned region rectangle without fabricating geometry;
- implementing M34 requires authored TOML changes, LayoutPlan changes, or ViewWitness repository changes;
- a comparator/tolerance layer becomes necessary merely to expose identity;
- an accepted canonical screen requires IDs to be normalized or prefixed.

Do not work around these by silently changing identity semantics. Preserve the conflict as evidence and return to Director review.