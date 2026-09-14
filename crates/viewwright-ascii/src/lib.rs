use viewwright_model::ResolvedBlueprint;
pub fn render(b: &ResolvedBlueprint) -> String {
    let w = 78;
    let mut o = format!(
        "┌{}┐\n│ {:<w$} │\n├{}┤\n",
        "─".repeat(w),
        b.screen.id.to_uppercase(),
        "─".repeat(w)
    );
    o.push_str(&format!(
        "│ {:<w$} │\n",
        b.regions
            .iter()
            .map(|r| r.id.to_uppercase())
            .collect::<Vec<_>>()
            .join(" │ ")
    ));
    for r in &b.regions {
        o.push_str(&format!("│ {} [{:?}]\n", r.id, r.importance));
        for e in b.elements.iter().filter(|e| e.region == r.id) {
            o.push_str(&format!("│   • {} ({:?})\n", e.label, e.kind));
        }
    }
    o.push_str(&format!("└{}┘\n", "─".repeat(w)));
    o
}
