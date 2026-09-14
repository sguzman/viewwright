use viewwright_model::{CompositionChild, ResolvedBlueprint};

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
    out.push_str(&format!("{indent}+ {} ({}, {})\n", c.id, c.kind, c.axis));
    for child in &c.children {
        match child {
            CompositionChild::Composition(child) => walk(child, b, out, depth + 1),
            CompositionChild::Region(region) => {
                out.push_str(&format!("{}  |-- region {}\n", indent, region));
                for e in b.elements.iter().filter(|e| e.region == *region) {
                    out.push_str(&format!("{}      * {} [{:?}]\n", indent, e.label, e.kind));
                }
            }
        }
    }
}
