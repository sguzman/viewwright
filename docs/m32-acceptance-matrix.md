# M32 — Acceptance Matrix

| Case | Expected result |
| --- | --- |
| region omits `overflow` | Resolves as `clip` |
| `overflow = "clip"` | Accepted |
| `overflow = "scroll_y"` | Accepted |
| unknown overflow value | Rejected |
| clip region content fits | Existing visible output unchanged |
| clip region content exceeds viewport | Excess paint/interaction confined to region |
| scroll_y content fits | Renders normally without geometry change |
| scroll_y content exceeds viewport | Scrolls vertically inside fixed region |
| scroll_y region frame/surface | Remains fixed while inner content moves |
| scroll_y and Search | Search remains editable |
| scroll_y and Collection/Tree/Document/Properties | Existing element rendering remains valid |
| horizontal scrolling | Not introduced |
| LayoutPlan rectangles | Unchanged by overflow policy |
| M31 overlay | Existing layer ordering/interaction unchanged |
| canonical M0–M31 sources | No migration; geometry unchanged |
| semantic/debug projection | Non-default scroll intent visible |
| ASCII projection | Non-default scroll intent visible |
| concept projection | Non-default scroll intent visible |
| Reader overflow pressure specimen | Resolves, previews, and scrolls vertically |
| pagination/virtualization/responsive behavior | Not implemented |
