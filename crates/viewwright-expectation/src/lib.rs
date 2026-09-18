use serde::Serialize;
use std::collections::HashSet;
use thiserror::Error;
use viewwright_layout::{layout, Rect};
use viewwright_model::{
    CompositionChild, DominantTarget, ElementKind as ResolvedElementKind,
    Importance as ResolvedImportance, OverflowPolicy, RegionRole as ResolvedRegionRole,
    ResolvedBlueprint,
};

pub const EXPECTATION_VERSION: &str = "0.2";

#[derive(Debug, Error, PartialEq)]
pub enum ExpectationError {
    #[error("logical viewport {axis} must be finite and non-negative, got {value}")]
    InvalidViewport { axis: &'static str, value: f32 },
    #[error("reachable region '{0}' has no rectangle in LayoutPlan")]
    MissingRegionBounds(String),
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct ViewWrightExpectation {
    #[serde(rename = "viewwright_expectation_version")]
    pub version: ExpectationVersion,
    pub epistemic: Epistemic,
    pub screen: ScreenExpectation,
    pub viewport: LogicalViewport,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dominant: Option<DominantExpectation>,
    pub regions: Vec<RegionExpectation>,
    pub elements: Vec<ElementExpectation>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub enum ExpectationVersion {
    #[serde(rename = "0.1")]
    V0_1,
    #[serde(rename = "0.2")]
    V0_2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Epistemic {
    Intended,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct ScreenExpectation {
    pub author_id: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize)]
pub struct LogicalViewport {
    pub width: f32,
    pub height: f32,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct DominantExpectation {
    pub kind: DominantKind,
    pub author_id: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum DominantKind {
    Region,
    Element,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct RegionExpectation {
    pub author_id: String,
    pub role: RegionRole,
    pub importance: Importance,
    pub bounds: Bounds,
    pub overflow: Overflow,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum RegionRole {
    Commands,
    Controls,
    Navigation,
    PrimaryContent,
    Inspector,
    Status,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Importance {
    Primary,
    Secondary,
    Tertiary,
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize)]
pub struct Bounds {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Overflow {
    Clip,
    ScrollY,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct ElementExpectation {
    pub author_id: String,
    pub region_author_id: String,
    pub kind: ElementKind,
    pub importance: Importance,
    pub label: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ElementKind {
    Text,
    Command,
    Search,
    Collection,
    PropertySheet,
    Tree,
    Preview,
    Status,
    Document,
    Choice,
    Boolean,
    Scalar,
}

/// Builds normative expectations from resolved intent and LayoutPlan at one explicit logical viewport.
pub fn build_expectation(
    blueprint: &ResolvedBlueprint,
    width: f32,
    height: f32,
) -> Result<ViewWrightExpectation, ExpectationError> {
    validate_dimension("width", width)?;
    validate_dimension("height", height)?;

    let viewport = LogicalViewport {
        width: canonical_zero(width),
        height: canonical_zero(height),
    };
    let plan = layout(blueprint, viewport.width, viewport.height);
    let mut ordered_region_ids = Vec::new();
    reachable_regions(
        blueprint,
        &blueprint.root,
        &mut HashSet::new(),
        &mut ordered_region_ids,
    );
    let regions = ordered_region_ids
        .iter()
        .map(|id| {
            let region = blueprint
                .regions
                .iter()
                .find(|region| region.id == *id)
                .expect("resolved composition children reference resolved regions");
            let rect = plan
                .region(id)
                .ok_or_else(|| ExpectationError::MissingRegionBounds(id.clone()))?;
            Ok(RegionExpectation {
                author_id: region.id.clone(),
                role: region.role.into(),
                importance: region.importance.into(),
                bounds: rect.into(),
                overflow: region.overflow.into(),
            })
        })
        .collect::<Result<Vec<_>, ExpectationError>>()?;

    // Region structural preorder, then authored element order within each region.
    let elements = ordered_region_ids
        .iter()
        .flat_map(|region_id| {
            blueprint
                .elements
                .iter()
                .filter(move |element| element.region == *region_id)
        })
        .map(|element| ElementExpectation {
            author_id: element.id.clone(),
            region_author_id: element.region.clone(),
            kind: element.kind.into(),
            importance: element.importance.into(),
            label: element.label.clone(),
            action: element
                .action
                .as_ref()
                .map(|action| action.as_str().to_owned()),
        })
        .collect();

    Ok(ViewWrightExpectation {
        version: ExpectationVersion::V0_2,
        epistemic: Epistemic::Intended,
        screen: ScreenExpectation {
            author_id: blueprint.screen.id.clone(),
        },
        viewport,
        dominant: blueprint.design.dominant.as_ref().map(Into::into),
        regions,
        elements,
    })
}

/// Serializes the typed ViewWright expectation model as deterministic YAML.
pub fn to_yaml(expectation: &ViewWrightExpectation) -> Result<String, serde_yaml_ng::Error> {
    serde_yaml_ng::to_string(expectation)
}

fn validate_dimension(axis: &'static str, value: f32) -> Result<(), ExpectationError> {
    if value.is_finite() && value >= 0.0 {
        Ok(())
    } else {
        Err(ExpectationError::InvalidViewport { axis, value })
    }
}

fn canonical_zero(value: f32) -> f32 {
    if value == 0.0 {
        0.0
    } else {
        value
    }
}

fn reachable_regions(
    blueprint: &ResolvedBlueprint,
    composition_id: &str,
    seen_compositions: &mut HashSet<String>,
    regions: &mut Vec<String>,
) {
    if !seen_compositions.insert(composition_id.to_owned()) {
        return;
    }
    if let Some(composition) = blueprint
        .compositions
        .iter()
        .find(|composition| composition.id == composition_id)
    {
        for child in &composition.children {
            match child {
                CompositionChild::Composition(child_id) => {
                    reachable_regions(blueprint, child_id, seen_compositions, regions)
                }
                CompositionChild::Region(region_id) => regions.push(region_id.clone()),
            }
        }
    }
}

impl From<Rect> for Bounds {
    fn from(rect: Rect) -> Self {
        Self {
            x: rect.x,
            y: rect.y,
            width: rect.width,
            height: rect.height,
        }
    }
}

impl From<ResolvedImportance> for Importance {
    fn from(value: ResolvedImportance) -> Self {
        match value {
            ResolvedImportance::Primary => Self::Primary,
            ResolvedImportance::Secondary => Self::Secondary,
            ResolvedImportance::Tertiary => Self::Tertiary,
        }
    }
}

impl From<ResolvedRegionRole> for RegionRole {
    fn from(value: ResolvedRegionRole) -> Self {
        match value {
            ResolvedRegionRole::Commands => Self::Commands,
            ResolvedRegionRole::Controls => Self::Controls,
            ResolvedRegionRole::Navigation => Self::Navigation,
            ResolvedRegionRole::PrimaryContent => Self::PrimaryContent,
            ResolvedRegionRole::Inspector => Self::Inspector,
            ResolvedRegionRole::Status => Self::Status,
        }
    }
}

impl From<OverflowPolicy> for Overflow {
    fn from(value: OverflowPolicy) -> Self {
        match value {
            OverflowPolicy::Clip => Self::Clip,
            OverflowPolicy::ScrollY => Self::ScrollY,
        }
    }
}

impl From<ResolvedElementKind> for ElementKind {
    fn from(value: ResolvedElementKind) -> Self {
        match value {
            ResolvedElementKind::Text => Self::Text,
            ResolvedElementKind::Command => Self::Command,
            ResolvedElementKind::Search => Self::Search,
            ResolvedElementKind::Collection => Self::Collection,
            ResolvedElementKind::PropertySheet => Self::PropertySheet,
            ResolvedElementKind::Tree => Self::Tree,
            ResolvedElementKind::Preview => Self::Preview,
            ResolvedElementKind::Status => Self::Status,
            ResolvedElementKind::Document => Self::Document,
            ResolvedElementKind::Choice => Self::Choice,
            ResolvedElementKind::Boolean => Self::Boolean,
            ResolvedElementKind::Scalar => Self::Scalar,
        }
    }
}

impl From<&DominantTarget> for DominantExpectation {
    fn from(value: &DominantTarget) -> Self {
        match value {
            DominantTarget::Region(author_id) => Self {
                kind: DominantKind::Region,
                author_id: author_id.clone(),
            },
            DominantTarget::Element(author_id) => Self {
                kind: DominantKind::Element,
                author_id: author_id.clone(),
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use viewwright_model::parse_and_resolve;

    fn browser() -> ResolvedBlueprint {
        parse_and_resolve(include_str!("../../../examples/project-browser.toml")).unwrap()
    }

    fn export(source: &str, width: f32, height: f32) -> ViewWrightExpectation {
        build_expectation(&parse_and_resolve(source).unwrap(), width, height).unwrap()
    }

    #[test]
    fn project_browser_preserves_typed_intent_and_exact_layout_bounds() {
        let blueprint = browser();
        let expectation = build_expectation(&blueprint, 1440.0, 900.0).unwrap();
        let plan = layout(&blueprint, 1440.0, 900.0);

        assert_eq!(expectation.version, ExpectationVersion::V0_2);
        assert_eq!(expectation.epistemic, Epistemic::Intended);
        assert_eq!(expectation.screen.author_id, "project_browser");
        assert_eq!(
            expectation.viewport,
            LogicalViewport {
                width: 1440.0,
                height: 900.0
            }
        );
        assert_eq!(
            expectation.dominant,
            Some(DominantExpectation {
                kind: DominantKind::Region,
                author_id: "projects".into(),
            })
        );
        assert_eq!(
            expectation
                .regions
                .iter()
                .map(|r| r.author_id.as_str())
                .collect::<Vec<_>>(),
            ["navigation", "projects", "inspector"]
        );
        for region in &expectation.regions {
            assert_eq!(
                region.bounds,
                Bounds::from(plan.region(&region.author_id).unwrap())
            );
            assert_eq!(region.overflow, Overflow::Clip);
        }
        assert_eq!(expectation.regions[0].role, RegionRole::Navigation);
        assert_eq!(expectation.regions[0].importance, Importance::Secondary);
        assert_eq!(expectation.regions[1].role, RegionRole::PrimaryContent);
        assert_eq!(expectation.regions[1].importance, Importance::Primary);
        assert_eq!(expectation.regions[2].role, RegionRole::Inspector);

        assert_eq!(
            expectation
                .elements
                .iter()
                .map(|e| e.author_id.as_str())
                .collect::<Vec<_>>(),
            [
                "project_search",
                "navigation_items",
                "project_collection",
                "project_inspector"
            ]
        );
        assert_eq!(expectation.elements[0].region_author_id, "navigation");
        assert_eq!(expectation.elements[0].kind, ElementKind::Search);
        assert_eq!(expectation.elements[0].importance, Importance::Secondary);
        assert_eq!(expectation.elements[0].label, "Search projects");
        assert_eq!(expectation.elements[1].region_author_id, "navigation");
        assert_eq!(expectation.elements[1].kind, ElementKind::Collection);
        assert_eq!(expectation.elements[1].importance, Importance::Secondary);
        assert_eq!(expectation.elements[1].label, "navigation items");
        assert_eq!(expectation.elements[2].region_author_id, "projects");
        assert_eq!(expectation.elements[2].kind, ElementKind::Collection);
        assert_eq!(expectation.elements[2].importance, Importance::Primary);
        assert_eq!(expectation.elements[2].label, "project collection");
        assert_eq!(expectation.elements[3].region_author_id, "inspector");
        assert_eq!(expectation.elements[3].kind, ElementKind::PropertySheet);
        assert_eq!(expectation.elements[3].importance, Importance::Secondary);
        assert_eq!(expectation.elements[3].label, "project inspector");
        assert!(expectation
            .elements
            .iter()
            .all(|element| element.action.is_none()));
    }

    #[test]
    fn m42_expectation_uses_wire_version_02_and_exports_semantic_control_kinds() {
        let blueprint = parse_and_resolve(include_str!(
            "../../../specimens/lantern-leaf-reader-controls.toml"
        ))
        .unwrap();
        let expectation = build_expectation(&blueprint, 1440.0, 900.0).unwrap();
        assert_eq!(expectation.version, ExpectationVersion::V0_2);
        let kinds: Vec<_> = expectation
            .elements
            .iter()
            .map(|element| element.kind)
            .collect();
        assert!(kinds.contains(&ElementKind::Choice));
        assert!(kinds.contains(&ElementKind::Boolean));
        assert!(kinds.contains(&ElementKind::Scalar));
        assert!(serde_yaml_ng::to_string(&expectation)
            .unwrap()
            .starts_with("viewwright_expectation_version: '0.2'"));
    }

    #[test]
    fn project_browser_yaml_is_deterministic_intended_and_not_a_witness() {
        let expectation = build_expectation(&browser(), 1440.0, 900.0).unwrap();
        let first = to_yaml(&expectation).unwrap();
        let second = to_yaml(&expectation).unwrap();
        assert_eq!(first, second);
        assert_eq!(
            first,
            include_str!("../tests/fixtures/project-browser-1440x900.intended.yaml")
        );
        assert!(first.contains("viewwright_expectation_version: '0.2'"));
        assert!(first.contains("epistemic: intended"));
        assert!(!first.contains("viewwitness_version"));
        assert!(!first.contains("witness"));
        assert!(!first.contains("many_projects"));
        assert!(!first.contains("Project A"));
        assert!(!first.contains("properties:"));
        assert!(!first.contains("Representative project"));
        assert!(!first.contains("fixture:"));
        assert!(!first.contains("selected:"));
        assert!(!first.contains("observed:"));
        assert!(!first.contains("scroll_offset"));
        assert!(!first.contains("derived_from_observation"));
        let element_section = first.split_once("elements:\n").unwrap().1;
        assert!(!element_section.contains("bounds:"));
    }

    #[test]
    fn project_browser_viewport_changes_only_layout_derived_geometry() {
        let blueprint = browser();
        let standard = build_expectation(&blueprint, 1440.0, 900.0).unwrap();
        let larger = build_expectation(&blueprint, 1600.0, 1000.0).unwrap();
        assert_eq!(standard.screen, larger.screen);
        assert_eq!(standard.dominant, larger.dominant);
        assert_eq!(
            standard
                .regions
                .iter()
                .map(|r| &r.author_id)
                .collect::<Vec<_>>(),
            larger
                .regions
                .iter()
                .map(|r| &r.author_id)
                .collect::<Vec<_>>()
        );
        assert_eq!(standard.elements, larger.elements);
        assert_eq!(
            larger.viewport,
            LogicalViewport {
                width: 1600.0,
                height: 1000.0
            }
        );
        assert_eq!(
            standard.regions[0].bounds.width,
            larger.regions[0].bounds.width
        );
        assert_eq!(
            standard.regions[2].bounds.width,
            larger.regions[2].bounds.width
        );
        assert!(larger.regions[1].bounds.width > standard.regions[1].bounds.width);
    }

    #[test]
    fn unused_region_and_element_declarations_remain_valid_and_are_omitted() {
        let source = r#"
[screen]
id = "reachable"
purpose = "Reachability test"
root = "root"
[[region]]
id = "first"
role = "navigation"
importance = "secondary"
[[region]]
id = "second"
role = "primary_content"
importance = "primary"
[[region]]
id = "unused"
role = "inspector"
importance = "tertiary"
[[element]]
id = "visible_element"
region = "first"
kind = "text"
importance = "secondary"
label = "Visible"
[[element]]
id = "unused_element"
region = "unused"
kind = "text"
importance = "tertiary"
label = "Not visible"
[[composition]]
id = "root"
kind = "split"
children = ["first", "second"]
"#;
        let expectation = export(source, 400.0, 300.0);
        assert_eq!(
            expectation
                .regions
                .iter()
                .map(|r| r.author_id.as_str())
                .collect::<Vec<_>>(),
            ["first", "second"]
        );
        assert_eq!(
            expectation
                .elements
                .iter()
                .map(|e| e.author_id.as_str())
                .collect::<Vec<_>>(),
            ["visible_element"]
        );
    }

    #[test]
    fn dominant_element_identity_and_command_action_are_preserved() {
        let source = r#"
[screen]
id = "commands"
purpose = "Command test"
root = "root"
[design]
dominant = "open"
[[region]]
id = "commands_region"
role = "commands"
importance = "primary"
[[region]]
id = "content"
role = "primary_content"
importance = "secondary"
[[element]]
id = "open"
region = "commands_region"
kind = "command"
importance = "primary"
label = "Open"
action = "document.open"
[[element]]
id = "description"
region = "content"
kind = "text"
importance = "secondary"
label = "Exact authored copy!"
[[composition]]
id = "root"
kind = "split"
children = ["commands_region", "content"]
"#;
        let expectation = export(source, 600.0, 400.0);
        assert_eq!(
            expectation.dominant,
            Some(DominantExpectation {
                kind: DominantKind::Element,
                author_id: "open".into(),
            })
        );
        assert_eq!(
            expectation.elements[0].action.as_deref(),
            Some("document.open")
        );
        assert_eq!(expectation.elements[1].label, "Exact authored copy!");
        assert_eq!(expectation.elements[1].action, None);
    }

    #[test]
    fn accepted_reader_commands_keep_exact_namespaced_action_ids() {
        let blueprint = parse_and_resolve(include_str!(
            "../../../specimens/reader-workspace-visual.toml"
        ))
        .unwrap();
        let expectation = build_expectation(&blueprint, 1440.0, 900.0).unwrap();
        let actions = expectation
            .elements
            .iter()
            .filter_map(|element| element.action.as_deref())
            .collect::<Vec<_>>();
        assert_eq!(
            actions,
            [
                "document.open",
                "reader.play_pause",
                "reader.choose_voice",
                "reader.choose_speed",
            ]
        );
        assert!(expectation
            .elements
            .iter()
            .filter(|element| element.kind != ElementKind::Command)
            .all(|element| element.action.is_none()));
    }

    #[test]
    fn overlay_exports_structural_layers_and_exact_layout_geometry() {
        let blueprint = parse_and_resolve(include_str!(
            "../../../specimens/overlay-command-palette-pressure.toml"
        ))
        .unwrap();
        let expectation = build_expectation(&blueprint, 1440.0, 900.0).unwrap();
        let plan = layout(&blueprint, 1440.0, 900.0);
        assert_eq!(
            expectation.dominant,
            Some(DominantExpectation {
                kind: DominantKind::Region,
                author_id: "palette_surface".into(),
            })
        );
        assert_eq!(
            expectation
                .regions
                .iter()
                .map(|r| r.author_id.as_str())
                .collect::<Vec<_>>(),
            ["navigation", "projects", "inspector", "palette_surface"]
        );
        let palette = expectation
            .regions
            .iter()
            .find(|r| r.author_id == "palette_surface")
            .unwrap();
        assert_eq!(
            palette.bounds,
            Bounds::from(plan.region("palette_surface").unwrap())
        );
        for region in &expectation.regions {
            assert_eq!(
                region.bounds,
                Bounds::from(plan.region(&region.author_id).unwrap())
            );
        }
        assert!(expectation
            .regions
            .iter()
            .any(|r| r.author_id == "projects"));
        let element_section = to_yaml(&expectation)
            .unwrap()
            .split_once("elements:\n")
            .unwrap()
            .1
            .to_owned();
        assert!(!element_section.contains("bounds:"));
    }

    #[test]
    fn overflow_exports_scroll_intent_without_runtime_offset() {
        let blueprint = parse_and_resolve(include_str!(
            "../../../specimens/reader-overflow-pressure.toml"
        ))
        .unwrap();
        let expectation = build_expectation(&blueprint, 1440.0, 900.0).unwrap();
        let plan = layout(&blueprint, 1440.0, 900.0);
        let reader = expectation
            .regions
            .iter()
            .find(|r| r.author_id == "reader")
            .unwrap();
        assert_eq!(reader.overflow, Overflow::ScrollY);
        assert_eq!(reader.bounds, Bounds::from(plan.region("reader").unwrap()));
        for region in &expectation.regions {
            assert_eq!(
                region.bounds,
                Bounds::from(plan.region(&region.author_id).unwrap())
            );
        }
        let yaml = to_yaml(&expectation).unwrap();
        assert!(yaml.contains("overflow: scroll_y"));
        assert!(!yaml.contains("scroll_offset"));
        assert!(!yaml.contains("paragraph_index"));
        assert!(expectation
            .elements
            .iter()
            .all(|element| element.action.is_none()
                || element.action.as_deref() == Some("document.open")
                || element.action.as_deref() == Some("reader.play_pause")));
    }

    #[test]
    fn viewport_must_be_finite_and_non_negative() {
        let blueprint = browser();
        assert_eq!(
            build_expectation(&blueprint, -1.0, 10.0),
            Err(ExpectationError::InvalidViewport {
                axis: "width",
                value: -1.0,
            })
        );
        assert!(matches!(
            build_expectation(&blueprint, f32::NAN, 10.0),
            Err(ExpectationError::InvalidViewport { axis: "width", .. })
        ));
        assert!(matches!(
            build_expectation(&blueprint, 10.0, f32::INFINITY),
            Err(ExpectationError::InvalidViewport { axis: "height", .. })
        ));
    }

    #[test]
    fn all_accepted_canonical_and_pressure_sources_remain_compatible() {
        for (name, source) in [
            (
                "Project Browser",
                include_str!("../../../examples/project-browser.toml"),
            ),
            (
                "Reader",
                include_str!("../../../specimens/reader-workspace.toml"),
            ),
            (
                "visual Reader",
                include_str!("../../../specimens/reader-workspace-visual.toml"),
            ),
            (
                "Dependency Workbench",
                include_str!("../../../specimens/dependency-workbench.toml"),
            ),
            (
                "comfortable density pressure",
                include_str!("../../../specimens/density-pressure-comfortable.toml"),
            ),
            (
                "dense density pressure",
                include_str!("../../../specimens/density-pressure-dense.toml"),
            ),
            (
                "overlay pressure",
                include_str!("../../../specimens/overlay-command-palette-pressure.toml"),
            ),
            (
                "overflow pressure",
                include_str!("../../../specimens/reader-overflow-pressure.toml"),
            ),
        ] {
            let blueprint = parse_and_resolve(source)
                .unwrap_or_else(|error| panic!("{name} no longer resolves: {error}"));
            build_expectation(&blueprint, 1440.0, 900.0)
                .unwrap_or_else(|error| panic!("{name} expectation failed: {error}"));
        }
    }
}
