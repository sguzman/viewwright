use egui::{CentralPanel, Context, SidePanel, TopBottomPanel};
use viewwright_model::{ElementKind, ResolvedBlueprint};
pub fn show(ctx: &Context, b: &ResolvedBlueprint, fixture: &str) {
    TopBottomPanel::top("title").show(ctx, |ui| {
        ui.heading(&b.screen.purpose);
        ui.label(format!("Fixture: {fixture}"));
    });
    if let Some(r) = b.regions.iter().find(|r| r.role == "navigation") {
        SidePanel::left("nav")
            .default_width(r.width.unwrap_or(240) as f32)
            .show(ctx, |ui| {
                ui.heading(&r.id);
                for e in b.elements.iter().filter(|e| e.region == r.id) {
                    ui.label(&e.label);
                    if e.kind == ElementKind::Search {
                        let mut q = String::new();
                        ui.text_edit_singleline(&mut q);
                    }
                }
            });
    }
    if let Some(r) = b.regions.iter().find(|r| r.role == "inspector") {
        SidePanel::right("inspector")
            .default_width(r.width.unwrap_or(320) as f32)
            .show(ctx, |ui| {
                ui.heading(&r.id);
                for e in b.elements.iter().filter(|e| e.region == r.id) {
                    ui.group(|ui| {
                        ui.strong(&e.label);
                        ui.label("Selected project");
                        ui.label("Status: Active");
                    });
                }
            });
    }
    CentralPanel::default().show(ctx, |ui| {
        ui.heading("Projects");
        ui.horizontal_wrapped(|ui| {
            for n in ["Project A", "Project B", "Project C", "Project D"] {
                ui.group(|ui| {
                    ui.strong(n);
                    ui.label("active");
                });
            }
        });
    });
}
