use viewwright_model::{CompositionChild, ResolvedBlueprint};

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
        b.design.dominant.as_deref().unwrap_or("unspecified")
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
    out.push_str(&format!("{}{} — {}\n", " ".repeat(depth), c.id, c.axis));
    for child in &c.children {
        match child {
            CompositionChild::Composition(id) => walk(id, b, out, depth + 2),
            CompositionChild::Region(id) => {
                let r = b.regions.iter().find(|r| r.id == *id).unwrap();
                out.push_str(&format!(
                    "{}{} — {:?}, {:?}\n",
                    " ".repeat(depth + 2),
                    id,
                    r.surface,
                    r.importance
                ));
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
        assert!(first.contains("reader — Canvas, Primary"));
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
    }
}
