use viewwright_model::{CompositionChild, CompositionKind, OverflowPolicy, ResolvedBlueprint};

pub fn render(b: &ResolvedBlueprint) -> String {
    let mut out = format!("ViewWright: {} — {}\n", b.screen.id, b.screen.purpose);
    walk(&b.root, b, &mut out, 0);
    out
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
                for e in b.elements.iter().filter(|e| e.region == *region) {
                    out.push_str(&format!("{}      * {} [{:?}]\n", indent, e.label, e.kind));
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::render;
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
}
