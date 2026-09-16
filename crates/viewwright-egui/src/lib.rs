use std::collections::HashMap;

use egui::{
    CentralPanel, Color32, Context, FontId, Frame, RichText, ScrollArea, Stroke, Ui, UiBuilder,
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
            let anchor_id = ui.make_persistent_id(("viewwright-screen", &blueprint.screen.id));
            with_identity_anchor(ui, anchor_id, &blueprint.screen.id, None, |ui| {
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
        });
    output
}

fn with_identity_anchor<R>(
    ui: &mut Ui,
    id: egui::Id,
    author_id: &str,
    bounds: Option<egui::Rect>,
    render: impl FnOnce(&mut Ui) -> R,
) -> R {
    let ctx = ui.ctx().clone();
    ctx.accesskit_node_builder(id, |node| {
        node.set_role(egui::accesskit::Role::GenericContainer);
        node.set_author_id(author_id.to_owned());
        if let Some(bounds) = bounds {
            node.set_bounds(egui::accesskit::Rect {
                x0: bounds.min.x.into(),
                y0: bounds.min.y.into(),
                x1: bounds.max.x.into(),
                y1: bounds.max.y.into(),
            });
        }
    });
    ctx.with_accessibility_parent(id, || render(ui))
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
    let anchor_id = ui.make_persistent_id(("viewwright-region", &b.screen.id, &r.id));
    with_identity_anchor(ui, anchor_id, &r.id, Some(egui_rect), |ui| {
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
    });
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
    let anchor_id = ui.make_persistent_id(("viewwright-element", &e.id));
    with_identity_anchor(ui, anchor_id, &e.id, None, |ui| {
        ui.add_space(density.element_gap);
        if separate_element_label(e.kind) {
            ui.label(element_text(&e.label, e.importance, b.visual.as_ref()));
        }
        match e.kind {
            ElementKind::Search => {
                ui.text_edit_singleline(state.search_value_mut(&e.id));
            }
            ElementKind::Collection => {
                render_collection(ui, e, b, fixture, b.visual.as_ref(), density)
            }
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
                if let Some(ResolvedFixtureContent::Text { text, .. }) =
                    content_for(b, fixture, &e.id)
                {
                    ui.label(text);
                }
            }
            ElementKind::Tree => render_tree(ui, e, b, fixture, b.visual.as_ref()),
            ElementKind::Text | ElementKind::Preview => {
                ui.label(element_text(&e.label, e.importance, b.visual.as_ref()));
            }
        }
    });
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
        RenderState,
    };
    use egui::{accesskit, Color32, Context, FullOutput, RawInput};
    use std::collections::BTreeSet;
    use viewwright_layout::layout;
    use viewwright_model::{parse_and_resolve, Density, ElementKind, RegionRole};

    fn accesskit_context() -> Context {
        let context = Context::default();
        context.enable_accesskit();
        context
    }

    fn accesskit_frame(
        context: &Context,
        blueprint: &viewwright_model::ResolvedBlueprint,
        fixture: &str,
        state: &mut RenderState,
    ) -> FullOutput {
        accesskit_frame_with_events(context, blueprint, fixture, state, Vec::new())
    }

    fn accesskit_frame_with_events(
        context: &Context,
        blueprint: &viewwright_model::ResolvedBlueprint,
        fixture: &str,
        state: &mut RenderState,
        events: Vec<egui::Event>,
    ) -> FullOutput {
        let raw_input = RawInput {
            screen_rect: Some(egui::Rect::from_min_size(
                egui::Pos2::ZERO,
                egui::vec2(1440.0, 900.0),
            )),
            events,
            ..Default::default()
        };
        context.run(raw_input, |context| {
            super::show(context, blueprint, fixture, state);
        })
    }

    fn interactive_frame(
        context: &Context,
        blueprint: &viewwright_model::ResolvedBlueprint,
        fixture: &str,
        state: &mut RenderState,
        events: Vec<egui::Event>,
    ) -> (FullOutput, Option<super::InteractionEvent>) {
        let raw_input = RawInput {
            screen_rect: Some(egui::Rect::from_min_size(
                egui::Pos2::ZERO,
                egui::vec2(1440.0, 900.0),
            )),
            events,
            ..Default::default()
        };
        let mut activation = None;
        let output = context.run(raw_input, |context| {
            activation = super::show(context, blueprint, fixture, state).activation;
        });
        (output, activation)
    }

    fn accesskit_output(
        blueprint: &viewwright_model::ResolvedBlueprint,
        fixture: &str,
    ) -> FullOutput {
        let context = accesskit_context();
        accesskit_frame(&context, blueprint, fixture, &mut RenderState::default())
    }

    fn update(output: &FullOutput) -> &accesskit::TreeUpdate {
        output
            .platform_output
            .accesskit_update
            .as_ref()
            .expect("AccessKit output is enabled for the test pass")
    }

    fn author_ids(update: &accesskit::TreeUpdate) -> BTreeSet<String> {
        update
            .nodes
            .iter()
            .filter_map(|(_, node)| node.author_id().map(str::to_owned))
            .collect()
    }

    fn assert_unique_author_ids(update: &accesskit::TreeUpdate) {
        let count = update
            .nodes
            .iter()
            .filter(|(_, node)| node.author_id().is_some())
            .count();
        assert_eq!(author_ids(update).len(), count, "author IDs must be unique");
    }

    fn author_node<'a>(
        update: &'a accesskit::TreeUpdate,
        author_id: &str,
    ) -> (accesskit::NodeId, &'a accesskit::Node) {
        let matches = update
            .nodes
            .iter()
            .filter(|(_, node)| node.author_id() == Some(author_id))
            .collect::<Vec<_>>();
        assert_eq!(matches.len(), 1, "expected one author_id={author_id}");
        let (id, node) = matches[0];
        (*id, node)
    }

    fn assert_parent(update: &accesskit::TreeUpdate, parent: &str, child: &str) {
        let (parent_id, parent_node) = author_node(update, parent);
        let (child_id, _) = author_node(update, child);
        assert!(
            parent_node.children().contains(&child_id),
            "{child} should be an accessibility child of {parent} ({parent_id:?})"
        );
    }

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

    #[test]
    fn project_browser_accesskit_tree_has_unique_exact_hierarchy_and_native_children() {
        let blueprint =
            parse_and_resolve(include_str!("../../../examples/project-browser.toml")).unwrap();
        let output = accesskit_output(&blueprint, "many_projects");
        let update = update(&output);
        assert_unique_author_ids(update);
        let expected = BTreeSet::from([
            "project_browser".to_owned(),
            "navigation".to_owned(),
            "projects".to_owned(),
            "inspector".to_owned(),
            "project_search".to_owned(),
            "navigation_items".to_owned(),
            "project_collection".to_owned(),
            "project_inspector".to_owned(),
        ]);
        assert_eq!(author_ids(update), expected);
        for id in &expected {
            author_node(update, id);
        }
        assert_parent(update, "project_browser", "navigation");
        assert_parent(update, "project_browser", "projects");
        assert_parent(update, "project_browser", "inspector");
        assert_parent(update, "navigation", "project_search");
        assert_parent(update, "navigation", "navigation_items");
        assert_parent(update, "projects", "project_collection");
        assert_parent(update, "inspector", "project_inspector");

        let (_, screen_node) = author_node(update, "project_browser");
        assert_eq!(screen_node.role(), accesskit::Role::GenericContainer);
        let (_, search_node) = author_node(update, "project_search");
        assert_eq!(search_node.role(), accesskit::Role::GenericContainer);
        assert!(!search_node.children().is_empty());
        assert!(update
            .nodes
            .iter()
            .any(|(_, node)| { node.role() == accesskit::Role::TextInput }));
        assert!(update.nodes.iter().any(|(_, node)| {
            node.role() == accesskit::Role::Label && node.value() == Some("Search projects")
        }));
        assert!(update.nodes.iter().any(|(_, node)| {
            node.role() == accesskit::Role::Label && node.value() == Some("Project A · active")
        }));
        assert!(!author_ids(update).contains("project_a"));

        let plan = layout(&blueprint, 1440.0, 900.0);
        for id in ["navigation", "projects", "inspector"] {
            let (_, node) = author_node(update, id);
            let actual = node.bounds().expect("region anchor has planned bounds");
            let planned = plan.region(id).unwrap();
            assert_eq!(actual.x0, planned.x as f64);
            assert_eq!(actual.y0, planned.y as f64);
            assert_eq!(actual.x1, (planned.x + planned.width) as f64);
            assert_eq!(actual.y1, (planned.y + planned.height) as f64);
        }
    }

    #[test]
    fn accesskit_semantic_ids_are_stable_across_redraw_and_fixture_switch() {
        let blueprint = parse_and_resolve(include_str!(
            "../../../specimens/reader-workspace-visual.toml"
        ))
        .unwrap();
        let context = accesskit_context();
        let mut state = RenderState::default();
        let reading = accesskit_frame(&context, &blueprint, "reading", &mut state);
        let reading_ids = author_ids(update(&reading));
        assert_unique_author_ids(update(&reading));
        assert_eq!(
            reading_ids,
            BTreeSet::from(
                [
                    "reader_workspace_visual",
                    "app_commands",
                    "library",
                    "reader",
                    "inspector",
                    "transport",
                    "open_document",
                    "library_search",
                    "document_outline",
                    "document_surface",
                    "reader_settings",
                    "play_pause",
                    "voice",
                    "speed",
                    "reading_status",
                ]
                .map(str::to_owned)
            )
        );
        assert_parent(update(&reading), "app_commands", "open_document");
        assert_parent(update(&reading), "reader", "document_surface");
        assert!(!author_node(update(&reading), "open_document")
            .1
            .children()
            .is_empty());
        assert_parent(update(&reading), "library", "library_search");
        assert!(!author_node(update(&reading), "library_search")
            .1
            .children()
            .is_empty());
        assert!(!author_node(update(&reading), "document_surface")
            .1
            .children()
            .is_empty());
        assert!(update(&reading)
            .nodes
            .iter()
            .any(|(_, node)| node.role() == accesskit::Role::TextInput));
        assert!(update(&reading).nodes.iter().any(|(_, node)| {
            node.role() == accesskit::Role::Button && node.label() == Some("Open")
        }));
        assert!(update(&reading).nodes.iter().any(|(_, node)| {
            node.role() == accesskit::Role::Label && node.value() == Some("The Quiet Machine")
        }));
        assert!(update(&reading).nodes.iter().any(|(_, node)| {
            node.role() == accesskit::Role::Label
                && node.value()
                    == Some("Rain gathered against the windows while the city settled into its evening rhythm.")
        }));
        let redraw = accesskit_frame(&context, &blueprint, "reading", &mut state);
        assert_eq!(author_ids(update(&redraw)), reading_ids);
        assert_unique_author_ids(update(&redraw));

        let empty = accesskit_frame(&context, &blueprint, "empty", &mut state);
        assert_eq!(author_ids(update(&empty)), reading_ids);
        assert_unique_author_ids(update(&empty));
        assert!(update(&empty).nodes.iter().any(|(_, node)| {
            node.role() == accesskit::Role::Label && node.value() == Some("No document loaded")
        }));
        assert!(reading_ids.contains("reader_workspace_visual"));
        assert!(reading_ids.contains("document_surface"));
        assert!(!reading_ids.contains("chapter_one"));
        assert!(!reading_ids.contains("chapter_two"));
    }

    #[test]
    fn unused_declarations_and_fixture_local_ids_are_not_author_anchors() {
        let source = r#"
[screen]
id = "reachable_screen"
purpose = "Reachability test"
root = "root"
[[region]]
id = "visible_region"
role = "navigation"
importance = "secondary"
[[region]]
id = "other_visible_region"
role = "primary_content"
importance = "primary"
[[region]]
id = "unused_region"
role = "inspector"
importance = "tertiary"
[[element]]
id = "visible_element"
region = "visible_region"
kind = "collection"
importance = "secondary"
label = "Items"
[[element]]
id = "text_description"
region = "visible_region"
kind = "text"
importance = "secondary"
label = "Authored text"
[[element]]
id = "preview_panel"
region = "visible_region"
kind = "preview"
importance = "primary"
label = "Preview"
[[element]]
id = "unused_element"
region = "unused_region"
kind = "text"
importance = "tertiary"
label = "Unused"
[[composition]]
id = "root"
kind = "split"
children = ["visible_region", "other_visible_region"]
[[fixture]]
id = "fixture"
state = "ready"
[[fixture.content]]
element = "visible_element"
items = [{ id = "fixture_item", label = "Fixture item" }]
"#;
        let blueprint = parse_and_resolve(source).unwrap();
        let output = accesskit_output(&blueprint, "fixture");
        assert_unique_author_ids(update(&output));
        assert_eq!(
            author_ids(update(&output)),
            BTreeSet::from([
                "reachable_screen".to_owned(),
                "visible_region".to_owned(),
                "other_visible_region".to_owned(),
                "visible_element".to_owned(),
                "text_description".to_owned(),
                "preview_panel".to_owned(),
            ])
        );
        assert_parent(update(&output), "visible_region", "text_description");
        assert_parent(update(&output), "visible_region", "preview_panel");
    }

    #[test]
    fn overlay_and_scroll_regions_keep_authored_identity_hierarchy() {
        let overlay = parse_and_resolve(include_str!(
            "../../../specimens/overlay-command-palette-pressure.toml"
        ))
        .unwrap();
        let overlay_output = accesskit_output(&overlay, "palette_open");
        let overlay_update = update(&overlay_output);
        assert_unique_author_ids(overlay_update);
        let overlay_plan = layout(&overlay, 1440.0, 900.0);
        for id in [
            "overlay_command_palette_pressure",
            "navigation",
            "projects",
            "inspector",
            "palette_surface",
            "palette_search",
            "palette_results",
        ] {
            author_node(overlay_update, id);
        }
        assert_parent(
            overlay_update,
            "overlay_command_palette_pressure",
            "palette_surface",
        );
        for id in ["navigation", "projects", "inspector", "palette_surface"] {
            let (_, node) = author_node(overlay_update, id);
            let actual = node.bounds().unwrap();
            let planned = overlay_plan.region(id).unwrap();
            assert_eq!(actual.x0, planned.x as f64);
            assert_eq!(actual.y0, planned.y as f64);
            assert_eq!(actual.x1, (planned.x + planned.width) as f64);
            assert_eq!(actual.y1, (planned.y + planned.height) as f64);
        }
        assert_parent(overlay_update, "palette_surface", "palette_search");
        assert_parent(overlay_update, "palette_surface", "palette_results");
        assert_eq!(
            author_ids(overlay_update),
            BTreeSet::from(
                [
                    "overlay_command_palette_pressure",
                    "navigation",
                    "projects",
                    "inspector",
                    "palette_surface",
                    "navigation_items",
                    "project_collection",
                    "project_inspector",
                    "palette_search",
                    "palette_results",
                ]
                .map(str::to_owned)
            )
        );

        let overflow = parse_and_resolve(include_str!(
            "../../../specimens/reader-overflow-pressure.toml"
        ))
        .unwrap();
        let context = accesskit_context();
        let mut state = RenderState::default();
        let overflow_output = accesskit_frame(&context, &overflow, "long_document", &mut state);
        let overflow_update = update(&overflow_output);
        assert_unique_author_ids(overflow_update);
        for id in [
            "reader_overflow_pressure",
            "app_commands",
            "library",
            "reader",
            "inspector",
            "transport",
            "document_surface",
            "play_pause",
        ] {
            author_node(overflow_update, id);
        }
        assert_parent(overflow_update, "reader_overflow_pressure", "reader");
        assert_parent(overflow_update, "reader", "document_surface");
        let overflow_plan = layout(&overflow, 1440.0, 900.0);
        let reader_bounds = author_node(overflow_update, "reader").1.bounds().unwrap();
        let planned_reader = overflow_plan.region("reader").unwrap();
        assert_eq!(reader_bounds.x0, planned_reader.x as f64);
        assert_eq!(reader_bounds.y0, planned_reader.y as f64);
        assert_eq!(
            reader_bounds.x1,
            (planned_reader.x + planned_reader.width) as f64
        );
        assert_eq!(
            reader_bounds.y1,
            (planned_reader.y + planned_reader.height) as f64
        );
        assert_eq!(
            author_ids(overflow_update),
            BTreeSet::from(
                [
                    "reader_overflow_pressure",
                    "app_commands",
                    "library",
                    "reader",
                    "inspector",
                    "transport",
                    "open_document",
                    "library_search",
                    "document_outline",
                    "document_surface",
                    "reader_settings",
                    "play_pause",
                    "reading_status",
                ]
                .map(str::to_owned)
            )
        );
        assert!(!author_ids(overflow_update).contains("region-scroll"));
        let after_scroll = accesskit_frame_with_events(
            &context,
            &overflow,
            "long_document",
            &mut state,
            vec![
                egui::Event::PointerMoved(egui::pos2(600.0, 400.0)),
                egui::Event::MouseWheel {
                    unit: egui::MouseWheelUnit::Point,
                    delta: egui::vec2(0.0, -480.0),
                    modifiers: egui::Modifiers::NONE,
                },
            ],
        );
        assert_eq!(
            author_ids(update(&after_scroll)),
            author_ids(overflow_update)
        );
        assert_unique_author_ids(update(&after_scroll));
    }

    #[test]
    fn accesskit_anchors_do_not_change_painted_shapes() {
        let blueprint =
            parse_and_resolve(include_str!("../../../examples/project-browser.toml")).unwrap();
        let context = Context::default();
        let mut state = RenderState::default();
        let output_without_accesskit =
            accesskit_frame(&context, &blueprint, "many_projects", &mut state);

        let context = accesskit_context();
        let mut state = RenderState::default();
        let output_with_accesskit =
            accesskit_frame(&context, &blueprint, "many_projects", &mut state);
        assert_eq!(
            format!("{:?}", output_without_accesskit.shapes),
            format!("{:?}", output_with_accesskit.shapes)
        );
    }

    #[test]
    fn semantic_anchors_preserve_command_activation() {
        let blueprint = parse_and_resolve(include_str!(
            "../../../specimens/reader-workspace-visual.toml"
        ))
        .unwrap();
        let context = accesskit_context();
        let mut state = RenderState::default();
        let initial = accesskit_frame(&context, &blueprint, "reading", &mut state);
        let (_, button) = update(&initial)
            .nodes
            .iter()
            .find(|(_, node)| {
                node.role() == accesskit::Role::Button && node.label() == Some("Open")
            })
            .expect("the native Open button remains accessible");
        let bounds = button.bounds().unwrap();
        let position = egui::pos2(
            ((bounds.x0 + bounds.x1) / 2.0) as f32,
            ((bounds.y0 + bounds.y1) / 2.0) as f32,
        );
        let press = egui::Event::PointerButton {
            pos: position,
            button: egui::PointerButton::Primary,
            pressed: true,
            modifiers: egui::Modifiers::NONE,
        };
        let release = egui::Event::PointerButton {
            pos: position,
            button: egui::PointerButton::Primary,
            pressed: false,
            modifiers: egui::Modifiers::NONE,
        };
        interactive_frame(
            &context,
            &blueprint,
            "reading",
            &mut state,
            vec![egui::Event::PointerMoved(position), press],
        );
        let (_, activation) = interactive_frame(
            &context,
            &blueprint,
            "reading",
            &mut state,
            vec![egui::Event::PointerMoved(position), release],
        );
        assert_eq!(
            activation,
            Some(super::InteractionEvent {
                element_id: "open_document".into(),
                action: "document.open".into(),
            })
        );

        let context = accesskit_context();
        let mut state = RenderState::default();
        let empty = accesskit_frame(&context, &blueprint, "empty", &mut state);
        let (_, disabled_button) = update(&empty)
            .nodes
            .iter()
            .find(|(_, node)| {
                node.role() == accesskit::Role::Button && node.label() == Some("Play / pause")
            })
            .expect("the disabled native command remains accessible");
        let bounds = disabled_button.bounds().unwrap();
        let position = egui::pos2(
            ((bounds.x0 + bounds.x1) / 2.0) as f32,
            ((bounds.y0 + bounds.y1) / 2.0) as f32,
        );
        interactive_frame(
            &context,
            &blueprint,
            "empty",
            &mut state,
            vec![
                egui::Event::PointerMoved(position),
                egui::Event::PointerButton {
                    pos: position,
                    button: egui::PointerButton::Primary,
                    pressed: true,
                    modifiers: egui::Modifiers::NONE,
                },
            ],
        );
        let (_, disabled_activation) = interactive_frame(
            &context,
            &blueprint,
            "empty",
            &mut state,
            vec![
                egui::Event::PointerMoved(position),
                egui::Event::PointerButton {
                    pos: position,
                    button: egui::PointerButton::Primary,
                    pressed: false,
                    modifiers: egui::Modifiers::NONE,
                },
            ],
        );
        assert_eq!(disabled_activation, None);
    }
}
