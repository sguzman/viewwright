use egui::{Align, CentralPanel, Color32, Context, FontId, Frame, Layout, RichText, Stroke};
use viewwright_model::{
    BorderPolicy, Color, CompositionChild, ElementKind, Importance, ResolvedBlueprint,
    ResolvedComposition, ResolvedRegion, SurfaceRole,
};

pub fn show(ctx: &Context, blueprint: &ResolvedBlueprint, fixture: &str) {
    let root_frame = blueprint.visual.as_ref().map(|v| {
        Frame::new()
            .fill(color32(v.palette.canvas))
            .inner_margin(0.0)
    });
    CentralPanel::default()
        .frame(root_frame.unwrap_or_else(|| Frame::new()))
        .show(ctx, |ui| {
            ui.scope(|ui| {
                apply_visuals(ui, blueprint);
                render_composition(ui, &blueprint.root, blueprint, fixture);
            });
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
        let separated_surface =
            !matches!(r.surface, SurfaceRole::Canvas | SurfaceRole::Transparent);
        let stroke = match v.border_policy {
            BorderPolicy::None => Stroke::NONE,
            BorderPolicy::Minimal if r.surface == SurfaceRole::Raised => {
                Stroke::new(1.0_f32, color32(v.palette.border))
            }
            BorderPolicy::Minimal if !separated_surface => Stroke::NONE,
            BorderPolicy::Minimal => Stroke::new(1.0_f32, color32(v.palette.border)),
            BorderPolicy::Defined if separated_surface => {
                Stroke::new(2.0_f32, color32(v.palette.border))
            }
            BorderPolicy::Defined => Stroke::NONE,
        };
        Frame::new()
            .fill(color32(fill))
            .corner_radius((v.corner_radius.min(u8::MAX as u32)) as u8)
            .stroke(stroke)
    } else {
        Frame::group(ui.style())
    };
    frame.show(ui, |ui| {
        ui.label(region_text(&r.id, r.importance, b.visual.as_ref()));
        ui.label(
            RichText::new(&r.role)
                .color(
                    b.visual
                        .as_ref()
                        .map(|v| color32(v.palette.text_muted))
                        .unwrap_or(Color32::GRAY),
                )
                .small(),
        );
        let elements: Vec<_> = b.elements.iter().filter(|e| e.region == r.id).collect();
        if r.role == "commands" {
            ui.horizontal_wrapped(|ui| {
                for e in elements {
                    render_element(ui, e, b, fixture);
                }
            });
        } else {
            for e in elements {
                render_element(ui, e, b, fixture);
            }
        }
    });
}
fn render_element(
    ui: &mut egui::Ui,
    e: &viewwright_model::ResolvedElement,
    b: &ResolvedBlueprint,
    fixture: &str,
) {
    ui.add_space(4.0);
    ui.label(element_text(&e.label, e.importance, b.visual.as_ref()));
    match e.kind {
        ElementKind::Search => {
            let mut query = String::new();
            ui.text_edit_singleline(&mut query);
        }
        ElementKind::Collection => render_collection(ui, &e.label, fixture, b.visual.as_ref()),
        ElementKind::Document => {
            ui.label(element_text(
                "Document surface",
                e.importance,
                b.visual.as_ref(),
            ));
            ui.label(if fixture.contains("no_document") {
                "No document loaded"
            } else {
                "Reading content"
            });
        }
        ElementKind::PropertySheet => {
            let text = RichText::new("Representative properties");
            ui.label(if let Some(v) = &b.visual {
                text.color(color32(v.palette.text_muted))
            } else {
                text
            });
        }
        ElementKind::Command => {
            let label = if e.importance == Importance::Primary {
                if let Some(v) = &b.visual {
                    RichText::new(&e.label)
                        .size(v.type_scale.body as f32)
                        .color(color32(v.palette.accent))
                        .strong()
                } else {
                    element_text(&e.label, e.importance, b.visual.as_ref())
                }
            } else {
                RichText::new(&e.label)
            };
            let _ = ui.button(label);
        }
        _ => {
            ui.label(format!("{} element", kind_name(e.kind)));
        }
    }
}
fn render_collection(
    ui: &mut egui::Ui,
    label: &str,
    fixture: &str,
    visual: Option<&viewwright_model::ResolvedVisual>,
) {
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
                    if let Some(v) = visual {
                        ui.label(
                            RichText::new("Selected")
                                .color(color32(v.palette.accent))
                                .strong(),
                        );
                    } else {
                        ui.strong("Selected");
                    }
                }
            });
        }
    }
}

fn region_text(
    text: &str,
    importance: Importance,
    visual: Option<&viewwright_model::ResolvedVisual>,
) -> RichText {
    let mut value = RichText::new(text);
    if let Some(v) = visual {
        value = value
            .size(match importance {
                Importance::Primary => v.type_scale.heading as f32,
                _ => v.type_scale.body as f32,
            })
            .color(if importance == Importance::Tertiary {
                color32(v.palette.text_muted)
            } else {
                color32(v.palette.text)
            });
    }
    value.strong()
}
fn element_text(
    text: &str,
    importance: Importance,
    visual: Option<&viewwright_model::ResolvedVisual>,
) -> RichText {
    let mut value = RichText::new(text);
    if let Some(v) = visual {
        value = value
            .size(match importance {
                Importance::Primary => v.type_scale.heading as f32,
                Importance::Secondary => v.type_scale.body as f32,
                Importance::Tertiary => v.type_scale.caption as f32,
            })
            .color(if importance == Importance::Tertiary {
                color32(v.palette.text_muted)
            } else {
                color32(v.palette.text)
            });
    }
    value
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
fn apply_visuals(ui: &mut egui::Ui, b: &ResolvedBlueprint) {
    let Some(v) = &b.visual else { return };
    let style = ui.style_mut();
    style.visuals.override_text_color = Some(color32(v.palette.text));
    style.visuals.window_fill = color32(v.palette.canvas);
    style.visuals.panel_fill = color32(v.palette.canvas);
    style.visuals.extreme_bg_color = color32(v.palette.surface);
    style.visuals.faint_bg_color = color32(v.palette.surface);
    style.visuals.selection.bg_fill = color32(v.palette.accent);
    style.visuals.selection.stroke = Stroke::new(1.0_f32, color32(v.palette.accent));
    let boundary = match v.border_policy {
        BorderPolicy::None => Stroke::NONE,
        BorderPolicy::Minimal => Stroke::new(1.0_f32, color32(v.palette.border)),
        BorderPolicy::Defined => Stroke::new(2.0_f32, color32(v.palette.border)),
    };
    for widget in [
        &mut style.visuals.widgets.inactive,
        &mut style.visuals.widgets.hovered,
        &mut style.visuals.widgets.active,
        &mut style.visuals.widgets.open,
    ] {
        widget.bg_fill = color32(v.palette.surface);
        widget.fg_stroke.color = color32(v.palette.text);
        widget.bg_stroke = boundary;
    }
    style.visuals.widgets.hovered.bg_fill = color32(v.palette.surface_raised);
    style.visuals.widgets.active.bg_fill = color32(v.palette.accent);
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
}
