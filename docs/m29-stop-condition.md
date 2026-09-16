# M29 — Stop Condition

Stop M29 once ViewWright rejects only the contradictory dominant-target cases earned by M15/M16:

- existing but unreachable dominant region;
- existing dominant element whose owning region is unreachable.

Keep these legal:

- no dominant target;
- reachable region target;
- reachable element target;
- unrelated unused declarations.

Do not continue into global dead-declaration linting, layout priority, renderer interpretation, focus/accessibility semantics, graph visualization, or M30 work.