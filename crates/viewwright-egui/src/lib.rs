use std::collections::HashMap;

use egui::{
    CentralPanel, Color32, Context, FontId, Frame, RichText, ScrollArea, Stroke, UiBuilder,
};
use viewwright_layout::{layout, LayoutPlan, Rect as LayoutRect};
use viewwright_model::{
    BorderPolicy, CollectionPresentation, Color, CompositionChild, Density, ElementKind,
    Importance, OverflowPolicy, RegionRole, ResolvedBlueprint, ResolvedFixtureContent,
    ResolvedRegion, SurfaceRole,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InteractionEvent {
    pub element_id: String,
    pub action: String,
}

#[derive(Debug, Default)]
pub struct RenderOutput {
    pub activation: Option<InteractionEvent>,
}

#[derive(Debug, Default)]
pub struct RenderState {
    search_values: HashMap<String, String>,
}

impl RenderState {
    pub fn clear(&mut self) {
        self.search_values.clear();
    }

    pub fn search_value_mut(&mut self, element_id: &str) -> &mut String {
        self.search_values.entry(element_id.to_owned()).or_default()
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct DensityPolicy {
    pub region_inset: f32,
    pub element_gap: f32,
    pub paragraph_gap: f32,
    pub item_spacing: f32,
    pub button_padding: egui::Vec2,
    pub min_interact_height: f32,
    pub card_height: f32,
    pub card_inner_margin: f32,
}

impl DensityPolicy {
    pub fn for_density(density: Density) -> Self {
        match density {
            Density::Comfortable => Self {
                region_inset: 10.0,
                element_gap: 4.0,
                paragraph_gap: 6.0,
                item_spacing: 8.0,
                button_padding: egui::vec2(8.0, 4.0),
                min_interact_height: 24.0,
                card_height: 64.0,
                card_inner_margin: 8.0,
            },
            Density::Dense => Self {
                region_inset: 6.0,
                element_gap: 2.0,
                paragraph_gap: 3.0,
                item_spacing: 4.0,
                button_padding: egui::vec2(6.0, 2.0),
                min_interact_height: 20.0,
                card_height: 52.0,
                card_inner_margin: 6.0,
            },
        }
    }
}

pub fn show(
    ctx: &Context,
    blueprint: &ResolvedBlueprint,
    fixture: &str,
    state: &mut RenderState,
) -> RenderOutput {
    let mut output = RenderOutput::default();
    let root_fill = root_fill(ctx, blueprint);
    CentralPanel::default()
        .frame(Frame::new().fill(root_fill).inner_margin(0.0))
        .show(ctx, |ui| {
            ui.scope(|ui| {
                apply_visuals(ui, blueprint);
                let density = DensityPolicy::for_density(blueprint.screen.density);
                apply_density(ui, density);
                let plan = layout(blueprint, ui.available_width(), ui.available_height());
                let origin = ui.min_rect().min;
                render_composition(
                    ui,
                    &blueprint.root,
                    blueprint,
                    fixture,
                    &plan,
                    origin,
                    &mut output.activation,
                    density,
                    state,
                );
            });
        });
    output
}

fn root_fill(ctx: &Context, blueprint: &ResolvedBlueprint) -> Color32 {
    blueprint
        .visual
        .as_ref()
        .map(|v| color32(v.palette.canvas))
        .unwrap_or_else(|| ctx.style().visuals.panel_fill)
}

fn render_composition(
    ui: &mut egui::Ui,
    id: &str,
    b: &ResolvedBlueprint,
    fixture: &str,
    plan: &LayoutPlan,
    origin: egui::Pos2,
    activation: &mut Option<InteractionEvent>,
    density: DensityPolicy,
    state: &mut RenderState,
) {
    let Some(c) = b.compositions.iter().find(|c| c.id == id) else {
        return;
    };
    for (index, child) in c.children.iter().enumerate() {
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
        if c.kind == viewwright_model::CompositionKind::Overlay && index == 1 {
            ui.interact(
                child_egui_rect,
                egui::Id::new(("overlay", id, child_id)),
                egui::Sense::click(),
            );
        }
        ui.scope_builder(
            UiBuilder::new()
                .id_salt((id, child_id))
                .max_rect(child_egui_rect),
            |ui| match child {
                CompositionChild::Composition(id) => {
                    render_composition(ui, id, b, fixture, plan, origin, activation, density, state)
                }
                CompositionChild::Region(id) => {
                    if let Some(region) = b.regions.iter().find(|r| r.id == *id) {
                        render_region(
                            ui, region, b, fixture, child_rect, origin, activation, density, state,
                        );
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
    activation: &mut Option<InteractionEvent>,
    density: DensityPolicy,
    state: &mut RenderState,
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
    let content = egui_rect.shrink(density.region_inset);
    ui.scope_builder(
        UiBuilder::new()
            .id_salt(("region", &r.id))
            .max_rect(content),
        |ui| match r.overflow {
            OverflowPolicy::Clip => {
                ui.set_clip_rect(content);
                render_region_contents(ui, r, b, fixture, activation, density, state);
            }
            OverflowPolicy::ScrollY => {
                ScrollArea::vertical()
                    .id_salt(scroll_area_id(&b.screen.id, &r.id))
                    .hscroll(false)
                    .auto_shrink([false, false])
                    .show(ui, |ui| {
                        render_region_contents(ui, r, b, fixture, activation, density, state);
                    });
            }
        },
    );
}

fn scroll_area_id(screen_id: &str, region_id: &str) -> egui::Id {
    egui::Id::new(("region-scroll", screen_id, region_id))
}

fn render_region_contents(
    ui: &mut egui::Ui,
    r: &ResolvedRegion,
    b: &ResolvedBlueprint,
    fixture: &str,
    activation: &mut Option<InteractionEvent>,
    density: DensityPolicy,
    state: &mut RenderState,
) {
    let elements: Vec<_> = b.elements.iter().filter(|e| e.region == r.id).collect();
    if is_command_region(r.role) {
        ui.horizontal_wrapped(|ui| {
            for e in elements {
                render_element(ui, e, b, fixture, activation, density, state);
            }
        });
    } else {
        for e in elements {
            render_element(ui, e, b, fixture, activation, density, state);
        }
    }
}

fn is_command_region(role: RegionRole) -> bool {
    role == RegionRole::Commands
}

fn render_element(
    ui: &mut egui::Ui,
    e: &viewwright_model::ResolvedElement,
    b: &ResolvedBlueprint,
    fixture: &str,
    activation: &mut Option<InteractionEvent>,
    density: DensityPolicy,
    state: &mut RenderState,
) {
    ui.add_space(density.element_gap);
    if separate_element_label(e.kind) {
        ui.label(element_text(&e.label, e.importance, b.visual.as_ref()));
    }
    match e.kind {
        ElementKind::Search => {
            ui.text_edit_singleline(state.search_value_mut(&e.id));
        }
        ElementKind::Collection => render_collection(ui, e, b, fixture, b.visual.as_ref(), density),
        ElementKind::Document => {
            if let Some(ResolvedFixtureContent::Document {
                title, paragraphs, ..
            }) = content_for(b, fixture, &e.id)
            {
                ui.label(element_text(title, e.importance, b.visual.as_ref()).strong());
                for paragraph in paragraphs {
                    ui.add_space(density.paragraph_gap);
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
            let state = b.command_state(fixture, &e.id);
            let response = ui.add_enabled(state.enabled, egui::Button::new(label));
            if !state.enabled {
                if let Some(reason) = state.reason {
                    response.on_hover_text(reason);
                }
            } else if response.clicked() {
                if let Some(action) = &e.action {
                    *activation = Some(InteractionEvent {
                        element_id: e.id.clone(),
                        action: action.as_str().to_owned(),
                    });
                }
            }
        }
        ElementKind::Status => {
            if let Some(ResolvedFixtureContent::Text { text, .. }) = content_for(b, fixture, &e.id)
            {
                ui.label(text);
            }
        }
        ElementKind::Tree => render_tree(ui, e, b, fixture, b.visual.as_ref()),
        ElementKind::Text | ElementKind::Preview => {
            ui.label(element_text(&e.label, e.importance, b.visual.as_ref()));
        }
    }
}

fn separate_element_label(kind: ElementKind) -> bool {
    !matches!(
        kind,
        ElementKind::Command | ElementKind::Text | ElementKind::Preview
    )
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
    density: DensityPolicy,
) {
    let Some(ResolvedFixtureContent::Collection {
        items, selected, ..
    }) = content_for(b, fixture, &element.id)
    else {
        return;
    };
    match element.presentation {
        Some(CollectionPresentation::List) | None => {
            render_collection_list(ui, items, selected, visual)
        }
        Some(CollectionPresentation::AdaptiveCards) => {
            render_collection_cards(ui, items, selected, visual, density)
        }
    }
}

fn render_collection_list(
    ui: &mut egui::Ui,
    items: &[viewwright_model::ResolvedCollectionItem],
    selected: &Option<String>,
    visual: Option<&viewwright_model::ResolvedVisual>,
) {
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

fn render_collection_cards(
    ui: &mut egui::Ui,
    items: &[viewwright_model::ResolvedCollectionItem],
    selected: &Option<String>,
    visual: Option<&viewwright_model::ResolvedVisual>,
    density: DensityPolicy,
) {
    const CARD_WIDTH: f32 = 180.0;
    ui.horizontal_wrapped(|ui| {
        for item in items {
            let selected_item = selected.as_deref() == Some(item.id.as_str());
            let (fill, stroke) = if let Some(v) = visual {
                (
                    if selected_item {
                        color32(v.palette.accent).gamma_multiply(0.22)
                    } else {
                        color32(v.palette.surface)
                    },
                    if selected_item || v.border_policy == BorderPolicy::Defined {
                        Stroke::new(
                            1.0_f32,
                            color32(if selected_item {
                                v.palette.accent
                            } else {
                                v.palette.border
                            }),
                        )
                    } else {
                        Stroke::NONE
                    },
                )
            } else {
                (
                    if selected_item {
                        ui.visuals().selection.bg_fill
                    } else {
                        ui.visuals().widgets.inactive.bg_fill
                    },
                    if selected_item {
                        ui.visuals().selection.stroke
                    } else {
                        Stroke::NONE
                    },
                )
            };
            ui.allocate_ui(egui::vec2(CARD_WIDTH, density.card_height), |ui| {
                Frame::new()
                    .fill(fill)
                    .stroke(stroke)
                    .inner_margin(density.card_inner_margin)
                    .show(ui, |ui| {
                        let text = if selected_item {
                            element_text(&item.label, Importance::Primary, visual).strong()
                        } else {
                            element_text(&item.label, Importance::Secondary, visual)
                        };
                        ui.label(text);
                    });
            });
        }
    });
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
            ResolvedFixtureContent::Command { element: id, .. } => id == element,
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

fn apply_density(ui: &mut egui::Ui, policy: DensityPolicy) {
    let spacing = &mut ui.style_mut().spacing;
    spacing.item_spacing = egui::vec2(policy.item_spacing, policy.item_spacing);
    spacing.button_padding = policy.button_padding;
    spacing.interact_size.y = policy.min_interact_height;
}

#[cfg(test)]
mod tests {
    use super::{
        is_command_region, root_fill, scroll_area_id, separate_element_label, DensityPolicy,
    };
    use egui::{Color32, Context};
    use viewwright_model::{parse_and_resolve, Density, ElementKind, RegionRole};

    #[test]
    fn command_labels_are_control_owned_while_content_labels_are_separate() {
        assert!(!separate_element_label(ElementKind::Command));
        for kind in [
            ElementKind::Search,
            ElementKind::Collection,
            ElementKind::Tree,
            ElementKind::PropertySheet,
            ElementKind::Document,
            ElementKind::Status,
        ] {
            assert!(separate_element_label(kind));
        }
        assert!(!separate_element_label(ElementKind::Text));
        assert!(!separate_element_label(ElementKind::Preview));
    }

    #[test]
    fn command_flow_uses_typed_region_role() {
        assert!(is_command_region(RegionRole::Commands));
        assert!(!is_command_region(RegionRole::Controls));
    }

    #[test]
    fn render_state_retains_independent_search_values_and_can_clear_them() {
        let mut state = super::RenderState::default();
        assert_eq!(state.search_value_mut("first"), "");

        state.search_value_mut("first").push_str("abc");
        assert_eq!(state.search_value_mut("first"), "abc");

        state.search_value_mut("second").push_str("xyz");
        assert_eq!(state.search_value_mut("first"), "abc");
        assert_eq!(state.search_value_mut("second"), "xyz");

        state.clear();
        assert_eq!(state.search_value_mut("first"), "");
        assert_eq!(state.search_value_mut("second"), "");
    }

    #[test]
    fn density_policy_is_tighter_without_typography_or_screen_specific_rules() {
        let comfortable = DensityPolicy::for_density(Density::Comfortable);
        let dense = DensityPolicy::for_density(Density::Dense);
        assert!(dense.region_inset < comfortable.region_inset);
        assert!(dense.element_gap < comfortable.element_gap);
        assert!(dense.paragraph_gap < comfortable.paragraph_gap);
        assert!(dense.item_spacing < comfortable.item_spacing);
        assert!(dense.button_padding.y < comfortable.button_padding.y);
        assert!(dense.min_interact_height < comfortable.min_interact_height);
        assert!(dense.card_height < comfortable.card_height);
        assert!(dense.card_inner_margin < comfortable.card_inner_margin);
        let comfortable_blueprint = parse_and_resolve(include_str!(
            "../../../specimens/density-pressure-comfortable.toml"
        ))
        .unwrap();
        let dense_blueprint = parse_and_resolve(include_str!(
            "../../../specimens/density-pressure-dense.toml"
        ))
        .unwrap();
        assert_eq!(
            comfortable_blueprint.elements.len(),
            dense_blueprint.elements.len()
        );
        assert_eq!(
            comfortable_blueprint.screen.purpose,
            dense_blueprint.screen.purpose
        );
        assert_ne!(
            comfortable_blueprint.screen.density,
            dense_blueprint.screen.density
        );
    }

    #[test]
    fn root_fill_uses_active_theme_without_visual_and_authored_canvas_with_visual() {
        let context = Context::default();
        let theme_fill = Color32::from_rgb(12, 34, 56);
        context.style_mut(|style| style.visuals.panel_fill = theme_fill);
        let project =
            parse_and_resolve(include_str!("../../../examples/project-browser.toml")).unwrap();
        assert_eq!(root_fill(&context, &project), theme_fill);

        let visual = parse_and_resolve(include_str!(
            "../../../specimens/reader-workspace-visual.toml"
        ))
        .unwrap();
        assert_eq!(root_fill(&context, &visual), Color32::from_rgb(38, 43, 51));

        let another_theme_fill = Color32::from_rgb(210, 220, 230);
        context.style_mut(|style| style.visuals.panel_fill = another_theme_fill);
        assert_eq!(root_fill(&context, &project), another_theme_fill);
        assert_eq!(root_fill(&context, &visual), Color32::from_rgb(38, 43, 51));
    }

    #[test]
    fn scroll_area_identity_is_stable_and_region_scoped() {
        assert_eq!(
            scroll_area_id("reader_overflow_pressure", "reader"),
            scroll_area_id("reader_overflow_pressure", "reader")
        );
        assert_ne!(
            scroll_area_id("reader_overflow_pressure", "reader"),
            scroll_area_id("reader_overflow_pressure", "library")
        );
    }
}
