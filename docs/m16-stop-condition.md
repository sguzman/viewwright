# M16 — Stop Condition

Stop M16 when valid ViewWright screens are guaranteed not to assign one placed region/composition to multiple structural positions.

Required stop state:

- duplicate siblings rejected;
- region multiple-parent placement rejected;
- nested-composition multiple-parent placement rejected;
- root-as-child rejected;
- existing cycle validation preserved;
- accepted canonical specimens unchanged;
- M4 layout behavior unchanged for valid trees.

Do not continue into reachability requirements, component reuse, scene graphs, portals, cloning, overlays, or M17 work.