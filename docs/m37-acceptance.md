# M37 — Acceptance

M37 is accepted when all of the following hold:

- ViewWright workspace `egui` and `eframe` are aligned to the 0.36.2 compatibility target used by the pinned ViewWitness revision;
- all required API migrations are narrow and behavior-preserving;
- a dev/test integration path enables the pinned ViewWitness `egui` feature without enabling observer/showcase/network features;
- a real AccessKit-enabled `viewwright-egui` frame produces an `egui::FullOutput` accepted directly by `viewwitness::witness_from_egui_output`;
- no hand-written conversion layer exists between ViewWright egui output and the ViewWitness adapter;
- the produced `Witness` validates successfully;
- representative M34 screen, region, and element `author_id` values survive conversion exactly;
- existing M34 hierarchy/bounds regressions remain green;
- canonical ViewWright model/layout/renderer tests remain green;
- `viewwright-compare` remains model-only in its normal dependency contract;
- no ViewWitness repository change is made;
- no M35 comparator behavior changes;
- no new runtime capture server or observer is added;
- representative visual QA finds no unintended toolkit-migration regression;
- README records M36 accepted and M37 implementation complete / Director audit pending;
- M38 is not started.

M37 is compatibility infrastructure. The intended UI is unchanged.
