use egui::{Align, CentralPanel, Context, Layout};
use viewwright_model::{ElementKind, ResolvedBlueprint, ResolvedComposition, ResolvedRegion};

/// Render a resolved blueprint from its authored composition tree.
pub fn show(ctx: &Context, blueprint: &ResolvedBlueprint, fixture: &str) {
    egui::TopBottomPanel::top("viewwright-title").show(ctx, |ui| {
        ui.heading(&blueprint.screen.purpose);
        ui.label(format!("Fixture: {fixture}"));
    });
    CentralPanel::default().show(ctx, |ui| {
        if let Some(root) = blueprint.compositions.first() {
            render_composition(ui, root, blueprint, fixture);
        } else {
            for region in &blueprint.regions {
                render_region(ui, region, blueprint, fixture);
            }
        }
    });
}

fn render_composition(
    ui: &mut egui::Ui,
    c: &ResolvedComposition,
    b: &ResolvedBlueprint,
    fixture: &str,
) {
    ui.add_space(c.padding as f32);
    let horizontal = c.axis == "horizontal" || c.kind == "row";
    if horizontal {
        let available = ui.available_width();
        let fixed: f32 = c
            .children
            .iter()
            .filter_map(|id| b.regions.iter().find(|r| r.id == *id))
            .filter_map(|r| r.width)
            .map(|w| w as f32)
            .sum();
        let growing: f32 = c
            .children
            .iter()
            .filter_map(|id| b.regions.iter().find(|r| r.id == *id))
            .map(|r| r.grow)
            .sum();
        ui.horizontal(|ui| {
            for (index, child) in c.children.iter().enumerate() {
                if let Some(region) = b.regions.iter().find(|r| r.id == *child) {
                    let width = region.width.map(|w| w as f32).unwrap_or_else(|| {
                        let gaps = c.gap as f32 * c.children.len().saturating_sub(1) as f32;
                        ((available - fixed - gaps).max(1.0) * region.grow.max(1.0)
                            / growing.max(1.0))
                        .max(1.0)
                    });
                    let height = ui.available_height();
                    ui.allocate_ui_with_layout(
                        egui::vec2(width, height),
                        Layout::top_down(Align::Min),
                        |ui| render_region(ui, region, b, fixture),
                    );
                    if index + 1 < c.children.len() {
                        ui.add_space(c.gap as f32);
                    }
                }
            }
        });
    } else {
        ui.vertical(|ui| {
            for (index, child) in c.children.iter().enumerate() {
                if let Some(region) = b.regions.iter().find(|r| r.id == *child) {
                    render_region(ui, region, b, fixture);
                    if index + 1 < c.children.len() {
                        ui.add_space(c.gap as f32);
                    }
                }
            }
        });
    }
}

fn render_region(ui: &mut egui::Ui, region: &ResolvedRegion, b: &ResolvedBlueprint, fixture: &str) {
    ui.group(|ui| {
        ui.heading(&region.id);
        ui.small(&region.role);
        for element in b.elements.iter().filter(|e| e.region == region.id) {
            ui.separator();
            ui.label(&element.label);
            match element.kind {
                ElementKind::Search => {
                    let mut query = String::new();
                    ui.text_edit_singleline(&mut query);
                }
                ElementKind::Collection => render_collection(ui, &element.label, fixture),
                ElementKind::PropertySheet => {
                    ui.label(format!("State: {}", fixture_state(b, fixture)));
                    ui.label("Representative properties");
                }
                ElementKind::Command => {
                    let _ = ui.button(&element.label);
                }
                _ => {
                    ui.label(format!("{} element", kind_name(element.kind)));
                }
            }
        }
    });
}

fn render_collection(ui: &mut egui::Ui, label: &str, fixture: &str) {
    ui.strong(label);
    let count = if fixture.contains("empty") {
        0
    } else if fixture.contains("dense") {
        6
    } else if fixture.contains("selection") {
        1
    } else {
        3
    };
    if count == 0 {
        ui.weak("No items");
    } else {
        for index in 1..=count {
            ui.group(|ui| {
                ui.label(format!("Item {index}"));
                if fixture.contains("selection") && index == 1 {
                    ui.strong("Selected");
                } else {
                    ui.small("Representative item");
                }
            });
        }
    }
}

fn fixture_state<'a>(b: &'a ResolvedBlueprint, fixture: &str) -> &'a str {
    b.fixtures
        .iter()
        .find(|f| f.id == fixture)
        .map(|f| f.state.as_str())
        .unwrap_or("default")
}

fn kind_name(kind: ElementKind) -> &'static str {
    match kind {
        ElementKind::Text => "text",
        ElementKind::Command => "command",
        ElementKind::Search => "search",
        ElementKind::Collection => "collection",
        ElementKind::PropertySheet => "property sheet",
        ElementKind::Tree => "tree",
        ElementKind::Preview => "preview",
        ElementKind::Status => "status",
    }
}
