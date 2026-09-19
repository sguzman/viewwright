use viewwright_model::{
    CompositionChild, CompositionKind, FurnishingChild, OverflowPolicy, ResolvedBlueprint,
    ResolvedFixtureContent, ResolvedViewportState,
};

pub fn render(b: &ResolvedBlueprint) -> String {
    render_state(b, b.default_viewport_state(), None)
}

pub fn render_at(b: &ResolvedBlueprint, width: f32, height: f32) -> String {
    render_state(b, b.viewport_state(width), Some((width, height)))
}

fn render_state(
    b: &ResolvedBlueprint,
    state: ResolvedViewportState<'_>,
    viewport: Option<(f32, f32)>,
) -> String {
    let mut out = format!(
        "ViewWright concept specification\n\nScreen\n  id: {}\n  purpose: {}\n  density: {:?}\n\n",
        b.screen.id, b.screen.purpose, b.screen.density
    );
    out.push_str("Design character\n");
    for value in &b.design.character {
        out.push_str(&format!("  {value}\n"));
    }
    out.push_str("Avoid\n");
    for value in &b.design.avoid {
        out.push_str(&format!("  {value}\n"));
    }
    out.push_str(&format!(
        "\nHierarchy\n  dominant: {}\nComposition\n",
        b.design
            .dominant
            .as_ref()
            .map(|target| target.id())
            .unwrap_or("unspecified")
    ));
    if let Some((width, height)) = viewport {
        out.push_str(&format!(
            "Viewport\n  width: {width}\n  height: {height}\n\n"
        ));
    }
    if let Some(variant) = state.variant_id() {
        out.push_str(&format!("Responsive variant\n  id: {variant}\n\n"));
    }
    walk(state.active_root(), b, &state, &mut out, 2);
    if let Some(v) = &b.visual {
        out.push_str(&format!("\nPalette\n  canvas: {}\n  surface: {}\n  surface_raised: {}\n  text: {}\n  text_muted: {}\n  accent: {}\n  border: {}\n\nTypography\n  display: {}\n  heading: {}\n  body: {}\n  caption: {}\n\nSurface policy\n  border: {:?}\n  corner radius: {}\n", hex(v.palette.canvas), hex(v.palette.surface), hex(v.palette.surface_raised), hex(v.palette.text), hex(v.palette.text_muted), hex(v.palette.accent), hex(v.palette.border), v.type_scale.display, v.type_scale.heading, v.type_scale.body, v.type_scale.caption, v.border_policy, v.corner_radius));
    }
    out.push_str("\nImportant semantic elements\n");
    for e in &b.elements {
        if !state.is_element_active(&e.id) {
            continue;
        }
        let mut attributes = Vec::new();
        if let Some(presentation) = e.presentation {
            attributes.push(format!("presentation {presentation:?}"));
        }
        if let Some(choice) = &e.choice {
            attributes.push(format!(
                "choice/{} options [{}]",
                choice.presentation.as_str(),
                choice
                    .options
                    .iter()
                    .map(|option| format!("{}={}", option.id, option.label))
                    .collect::<Vec<_>>()
                    .join(", ")
            ));
        }
        if let Some(scalar) = &e.scalar {
            attributes.push(format!(
                "scalar {}..{} step {}{}",
                scalar.min,
                scalar.max,
                scalar.step,
                scalar
                    .unit
                    .as_deref()
                    .map(|unit| format!(" {unit}"))
                    .unwrap_or_default()
            ));
        }
        if let Some(action) = &e.action {
            attributes.push(format!("action {}", action.as_str()));
        }
        out.push_str(&format!(
            "  {:?}: {}{}\n",
            e.kind,
            e.label,
            if attributes.is_empty() {
                String::new()
            } else {
                format!(" ({})", attributes.join("; "))
            }
        ));
    }
    out
}

/// Renders the structural concept specification plus typed representative values for one fixture.
/// Returns `None` when the fixture does not exist on this blueprint.
pub fn render_fixture(b: &ResolvedBlueprint, fixture_id: &str) -> Option<String> {
    render_fixture_state(b, fixture_id, b.default_viewport_state())
}

pub fn render_fixture_at(
    b: &ResolvedBlueprint,
    fixture_id: &str,
    width: f32,
    height: f32,
) -> Option<String> {
    let mut output = render_fixture_state(b, fixture_id, b.viewport_state(width))?;
    output.push_str(&format!("Viewport\n  width: {width}\n  height: {height}\n"));
    Some(output)
}

fn render_fixture_state(
    b: &ResolvedBlueprint,
    fixture_id: &str,
    state: ResolvedViewportState<'_>,
) -> Option<String> {
    let fixture = b.fixtures.iter().find(|fixture| fixture.id == fixture_id)?;
    let mut out = render_state(b, state.clone(), None);
    out.push_str(&format!(
        "\nRepresentative control values — fixture {}\n",
        fixture.id
    ));
    for content in &fixture.content {
        if !state.is_element_active(content_element_id(content)) {
            continue;
        }
        match content {
            viewwright_model::ResolvedFixtureContent::Document {
                element,
                document: viewwright_model::ResolvedDocument::Rich { blocks, spoken },
            } => {
                out.push_str(&format!("  document {element}: rich blocks\n"));
                for block in blocks {
                    match block {
                        viewwright_model::ResolvedDocumentBlock::Heading { id, level, text } => {
                            out.push_str(&format!("    heading {id} level {level}: {text}\n"))
                        }
                        viewwright_model::ResolvedDocumentBlock::Eyebrow { id, text }
                        | viewwright_model::ResolvedDocumentBlock::Paragraph { id, text }
                        | viewwright_model::ResolvedDocumentBlock::Quote { id, text } => {
                            out.push_str(&format!("    {} {id}: {text}\n", block.kind().as_str()));
                        }
                        viewwright_model::ResolvedDocumentBlock::Divider { id } => {
                            out.push_str(&format!("    divider {id}\n"));
                        }
                    }
                }
                if let Some(spoken) = spoken {
                    out.push_str(&format!(
                        "    spoken: block {} range {}..{}\n",
                        spoken.block, spoken.start, spoken.end
                    ));
                }
            }
            viewwright_model::ResolvedFixtureContent::Choice { element, selected } => {
                if let Some(control) = b.elements.iter().find(|control| control.id == *element) {
                    let label = control
                        .choice
                        .as_ref()
                        .and_then(|choice| {
                            choice.options.iter().find(|option| option.id == *selected)
                        })
                        .map(|option| option.label.as_str())
                        .unwrap_or(selected);
                    out.push_str(&format!("  {} = {}\n", control.label, label));
                }
            }
            viewwright_model::ResolvedFixtureContent::Boolean { element, value } => {
                if let Some(control) = b.elements.iter().find(|control| control.id == *element) {
                    out.push_str(&format!("  {} = {}\n", control.label, value));
                }
            }
            viewwright_model::ResolvedFixtureContent::Scalar { element, value } => {
                if let Some(control) = b.elements.iter().find(|control| control.id == *element) {
                    let unit = control
                        .scalar
                        .as_ref()
                        .and_then(|scalar| scalar.unit.as_deref())
                        .unwrap_or("");
                    out.push_str(&format!("  {} = {}{}\n", control.label, value, unit));
                }
            }
            viewwright_model::ResolvedFixtureContent::Text { element, text } => {
                if let Some(control) = b.elements.iter().find(|control| control.id == *element) {
                    out.push_str(&format!("  {} = {}\n", control.label, text));
                }
            }
            viewwright_model::ResolvedFixtureContent::Properties {
                element,
                properties,
            } => {
                if let Some(control) = b.elements.iter().find(|control| control.id == *element) {
                    out.push_str(&format!("  {}\n", control.label));
                    for property in properties {
                        out.push_str(&format!("    {} = {}\n", property.name, property.value));
                    }
                }
            }
            viewwright_model::ResolvedFixtureContent::Collection { element, items, .. } => {
                if let Some(control) = b.elements.iter().find(|control| control.id == *element) {
                    out.push_str(&format!("  {}\n", control.label));
                    for item in items {
                        out.push_str(&format!("    {}\n", item.label));
                    }
                }
            }
            _ => {}
        }
    }
    Some(out)
}
fn content_element_id(content: &ResolvedFixtureContent) -> &str {
    match content {
        ResolvedFixtureContent::Collection { element, .. }
        | ResolvedFixtureContent::Properties { element, .. }
        | ResolvedFixtureContent::Text { element, .. }
        | ResolvedFixtureContent::Tree { element, .. }
        | ResolvedFixtureContent::Document { element, .. }
        | ResolvedFixtureContent::Command { element, .. }
        | ResolvedFixtureContent::Choice { element, .. }
        | ResolvedFixtureContent::Boolean { element, .. }
        | ResolvedFixtureContent::Scalar { element, .. } => element,
    }
}

fn walk(
    id: &str,
    b: &ResolvedBlueprint,
    state: &ResolvedViewportState<'_>,
    out: &mut String,
    depth: usize,
) {
    let Some(c) = b.compositions.iter().find(|c| c.id == id) else {
        return;
    };
    if let Some(axis) = c.axis {
        out.push_str(&format!("{}{} — {}\n", " ".repeat(depth), c.id, axis));
    } else {
        out.push_str(&format!(
            "{}{} — overlay (axisless)\n",
            " ".repeat(depth),
            c.id
        ));
    }
    for (index, child) in c.children.iter().enumerate() {
        let layer = if c.kind == CompositionKind::Overlay {
            Some(if index == 0 { "base" } else { "floating" })
        } else {
            None
        };
        match child {
            CompositionChild::Composition(id) => {
                if let Some(layer) = layer {
                    let description = if layer == "base" {
                        "fills the overlay inner rectangle"
                    } else {
                        "above base"
                    };
                    out.push_str(&format!(
                        "{}{} layer — {description}\n",
                        " ".repeat(depth + 2),
                        layer
                    ));
                }
                walk(
                    id,
                    b,
                    state,
                    out,
                    depth + if layer.is_some() { 4 } else { 2 },
                );
            }
            CompositionChild::Region(id) => {
                let r = b.regions.iter().find(|r| r.id == *id).unwrap();
                let overflow = if r.overflow == OverflowPolicy::ScrollY {
                    " — vertical scroll"
                } else {
                    ""
                };
                if let Some(layer) = layer {
                    let effective = state.effective_region(r);
                    let width = effective
                        .width
                        .map(|width| format!("{width}px"))
                        .unwrap_or_else(|| "unspecified".into());
                    let height = effective
                        .height
                        .map(|height| format!("{height}px"))
                        .unwrap_or_else(|| "unspecified".into());
                    out.push_str(&format!(
                        "{}{} layer: {} — centered, fixed {width} × {height}, role {}, {:?}, {:?}{}{}\n",
                        " ".repeat(depth + 2),
                        layer,
                        id,
                        r.role,
                        r.surface,
                        r.importance,
                        effective.furnishing
                            .map(|root| format!(", furnishing {root}"))
                            .unwrap_or_default(),
                        overflow
                    ));
                } else {
                    let effective = state.effective_region(r);
                    let geometry = [
                        effective.width.map(|value| format!("width {value}px")),
                        effective.height.map(|value| format!("height {value}px")),
                    ]
                    .into_iter()
                    .flatten()
                    .collect::<Vec<_>>();
                    out.push_str(&format!(
                        "{}{} — role {}, {:?}, {:?}{}{}{}\n",
                        " ".repeat(depth + 2),
                        id,
                        r.role,
                        r.surface,
                        r.importance,
                        if geometry.is_empty() {
                            String::new()
                        } else {
                            format!(", {}", geometry.join(", "))
                        },
                        state
                            .effective_region(r)
                            .furnishing
                            .map(|root| format!(", furnishing {root}"))
                            .unwrap_or_default(),
                        overflow
                    ));
                }
                if let Some(root) = state.effective_region(r).furnishing {
                    walk_furnishing(root, b, state, out, depth + 4);
                }
            }
        }
    }
}

fn walk_furnishing(
    id: &str,
    b: &ResolvedBlueprint,
    state: &ResolvedViewportState<'_>,
    out: &mut String,
    depth: usize,
) {
    let Some(furnishing) = b.furnishings.iter().find(|furnishing| furnishing.id == id) else {
        return;
    };
    let mut attributes = vec![
        format!("gap {}px", furnishing.gap),
        format!("padding {}px", furnishing.padding),
        format!("overflow {}", furnishing.overflow.as_str()),
    ];
    if let Some(width) = furnishing.width {
        attributes.push(format!("width {width}px"));
    }
    if let Some(height) = furnishing.height {
        attributes.push(format!("height {height}px"));
    }
    if furnishing.grow > 0.0 {
        attributes.push(format!("grow {}", furnishing.grow));
    }
    out.push_str(&format!(
        "{}{} — {} ({})\n",
        " ".repeat(depth),
        furnishing.id,
        furnishing.kind,
        attributes.join(", ")
    ));
    for child in &furnishing.children {
        match child {
            FurnishingChild::Furnishing(child) => walk_furnishing(child, b, state, out, depth + 2),
            FurnishingChild::Element(id) => {
                if state.is_element_active(id) {
                    if let Some(element) = b.elements.iter().find(|element| element.id == *id) {
                        out.push_str(&format!(
                            "{}element {} — {:?}: {}\n",
                            " ".repeat(depth + 2),
                            element.id,
                            element.kind,
                            element.label
                        ));
                    }
                }
            }
        }
    }
}
fn hex(c: viewwright_model::Color) -> String {
    format!("#{:02X}{:02X}{:02X}", c.r, c.g, c.b)
}

#[cfg(test)]
mod tests {
    use super::{render, render_at, render_fixture, render_fixture_at};
    use viewwright_model::parse_and_resolve;

    #[test]
    fn visual_concept_is_deterministic_and_carries_hierarchy() {
        let blueprint = parse_and_resolve(include_str!(
            "../../../specimens/reader-workspace-visual.toml"
        ))
        .unwrap();
        let first = render(&blueprint);
        assert_eq!(first, render(&blueprint));
        assert!(first.contains("dominant: reader"));
        assert!(first.contains("canvas: #262B33"));
        assert!(first.contains("workspace — vertical"));
        assert!(first.contains("reading_body — horizontal"));
        assert!(first.contains("reader — role primary_content, Canvas, Primary"));
        assert!(first.contains("library — role navigation, Panel, Secondary"));
        assert!(first.contains("Document: Document"));
        let project = render(
            &parse_and_resolve(include_str!("../../../examples/project-browser.toml")).unwrap(),
        );
        assert!(project.contains("Collection: navigation items (presentation List)"));
        assert!(project.contains("Collection: project collection (presentation AdaptiveCards)"));
        let dependency = render(
            &parse_and_resolve(include_str!("../../../specimens/dependency-workbench.toml"))
                .unwrap(),
        );
        assert!(dependency.contains("density: Dense"));
        let overlay = render(
            &parse_and_resolve(include_str!(
                "../../../specimens/overlay-command-palette-pressure.toml"
            ))
            .unwrap(),
        );
        assert!(overlay.contains("root_overlay — overlay (axisless)"));
        assert!(overlay.contains("base layer"));
        assert!(overlay.contains("floating layer: palette_surface — centered"));
        assert!(overlay.contains("520px × 300px"));
        assert!(!overlay.contains("root_overlay — horizontal"));
        let overflow = render(
            &parse_and_resolve(include_str!(
                "../../../specimens/reader-overflow-pressure.toml"
            ))
            .unwrap(),
        );
        assert!(
            overflow.contains("reader — role primary_content, Canvas, Primary — vertical scroll")
        );
        assert!(!overflow.contains("reader — vertical scroll, pagination"));
    }

    #[test]
    fn furnished_lantern_leaf_concept_exposes_local_structure_without_new_semantics() {
        let blueprint = parse_and_resolve(include_str!(
            "../../../specimens/lantern-leaf-reader-furnished.toml"
        ))
        .unwrap();
        let output = render(&blueprint);
        assert!(output.contains(
            "reader — role primary_content, Panel, Primary, furnishing reader_furnishing"
        ));
        assert!(output.contains("reader_furnishing — column (gap 8px, padding 8px, overflow clip)"));
        assert!(output
            .contains("toolbar_slot — row (gap 12px, padding 0px, overflow clip, height 52px)"));
        assert!(output
            .contains("toolbar_actions — row (gap 8px, padding 0px, overflow clip, width 560px)"));
        assert!(output
            .contains("document_slot — column (gap 0px, padding 0px, overflow scroll_y, grow 1)"));
        assert!(output.contains("tts_furnishing — row"));
        assert!(output
            .contains("choices_slot — row (gap 8px, padding 0px, overflow clip, width 300px)"));
        assert!(output.contains("Important semantic elements"));
        assert!(!output.contains("Slider:"));
        assert!(!output.contains("Select:"));
    }

    #[test]
    fn m42_concept_exposes_control_configuration_actions_and_fixture_values() {
        let blueprint = parse_and_resolve(include_str!(
            "../../../specimens/lantern-leaf-reader-controls.toml"
        ))
        .unwrap();
        let structural = render(&blueprint);
        assert!(
            structural.contains("Choice: Font Family (choice/select options [literata=Literata")
        );
        assert!(structural.contains("action reader.font_family.set"));
        assert!(structural.contains("Scalar: Font Size (scalar 12..32 step 1 px;"));
        assert!(structural.contains("Boolean: Dyslexia-friendly font"));
        let fixture = render_fixture(&blueprint, "reading").unwrap();
        assert!(fixture.contains("Font Family = Literata"));
        assert!(fixture.contains("Font Size = 18px"));
        assert!(fixture.contains("Text Alignment = Left"));
        assert!(fixture.contains("Show Highlights = true"));
        assert!(fixture.contains("Voice = Aria"));
        assert!(fixture.contains("Speed = 1.2×"));
        assert!(fixture.contains("Volume = 82%"));
        assert!(render_fixture(&blueprint, "missing").is_none());
    }

    #[test]
    fn m43_fixture_concept_exposes_rich_document_structure_and_spoken_range() {
        let blueprint = parse_and_resolve(include_str!(
            "../../../specimens/lantern-leaf-reader-document.toml"
        ))
        .unwrap();
        let output = render_fixture(&blueprint, "reading").unwrap();
        for expected in [
            "document chapter_document: rich blocks",
            "eyebrow chapter_3",
            "heading chapter_title level 1",
            "paragraph p2",
            "quote closing_quote",
            "spoken: block p2 range 174..212",
        ] {
            assert!(output.contains(expected), "missing {expected:?}:\n{output}");
        }
    }

    #[test]
    fn m44_responsive_concept_filters_dormant_semantics_and_uses_effective_state() {
        let blueprint = parse_and_resolve(include_str!(
            "../../../specimens/lantern-leaf-reader-responsive.toml"
        ))
        .unwrap();
        let narrow = render_at(&blueprint, 800.0, 900.0);
        assert!(narrow.contains("workspace_narrow"));
        assert!(narrow.contains("reader_furnishing_narrow"));
        assert!(narrow.contains("tts_furnishing_narrow"));
        assert!(!narrow.contains("inspector — role"));
        assert!(!narrow.contains("library — role"));
        assert!(!narrow.contains("Font Family"));

        let compact = render_at(&blueprint, 1100.0, 900.0);
        assert!(compact.contains("library — role"));
        assert!(compact.contains("reader — role"));
        assert!(compact.contains("tts_player — role"));
        assert!(compact.contains("210px"));
        assert!(!compact.contains("inspector — role"));

        let default = render(&blueprint);
        let wide = render_at(&blueprint, 1440.0, 900.0);
        for marker in [
            "workspace",
            "library — role",
            "reader — role",
            "inspector — role",
        ] {
            assert!(default.contains(marker));
            assert!(wide.contains(marker));
        }
        let fixture = render_fixture_at(&blueprint, "reading", 800.0, 900.0).unwrap();
        assert!(fixture.contains("The Mountain Path"));
        assert!(fixture.contains("spoken:"));
        assert!(fixture.contains("The Wandering Horizon"));
        assert!(!fixture.contains("Font Family"));
        assert!(!fixture.contains("Show Highlights"));
        let compact_fixture = render_fixture_at(&blueprint, "reading", 1100.0, 900.0).unwrap();
        assert!(compact_fixture.contains("Library navigation"));
        assert!(!compact_fixture.contains("Font Family"));
        let wide_fixture = render_fixture_at(&blueprint, "reading", 1440.0, 900.0).unwrap();
        assert!(wide_fixture.contains("Font Family"));
    }
}
