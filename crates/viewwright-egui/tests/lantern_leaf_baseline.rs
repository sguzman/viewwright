use viewwright_layout::layout;
use viewwright_model::{parse_and_resolve, ResolvedFixtureContent};

const SOURCE: &str = include_str!("../../../specimens/lantern-leaf-reader-baseline.toml");

#[test]
fn lantern_leaf_baseline_traverses_existing_01_projections_and_layout() {
    let blueprint = parse_and_resolve(SOURCE).expect("baseline must resolve without new schema");
    assert_eq!(blueprint.screen.id, "lantern_leaf_reader_baseline");
    assert_eq!(
        blueprint.design.dominant.as_ref().unwrap().id(),
        "reading_surface"
    );
    assert!(blueprint.visual.is_some());

    let semantic = blueprint.semantic_tree();
    for expected in [
        "composition workspace (column, vertical)",
        "composition reading_body (split, horizontal)",
        "region library (role navigation",
        "region reading_surface (role primary_content",
        "region inspector (role inspector",
        "region tts_player (role commands",
        "fixture reading [chapter_three_preview]",
    ] {
        assert!(
            semantic.contains(expected),
            "semantic tree missing {expected:?}:\n{semantic}"
        );
    }

    let ascii = viewwright_ascii::render(&blueprint);
    for expected in [
        "region library",
        "region reader_toolbar",
        "region reading_surface",
        "region inspector",
        "region tts_player",
        "Search in book",
    ] {
        assert!(
            ascii.contains(expected),
            "ASCII projection missing {expected:?}:\n{ascii}"
        );
    }

    let concept = viewwright_concept::render(&blueprint);
    for expected in [
        "dominant: reading_surface",
        "workspace — vertical",
        "reading_body — horizontal",
        "library — role navigation",
        "reading_surface — role primary_content",
        "canvas: #09100F",
        "accent: #62E89C",
    ] {
        assert!(
            concept.contains(expected),
            "concept projection missing {expected:?}:\n{concept}"
        );
    }

    let plan = layout(&blueprint, 1440.0, 900.0);
    assert_eq!(plan.region("library").unwrap().width, 260.0);
    assert_eq!(plan.region("inspector").unwrap().width, 330.0);
    assert_eq!(plan.region("tts_player").unwrap().height, 96.0);
    assert!(plan.region("reading_surface").unwrap().width > 700.0);
    assert!(plan.region("reading_surface").unwrap().height > 600.0);

    let reading = blueprint
        .fixtures
        .iter()
        .find(|fixture| fixture.id == "reading")
        .unwrap();
    assert!(reading.content.iter().any(|content| matches!(
        content,
        ResolvedFixtureContent::Collection { element, selected: Some(selected), items }
            if element == "library_navigation" && selected == "library" && items.len() == 5
    )));
    assert!(reading.content.iter().any(|content| matches!(
        content,
        ResolvedFixtureContent::Tree { element, selected: Some(selected), nodes }
            if element == "table_of_contents" && selected == "chapter_3" && nodes.len() == 8
    )));
    assert!(reading.content.iter().any(|content| matches!(
        content,
        ResolvedFixtureContent::Document { element, title, paragraphs }
            if element == "chapter_document" && title == "The Mountain Path" && paragraphs.len() == 4
    )));
}
