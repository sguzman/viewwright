# M21 — Pressure Evidence

M4's authored model permits regions to carry width, height, and grow simultaneously. Its layout algorithm reserves fixed main-axis sizes and then distributes remaining space among positive growth weights.

Current implementation diverges in `growth_weight()`:

- horizontal parent + region width present -> region grow is forced to zero;
- vertical parent + region height present -> region grow is forced to zero.

This means an authored positive grow can disappear from layout without a diagnostic, even though it survives source parsing and resolution.

The surrounding allocation formula already adds `fixed_size + growth_share`; the defect is specifically the suppression of the growth weight before that formula is applied.

M21 therefore does not invent new sizing behavior. It makes an existing accepted combination obey the M4 allocation contract.