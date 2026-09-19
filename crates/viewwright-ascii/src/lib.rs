use viewwright_model::{
    CompositionChild, CompositionKind, FurnishingChild, OverflowPolicy, ResolvedBlueprint,
    ResolvedDocument, ResolvedDocumentBlock, ResolvedFixtureContent,
};

pub fn render(b: &ResolvedBlueprint) -> String {
    let root = b
        .default_variant()
        .map_or(b.root.as_str(), |variant| variant.root.as_str());
    render_root(b, root, None)
}

pub fn render_at(b: &ResolvedBlueprint, width: f32, height: f32) -> String {
    let root = b.active_root(width);
    let mut output = render_root(b, root, Some((width, height)));
    if let Some(variant) = b.responsive_variant(width) {
        output.push_str(&format!("responsive variant {}\n", variant.id));
    }
    output
}

fn render_root(b: &ResolvedBlueprint, root: &str, viewport: Option<(f32, f32)>) -> String {
    let mut out = format!("ViewWright: {} — {}\n", b.screen.id, b.screen.purpose);
    if let Some((width, height)) = viewport {
        out.push_str(&format!("viewport {width}x{height}\n"));
    }
    walk(root, b, &mut out, 0);
    out
}

pub fn render_fixture(b: &ResolvedBlueprint, fixture_id: &str) -> Option<String> {
    let fixture = b.fixtures.iter().find(|fixture| fixture.id == fixture_id)?;
    let mut out = render(b);
    for content in &fixture.content {
        let ResolvedFixtureContent::Document { element, document } = content else {
            continue;
        };
        let ResolvedDocument::Rich { blocks, spoken } = document else {
            continue;
        };
        out.push_str(&format!("fixture {fixture_id} document {element}\n"));
        for block in blocks {
            match block {
                ResolvedDocumentBlock::Heading { id, level, text } => {
                    out.push_str(&format!("  {id} heading level {level}: {text}\n"))
                }
                ResolvedDocumentBlock::Eyebrow { id, text }
                | ResolvedDocumentBlock::Paragraph { id, text }
                | ResolvedDocumentBlock::Quote { id, text } => {
                    out.push_str(&format!("  {id} {}: {text}\n", block.kind().as_str()))
                }
                ResolvedDocumentBlock::Divider { id } => {
                    out.push_str(&format!("  {id} divider\n"));
                }
            }
        }
        if let Some(spoken) = spoken {
            out.push_str(&format!(
                "  spoken block {} range {}..{}\n",
                spoken.block, spoken.start, spoken.end
            ));
        }
    }
    Some(out)
}

fn walk(id: &str, b: &ResolvedBlueprint, out: &mut String, depth: usize) {
    let Some(c) = b.compositions.iter().find(|c| c.id == id) else {
        return;
    };
    let indent = "  ".repeat(depth);
    if let Some(axis) = c.axis {
        out.push_str(&format!("{indent}+ {} ({}, {})\n", c.id, c.kind, axis));
    } else {
        out.push_str(&format!("{indent}+ {} ({}, axisless)\n", c.id, c.kind));
    }
    for (index, child) in c.children.iter().enumerate() {
        let layer = if c.kind == CompositionKind::Overlay {
            Some(if index == 0 { "base" } else { "floating" })
        } else {
            None
        };
        match child {
            CompositionChild::Composition(child) => {
                if let Some(layer) = layer {
                    out.push_str(&format!("{}  |-- {layer} layer\n", indent));
                }
                walk(child, b, out, depth + if layer.is_some() { 2 } else { 1 });
            }
            CompositionChild::Region(region) => {
                let overflow = b
                    .regions
                    .iter()
                    .find(|candidate| candidate.id == *region)
                    .map(|candidate| candidate.overflow)
                    .unwrap_or(OverflowPolicy::Clip);
                out.push_str(&format!(
                    "{}  |-- {}region {}{}\n",
                    indent,
                    layer.map(|layer| format!("{layer} ")).unwrap_or_default(),
                    region,
                    if overflow == OverflowPolicy::ScrollY {
                        " (overflow scroll_y)"
                    } else {
                        ""
                    }
                ));
                let resolved = b.regions.iter().find(|candidate| candidate.id == *region);
                if let Some(root) = resolved.and_then(|region| region.furnishing.as_deref()) {
                    walk_furnishing(root, b, out, depth + 1);
                } else {
                    for e in b.elements.iter().filter(|e| e.region == *region) {
                        out.push_str(&format!(
                            "{}      * {} [{}]\n",
                            indent,
                            e.label,
                            element_semantics(e)
                        ));
                    }
                }
            }
        }
    }
}

fn walk_furnishing(id: &str, b: &ResolvedBlueprint, out: &mut String, depth: usize) {
    let Some(furnishing) = b.furnishings.iter().find(|furnishing| furnishing.id == id) else {
        return;
    };
    let indent = "  ".repeat(depth);
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
        "{indent}  + furnishing {} ({}, {})\n",
        furnishing.id,
        furnishing.kind,
        attributes.join(", ")
    ));
    for child in &furnishing.children {
        match child {
            FurnishingChild::Furnishing(child) => walk_furnishing(child, b, out, depth + 1),
            FurnishingChild::Element(id) => {
                if let Some(element) = b.elements.iter().find(|element| element.id == *id) {
                    out.push_str(&format!(
                        "{}    * {} [{}]\n",
                        indent,
                        element.label,
                        element_semantics(element)
                    ));
                }
            }
        }
    }
}

fn element_semantics(element: &viewwright_model::ResolvedElement) -> String {
    if let Some(choice) = &element.choice {
        format!("choice/{}", choice.presentation.as_str())
    } else if let Some(scalar) = &element.scalar {
        format!(
            "scalar {}..{}{}",
            scalar.min,
            scalar.max,
            scalar
                .unit
                .as_deref()
                .map(|unit| format!(" {unit}"))
                .unwrap_or_default()
        )
    } else {
        format!("{:?}", element.kind)
    }
}

#[cfg(test)]
mod tests {
    use super::{render, render_fixture};
    use viewwright_model::parse_and_resolve;

    #[test]
    fn overlay_ascii_preserves_layer_order_without_axis() {
        let blueprint = parse_and_resolve(include_str!(
            "../../../specimens/overlay-command-palette-pressure.toml"
        ))
        .unwrap();
        let output = render(&blueprint);
        assert!(output.contains("+ root_overlay (overlay, axisless)"));
        assert!(output.contains("|-- base layer"));
        assert!(output.contains("|-- floating region palette_surface"));
        assert!(!output.contains("root_overlay (overlay, horizontal)"));

        let overflow = viewwright_model::parse_and_resolve(include_str!(
            "../../../specimens/reader-overflow-pressure.toml"
        ))
        .unwrap();
        assert!(render(&overflow).contains("region reader (overflow scroll_y)"));
    }

    #[test]
    fn furnished_lantern_leaf_ascii_exposes_order_slots_and_overflow() {
        let blueprint = parse_and_resolve(include_str!(
            "../../../specimens/lantern-leaf-reader-furnished.toml"
        ))
        .unwrap();
        let output = render(&blueprint);
        assert!(output.contains("region reader\n"));
        assert!(output.contains("furnishing reader_furnishing (column"));
        assert!(output.contains("furnishing toolbar_slot (row"));
        assert!(output.contains("furnishing search_slot (row"));
        assert!(output.contains("furnishing toolbar_actions (row"));
        assert!(output.contains("furnishing document_slot (column"));
        assert!(output.contains("overflow scroll_y, grow 1"));
        assert!(output.contains("Back 15 seconds [Command]"));
        assert!(
            output.find("Back 15 seconds [Command]").unwrap()
                < output.find("Play / pause [Command]").unwrap()
        );
    }

    #[test]
    fn m42_ascii_exposes_semantic_controls_without_toolkit_widget_names() {
        let blueprint = parse_and_resolve(include_str!(
            "../../../specimens/lantern-leaf-reader-controls.toml"
        ))
        .unwrap();
        let output = render(&blueprint);
        assert!(output.contains("Font Family [choice/select]"));
        assert!(output.contains("Text Alignment [choice/segmented]"));
        assert!(output.contains("Font Size [scalar 12..32 px]"));
        assert!(output.contains("Dyslexia-friendly font [Boolean]"));
        assert!(!output.contains("Slider"));
        assert!(!output.contains("ComboBox"));
    }

    #[test]
    fn m43_fixture_ascii_exposes_rich_document_blocks_and_spoken_range() {
        let blueprint = parse_and_resolve(include_str!(
            "../../../specimens/lantern-leaf-reader-document.toml"
        ))
        .unwrap();
        let output = render_fixture(&blueprint, "reading").unwrap();
        for expected in [
            "chapter_3 eyebrow",
            "chapter_title heading level 1",
            "opening_divider divider",
            "p1 paragraph",
            "p2 paragraph",
            "p3 paragraph",
            "closing_quote quote",
            "spoken block p2 range 174..212",
        ] {
            assert!(output.contains(expected), "missing {expected:?}:\n{output}");
        }
    }
}
