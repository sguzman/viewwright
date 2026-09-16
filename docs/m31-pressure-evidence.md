# M31 — Pressure Evidence

The pressure source is `specimens/overlay-command-palette-pressure.toml`.

It deliberately differs from the accepted linear screens:

- an ordinary Project Browser-style workspace remains spatially intact;
- a command-palette region must occupy space above that workspace rather than beside it;
- the palette has authored fixed width and height;
- the palette contains an existing Search affordance plus fixture-backed collection results;
- `design.dominant` names the floating palette region;
- all fixture targets remain root-reachable under the intended overlay topology.

Trying to encode this through `split`, `row`, or `column` would be dishonest because the floating surface would consume a slot and displace the base workspace.

M13 explicitly deferred `overlay` until a future pressure case established geometry, z-order, overlap, hit-testing, and projection semantics. This specimen supplies that concrete case.

The pressure does not require:

- modal blocking;
- a backdrop;
- element anchoring;
- arbitrary placement;
- multiple floating layers;
- conditional open/closed structure;
- runtime command-palette behavior.

An always-open static overlay is sufficient to prove the missing topology.
