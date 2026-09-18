use std::collections::HashMap;

use egui::{CentralPanel, Color32, FontId, Frame, RichText, ScrollArea, Stroke, Ui, UiBuilder};
use viewwright_layout::{layout, LayoutPlan, Rect as LayoutRect};
use viewwright_model::{
    BorderPolicy, ChoicePresentation, CollectionPresentation, Color, CompositionChild, Density,
    ElementKind, FurnishingChild, FurnishingKind, Importance, OverflowPolicy, RegionRole,
    ResolvedBlueprint, ResolvedFixtureContent, ResolvedFurnishing, ResolvedRegion, SurfaceRole,
};

#[derive(Debug, Clone, PartialEq)]
pub struct InteractionEvent {
    pub element_id: String,
    pub action: String,
    pub value: Option<TypedValue>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum TypedValue {
    Choice(String),
    Boolean(bool),
    Scalar(f32),
}

#[derive(Debug, Default)]
pub struct RenderOutput {
    pub activation: Option<InteractionEvent>,
}

#[derive(Debug, Default)]
pub struct RenderState {
    search_values: HashMap<String, String>,
    control_values: HashMap<(String, String), TypedValue>,
}

impl RenderState {
    pub fn clear(&mut self) {
        self.search_values.clear();
        self.control_values.clear();
    }

    pub fn search_value_mut(&mut self, element_id: &str) -> &mut String {
        self.search_values.entry(element_id.to_owned()).or_default()
    }

    fn control_value_mut(
        &mut self,
        fixture: &str,
        element_id: &str,
        seed: TypedValue,
    ) -> &mut TypedValue {
        self.control_values
            .entry((fixture.to_owned(), element_id.to_owned()))
            .or_insert(seed)
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
    ui: &mut Ui,
    blueprint: &ResolvedBlueprint,
    fixture: &str,
    state: &mut RenderState,
) -> RenderOutput {
    let mut output = RenderOutput::default();
    let root_fill = root_fill(ui, blueprint);
    CentralPanel::default()
        .frame(Frame::new().fill(root_fill).inner_margin(0.0))
        .show(ui, |ui| {
            let anchor_id = ui.make_persistent_id(("viewwright-screen", &blueprint.screen.id));
            with_identity_anchor(
                ui,
                anchor_id,
                &blueprint.screen.id,
                None,
                None,
                |ui, screen_id| {
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
                            screen_id,
                        );
                    });
                },
            );
        });
    output
}

fn with_identity_anchor<R>(
    ui: &mut Ui,
    id: egui::Id,
    author_id: &str,
    bounds: Option<egui::Rect>,
    parent: Option<egui::Id>,
    render: impl FnOnce(&mut Ui, egui::Id) -> R,
) -> R {
    let builder = identity_anchor_builder(ui, id, parent);
    let mut anchor_ui = ui.new_child(builder);
    let anchor_id = set_identity_anchor(&anchor_ui, author_id, bounds);
    render(&mut anchor_ui, anchor_id)
}

fn with_flow_identity_anchor<R>(
    ui: &mut Ui,
    id: egui::Id,
    author_id: &str,
    parent: egui::Id,
    render: impl FnOnce(&mut Ui, egui::Id) -> R,
) -> R {
    let builder = identity_anchor_builder(ui, id, Some(parent));
    ui.scope_builder(builder, |anchor_ui| {
        let anchor_id = set_identity_anchor(anchor_ui, author_id, None);
        render(anchor_ui, anchor_id)
    })
    .inner
}

fn identity_anchor_builder(ui: &Ui, id: egui::Id, parent: Option<egui::Id>) -> UiBuilder {
    let mut builder = UiBuilder::new()
        .id_salt(id)
        .max_rect(ui.available_rect_before_wrap());
    if let Some(parent) = parent {
        builder = builder.accessibility_parent(parent);
    }
    builder
}

fn set_identity_anchor(ui: &Ui, author_id: &str, bounds: Option<egui::Rect>) -> egui::Id {
    let anchor_id = ui.unique_id();
    ui.ctx().accesskit_node_builder(anchor_id, |node| {
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
    anchor_id
}

fn root_fill(ui: &Ui, blueprint: &ResolvedBlueprint) -> Color32 {
    blueprint
        .visual
        .as_ref()
        .map(|v| color32(v.palette.canvas))
        .unwrap_or_else(|| ui.style().visuals.panel_fill)
}

// Keep the explicit render context visible at this established renderer boundary.
#[allow(clippy::too_many_arguments)]
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
    parent_anchor: egui::Id,
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
                CompositionChild::Composition(id) => render_composition(
                    ui,
                    id,
                    b,
                    fixture,
                    plan,
                    origin,
                    activation,
                    density,
                    state,
                    parent_anchor,
                ),
                CompositionChild::Region(id) => {
                    if let Some(region) = b.regions.iter().find(|r| r.id == *id) {
                        render_region(
                            ui,
                            region,
                            b,
                            fixture,
                            plan,
                            child_rect,
                            origin,
                            activation,
                            density,
                            state,
                            parent_anchor,
                        );
                    }
                }
            },
        );
    }
}

// Region rendering carries layout, fixture, interaction, and identity context explicitly.
#[allow(clippy::too_many_arguments)]
fn render_region(
    ui: &mut egui::Ui,
    r: &ResolvedRegion,
    b: &ResolvedBlueprint,
    fixture: &str,
    plan: &LayoutPlan,
    rect: LayoutRect,
    origin: egui::Pos2,
    activation: &mut Option<InteractionEvent>,
    density: DensityPolicy,
    state: &mut RenderState,
    parent_anchor: egui::Id,
) {
    let egui_rect = to_egui_rect(rect, origin);
    let anchor_id = ui.make_persistent_id(("viewwright-region", &b.screen.id, &r.id));
    with_identity_anchor(
        ui,
        anchor_id,
        &r.id,
        Some(egui_rect),
        Some(parent_anchor),
        |ui, region_anchor| {
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
                let separated_surface =
                    matches!(r.surface, SurfaceRole::Panel | SurfaceRole::Raised);
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
            let content = if r.furnishing.is_some() {
                egui_rect
            } else {
                egui_rect.shrink(density.region_inset)
            };
            ui.scope_builder(
                UiBuilder::new()
                    .id_salt(("region", &r.id))
                    .max_rect(content),
                |ui| match r.overflow {
                    OverflowPolicy::Clip => {
                        ui.set_clip_rect(content);
                        render_region_contents(
                            ui,
                            r,
                            b,
                            fixture,
                            plan,
                            origin,
                            activation,
                            density,
                            state,
                            region_anchor,
                        );
                    }
                    OverflowPolicy::ScrollY => {
                        ScrollArea::vertical()
                            .id_salt(scroll_area_id(&b.screen.id, &r.id))
                            .hscroll(false)
                            .auto_shrink([false, false])
                            .show(ui, |ui| {
                                render_region_contents(
                                    ui,
                                    r,
                                    b,
                                    fixture,
                                    plan,
                                    origin,
                                    activation,
                                    density,
                                    state,
                                    region_anchor,
                                );
                            });
                    }
                },
            );
        },
    );
}

fn scroll_area_id(screen_id: &str, region_id: &str) -> egui::Id {
    egui::Id::new(("region-scroll", screen_id, region_id))
}

// Preserve the explicit traversal context rather than refactoring renderer behavior in M39.
#[allow(clippy::too_many_arguments)]
fn render_region_contents(
    ui: &mut egui::Ui,
    r: &ResolvedRegion,
    b: &ResolvedBlueprint,
    fixture: &str,
    plan: &LayoutPlan,
    origin: egui::Pos2,
    activation: &mut Option<InteractionEvent>,
    density: DensityPolicy,
    state: &mut RenderState,
    parent_anchor: egui::Id,
) {
    if let Some(root) = r.furnishing.as_deref() {
        render_furnishing(
            ui,
            root,
            b,
            fixture,
            plan,
            origin,
            activation,
            density,
            state,
            parent_anchor,
        );
        return;
    }
    let elements: Vec<_> = b.elements.iter().filter(|e| e.region == r.id).collect();
    if is_command_region(r.role) {
        ui.horizontal_wrapped(|ui| {
            for e in elements {
                render_element(
                    ui,
                    e,
                    b,
                    fixture,
                    activation,
                    density,
                    state,
                    parent_anchor,
                    density.element_gap,
                );
            }
        });
    } else {
        for e in elements {
            render_element(
                ui,
                e,
                b,
                fixture,
                activation,
                density,
                state,
                parent_anchor,
                density.element_gap,
            );
        }
    }
}

#[allow(clippy::too_many_arguments)]
fn render_furnishing(
    ui: &mut egui::Ui,
    id: &str,
    b: &ResolvedBlueprint,
    fixture: &str,
    plan: &LayoutPlan,
    origin: egui::Pos2,
    activation: &mut Option<InteractionEvent>,
    density: DensityPolicy,
    state: &mut RenderState,
    region_anchor: egui::Id,
) {
    let Some(furnishing) = b.furnishings.iter().find(|node| node.id == id) else {
        return;
    };
    let Some(rect) = plan.furnishing(id) else {
        return;
    };
    let rect = to_egui_rect(rect, origin);
    ui.scope_builder(
        UiBuilder::new()
            .id_salt(("furnishing", &furnishing.id))
            .max_rect(rect),
        |ui| {
            ui.set_clip_rect(rect);
            match furnishing.overflow {
                OverflowPolicy::Clip => render_furnishing_contents(
                    ui,
                    furnishing,
                    b,
                    fixture,
                    plan,
                    origin,
                    activation,
                    density,
                    state,
                    region_anchor,
                ),
                OverflowPolicy::ScrollY => {
                    ScrollArea::vertical()
                        .id_salt(("furnishing-scroll", &b.screen.id, &furnishing.id))
                        .hscroll(false)
                        .auto_shrink([false, false])
                        .show(ui, |ui| {
                            render_furnishing_contents(
                                ui,
                                furnishing,
                                b,
                                fixture,
                                plan,
                                origin,
                                activation,
                                density,
                                state,
                                region_anchor,
                            );
                        });
                }
            }
        },
    );
}

#[allow(clippy::too_many_arguments)]
fn render_furnishing_contents(
    ui: &mut egui::Ui,
    furnishing: &ResolvedFurnishing,
    b: &ResolvedBlueprint,
    fixture: &str,
    plan: &LayoutPlan,
    origin: egui::Pos2,
    activation: &mut Option<InteractionEvent>,
    density: DensityPolicy,
    state: &mut RenderState,
    region_anchor: egui::Id,
) {
    match furnishing.children.first() {
        Some(FurnishingChild::Furnishing(_)) => {
            for child in &furnishing.children {
                let FurnishingChild::Furnishing(child_id) = child else {
                    continue;
                };
                render_furnishing(
                    ui,
                    child_id,
                    b,
                    fixture,
                    plan,
                    origin,
                    activation,
                    density,
                    state,
                    region_anchor,
                );
            }
        }
        Some(FurnishingChild::Element(_)) => {
            let Some(rect) = plan.furnishing(&furnishing.id) else {
                return;
            };
            let rect = to_egui_rect(rect, origin).shrink(furnishing.padding as f32);
            ui.scope_builder(
                UiBuilder::new()
                    .id_salt(("furnishing-leaf", &furnishing.id))
                    .max_rect(rect),
                |ui| {
                    ui.set_clip_rect(rect);
                    let mut render_element_child = |element_id: &str, ui: &mut egui::Ui| {
                        if let Some(element) = b.elements.iter().find(|e| e.id == element_id) {
                            ui.scope(|element_ui| {
                                element_ui.style_mut().spacing.item_spacing =
                                    egui::vec2(density.item_spacing, density.item_spacing);
                                render_element(
                                    element_ui,
                                    element,
                                    b,
                                    fixture,
                                    activation,
                                    density,
                                    state,
                                    region_anchor,
                                    0.0,
                                );
                            });
                        }
                    };
                    if furnishing.kind == FurnishingKind::Row {
                        ui.style_mut().spacing.item_spacing =
                            egui::vec2(furnishing.gap as f32, furnishing.gap as f32);
                        ui.horizontal_wrapped(|ui| {
                            for child in &furnishing.children {
                                if let FurnishingChild::Element(element_id) = child {
                                    render_element_child(element_id, ui);
                                }
                            }
                        });
                    } else {
                        for (index, child) in furnishing.children.iter().enumerate() {
                            if let FurnishingChild::Element(element_id) = child {
                                if index > 0 {
                                    ui.add_space(furnishing.gap as f32);
                                }
                                render_element_child(element_id, ui);
                            }
                        }
                    }
                },
            );
        }
        None => {}
    }
}

fn is_command_region(role: RegionRole) -> bool {
    role == RegionRole::Commands
}

// Preserve the explicit traversal context rather than refactoring renderer behavior in M39.
#[allow(clippy::too_many_arguments)]
fn render_element(
    ui: &mut egui::Ui,
    e: &viewwright_model::ResolvedElement,
    b: &ResolvedBlueprint,
    fixture: &str,
    activation: &mut Option<InteractionEvent>,
    density: DensityPolicy,
    state: &mut RenderState,
    parent_anchor: egui::Id,
    leading_gap: f32,
) {
    let anchor_id = ui.make_persistent_id(("viewwright-element", &e.id));
    with_flow_identity_anchor(
        ui,
        anchor_id,
        &e.id,
        parent_anchor,
        |ui, _element_anchor| {
            if leading_gap > 0.0 {
                ui.add_space(leading_gap);
            }
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
                    let state = b.command_state(fixture, &e.id);
                    let label = if let (Some(color), Some(v)) = (
                        command_label_color(e.importance, state.enabled, b.visual.as_ref()),
                        b.visual.as_ref(),
                    ) {
                        RichText::new(&e.label)
                            .size(v.type_scale.body as f32)
                            .color(color)
                            .strong()
                    } else {
                        RichText::new(&e.label)
                    };
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
                                value: None,
                            });
                        }
                    }
                }
                ElementKind::Choice => {
                    let (Some(config), Some(ResolvedFixtureContent::Choice { selected, .. })) =
                        (e.choice.as_ref(), content_for(b, fixture, &e.id))
                    else {
                        return;
                    };
                    let value = state.control_value_mut(
                        fixture,
                        &e.id,
                        TypedValue::Choice(selected.clone()),
                    );
                    let TypedValue::Choice(selected) = value else {
                        return;
                    };
                    let mut changed = false;
                    match config.presentation {
                        ChoicePresentation::Select => {
                            let selected_label = config
                                .options
                                .iter()
                                .find(|option| option.id == *selected)
                                .map(|option| option.label.as_str())
                                .unwrap_or(selected);
                            egui::ComboBox::from_id_salt(("viewwright-choice", &e.id))
                                .selected_text(selected_label)
                                .show_ui(ui, |ui| {
                                    for option in &config.options {
                                        changed |= ui
                                            .selectable_value(
                                                selected,
                                                option.id.clone(),
                                                &option.label,
                                            )
                                            .changed();
                                    }
                                });
                        }
                        ChoicePresentation::Segmented => {
                            ui.horizontal(|ui| {
                                for option in &config.options {
                                    changed |= ui
                                        .selectable_value(
                                            selected,
                                            option.id.clone(),
                                            &option.label,
                                        )
                                        .changed();
                                }
                            });
                        }
                    }
                    if changed {
                        *activation = value_event(e, TypedValue::Choice(selected.clone()));
                    }
                }
                ElementKind::Boolean => {
                    let Some(ResolvedFixtureContent::Boolean { value: seed, .. }) =
                        content_for(b, fixture, &e.id)
                    else {
                        return;
                    };
                    let value = state.control_value_mut(fixture, &e.id, TypedValue::Boolean(*seed));
                    let TypedValue::Boolean(value) = value else {
                        return;
                    };
                    if ui.checkbox(value, &e.label).changed() {
                        *activation = value_event(e, TypedValue::Boolean(*value));
                    }
                }
                ElementKind::Scalar => {
                    let (Some(config), Some(ResolvedFixtureContent::Scalar { value: seed, .. })) =
                        (e.scalar.as_ref(), content_for(b, fixture, &e.id))
                    else {
                        return;
                    };
                    let value = state.control_value_mut(fixture, &e.id, TypedValue::Scalar(*seed));
                    let TypedValue::Scalar(value) = value else {
                        return;
                    };
                    let mut slider = egui::Slider::new(value, config.min..=config.max)
                        .step_by(f64::from(config.step))
                        .show_value(true);
                    if let Some(unit) = &config.unit {
                        slider = slider.suffix(unit);
                    }
                    if ui.add(slider).changed() {
                        *activation = value_event(e, TypedValue::Scalar(*value));
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
        },
    );
}

fn separate_element_label(kind: ElementKind) -> bool {
    !matches!(
        kind,
        ElementKind::Command | ElementKind::Text | ElementKind::Preview | ElementKind::Boolean
    )
}

fn value_event(
    element: &viewwright_model::ResolvedElement,
    value: TypedValue,
) -> Option<InteractionEvent> {
    Some(InteractionEvent {
        element_id: element.id.clone(),
        action: element.action.as_ref()?.as_str().to_owned(),
        value: Some(value),
    })
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

fn command_label_color(
    importance: Importance,
    enabled: bool,
    visual: Option<&viewwright_model::ResolvedVisual>,
) -> Option<Color32> {
    if importance != Importance::Primary {
        return None;
    }
    let visual = visual?;
    Some(color32(if enabled {
        visual.palette.accent
    } else {
        visual.palette.text_muted
    }))
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
            | ResolvedFixtureContent::Document { element: id, .. }
            | ResolvedFixtureContent::Command { element: id, .. }
            | ResolvedFixtureContent::Choice { element: id, .. }
            | ResolvedFixtureContent::Boolean { element: id, .. }
            | ResolvedFixtureContent::Scalar { element: id, .. } => id == element,
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
        &mut style.visuals.widgets.noninteractive,
        &mut style.visuals.widgets.inactive,
        &mut style.visuals.widgets.hovered,
        &mut style.visuals.widgets.active,
        &mut style.visuals.widgets.open,
    ] {
        widget.bg_fill = color32(v.palette.surface);
        widget.fg_stroke.color = color32(v.palette.text);
        widget.bg_stroke = boundary;
    }
    style.visuals.widgets.noninteractive.bg_fill = color32(v.palette.surface);
    style.visuals.widgets.noninteractive.weak_bg_fill = color32(v.palette.surface);
    style.visuals.widgets.noninteractive.fg_stroke.color = color32(v.palette.text_muted);
    style.visuals.widgets.hovered.bg_fill = color32(v.palette.surface_raised);
    style.visuals.widgets.active.bg_fill = color32(v.palette.accent);
    style.visuals.widgets.inactive.weak_bg_fill = color32(v.palette.surface);
    style.visuals.widgets.hovered.weak_bg_fill = color32(v.palette.surface_raised);
    style.visuals.widgets.active.weak_bg_fill = color32(v.palette.surface_raised);
    style.visuals.widgets.open.weak_bg_fill = color32(v.palette.surface_raised);
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
        apply_visuals, command_label_color, is_command_region, root_fill, scroll_area_id,
        separate_element_label, DensityPolicy, RenderState,
    };
    use egui::widget_style::{Classes, WidgetState};
    use egui::{accesskit, Color32, Context, FullOutput, RawInput, Stroke};
    use std::collections::BTreeSet;
    use viewwright_layout::layout;
    use viewwright_model::{
        parse_and_resolve, BorderPolicy, Density, ElementKind, Importance, RegionRole,
        ResolvedFixtureContent,
    };

    #[test]
    fn authored_visuals_style_noninteractive_widgets_from_the_palette() {
        let blueprint = parse_and_resolve(include_str!(
            "../../../specimens/reader-workspace-visual.toml"
        ))
        .unwrap();
        let visual = blueprint.visual.as_ref().unwrap();
        let expected_bg = Color32::from_rgb(
            visual.palette.surface.r,
            visual.palette.surface.g,
            visual.palette.surface.b,
        );
        let expected_muted = Color32::from_rgb(
            visual.palette.text_muted.r,
            visual.palette.text_muted.g,
            visual.palette.text_muted.b,
        );
        let context = Context::default();
        let mut actual = None;
        let mut output = context.run_ui(RawInput::default(), |ui| {
            let default_noninteractive = ui.style().visuals.widgets.noninteractive;
            apply_visuals(ui, &blueprint);
            let noninteractive = ui.style().visuals.widgets.noninteractive;
            actual = Some((
                default_noninteractive,
                noninteractive.bg_fill,
                noninteractive.weak_bg_fill,
                noninteractive.fg_stroke.color,
                noninteractive.bg_stroke,
            ));
        });
        output.textures_delta.clear();
        let (default, bg, weak_bg, fg, border) = actual.unwrap();
        assert_ne!(
            bg, default.bg_fill,
            "disabled bg must not retain theme fill"
        );
        assert_eq!(bg, expected_bg);
        assert_eq!(weak_bg, expected_bg);
        assert_eq!(fg, expected_muted);
        assert_eq!(border, Stroke::NONE, "minimal borders stay absent");
    }

    #[test]
    fn primary_command_uses_accent_when_enabled_and_muted_text_when_disabled() {
        let blueprint = parse_and_resolve(include_str!(
            "../../../specimens/reader-workspace-visual.toml"
        ))
        .unwrap();
        let visual = blueprint.visual.as_ref().unwrap();
        let accent = Color32::from_rgb(
            visual.palette.accent.r,
            visual.palette.accent.g,
            visual.palette.accent.b,
        );
        let muted = Color32::from_rgb(
            visual.palette.text_muted.r,
            visual.palette.text_muted.g,
            visual.palette.text_muted.b,
        );
        assert_eq!(
            command_label_color(Importance::Primary, true, Some(visual)),
            Some(accent)
        );
        assert_eq!(
            command_label_color(Importance::Primary, false, Some(visual)),
            Some(muted)
        );
        assert_eq!(
            command_label_color(Importance::Secondary, true, Some(visual)),
            None
        );
    }

    #[test]
    fn button_style_uses_authored_fills_for_each_interaction_state() {
        let blueprint = parse_and_resolve(include_str!(
            "../../../specimens/reader-workspace-visual.toml"
        ))
        .unwrap();
        let visual = blueprint.visual.as_ref().unwrap();
        let surface = Color32::from_rgb(
            visual.palette.surface.r,
            visual.palette.surface.g,
            visual.palette.surface.b,
        );
        let raised = Color32::from_rgb(
            visual.palette.surface_raised.r,
            visual.palette.surface_raised.g,
            visual.palette.surface_raised.b,
        );
        let context = Context::default();
        let mut actual = None;
        let mut output = context.run_ui(RawInput::default(), |ui| {
            apply_visuals(ui, &blueprint);
            let style = ui.style();
            actual = Some([
                style
                    .button_style(&Classes::default(), WidgetState::Inactive)
                    .frame
                    .fill,
                style
                    .button_style(&Classes::default(), WidgetState::Hovered)
                    .frame
                    .fill,
                style
                    .button_style(&Classes::default(), WidgetState::Active)
                    .frame
                    .fill,
                style
                    .button_style(&Classes::default(), WidgetState::Noninteractive)
                    .frame
                    .fill,
            ]);
        });
        output.textures_delta.clear();
        assert_eq!(actual.unwrap(), [surface, raised, raised, surface]);
        assert_ne!(surface, Color32::from_gray(230));

        // The explicit primary-command text colors remain distinct from the
        // active button frame, which stays on the authored raised surface.
        assert_eq!(
            command_label_color(Importance::Primary, true, Some(visual)),
            Some(Color32::from_rgb(
                visual.palette.accent.r,
                visual.palette.accent.g,
                visual.palette.accent.b,
            ))
        );
        assert_eq!(
            command_label_color(Importance::Primary, false, Some(visual)),
            Some(Color32::from_rgb(
                visual.palette.text_muted.r,
                visual.palette.text_muted.g,
                visual.palette.text_muted.b,
            ))
        );
    }

    #[test]
    fn defined_noninteractive_border_uses_authored_border_color() {
        let mut blueprint = parse_and_resolve(include_str!(
            "../../../specimens/reader-workspace-visual.toml"
        ))
        .unwrap();
        blueprint.visual.as_mut().unwrap().border_policy = BorderPolicy::Defined;
        let border_color = blueprint.visual.as_ref().unwrap().palette.border;
        let expected = Color32::from_rgb(border_color.r, border_color.g, border_color.b);
        let context = Context::default();
        let mut actual = None;
        let mut output = context.run_ui(RawInput::default(), |ui| {
            apply_visuals(ui, &blueprint);
            actual = Some(ui.style().visuals.widgets.noninteractive.bg_stroke);
        });
        output.textures_delta.clear();
        assert_eq!(actual.unwrap(), Stroke::new(1.0, expected));
    }

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
        let mut output = context.run_ui(raw_input, |ui| {
            super::show(ui, blueprint, fixture, state);
        });
        output.textures_delta.clear();
        output
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
        let mut output = context.run_ui(raw_input, |ui| {
            activation = super::show(ui, blueprint, fixture, state).activation;
        });
        output.textures_delta.clear();
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

    fn rendered_node_bounds(
        update: &accesskit::TreeUpdate,
        role: accesskit::Role,
        label: Option<&str>,
    ) -> accesskit::Rect {
        update
            .nodes
            .iter()
            .map(|(_, node)| node)
            .find(|node| {
                node.role() == role
                    && label.is_none_or(|label| {
                        node.value() == Some(label) || node.label() == Some(label)
                    })
            })
            .and_then(accesskit::Node::bounds)
            .unwrap_or_else(|| panic!("expected rendered {role:?} node bounds for {label:?}"))
    }

    fn pointer_click(
        context: &Context,
        blueprint: &viewwright_model::ResolvedBlueprint,
        fixture: &str,
        state: &mut RenderState,
        position: egui::Pos2,
    ) -> Option<super::InteractionEvent> {
        let event = |pressed| egui::Event::PointerButton {
            pos: position,
            button: egui::PointerButton::Primary,
            pressed,
            modifiers: egui::Modifiers::NONE,
        };
        let (_, pressed_activation) = interactive_frame(
            context,
            blueprint,
            fixture,
            state,
            vec![egui::Event::PointerMoved(position), event(true)],
        );
        let (_, released_activation) = interactive_frame(
            context,
            blueprint,
            fixture,
            state,
            vec![egui::Event::PointerMoved(position), event(false)],
        );
        pressed_activation.or(released_activation)
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
        assert!(separate_element_label(ElementKind::Choice));
        assert!(!separate_element_label(ElementKind::Boolean));
        assert!(separate_element_label(ElementKind::Scalar));
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
        let context = egui::Context::default();
        let project =
            parse_and_resolve(include_str!("../../../examples/project-browser.toml")).unwrap();
        let visual = parse_and_resolve(include_str!(
            "../../../specimens/reader-workspace-visual.toml"
        ))
        .unwrap();
        for (theme, theme_fill) in [
            (egui::Theme::Light, Color32::from_rgb(12, 34, 56)),
            (egui::Theme::Dark, Color32::from_rgb(210, 220, 230)),
        ] {
            context.set_theme(theme);
            context.style_mut_of(theme, |style| style.visuals.panel_fill = theme_fill);
            let mut output = context.run_ui(
                egui::RawInput {
                    screen_rect: Some(egui::Rect::from_min_size(
                        egui::Pos2::ZERO,
                        egui::vec2(1440.0, 900.0),
                    )),
                    ..Default::default()
                },
                |ui| {
                    assert_eq!(root_fill(ui, &project), theme_fill);
                    assert_eq!(root_fill(ui, &visual), Color32::from_rgb(38, 43, 51));
                },
            );
            output.textures_delta.clear();
        }
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
    fn furnishing_scopes_preserve_region_to_element_accesskit_identity_parentage() {
        let blueprint = parse_and_resolve(include_str!(
            "../../../specimens/lantern-leaf-reader-furnished.toml"
        ))
        .unwrap();
        let output = accesskit_output(&blueprint, "reading");
        let update = update(&output);
        assert_unique_author_ids(update);
        let ids = author_ids(update);
        for region in blueprint
            .regions
            .iter()
            .filter(|region| region.furnishing.is_some())
        {
            assert!(
                ids.contains(&region.id),
                "missing region author id {}",
                region.id
            );
            for element in blueprint
                .elements
                .iter()
                .filter(|element| element.region == region.id)
            {
                assert!(
                    ids.contains(&element.id),
                    "missing element author id {}",
                    element.id
                );
                assert_parent(update, &region.id, &element.id);
            }
        }
        for furnishing in &blueprint.furnishings {
            assert!(
                !ids.contains(&furnishing.id),
                "structural furnishing {} must not become an author id",
                furnishing.id
            );
        }
    }

    #[test]
    fn semantic_elements_consume_vertical_and_command_flow_layout_slots() {
        let blueprint = parse_and_resolve(include_str!(
            "../../../specimens/reader-workspace-visual.toml"
        ))
        .unwrap();
        let output = accesskit_output(&blueprint, "reading");
        let update = update(&output);

        let search = rendered_node_bounds(update, accesskit::Role::TextInput, None);
        let outline = rendered_node_bounds(update, accesskit::Role::Label, Some("Contents"));
        assert!(
            search.y1 <= outline.y0,
            "search control and next element label must not overlap: search={search:?}, outline={outline:?}"
        );

        let command_bounds = [
            rendered_node_bounds(update, accesskit::Role::Button, Some("Play / pause")),
            rendered_node_bounds(update, accesskit::Role::Button, Some("Voice")),
            rendered_node_bounds(update, accesskit::Role::Button, Some("Speed")),
            rendered_node_bounds(update, accesskit::Role::Label, Some("Reading status")),
        ];
        for ((first_id, first), (second_id, second)) in [
            ("play_pause", command_bounds[0]),
            ("voice", command_bounds[1]),
            ("speed", command_bounds[2]),
        ]
        .into_iter()
        .zip([
            ("voice", command_bounds[1]),
            ("speed", command_bounds[2]),
            ("reading_status", command_bounds[3]),
        ]) {
            assert!(
                first.x1 <= second.x0,
                "command/status elements must advance horizontally in the 1440px viewport: {first_id}={first:?}, {second_id}={second:?}"
            );
        }
    }

    #[test]
    fn m42_controls_keep_element_identity_and_region_parent_without_option_or_furnishing_ids() {
        let blueprint = parse_and_resolve(include_str!(
            "../../../specimens/lantern-leaf-reader-controls.toml"
        ))
        .unwrap();
        let output = accesskit_output(&blueprint, "reading");
        let tree = update(&output);
        assert_unique_author_ids(tree);
        let ids = author_ids(tree);
        for element in blueprint.elements.iter().filter(|element| {
            matches!(
                element.kind,
                ElementKind::Choice | ElementKind::Boolean | ElementKind::Scalar
            )
        }) {
            assert!(
                ids.contains(&element.id),
                "missing semantic control ID {}",
                element.id
            );
            assert_parent(tree, &element.region, &element.id);
        }
        for furnishing in &blueprint.furnishings {
            assert!(!ids.contains(&furnishing.id));
        }
        for option_id in [
            "literata",
            "source_serif",
            "georgia",
            "left",
            "justified",
            "centered",
            "continuous",
            "paginated",
            "aria",
            "willow",
            "rowan",
            "0.9x",
            "1.0x",
            "1.2x",
            "1.5x",
        ] {
            assert!(
                !ids.contains(option_id),
                "choice option became an author ID: {option_id}"
            );
        }
    }

    #[test]
    fn m42_typed_value_events_render_state_persistence_and_fixture_seed_reset() {
        let mut blueprint = parse_and_resolve(include_str!(
            "../../../specimens/lantern-leaf-reader-controls.toml"
        ))
        .unwrap();
        let mut alternate = blueprint.fixtures[0].clone();
        alternate.id = "alternate".into();
        for content in &mut alternate.content {
            match content {
                ResolvedFixtureContent::Choice { element, selected }
                    if element == "font_family" =>
                {
                    *selected = "georgia".into()
                }
                ResolvedFixtureContent::Choice { element, selected }
                    if element == "text_alignment" =>
                {
                    *selected = "centered".into()
                }
                ResolvedFixtureContent::Boolean { element, value }
                    if element == "show_highlights" =>
                {
                    *value = false
                }
                ResolvedFixtureContent::Scalar { element, value } if element == "font_size" => {
                    *value = 24.0
                }
                _ => {}
            }
        }
        blueprint.fixtures.push(alternate);

        let context = accesskit_context();
        let mut state = RenderState::default();
        let controls = [
            (
                "font_family",
                "reader.font_family.set",
                super::TypedValue::Choice("source_serif".into()),
                super::TypedValue::Choice("literata".into()),
            ),
            (
                "text_alignment",
                "reader.text_alignment.set",
                super::TypedValue::Choice("justified".into()),
                super::TypedValue::Choice("left".into()),
            ),
            (
                "show_highlights",
                "reader.show_highlights.set",
                super::TypedValue::Boolean(false),
                super::TypedValue::Boolean(true),
            ),
            (
                "font_size",
                "reader.font_size.set",
                super::TypedValue::Scalar(20.0),
                super::TypedValue::Scalar(18.0),
            ),
        ];
        for (element_id, action, changed, seed) in controls {
            let element = blueprint
                .elements
                .iter()
                .find(|element| element.id == element_id)
                .unwrap();
            let local = state.control_value_mut("reading", element_id, seed.clone());
            assert_eq!(local, &seed);
            *local = changed.clone();
            let event = super::value_event(element, changed.clone()).unwrap();
            assert_eq!(event.element_id, element_id);
            assert_eq!(event.action, action);
            assert_eq!(event.value, Some(changed));
        }

        let changed_frame = accesskit_frame(&context, &blueprint, "reading", &mut state);
        let nodes = &update(&changed_frame).nodes;
        assert!(nodes
            .iter()
            .any(|(_, node)| node.role() == accesskit::Role::ComboBox
                && node.value() == Some("Source Serif")));
        assert!(nodes
            .iter()
            .any(|(_, node)| node.role() == accesskit::Role::Button
                && node.label() == Some("Justified")));
        assert!(nodes
            .iter()
            .any(|(_, node)| node.role() == accesskit::Role::SpinButton
                && node.value() == Some("20.0px")));
        let redraw = accesskit_frame(&context, &blueprint, "reading", &mut state);
        assert!(update(&redraw)
            .nodes
            .iter()
            .any(|(_, node)| node.role() == accesskit::Role::ComboBox
                && node.value() == Some("Source Serif")));
        assert_eq!(
            state.control_value_mut("reading", "font_size", super::TypedValue::Scalar(18.0)),
            &super::TypedValue::Scalar(20.0)
        );

        state.clear();
        let reset = accesskit_frame(&context, &blueprint, "reading", &mut state);
        assert!(update(&reset)
            .nodes
            .iter()
            .any(|(_, node)| node.role() == accesskit::Role::ComboBox
                && node.value() == Some("Literata")));
        assert_eq!(
            state.control_value_mut("reading", "font_size", super::TypedValue::Scalar(18.0)),
            &super::TypedValue::Scalar(18.0)
        );
        state.clear();
        let alternate = accesskit_frame(&context, &blueprint, "alternate", &mut state);
        assert!(update(&alternate)
            .nodes
            .iter()
            .any(|(_, node)| node.role() == accesskit::Role::ComboBox
                && node.value() == Some("Georgia")));
        assert_eq!(
            state.control_value_mut("alternate", "font_size", super::TypedValue::Scalar(24.0)),
            &super::TypedValue::Scalar(24.0)
        );
    }

    #[test]
    fn m42_segmented_boolean_and_scalar_native_controls_emit_values_on_pointer_input() {
        let blueprint = parse_and_resolve(include_str!(
            "../../../specimens/lantern-leaf-reader-controls.toml"
        ))
        .unwrap();
        let context = accesskit_context();
        let mut state = RenderState::default();

        let frame = accesskit_frame(&context, &blueprint, "reading", &mut state);
        let bounds =
            rendered_node_bounds(update(&frame), accesskit::Role::Button, Some("Justified"));
        let event = pointer_click(
            &context,
            &blueprint,
            "reading",
            &mut state,
            egui::pos2(
                ((bounds.x0 + bounds.x1) / 2.0) as f32,
                ((bounds.y0 + bounds.y1) / 2.0) as f32,
            ),
        );
        assert_eq!(
            event.as_ref().map(|event| event.action.as_str()),
            Some("reader.text_alignment.set")
        );
        assert_eq!(
            event.as_ref().and_then(|event| event.value.as_ref()),
            Some(&super::TypedValue::Choice("justified".into()))
        );

        let frame = accesskit_frame(&context, &blueprint, "reading", &mut state);
        let bounds = rendered_node_bounds(
            update(&frame),
            accesskit::Role::CheckBox,
            Some("Show Highlights"),
        );
        let event = pointer_click(
            &context,
            &blueprint,
            "reading",
            &mut state,
            egui::pos2(
                ((bounds.x0 + bounds.x1) / 2.0) as f32,
                ((bounds.y0 + bounds.y1) / 2.0) as f32,
            ),
        );
        assert_eq!(
            event.as_ref().map(|event| event.action.as_str()),
            Some("reader.show_highlights.set")
        );
        assert_eq!(
            event.as_ref().and_then(|event| event.value.as_ref()),
            Some(&super::TypedValue::Boolean(false))
        );

        let frame = accesskit_frame(&context, &blueprint, "reading", &mut state);
        let tree = update(&frame);
        let (_, font_node) = author_node(tree, "font_size");
        let value_container = font_node
            .children()
            .iter()
            .filter_map(|id| {
                tree.nodes
                    .iter()
                    .find(|(node_id, _)| node_id == id)
                    .map(|(_, node)| node)
            })
            .find(|node| node.role() == accesskit::Role::GenericContainer)
            .expect("scalar element owns its native control container");
        let slider = value_container
            .children()
            .iter()
            .filter_map(|id| {
                tree.nodes
                    .iter()
                    .find(|(node_id, _)| node_id == id)
                    .map(|(_, node)| node)
            })
            .find(|node| node.role() == accesskit::Role::Slider)
            .expect("scalar element exposes an accessible slider");
        let bounds = slider.bounds().unwrap();
        let event = pointer_click(
            &context,
            &blueprint,
            "reading",
            &mut state,
            egui::pos2(
                (bounds.x0 + (bounds.x1 - bounds.x0) * 0.8) as f32,
                ((bounds.y0 + bounds.y1) / 2.0) as f32,
            ),
        );
        assert_eq!(
            event.as_ref().map(|event| event.action.as_str()),
            Some("reader.font_size.set")
        );
        assert!(
            matches!(event.and_then(|event| event.value), Some(super::TypedValue::Scalar(value)) if (12.0..=32.0).contains(&value) && value != 18.0)
        );
    }

    #[test]
    fn m42_select_choice_native_dropdown_emits_typed_value_on_pointer_input() {
        let blueprint = parse_and_resolve(include_str!(
            "../../../specimens/lantern-leaf-reader-controls.toml"
        ))
        .unwrap();
        let context = accesskit_context();
        let mut state = RenderState::default();

        let frame = accesskit_frame(&context, &blueprint, "reading", &mut state);
        let bounds = update(&frame)
            .nodes
            .iter()
            .find(|(_, node)| {
                node.role() == accesskit::Role::ComboBox && node.value() == Some("Literata")
            })
            .and_then(|(_, node)| node.bounds())
            .expect("font-family select exposes its selected label and bounds");
        pointer_click(
            &context,
            &blueprint,
            "reading",
            &mut state,
            egui::pos2(
                ((bounds.x0 + bounds.x1) / 2.0) as f32,
                ((bounds.y0 + bounds.y1) / 2.0) as f32,
            ),
        );

        let opened = accesskit_frame(&context, &blueprint, "reading", &mut state);
        let font_family_bounds = update(&opened)
            .nodes
            .iter()
            .find(|(_, node)| {
                node.role() == accesskit::Role::ComboBox && node.value() == Some("Literata")
            })
            .and_then(|(_, node)| node.bounds())
            .expect("font-family select remains visible while its popup is open");
        let option_position = egui::pos2(
            ((font_family_bounds.x0 + font_family_bounds.x1) / 2.0) as f32,
            (font_family_bounds.y1 + (font_family_bounds.y1 - font_family_bounds.y0) * 1.5) as f32,
        );
        let event = pointer_click(&context, &blueprint, "reading", &mut state, option_position);
        assert_eq!(
            event.as_ref().map(|event| event.element_id.as_str()),
            Some("font_family")
        );
        assert_eq!(
            event.as_ref().map(|event| event.action.as_str()),
            Some("reader.font_family.set")
        );
        assert_eq!(
            event.and_then(|event| event.value),
            Some(super::TypedValue::Choice("source_serif".into()))
        );
        let selected = accesskit_frame(&context, &blueprint, "reading", &mut state);
        assert!(update(&selected).nodes.iter().any(|(_, node)| {
            node.role() == accesskit::Role::ComboBox && node.value() == Some("Source Serif")
        }));
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
                    phase: egui::TouchPhase::Move,
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
                value: None,
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
