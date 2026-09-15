# M23 — Pressure Evidence

M3 collection items author `id` plus `label` as representative list content. M5 tree nodes author `id` plus a display `label`. The egui backend renders those labels directly.

Current resolution checks local IDs but accepts blank labels. A fixture can therefore contain a collection item or tree node that exists structurally but has no authored visible label.

M14 already separates stable identity from visible authored copy. M23 applies that principle only to these local label fields.