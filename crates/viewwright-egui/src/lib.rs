use egui::{Align, CentralPanel, Color32, Context, FontId, Frame, Layout, Stroke};
use viewwright_model::{
    BorderPolicy, Color, CompositionChild, ElementKind, ResolvedBlueprint, ResolvedComposition,
    ResolvedRegion, SurfaceRole,
};

pub fn show(ctx: &Context, blueprint: &ResolvedBlueprint, fixture: &str) {
    apply_visuals(ctx, blueprint);
    egui::TopBottomPanel::top("viewwright-title").show(ctx, |ui| {
        ui.heading(&blueprint.screen.purpose);
        ui.label(format!("Fixture: {fixture}"));
    });
    CentralPanel::default().show(ctx, |ui| {
        render_composition(ui, &blueprint.root, blueprint, fixture)
    });
}

fn render_composition(ui: &mut egui::Ui, id: &str, b: &ResolvedBlueprint, fixture: &str) {
    let Some(c) = b.compositions.iter().find(|c| c.id == id) else {
        return;
    };
    ui.add_space(c.padding as f32);
    if c.axis == "horizontal" || c.kind == "row" {
        let available = ui.available_width();
        let fixed: f32 = c
            .children
            .iter()
            .filter_map(|child| region(child, b))
            .filter_map(|r| r.width)
            .map(|w| w as f32)
            .sum();
        let growing: f32 = c
            .children
            .iter()
            .filter_map(|child| region(child, b))
            .map(|r| r.grow)
            .sum();
        ui.horizontal(|ui| {
            for (index, child) in c.children.iter().enumerate() {
                let width = child_width(child, b, available, fixed, growing, c);
                let height = ui.available_height();
                ui.allocate_ui_with_layout(
                    egui::vec2(width, height),
                    Layout::top_down(Align::Min),
                    |ui| {
                        render_child(ui, child, b, fixture);
                    },
                );
                if index + 1 < c.children.len() {
                    ui.add_space(c.gap as f32);
                }
            }
        });
    } else {
        let available = ui.available_height();
        let fixed: f32 = c
            .children
            .iter()
            .filter_map(|child| region(child, b))
            .filter_map(|r| r.height)
            .map(|h| h as f32)
            .sum();
        let growing: f32 = c
            .children
            .iter()
            .filter_map(|child| region(child, b))
            .map(|r| r.grow)
            .sum();
        for (index, child) in c.children.iter().enumerate() {
            let height = child_height(child, b, available, fixed, growing, c);
            ui.allocate_ui_with_layout(
                egui::vec2(ui.available_width(), height),
                Layout::top_down(Align::Min),
                |ui| {
                    render_child(ui, child, b, fixture);
                },
            );
            if index + 1 < c.children.len() {
                ui.add_space(c.gap as f32);
            }
        }
    }
}

fn render_child(ui: &mut egui::Ui, child: &CompositionChild, b: &ResolvedBlueprint, fixture: &str) {
    match child {
        CompositionChild::Composition(id) => render_composition(ui, id, b, fixture),
        CompositionChild::Region(id) => {
            if let Some(r) = b.regions.iter().find(|r| r.id == *id) {
                render_region(ui, r, b, fixture);
            }
        }
    }
}
fn region<'a>(child: &CompositionChild, b: &'a ResolvedBlueprint) -> Option<&'a ResolvedRegion> {
    match child {
        CompositionChild::Region(id) => b.regions.iter().find(|r| r.id == *id),
        _ => None,
    }
}
fn child_width(
    child: &CompositionChild,
    b: &ResolvedBlueprint,
    available: f32,
    fixed: f32,
    growing: f32,
    c: &ResolvedComposition,
) -> f32 {
    region(child, b)
        .and_then(|r| r.width)
        .map(|w| w as f32)
        .unwrap_or_else(|| {
            ((available - fixed - c.gap as f32 * c.children.len().saturating_sub(1) as f32)
                .max(1.0)
                * region(child, b).map(|r| r.grow.max(1.0)).unwrap_or(1.0)
                / growing.max(1.0))
            .max(1.0)
        })
}
fn child_height(
    child: &CompositionChild,
    b: &ResolvedBlueprint,
    available: f32,
    fixed: f32,
    growing: f32,
    c: &ResolvedComposition,
) -> f32 {
    region(child, b)
        .and_then(|r| r.height)
        .map(|h| h as f32)
        .unwrap_or_else(|| {
            ((available - fixed - c.gap as f32 * c.children.len().saturating_sub(1) as f32)
                .max(1.0)
                * region(child, b).map(|r| r.grow.max(1.0)).unwrap_or(1.0)
                / growing.max(1.0))
            .max(1.0)
        })
}

fn render_region(ui: &mut egui::Ui, r: &ResolvedRegion, b: &ResolvedBlueprint, fixture: &str) {
    let frame = if let Some(v) = &b.visual {
        let fill = match r.surface {
            SurfaceRole::Canvas => v.palette.canvas,
            SurfaceRole::Panel => v.palette.surface,
            SurfaceRole::Raised => v.palette.surface_raised,
            SurfaceRole::Transparent => Color {
                r: 0,
                g: 0,
                b: 0,
                a: 0,
            },
        };
        let stroke = match v.border_policy {
            BorderPolicy::None => Stroke::NONE,
            BorderPolicy::Minimal => Stroke::new(1.0_f32, color32(v.palette.border)),
            BorderPolicy::Defined => Stroke::new(2.0_f32, color32(v.palette.border)),
        };
        Frame::new()
            .fill(color32(fill))
            .corner_radius((v.corner_radius.min(u8::MAX as u32)) as u8)
            .stroke(stroke)
    } else {
        Frame::group(ui.style())
    };
    frame.show(ui, |ui| {
        ui.heading(&r.id);
        ui.small(&r.role);
        for e in b.elements.iter().filter(|e| e.region == r.id) {
            ui.separator();
            ui.label(&e.label);
            match e.kind {
                ElementKind::Search => {
                    let mut query = String::new();
                    ui.text_edit_singleline(&mut query);
                }
                ElementKind::Collection => render_collection(ui, &e.label, fixture),
                ElementKind::Document => {
                    ui.heading("Document surface");
                    ui.label(if fixture.contains("no_document") {
                        "No document loaded"
                    } else {
                        "Reading content"
                    });
                }
                ElementKind::PropertySheet => {
                    ui.label("Representative properties");
                }
                ElementKind::Command => {
                    let _ = ui.button(&e.label);
                }
                _ => {
                    ui.label(format!("{} element", kind_name(e.kind)));
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
                }
            });
        }
    }
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
        ElementKind::Document => "document",
    }
}

fn color32(c: Color) -> Color32 {
    Color32::from_rgba_unmultiplied(c.r, c.g, c.b, c.a)
}
fn apply_visuals(ctx: &Context, b: &ResolvedBlueprint) {
    let Some(v) = &b.visual else { return };
    let mut style = (*ctx.style()).clone();
    style.visuals.override_text_color = Some(color32(v.palette.text));
    style.text_styles.insert(
        egui::TextStyle::Heading,
        FontId::proportional(v.type_scale.heading as f32),
    );
    style.text_styles.insert(
        egui::TextStyle::Body,
        FontId::proportional(v.type_scale.body as f32),
    );
    style.text_styles.insert(
        egui::TextStyle::Small,
        FontId::proportional(v.type_scale.caption as f32),
    );
    ctx.set_style(style);
}
