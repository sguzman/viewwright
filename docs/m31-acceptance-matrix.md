# M31 — Acceptance Matrix

| Case | Expected result |
| --- | --- |
| `split`, `row`, `column` canonical sources | Resolve/render unchanged |
| `stack` | Rejected |
| overlay with exactly `[base_composition, floating_region]` | Accepted |
| overlay with authored `axis` | Rejected |
| overlay with authored `gap` | Rejected |
| overlay with fewer/more than two children | Rejected |
| overlay child 0 is region | Rejected |
| overlay child 1 is composition | Rejected |
| floating region missing fixed width | Rejected |
| floating region missing fixed height | Rejected |
| floating region with positive grow | Rejected |
| overlay padding | Legal and applied as common inset |
| overlay base geometry | Fills inner rectangle |
| floating geometry | Authored fixed size, centered in inner rectangle |
| child z-order | Base first, floating region second |
| overlapping pointer hit | Floating layer has precedence |
| pointer outside floating rect | Base remains interactive |
| resolved overlay axis | Absent; no fake horizontal/vertical value |
| semantic/debug projection | Shows overlay topology without fake axis |
| ASCII projection | Shows base + floating layers distinctly |
| concept projection | Shows overlay/layering intent distinctly |
| M31 pressure specimen | Resolves, deterministic layout, previewable |
| M16/M18 ownership/cycles | Unchanged |
| M29/M30 reachability | Unchanged under overlay traversal |
| M21 fixed-plus-grow linear allocation | Unchanged |
| existing canonical screenshots/layout | No intended delta |
| modality/backdrop/anchor/window management | Not implemented |
