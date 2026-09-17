use std::collections::BTreeSet;

use egui::{vec2, Context, Pos2, RawInput, Rect};
use viewwitness::{witness_from_egui_output, EguiCaptureContext, Viewport};
use viewwright_egui::RenderState;
use viewwright_model::parse_and_resolve;

#[test]
fn project_browser_full_output_converts_directly_to_valid_viewwitness() {
    let blueprint =
        parse_and_resolve(include_str!("../../../examples/project-browser.toml")).unwrap();
    let context = Context::default();
    context.enable_accesskit();

    let mut output = context.run_ui(
        RawInput {
            screen_rect: Some(Rect::from_min_size(Pos2::ZERO, vec2(1440.0, 900.0))),
            ..Default::default()
        },
        |ui| {
            viewwright_egui::show(ui, &blueprint, "many_projects", &mut RenderState::default());
        },
    );

    let witness = witness_from_egui_output(
        &output,
        EguiCaptureContext::new(Viewport {
            width: 1440.0,
            height: 900.0,
            scale_factor: 1.0,
        }),
    )
    .expect("AccessKit-enabled ViewWright frame should produce a witness");
    output.textures_delta.clear();

    assert_eq!(witness.viewwitness_version, "0.1");
    let issues = witness.validation_issues();
    assert!(issues.is_empty(), "Witness validation issues: {issues:?}");

    let author_ids = witness
        .nodes
        .iter()
        .filter_map(|node| {
            node.identity
                .as_ref()
                .and_then(|identity| identity.author_id.as_deref())
        })
        .collect::<BTreeSet<_>>();

    for expected in [
        "project_browser",
        "navigation",
        "projects",
        "inspector",
        "project_search",
        "navigation_items",
        "project_collection",
        "project_inspector",
    ] {
        assert!(
            author_ids.contains(expected),
            "expected exact M34 author_id {expected}; observed {author_ids:?}"
        );
    }

    for fixture_local_id in ["project_a", "project_b", "project_c", "project_d"] {
        assert!(
            !author_ids.contains(fixture_local_id),
            "fixture-local ID {fixture_local_id} must not become an M34 author_id"
        );
    }
}
