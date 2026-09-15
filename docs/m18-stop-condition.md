# M18 Stop Condition

Stop M18 when composition-cycle validation is global across all declared compositions, including unreachable and self-referential cycles, while unused acyclic declarations remain valid.

Do not continue into reachability enforcement, unused-declaration cleanup, reusable components, graph infrastructure, layout changes, renderer changes, or M19.

If canonical accepted sources unexpectedly contain an unreachable cycle, stop and report the contradiction instead of silently migrating them.
