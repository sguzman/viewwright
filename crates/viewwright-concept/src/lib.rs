use viewwright_model::{
    CompositionChild, CompositionKind, FurnishingChild, OverflowPolicy, ResolvedBlueprint,
};

pub fn render(b: &ResolvedBlueprint) -> String {
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
    walk(&b.root, b, &mut out, 2);
    if let Some(v) = &b.visual {
        out.push_str(&format!("\nPalette\n  canvas: {}\n  surface: {}\n  surface_raised: {}\n  text: {}\n  text_muted: {}\n  accent: {}\n  border: {}\n\nTypography\n  display: {}\n  heading: {}\n  body: {}\n  caption: {}\n\nSurface policy\n  border: {:?}\n  corner radius: {}\n", hex(v.palette.canvas), hex(v.palette.surface), hex(v.palette.surface_raised), hex(v.palette.text), hex(v.palette.text_muted), hex(v.palette.accent), hex(v.palette.border), v.type_scale.display, v.type_scale.heading, v.type_scale.body, v.type_scale.caption, v.border_policy, v.corner_radius));
    }
    out.push_str("\nImportant semantic elements\n");
    for e in &b.elements {
        if let Some(presentation) = e.presentation {
            out.push_str(&format!(
                "  {:?}: {} (presentation {:?})\n",
                e.kind, e.label, presentation
            ));
        } else {
            out.push_str(&format!("  {:?}: {}\n", e.kind, e.label));
        }
    }
    out
}
fn walk(id: &str, b: &ResolvedBlueprint, out: &mut String, depth: usize) {
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
                walk(id, b, out, depth + if layer.is_some() { 4 } else { 2 });
            }
            CompositionChild::Region(id) => {
                let r = b.regions.iter().find(|r| r.id == *id).unwrap();
                let overflow = if r.overflow == OverflowPolicy::ScrollY {
                    " — vertical scroll"
                } else {
                    ""
                };
                if let Some(layer) = layer {
                    let width = r
                        .width
                        .map(|width| format!("{width}px"))
                        .unwrap_or_else(|| "unspecified".into());
                    let height = r
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
                        r.furnishing
                            .as_deref()
                            .map(|root| format!(", furnishing {root}"))
                            .unwrap_or_default(),
                        overflow
                    ));
                } else {
                    out.push_str(&format!(
                        "{}{} — role {}, {:?}, {:?}{}{}\n",
                        " ".repeat(depth + 2),
                        id,
                        r.role,
                        r.surface,
                        r.importance,
                        r.furnishing
                            .as_deref()
                            .map(|root| format!(", furnishing {root}"))
                            .unwrap_or_default(),
                        overflow
                    ));
                }
                if let Some(root) = r.furnishing.as_deref() {
                    walk_furnishing(root, b, out, depth + 4);
                }
            }
        }
    }
}

fn walk_furnishing(id: &str, b: &ResolvedBlueprint, out: &mut String, depth: usize) {
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
            FurnishingChild::Furnishing(child) => walk_furnishing(child, b, out, depth + 2),
            FurnishingChild::Element(id) => {
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
fn hex(c: viewwright_model::Color) -> String {
    format!("#{:02X}{:02X}{:02X}", c.r, c.g, c.b)
}

#[cfg(test)]
mod tests {
    use super::render;
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
}
