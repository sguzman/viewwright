use egui::{vec2, Context, Pos2, RawInput, Rect};
use viewwitness::{witness_from_egui_output, EguiCaptureContext, Viewport};
use viewwright_egui::RenderState;
use viewwright_model::parse_and_resolve;

const WIDTH: f32 = 1440.0;
const HEIGHT: f32 = 900.0;

fn assert_real_render_exactly_matches_expectation(source: &str, fixture: &str, case: &str) {
    assert_real_render_exactly_matches_expectation_at(source, fixture, case, WIDTH, HEIGHT);
}

fn assert_real_render_exactly_matches_expectation_at(
    source: &str,
    fixture: &str,
    case: &str,
    width: f32,
    height: f32,
) {
    let blueprint = parse_and_resolve(source)
        .unwrap_or_else(|error| panic!("{case}: source resolution failed: {error}"));
    let expectation = viewwright_expectation::build_expectation(&blueprint, width, height)
        .unwrap_or_else(|error| panic!("{case}: expectation construction failed: {error}"));

    let context = Context::default();
    context.enable_accesskit();
    let mut output = context.run_ui(
        RawInput {
            screen_rect: Some(Rect::from_min_size(Pos2::ZERO, vec2(width, height))),
            ..Default::default()
        },
        |ui| {
            viewwright_egui::show(ui, &blueprint, fixture, &mut RenderState::default());
        },
    );

    let witness = witness_from_egui_output(
        &output,
        EguiCaptureContext::new(Viewport {
            width,
            height,
            scale_factor: 1.0,
        }),
    )
    .unwrap_or_else(|| panic!("{case}: real egui::FullOutput had no AccessKit Witness"));
    output.textures_delta.clear();

    let validation_issues = witness.validation_issues();
    assert!(
        validation_issues.is_empty(),
        "{case}: real Witness failed validation: {validation_issues:#?}"
    );

    let report = viewwright_compare::compare(&expectation, &witness)
        .unwrap_or_else(|error| panic!("{case}: comparison failed: {error:#?}"));
    println!(
        "{case}: Witness validation issues={}, ComparisonReport={report:#?}, exact={}",
        validation_issues.len(),
        report.is_exact_match()
    );
    assert!(
        report.mismatches.is_empty(),
        "{case}: unexpected comparison mismatches: {report:#?}"
    );
    assert!(
        report.evidence_gaps.is_empty(),
        "{case}: unexpected comparison evidence gaps: {report:#?}"
    );
    assert!(
        report.is_exact_match(),
        "{case}: expected an exact comparison: {report:#?}"
    );
}

#[test]
fn project_browser_many_projects_exactly_matches_real_renderer_witness() {
    assert_real_render_exactly_matches_expectation(
        include_str!("../../../examples/project-browser.toml"),
        "many_projects",
        "Project Browser / many_projects at 1440x900",
    );
}

#[test]
fn overlay_palette_open_exactly_matches_real_renderer_witness() {
    assert_real_render_exactly_matches_expectation(
        include_str!("../../../specimens/overlay-command-palette-pressure.toml"),
        "palette_open",
        "M31 overlay / palette_open at 1440x900",
    );
}

#[test]
fn reader_long_document_exactly_matches_real_renderer_witness() {
    assert_real_render_exactly_matches_expectation(
        include_str!("../../../specimens/reader-overflow-pressure.toml"),
        "long_document",
        "M32 overflow / long_document at 1440x900",
    );
}

#[test]
fn lantern_leaf_baseline_exactly_matches_real_renderer_witness() {
    assert_real_render_exactly_matches_expectation(
        include_str!("../../../specimens/lantern-leaf-reader-baseline.toml"),
        "reading",
        "M40 Lantern Leaf baseline / reading at 1440x900",
    );
}

#[test]
fn lantern_leaf_responsive_variants_exactly_match_at_required_widths() {
    let source = include_str!("../../../specimens/lantern-leaf-reader-responsive.toml");
    for (width, variant) in [(1440.0, "wide"), (1100.0, "compact"), (800.0, "narrow")] {
        assert_real_render_exactly_matches_expectation_at(
            source,
            "reading",
            &format!("M44 Lantern Leaf responsive / {variant} at {width}x900"),
            width,
            900.0,
        );
    }
}

#[test]
fn lantern_leaf_furnished_exactly_matches_real_renderer_witness() {
    assert_real_render_exactly_matches_expectation(
        include_str!("../../../specimens/lantern-leaf-reader-furnished.toml"),
        "reading",
        "M41 Lantern Leaf furnished / reading at 1440x900",
    );
}

#[test]
fn lantern_leaf_semantic_controls_exactly_match_real_renderer_witness() {
    assert_real_render_exactly_matches_expectation(
        include_str!("../../../specimens/lantern-leaf-reader-controls.toml"),
        "reading",
        "M42 Lantern Leaf controls / reading at 1440x900",
    );
}

#[test]
fn lantern_leaf_rich_document_exactly_matches_real_renderer_witness() {
    assert_real_render_exactly_matches_expectation(
        include_str!("../../../specimens/lantern-leaf-reader-document.toml"),
        "reading",
        "M43 Lantern Leaf rich document / reading at 1440x900",
    );
}
