use std::collections::HashMap;
use viewwright_model::{Axis, CompositionChild, CompositionKind, ResolvedBlueprint};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Rect {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
}

impl Rect {
    pub const fn new(x: f32, y: f32, width: f32, height: f32) -> Self {
        Self {
            x,
            y,
            width,
            height,
        }
    }

    fn inset(self, amount: f32) -> Self {
        Self::new(
            self.x + amount,
            self.y + amount,
            (self.width - amount * 2.0).max(0.0),
            (self.height - amount * 2.0).max(0.0),
        )
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct LayoutPlan {
    pub viewport: Rect,
    compositions: HashMap<String, Rect>,
    regions: HashMap<String, Rect>,
}

impl LayoutPlan {
    pub fn composition(&self, id: &str) -> Option<Rect> {
        self.compositions.get(id).copied()
    }

    pub fn region(&self, id: &str) -> Option<Rect> {
        self.regions.get(id).copied()
    }
}

pub fn layout(blueprint: &ResolvedBlueprint, width: f32, height: f32) -> LayoutPlan {
    let viewport = Rect::new(0.0, 0.0, width.max(0.0), height.max(0.0));
    let mut plan = LayoutPlan {
        viewport,
        compositions: HashMap::new(),
        regions: HashMap::new(),
    };
    layout_composition(blueprint, &mut plan, &blueprint.root, viewport);
    plan
}

fn layout_composition(blueprint: &ResolvedBlueprint, plan: &mut LayoutPlan, id: &str, rect: Rect) {
    let Some(composition) = blueprint.compositions.iter().find(|c| c.id == id) else {
        return;
    };
    plan.compositions.insert(id.to_owned(), rect);
    let inner = rect.inset(composition.padding as f32);
    if composition.kind == CompositionKind::Overlay {
        if composition.children.len() != 2 {
            return;
        }
        if let CompositionChild::Composition(base) = &composition.children[0] {
            layout_composition(blueprint, plan, base, inner);
        }
        if let CompositionChild::Region(floating) = &composition.children[1] {
            if let Some(region) = blueprint
                .regions
                .iter()
                .find(|region| region.id == *floating)
            {
                let width = region.width.unwrap_or(0) as f32;
                let height = region.height.unwrap_or(0) as f32;
                plan.regions.insert(
                    floating.clone(),
                    Rect::new(
                        inner.x + (inner.width - width) / 2.0,
                        inner.y + (inner.height - height) / 2.0,
                        width,
                        height,
                    ),
                );
            }
        }
        return;
    }
    let horizontal = composition.axis == Some(Axis::Horizontal);
    let gap = composition.gap as f32;
    let total_gap = gap * composition.children.len().saturating_sub(1) as f32;
    let fixed: f32 = composition
        .children
        .iter()
        .map(|child| fixed_size(blueprint, child, horizontal))
        .sum();
    let growth: f32 = composition
        .children
        .iter()
        .map(|child| growth_weight(blueprint, child, horizontal))
        .sum();
    let remaining = if horizontal {
        (inner.width - total_gap - fixed).max(0.0)
    } else {
        (inner.height - total_gap - fixed).max(0.0)
    };
    let mut cursor = if horizontal { inner.x } else { inner.y };
    for child in &composition.children {
        let main = fixed_size(blueprint, child, horizontal).max(0.0)
            + if growth > 0.0 {
                remaining * growth_weight(blueprint, child, horizontal) / growth
            } else {
                0.0
            };
        let child_rect = if horizontal {
            let result = Rect::new(cursor, inner.y, main, inner.height);
            cursor += main + gap;
            result
        } else {
            let result = Rect::new(inner.x, cursor, inner.width, main);
            cursor += main + gap;
            result
        };
        match child {
            CompositionChild::Region(id) => {
                plan.regions.insert(id.clone(), child_rect);
            }
            CompositionChild::Composition(id) => {
                layout_composition(blueprint, plan, id, child_rect);
            }
        }
    }
}

fn fixed_size(blueprint: &ResolvedBlueprint, child: &CompositionChild, horizontal: bool) -> f32 {
    match child {
        CompositionChild::Region(id) => blueprint
            .regions
            .iter()
            .find(|r| r.id == *id)
            .and_then(|r| if horizontal { r.width } else { r.height })
            .unwrap_or(0) as f32,
        CompositionChild::Composition(_) => 0.0,
    }
}

fn growth_weight(
    blueprint: &ResolvedBlueprint,
    child: &CompositionChild,
    _horizontal: bool,
) -> f32 {
    match child {
        CompositionChild::Region(id) => blueprint
            .regions
            .iter()
            .find(|r| r.id == *id)
            .map(|r| r.grow)
            .unwrap_or(0.0),
        CompositionChild::Composition(id) => blueprint
            .compositions
            .iter()
            .find(|c| c.id == *id)
            .map(|c| c.grow)
            .unwrap_or(0.0),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use viewwright_model::{parse_and_resolve, CompositionChild};

    fn blueprint() -> viewwright_model::ResolvedBlueprint {
        parse_and_resolve(
            r#"
[screen]
id = "layout"
purpose = "layout"
root = "root"
[tokens.spacing]
gap = 10
pad = 5
[[region]]
id = "fixed"
role = "controls"
importance = "secondary"
width = "100px"
[[region]]
id = "a"
role = "primary_content"
importance = "primary"
grow = 1
[[region]]
id = "b"
role = "primary_content"
importance = "primary"
grow = 2
[[composition]]
id = "root"
kind = "split"
axis = "horizontal"
children = ["fixed", "a", "b"]
gap = "gap"
padding = "pad"
"#,
        )
        .unwrap()
    }

    #[test]
    fn density_pressure_pair_keeps_major_layout_identical() {
        let comfortable = viewwright_model::parse_and_resolve(include_str!(
            "../../../specimens/density-pressure-comfortable.toml"
        ))
        .unwrap();
        let dense = viewwright_model::parse_and_resolve(include_str!(
            "../../../specimens/density-pressure-dense.toml"
        ))
        .unwrap();
        assert_eq!(
            layout(&comfortable, 1440.0, 900.0),
            layout(&dense, 1440.0, 900.0)
        );
        assert_eq!(
            comfortable.screen.density,
            viewwright_model::Density::Comfortable
        );
        assert_eq!(dense.screen.density, viewwright_model::Density::Dense);
        assert_eq!(comfortable.visual, dense.visual);
        assert_eq!(comfortable.elements.len(), dense.elements.len());
        assert_eq!(
            comfortable.fixtures[0].content.len(),
            dense.fixtures[0].content.len()
        );
    }

    #[test]
    fn root_padding_gaps_fixed_and_proportional_growth_are_deterministic() {
        let b = blueprint();
        let plan = layout(&b, 1000.0, 600.0);
        assert_eq!(
            plan.composition("root"),
            Some(Rect::new(0.0, 0.0, 1000.0, 600.0))
        );
        assert_eq!(
            plan.region("fixed"),
            Some(Rect::new(5.0, 5.0, 100.0, 590.0))
        );
        assert_eq!(plan.region("a"), Some(Rect::new(115.0, 5.0, 290.0, 590.0)));
        assert_eq!(plan.region("b"), Some(Rect::new(415.0, 5.0, 580.0, 590.0)));
    }

    #[test]
    fn nested_compositions_fill_cross_axis_and_are_addressable() {
        let mut b = blueprint();
        b.compositions[0].children = vec![CompositionChild::Composition("body".into())];
        b.compositions[0].grow = 0.0;
        b.compositions.push(viewwright_model::ResolvedComposition {
            id: "body".into(),
            kind: viewwright_model::CompositionKind::Split,
            axis: Some(Axis::Horizontal),
            children: vec![
                CompositionChild::Region("a".into()),
                CompositionChild::Region("b".into()),
            ],
            gap: 0,
            padding: 0,
            grow: 1.0,
        });
        let plan = layout(&b, 800.0, 400.0);
        assert_eq!(
            plan.composition("body"),
            Some(Rect::new(5.0, 5.0, 790.0, 390.0))
        );
        assert_eq!(plan.region("a").unwrap().height, 390.0);
        assert_eq!(plan.region("b").unwrap().height, 390.0);
    }

    #[test]
    fn vertical_slots_keep_fixed_bottom_and_growing_middle() {
        let source = r#"
[screen]
id = "vertical"
purpose = "layout"
root = "root"
[tokens.spacing]
gap = 10
pad = 5
[[region]]
id = "top"
role = "commands"
importance = "secondary"
height = "50px"
[[region]]
id = "middle"
role = "primary_content"
importance = "primary"
grow = 1
[[region]]
id = "bottom"
role = "status"
importance = "tertiary"
height = "30px"
[[composition]]
id = "root"
kind = "column"
axis = "vertical"
children = ["top", "middle", "bottom"]
gap = "gap"
padding = "pad"
"#;
        let b = parse_and_resolve(source).unwrap();
        let plan = layout(&b, 500.0, 400.0);
        assert_eq!(plan.region("top"), Some(Rect::new(5.0, 5.0, 490.0, 50.0)));
        assert_eq!(
            plan.region("middle"),
            Some(Rect::new(5.0, 65.0, 490.0, 290.0))
        );
        assert_eq!(
            plan.region("bottom"),
            Some(Rect::new(5.0, 365.0, 490.0, 30.0))
        );
    }

    #[test]
    fn horizontal_fixed_plus_grow_preserves_base_and_share() {
        let source = r#"
[screen]
id = "horizontal"
purpose = "layout"
root = "root"
[[region]]
id = "fixed_grow"
role = "controls"
importance = "primary"
width = "100px"
grow = 1
[[region]]
id = "grow_only"
role = "primary_content"
importance = "secondary"
grow = 1
[[composition]]
id = "root"
kind = "split"
axis = "horizontal"
children = ["fixed_grow", "grow_only"]
"#;
        let b = parse_and_resolve(source).unwrap();
        let plan = layout(&b, 600.0, 400.0);
        assert_eq!(plan.region("fixed_grow").unwrap().width, 350.0);
        assert_eq!(plan.region("grow_only").unwrap().width, 250.0);
    }

    #[test]
    fn vertical_fixed_plus_grow_preserves_base_and_share() {
        let source = r#"
[screen]
id = "vertical-growth"
purpose = "layout"
root = "root"
[[region]]
id = "fixed_grow"
role = "controls"
importance = "primary"
height = "100px"
grow = 1
[[region]]
id = "grow_only"
role = "primary_content"
importance = "secondary"
grow = 3
[[composition]]
id = "root"
kind = "column"
axis = "vertical"
children = ["fixed_grow", "grow_only"]
"#;
        let b = parse_and_resolve(source).unwrap();
        let plan = layout(&b, 400.0, 600.0);
        assert_eq!(plan.region("fixed_grow").unwrap().height, 225.0);
        assert_eq!(plan.region("grow_only").unwrap().height, 375.0);
    }

    #[test]
    fn multiple_fixed_plus_grow_regions_share_remaining_space_proportionally() {
        let source = r#"
[screen]
id = "multiple-growth"
purpose = "layout"
root = "root"
[[region]]
id = "a"
role = "controls"
importance = "primary"
width = "100px"
grow = 1
[[region]]
id = "b"
role = "primary_content"
importance = "secondary"
width = "200px"
grow = 2
[[region]]
id = "c"
role = "status"
importance = "tertiary"
grow = 1
[[composition]]
id = "root"
kind = "split"
axis = "horizontal"
children = ["a", "b", "c"]
"#;
        let b = parse_and_resolve(source).unwrap();
        let plan = layout(&b, 1000.0, 400.0);
        assert_eq!(plan.region("a").unwrap().width, 275.0);
        assert_eq!(plan.region("b").unwrap().width, 550.0);
        assert_eq!(plan.region("c").unwrap().width, 175.0);
    }

    #[test]
    fn zero_growth_has_no_hidden_weight() {
        let mut b = blueprint();
        b.regions.iter_mut().for_each(|r| r.grow = 0.0);
        let plan = layout(&b, 1000.0, 600.0);
        assert_eq!(plan.region("a").unwrap().width, 0.0);
        assert_eq!(plan.region("b").unwrap().width, 0.0);
    }

    #[test]
    fn accepted_specimens_preserve_major_slot_relationships() {
        let reader =
            parse_and_resolve(include_str!("../../../specimens/reader-workspace.toml")).unwrap();
        let reader_plan = layout(&reader, 1440.0, 900.0);
        let body = reader_plan.region("reader").unwrap();
        assert_eq!(body.height, reader_plan.region("library").unwrap().height);
        assert_eq!(body.height, reader_plan.region("inspector").unwrap().height);
        assert_eq!(reader_plan.region("library").unwrap().width, 250.0);
        assert_eq!(reader_plan.region("inspector").unwrap().width, 300.0);
        assert!(reader_plan.region("reader").unwrap().width > 800.0);
        assert_eq!(
            reader_plan.region("transport").unwrap().y
                + reader_plan.region("transport").unwrap().height,
            888.0
        );

        let visual = parse_and_resolve(include_str!(
            "../../../specimens/reader-workspace-visual.toml"
        ))
        .unwrap();
        let visual_plan = layout(&visual, 1440.0, 900.0);
        assert_eq!(
            visual_plan.region("reader").unwrap().height,
            visual_plan.region("library").unwrap().height
        );

        let dependency =
            parse_and_resolve(include_str!("../../../specimens/dependency-workbench.toml"))
                .unwrap();
        let dependency_plan = layout(&dependency, 1440.0, 900.0);
        assert_eq!(dependency_plan.region("filters").unwrap().width, 220.0);
        assert_eq!(dependency_plan.region("inspector").unwrap().width, 300.0);
        assert_eq!(
            dependency_plan.region("packages").unwrap().height,
            dependency_plan.region("filters").unwrap().height
        );
        assert_eq!(
            dependency_plan.region("status").unwrap().y
                + dependency_plan.region("status").unwrap().height,
            890.0
        );

        let project =
            parse_and_resolve(include_str!("../../../examples/project-browser.toml")).unwrap();
        let project_plan = layout(&project, 1440.0, 900.0);
        assert_eq!(project_plan.region("navigation").unwrap().width, 240.0);
        assert_eq!(project_plan.region("inspector").unwrap().width, 320.0);
        assert!(project_plan.region("projects").unwrap().width > 800.0);
    }

    #[test]
    fn overlay_centers_fixed_floating_region_over_full_padded_base() {
        let blueprint = viewwright_model::parse_and_resolve(include_str!(
            "../../../specimens/overlay-command-palette-pressure.toml"
        ))
        .unwrap();
        let plan = layout(&blueprint, 1440.0, 900.0);

        assert_eq!(
            plan.composition("root_overlay"),
            Some(Rect::new(0.0, 0.0, 1440.0, 900.0))
        );
        assert_eq!(
            plan.composition("workspace"),
            Some(Rect::new(0.0, 0.0, 1440.0, 900.0))
        );
        assert_eq!(
            plan.region("palette_surface"),
            Some(Rect::new(460.0, 300.0, 520.0, 300.0))
        );
        assert_eq!(plan.region("navigation").unwrap().x, 16.0);
        assert_eq!(plan.region("inspector").unwrap().x, 1144.0);
    }

    #[test]
    fn overflow_policy_does_not_change_layout_plan_geometry() {
        let scroll = viewwright_model::parse_and_resolve(include_str!(
            "../../../specimens/reader-overflow-pressure.toml"
        ))
        .unwrap();
        let clip = viewwright_model::parse_and_resolve(
            &include_str!("../../../specimens/reader-overflow-pressure.toml")
                .replace("overflow = \"scroll_y\"", "overflow = \"clip\""),
        )
        .unwrap();
        let scroll_plan = layout(&scroll, 1440.0, 900.0);
        let clip_plan = layout(&clip, 1440.0, 900.0);
        assert_eq!(scroll_plan, clip_plan);
        for id in [
            "workspace",
            "reading_body",
            "library",
            "reader",
            "inspector",
            "transport",
        ] {
            assert_eq!(scroll_plan.region(id), clip_plan.region(id));
            assert_eq!(scroll_plan.composition(id), clip_plan.composition(id));
        }
    }
}
