# M37 — Stop Condition

M37 stops when real ViewWright egui output can cross the pinned ViewWitness egui adapter directly and all accepted ViewWright behavior remains intact.

Stop after proving:

- workspace egui/eframe alignment is complete;
- ViewWright builds and tests on the aligned toolkit;
- actual AccessKit-enabled ViewWright `egui::FullOutput` is accepted by `witness_from_egui_output`;
- the resulting Witness validates;
- representative exact M34 author IDs survive;
- existing M34 identity/hierarchy/bounds tests remain green;
- no unintended visible migration regression is accepted;
- ViewWitness itself is unchanged.

Do not continue into:

- M33-vs-Witness comparison;
- live capture servers;
- external observer orchestration;
- paint/raster evidence;
- responsive layout;
- M38.

If the toolkit upgrade materially changes accepted semantics or cannot preserve the M34 bridge, stop and report the incompatibility instead of adding translation machinery.
