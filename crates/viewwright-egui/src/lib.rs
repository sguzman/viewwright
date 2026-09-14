use egui::{CentralPanel, Color32, Context, FontId, Frame, RichText, Stroke, UiBuilder};
use viewwright_layout::{layout, LayoutPlan, Rect as LayoutRect};
use viewwright_model::{
    BorderPolicy, Color, CompositionChild, ElementKind, Importance, ResolvedBlueprint,
    ResolvedFixtureContent, ResolvedRegion, SurfaceRole,
};

pub fn show(ctx: &Context, blueprint: &ResolvedBlueprint, fixture: &str) {
    let root_fill = blueprint
        .visual
        .as_ref()
        .map(|v| color32(v.palette.canvas))
        .unwrap_or(Color32::TRANSPARENT);
    CentralPanel::default()
        .frame(Frame::new().fill(root_fill).inner_margin(0.0))
        .show(ctx, |ui| {
            ui.scope(|ui| {
                apply_visuals(ui, blueprint);
                let plan = layout(blueprint, ui.available_width(), ui.available_height());
                let origin = ui.min_rect().min;
                render_composition(ui, &blueprint.root, blueprint, fixture, &plan, origin);
            });
        });
}

fn render_composition(
    ui: &mut egui::Ui,
    id: &str,
    b: &ResolvedBlueprint,
    fixture: &str,
    plan: &LayoutPlan,
    origin: egui::Pos2,
) {
    let Some(c) = b.compositions.iter().find(|c| c.id == id) else {
        return;
    };
    for child in &c.children {
        let child_id = match child {
            CompositionChild::Region(id) | CompositionChild::Composition(id) => id,
        };
        let Some(child_rect) = (match child {
            CompositionChild::Region(_) => plan.region(child_id),
            CompositionChild::Composition(_) => plan.composition(child_id),
        }) else {
            continue;
        };
        let child_egui_rect = to_egui_rect(child_rect, origin);
        ui.scope_builder(
            UiBuilder::new()
                .id_salt((id, child_id))
                .max_rect(child_egui_rect),
            |ui| match child {
                CompositionChild::Composition(id) => {
                    render_composition(ui, id, b, fixture, plan, origin)
                }
                CompositionChild::Region(id) => {
                    if let Some(region) = b.regions.iter().find(|r| r.id == *id) {
                        render_region(ui, region, b, fixture, child_rect, origin);
                    }
                }
            },
        );
    }
}

fn render_region(
    ui: &mut egui::Ui,
    r: &ResolvedRegion,
    b: &ResolvedBlueprint,
    fixture: &str,
    rect: LayoutRect,
    origin: egui::Pos2,
) {
    let egui_rect = to_egui_rect(rect, origin);
    if let Some(v) = &b.visual {
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
        let separated_surface = matches!(r.surface, SurfaceRole::Panel | SurfaceRole::Raised);
        let stroke = match v.border_policy {
            BorderPolicy::None => Stroke::NONE,
            BorderPolicy::Minimal if r.surface == SurfaceRole::Raised => {
                Stroke::new(1.0_f32, color32(v.palette.border))
            }
            BorderPolicy::Defined if separated_surface => {
                Stroke::new(2.0_f32, color32(v.palette.border))
            }
            _ => Stroke::NONE,
        };
        ui.painter().add(
            Frame::new()
                .fill(color32(fill))
                .corner_radius(v.corner_radius.min(u8::MAX as u32) as u8)
                .stroke(stroke)
                .paint(egui_rect),
        );
    }
    let content = egui_rect.shrink(10.0);
    ui.scope_builder(
        UiBuilder::new()
            .id_salt(("region", &r.id))
            .max_rect(content),
        |ui| {
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
        },
    );
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
        ElementKind::Collection => render_collection(ui, e, b, fixture, b.visual.as_ref()),
        ElementKind::Document => {
            if let Some(ResolvedFixtureContent::Document {
                title, paragraphs, ..
            }) = content_for(b, fixture, &e.id)
            {
                ui.label(element_text(title, e.importance, b.visual.as_ref()).strong());
                for paragraph in paragraphs {
                    ui.add_space(6.0);
                    ui.label(element_text(paragraph, e.importance, b.visual.as_ref()));
                }
            }
        }
        ElementKind::PropertySheet => {
            if let Some(ResolvedFixtureContent::Properties { properties, .. }) =
                content_for(b, fixture, &e.id)
            {
                for property in properties {
                    ui.horizontal(|ui| {
                        ui.label(RichText::new(&property.name).strong());
                        ui.label(&property.value);
                    });
                }
            }
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
        ElementKind::Status => {
            if let Some(ResolvedFixtureContent::Text { text, .. }) = content_for(b, fixture, &e.id)
            {
                ui.label(text);
            }
        }
        ElementKind::Tree => render_tree(ui, e, b, fixture, b.visual.as_ref()),
        _ => {
            ui.label(format!("{} element", kind_name(e.kind)));
        }
    }
}

fn render_tree(
    ui: &mut egui::Ui,
    element: &viewwright_model::ResolvedElement,
    b: &ResolvedBlueprint,
    fixture: &str,
    visual: Option<&viewwright_model::ResolvedVisual>,
) {
    let Some(ResolvedFixtureContent::Tree {
        nodes, selected, ..
    }) = content_for(b, fixture, &element.id)
    else {
        return;
    };
    for node in nodes {
        let mut depth = 0;
        let mut parent = node.parent.as_deref();
        while let Some(parent_id) = parent {
            depth += 1;
            parent = nodes
                .iter()
                .find(|candidate| candidate.id == parent_id)
                .and_then(|candidate| candidate.parent.as_deref());
        }
        let text = RichText::new(&node.label);
        let text = if selected.as_deref() == Some(node.id.as_str()) {
            if let Some(v) = visual {
                text.color(color32(v.palette.accent)).strong()
            } else {
                text.strong()
            }
        } else {
            text
        };
        ui.horizontal(|ui| {
            ui.add_space(depth as f32 * 12.0);
            ui.label(text);
        });
    }
}

fn render_collection(
    ui: &mut egui::Ui,
    element: &viewwright_model::ResolvedElement,
    b: &ResolvedBlueprint,
    fixture: &str,
    visual: Option<&viewwright_model::ResolvedVisual>,
) {
    let Some(ResolvedFixtureContent::Collection {
        items, selected, ..
    }) = content_for(b, fixture, &element.id)
    else {
        return;
    };
    for item in items {
        let selected_item = selected.as_deref() == Some(item.id.as_str());
        let text = RichText::new(&item.label);
        let text = if selected_item {
            if let Some(v) = visual {
                text.color(color32(v.palette.accent)).strong()
            } else {
                text.strong()
            }
        } else {
            text
        };
        ui.label(text);
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

fn content_for<'a>(
    b: &'a ResolvedBlueprint,
    fixture: &str,
    element: &str,
) -> Option<&'a ResolvedFixtureContent> {
    b.fixtures
        .iter()
        .find(|f| f.id == fixture)?
        .content
        .iter()
        .find(|content| match content {
            ResolvedFixtureContent::Collection { element: id, .. }
            | ResolvedFixtureContent::Properties { element: id, .. }
            | ResolvedFixtureContent::Text { element: id, .. }
            | ResolvedFixtureContent::Tree { element: id, .. }
            | ResolvedFixtureContent::Document { element: id, .. } => id == element,
        })
}

fn color32(c: Color) -> Color32 {
    Color32::from_rgba_unmultiplied(c.r, c.g, c.b, c.a)
}

fn to_egui_rect(rect: LayoutRect, origin: egui::Pos2) -> egui::Rect {
    egui::Rect::from_min_size(
        origin + egui::vec2(rect.x, rect.y),
        egui::vec2(rect.width, rect.height),
    )
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
        BorderPolicy::None | BorderPolicy::Minimal => Stroke::NONE,
        BorderPolicy::Defined => Stroke::new(1.0_f32, color32(v.palette.border)),
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
