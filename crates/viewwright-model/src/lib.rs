use serde::Deserialize;
use std::collections::{HashMap, HashSet};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum BlueprintError {
    #[error("TOML parse error: {0}")]
    Parse(#[from] toml::de::Error),
    #[error("blueprint validation failed:\n{0}")]
    Validation(String),
}

#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SourceBlueprint {
    pub screen: ScreenSource,
    #[serde(default)]
    pub design: DesignSource,
    #[serde(default)]
    pub tokens: TokensSource,
    pub visual: Option<VisualSource>,
    #[serde(default)]
    pub region: Vec<RegionSource>,
    #[serde(default)]
    pub composition: Vec<CompositionSource>,
    #[serde(default)]
    pub element: Vec<ElementSource>,
    #[serde(default)]
    pub fixture: Vec<FixtureSource>,
}
#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ScreenSource {
    pub id: String,
    pub purpose: String,
    #[serde(default = "default_density")]
    pub density: String,
    pub root: Option<String>,
}
fn default_density() -> String {
    "comfortable".into()
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Density {
    Comfortable,
    Dense,
}
#[derive(Debug, Clone, Default, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct DesignSource {
    #[serde(default)]
    pub character: Vec<String>,
    pub dominant: Option<String>,
    #[serde(default)]
    pub avoid: Vec<String>,
}
#[derive(Debug, Clone, Default, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TokensSource {
    #[serde(default)]
    pub spacing: HashMap<String, u32>,
    #[serde(default)]
    pub corners: HashMap<String, u32>,
    #[serde(default)]
    pub color: ColorTokensSource,
    #[serde(default)]
    pub r#type: TypeTokensSource,
}
#[derive(Debug, Clone, Default, Deserialize)]
pub struct ColorTokensSource {
    #[serde(flatten)]
    pub values: HashMap<String, String>,
}
#[derive(Debug, Clone, Default, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TypeTokensSource {
    pub display: Option<u16>,
    pub heading: Option<u16>,
    pub body: Option<u16>,
    pub caption: Option<u16>,
}
#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct VisualSource {
    pub canvas: String,
    pub surface: String,
    pub surface_raised: String,
    pub text: String,
    pub text_muted: String,
    pub accent: String,
    pub border: String,
    pub corner: String,
    pub border_policy: String,
}
#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RegionSource {
    pub id: String,
    pub role: String,
    pub importance: String,
    pub width: Option<String>,
    pub height: Option<String>,
    pub grow: Option<f32>,
    pub surface: Option<String>,
}
#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CompositionSource {
    pub id: String,
    pub kind: String,
    pub axis: Option<String>,
    pub children: Vec<String>,
    pub gap: Option<String>,
    pub padding: Option<String>,
    pub grow: Option<f32>,
}
#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ElementSource {
    pub id: String,
    pub region: String,
    pub kind: String,
    pub importance: String,
    pub label: Option<String>,
    pub presentation: Option<String>,
    pub action: Option<String>,
}
#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct FixtureSource {
    pub id: String,
    pub state: String,
    #[serde(default)]
    pub content: Vec<FixtureContentSource>,
}
#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct FixtureContentSource {
    pub element: String,
    pub items: Option<Vec<CollectionItemSource>>,
    pub selected: Option<String>,
    pub properties: Option<Vec<PropertySource>>,
    pub text: Option<String>,
    pub nodes: Option<Vec<TreeNodeSource>>,
    pub document: Option<DocumentSource>,
    pub command: Option<CommandSource>,
}
#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CollectionItemSource {
    pub id: String,
    pub label: String,
}
#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PropertySource {
    pub name: String,
    pub value: String,
}
#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TreeNodeSource {
    pub id: String,
    pub label: String,
    pub parent: Option<String>,
}
#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct DocumentSource {
    pub title: String,
    pub paragraphs: Vec<String>,
}
#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CommandSource {
    pub enabled: bool,
    pub reason: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Importance {
    Primary,
    Secondary,
    Tertiary,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RegionRole {
    Commands,
    Controls,
    Navigation,
    PrimaryContent,
    Inspector,
    Status,
}

impl RegionRole {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Commands => "commands",
            Self::Controls => "controls",
            Self::Navigation => "navigation",
            Self::PrimaryContent => "primary_content",
            Self::Inspector => "inspector",
            Self::Status => "status",
        }
    }
}

impl std::fmt::Display for RegionRole {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ActionId(String);

impl ActionId {
    pub fn as_str(&self) -> &str {
        &self.0
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SurfaceRole {
    Canvas,
    Panel,
    Raised,
    Transparent,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BorderPolicy {
    None,
    Minimal,
    Defined,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TypeScale {
    pub display: u16,
    pub heading: u16,
    pub body: u16,
    pub caption: u16,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Palette {
    pub canvas: Color,
    pub surface: Color,
    pub surface_raised: Color,
    pub text: Color,
    pub text_muted: Color,
    pub accent: Color,
    pub border: Color,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ResolvedVisual {
    pub palette: Palette,
    pub type_scale: TypeScale,
    pub corner_radius: u32,
    pub border_policy: BorderPolicy,
}
fn importance(s: &str, where_: &str, errors: &mut Vec<String>) -> Importance {
    match s {
        "primary" => Importance::Primary,
        "secondary" => Importance::Secondary,
        "tertiary" => Importance::Tertiary,
        _ => {
            errors.push(format!("{where_}: unknown importance '{s}'"));
            Importance::Tertiary
        }
    }
}
fn region_role(value: &str, id: &str, errors: &mut Vec<String>) -> RegionRole {
    match value {
        "commands" => RegionRole::Commands,
        "controls" => RegionRole::Controls,
        "navigation" => RegionRole::Navigation,
        "primary_content" => RegionRole::PrimaryContent,
        "inspector" => RegionRole::Inspector,
        "status" => RegionRole::Status,
        other => {
            errors.push(format!(
                "region '{id}': unknown role '{other}' (expected commands, controls, navigation, primary_content, inspector, or status)"
            ));
            RegionRole::PrimaryContent
        }
    }
}

fn composition_kind(value: &str, id: &str, errors: &mut Vec<String>) -> CompositionKind {
    match value {
        "split" => CompositionKind::Split,
        "row" => CompositionKind::Row,
        "column" => CompositionKind::Column,
        other => {
            errors.push(format!(
                "composition '{id}': unknown composition kind '{other}'"
            ));
            CompositionKind::Split
        }
    }
}

fn axis(value: Option<&str>, kind: CompositionKind, id: &str, errors: &mut Vec<String>) -> Axis {
    let implied = match kind {
        CompositionKind::Column => Axis::Vertical,
        CompositionKind::Split | CompositionKind::Row => Axis::Horizontal,
    };
    let Some(value) = value else {
        return implied;
    };
    let Some(axis) = (match value {
        "horizontal" => Some(Axis::Horizontal),
        "vertical" => Some(Axis::Vertical),
        other => {
            errors.push(format!(
                "composition '{id}': unknown axis '{other}' (expected horizontal or vertical)"
            ));
            None
        }
    }) else {
        return implied;
    };
    if kind == CompositionKind::Row && axis != Axis::Horizontal {
        errors.push(format!(
            "composition '{id}': row requires horizontal axis, got '{value}'"
        ));
    }
    if kind == CompositionKind::Column && axis != Axis::Vertical {
        errors.push(format!(
            "composition '{id}': column requires vertical axis, got '{value}'"
        ));
    }
    axis
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CollectionPresentation {
    List,
    AdaptiveCards,
}
fn kind(s: &str, where_: &str, errors: &mut Vec<String>) -> ElementKind {
    match s {
        "text" => ElementKind::Text,
        "command" => ElementKind::Command,
        "search" => ElementKind::Search,
        "collection" => ElementKind::Collection,
        "property_sheet" => ElementKind::PropertySheet,
        "tree" => ElementKind::Tree,
        "preview" => ElementKind::Preview,
        "status" => ElementKind::Status,
        "document" => ElementKind::Document,
        _ => {
            errors.push(format!("{where_}: unknown element kind '{s}'"));
            ElementKind::Text
        }
    }
}
fn collection_presentation(
    value: Option<&str>,
    element: &str,
    element_kind: ElementKind,
    errors: &mut Vec<String>,
) -> Option<CollectionPresentation> {
    if element_kind != ElementKind::Collection {
        if value.is_some() {
            errors.push(format!(
                "element '{element}': presentation is only valid for collection elements"
            ));
        }
        return None;
    }
    match value.unwrap_or("list") {
        "list" => Some(CollectionPresentation::List),
        "adaptive_cards" => Some(CollectionPresentation::AdaptiveCards),
        other => {
            errors.push(format!(
                "element '{element}': unknown collection presentation '{other}'"
            ));
            None
        }
    }
}
fn action_id(value: &str, element: &str, errors: &mut Vec<String>) -> Option<ActionId> {
    let valid = !value.is_empty()
        && value.contains('.')
        && !value.starts_with('.')
        && !value.ends_with('.')
        && value.split('.').all(|segment| {
            !segment.is_empty()
                && segment
                    .bytes()
                    .all(|byte| byte.is_ascii_alphanumeric() || byte == b'_' || byte == b'-')
        });
    if valid {
        Some(ActionId(value.to_owned()))
    } else {
        errors.push(format!(
            "element '{element}': action must be a non-empty dot-separated identifier with ASCII letters, digits, '_' or '-'"
        ));
        None
    }
}
#[derive(Debug, Clone)]
pub struct ResolvedRegion {
    pub id: String,
    pub role: RegionRole,
    pub importance: Importance,
    pub width: Option<u32>,
    pub height: Option<u32>,
    pub grow: f32,
    pub surface: SurfaceRole,
}
#[derive(Debug, Clone)]
pub struct ResolvedElement {
    pub id: String,
    pub region: String,
    pub kind: ElementKind,
    pub importance: Importance,
    pub label: String,
    pub presentation: Option<CollectionPresentation>,
    pub action: Option<ActionId>,
}
#[derive(Debug, Clone)]
pub enum CompositionChild {
    Region(String),
    Composition(String),
}
#[derive(Debug, Clone)]
pub struct ResolvedComposition {
    pub id: String,
    pub kind: CompositionKind,
    pub axis: Axis,
    pub children: Vec<CompositionChild>,
    pub gap: u32,
    pub padding: u32,
    pub grow: f32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CompositionKind {
    Split,
    Row,
    Column,
}

impl CompositionKind {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Split => "split",
            Self::Row => "row",
            Self::Column => "column",
        }
    }
}

impl std::fmt::Display for CompositionKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Axis {
    Horizontal,
    Vertical,
}

impl Axis {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Horizontal => "horizontal",
            Self::Vertical => "vertical",
        }
    }
}

impl std::fmt::Display for Axis {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[derive(Debug, Clone)]
pub struct ResolvedFixture {
    pub id: String,
    pub state: String,
    pub content: Vec<ResolvedFixtureContent>,
}
#[derive(Debug, Clone)]
pub enum ResolvedFixtureContent {
    Collection {
        element: String,
        items: Vec<ResolvedCollectionItem>,
        selected: Option<String>,
    },
    Properties {
        element: String,
        properties: Vec<ResolvedProperty>,
    },
    Text {
        element: String,
        text: String,
    },
    Tree {
        element: String,
        nodes: Vec<ResolvedTreeNode>,
        selected: Option<String>,
    },
    Document {
        element: String,
        title: String,
        paragraphs: Vec<String>,
    },
    Command {
        element: String,
        enabled: bool,
        reason: Option<String>,
    },
}
#[derive(Debug, Clone)]
pub struct ResolvedCollectionItem {
    pub id: String,
    pub label: String,
}
#[derive(Debug, Clone)]
pub struct ResolvedProperty {
    pub name: String,
    pub value: String,
}
#[derive(Debug, Clone)]
pub struct ResolvedTreeNode {
    pub id: String,
    pub label: String,
    pub parent: Option<String>,
}
#[derive(Debug, Clone)]
pub struct ResolvedBlueprint {
    pub screen: ResolvedScreen,
    pub design: DesignSource,
    pub root: String,
    pub regions: Vec<ResolvedRegion>,
    pub compositions: Vec<ResolvedComposition>,
    pub elements: Vec<ResolvedElement>,
    pub fixtures: Vec<ResolvedFixture>,
    pub visual: Option<ResolvedVisual>,
}
#[derive(Debug, Clone)]
pub struct ResolvedScreen {
    pub id: String,
    pub purpose: String,
    pub density: Density,
}

pub fn parse_and_resolve(source: &str) -> Result<ResolvedBlueprint, BlueprintError> {
    resolve(toml::from_str(source)?)
}
fn add_id(ids: &mut HashSet<String>, errors: &mut Vec<String>, id: &str, kind: &str) {
    if !ids.insert(id.to_owned()) {
        errors.push(format!("duplicate id '{id}' ({kind})"));
    }
}
fn logical_size(
    value: Option<&String>,
    label: &str,
    id: &str,
    errors: &mut Vec<String>,
) -> Option<u32> {
    let Some(value) = value else { return None };
    match value.strip_suffix("px").and_then(|n| n.parse().ok()) {
        Some(size) => Some(size),
        None => {
            errors.push(format!(
                "{label} '{id}': size must be a pixel value such as 52px"
            ));
            None
        }
    }
}
fn parse_color(value: &str, name: &str, errors: &mut Vec<String>) -> Color {
    let valid = value
        .strip_prefix('#')
        .filter(|v| v.len() == 6)
        .and_then(|v| u32::from_str_radix(v, 16).ok());
    match valid {
        Some(v) => Color {
            r: (v >> 16) as u8,
            g: (v >> 8) as u8,
            b: v as u8,
            a: 255,
        },
        None => {
            errors.push(format!("color token '{name}': expected #RRGGBB"));
            Color {
                r: 0,
                g: 0,
                b: 0,
                a: 255,
            }
        }
    }
}
fn color_ref(
    tokens: &ColorTokensSource,
    reference: &str,
    role: &str,
    errors: &mut Vec<String>,
) -> Color {
    match tokens.values.get(reference) {
        Some(value) => parse_color(value, reference, errors),
        None => {
            errors.push(format!("visual.{role}: missing color token '{reference}'"));
            Color {
                r: 0,
                g: 0,
                b: 0,
                a: 255,
            }
        }
    }
}
fn resolve_visual(
    source: Option<&VisualSource>,
    tokens: &TokensSource,
    errors: &mut Vec<String>,
) -> Option<ResolvedVisual> {
    let Some(v) = source else { return None };
    let scale = match (
        tokens.r#type.display,
        tokens.r#type.heading,
        tokens.r#type.body,
        tokens.r#type.caption,
    ) {
        (Some(display), Some(heading), Some(body), Some(caption))
            if [display, heading, body, caption].iter().all(|n| *n > 0) =>
        {
            TypeScale {
                display,
                heading,
                body,
                caption,
            }
        }
        _ => {
            errors.push("visual: tokens.type must define positive display, heading, body, and caption values".into());
            TypeScale {
                display: 1,
                heading: 1,
                body: 1,
                caption: 1,
            }
        }
    };
    let corner_radius = match tokens.corners.get(&v.corner) {
        Some(radius) => *radius,
        None => {
            errors.push(format!(
                "visual.corner: missing corner token '{}'",
                v.corner
            ));
            0
        }
    };
    let border_policy = match v.border_policy.as_str() {
        "none" => BorderPolicy::None,
        "minimal" => BorderPolicy::Minimal,
        "defined" => BorderPolicy::Defined,
        _ => {
            errors.push(format!(
                "visual.border_policy: invalid policy '{}'",
                v.border_policy
            ));
            BorderPolicy::None
        }
    };
    Some(ResolvedVisual {
        palette: Palette {
            canvas: color_ref(&tokens.color, &v.canvas, "canvas", errors),
            surface: color_ref(&tokens.color, &v.surface, "surface", errors),
            surface_raised: color_ref(&tokens.color, &v.surface_raised, "surface_raised", errors),
            text: color_ref(&tokens.color, &v.text, "text", errors),
            text_muted: color_ref(&tokens.color, &v.text_muted, "text_muted", errors),
            accent: color_ref(&tokens.color, &v.accent, "accent", errors),
            border: color_ref(&tokens.color, &v.border, "border", errors),
        },
        type_scale: scale,
        corner_radius,
        border_policy,
    })
}
fn surface(value: Option<&String>, id: &str, errors: &mut Vec<String>) -> SurfaceRole {
    match value.map(String::as_str).unwrap_or("panel") {
        "canvas" => SurfaceRole::Canvas,
        "panel" => SurfaceRole::Panel,
        "raised" => SurfaceRole::Raised,
        "transparent" => SurfaceRole::Transparent,
        other => {
            errors.push(format!("region '{id}': invalid surface role '{other}'"));
            SurfaceRole::Panel
        }
    }
}

pub fn resolve(source: SourceBlueprint) -> Result<ResolvedBlueprint, BlueprintError> {
    let mut errors = Vec::new();
    let mut ids = HashSet::new();
    let density = match source.screen.density.as_str() {
        "comfortable" => Density::Comfortable,
        "dense" => Density::Dense,
        other => {
            errors.push(format!(
                "screen.density: unknown density '{other}' (expected comfortable or dense)"
            ));
            Density::Comfortable
        }
    };
    let spacing = &source.tokens.spacing;
    let mut regions = Vec::new();
    for r in &source.region {
        add_id(&mut ids, &mut errors, &r.id, "region");
        let grow = r.grow.unwrap_or(0.0);
        if !grow.is_finite() || grow < 0.0 {
            errors.push(format!(
                "region '{}': grow must be finite and non-negative",
                r.id
            ));
        }
        regions.push(ResolvedRegion {
            id: r.id.clone(),
            role: region_role(&r.role, &r.id, &mut errors),
            importance: importance(&r.importance, &format!("region '{}'", r.id), &mut errors),
            width: logical_size(r.width.as_ref(), "region width", &r.id, &mut errors),
            height: logical_size(r.height.as_ref(), "region height", &r.id, &mut errors),
            grow: if grow.is_finite() && grow >= 0.0 {
                grow
            } else {
                0.0
            },
            surface: surface(r.surface.as_ref(), &r.id, &mut errors),
        });
    }
    let region_ids: HashSet<_> = regions.iter().map(|r| r.id.as_str()).collect();
    let mut elements = Vec::new();
    for e in &source.element {
        add_id(&mut ids, &mut errors, &e.id, "element");
        if !region_ids.contains(e.region.as_str()) {
            errors.push(format!("element '{}': missing region '{}'", e.id, e.region));
        }
        let element_kind = kind(&e.kind, &format!("element '{}'", e.id), &mut errors);
        let presentation =
            collection_presentation(e.presentation.as_deref(), &e.id, element_kind, &mut errors);
        let action = match (element_kind, e.action.as_deref()) {
            (ElementKind::Command, Some(action)) => action_id(action, &e.id, &mut errors),
            (ElementKind::Command, None) => {
                errors.push(format!(
                    "element '{}': command elements require an action",
                    e.id
                ));
                None
            }
            (_kind, Some(_)) => {
                errors.push(format!(
                    "element '{}': action is only valid for command elements",
                    e.id
                ));
                None
            }
            (_, None) => None,
        };
        elements.push(ResolvedElement {
            id: e.id.clone(),
            region: e.region.clone(),
            kind: element_kind,
            importance: importance(&e.importance, &format!("element '{}'", e.id), &mut errors),
            label: e.label.clone().unwrap_or_else(|| e.id.replace('_', " ")),
            presentation,
            action,
        });
    }
    let composition_ids: HashSet<_> = source.composition.iter().map(|c| c.id.as_str()).collect();
    let element_ids: HashSet<_> = elements.iter().map(|e| e.id.as_str()).collect();
    let mut compositions = Vec::new();
    for c in &source.composition {
        add_id(&mut ids, &mut errors, &c.id, "composition");
        let grow = c.grow.unwrap_or(0.0);
        if !grow.is_finite() || grow < 0.0 {
            errors.push(format!(
                "composition '{}': grow must be finite and non-negative",
                c.id
            ));
        }
        let composition_kind = composition_kind(&c.kind, &c.id, &mut errors);
        let composition_axis = axis(c.axis.as_deref(), composition_kind, &c.id, &mut errors);
        let mut children = Vec::new();
        for child in &c.children {
            if region_ids.contains(child.as_str()) {
                children.push(CompositionChild::Region(child.clone()));
            } else if composition_ids.contains(child.as_str()) {
                children.push(CompositionChild::Composition(child.clone()));
            } else if element_ids.contains(child.as_str()) {
                errors.push(format!(
                    "composition '{}': child '{}' is an element; expected a region or composition",
                    c.id, child
                ));
            } else {
                errors.push(format!(
                    "composition '{}': missing child reference '{}'",
                    c.id, child
                ));
            }
        }
        let gap = c
            .gap
            .as_ref()
            .and_then(|g| spacing.get(g))
            .copied()
            .unwrap_or(0);
        if c.gap.is_some() && !spacing.contains_key(c.gap.as_ref().unwrap()) {
            errors.push(format!(
                "composition '{}': missing spacing token '{}'",
                c.id,
                c.gap.as_ref().unwrap()
            ));
        }
        let padding = c
            .padding
            .as_ref()
            .and_then(|g| spacing.get(g))
            .copied()
            .unwrap_or(0);
        if c.padding.is_some() && !spacing.contains_key(c.padding.as_ref().unwrap()) {
            errors.push(format!(
                "composition '{}': missing spacing token '{}'",
                c.id,
                c.padding.as_ref().unwrap()
            ));
        }
        if children.len() < 2 {
            errors.push(format!(
                "composition '{}': requires at least two children",
                c.id
            ));
        }
        compositions.push(ResolvedComposition {
            id: c.id.clone(),
            kind: composition_kind,
            axis: composition_axis,
            children,
            gap,
            padding,
            grow: if grow.is_finite() && grow >= 0.0 {
                grow
            } else {
                0.0
            },
        });
    }
    let root = match source.screen.root.as_deref() {
        None => {
            errors.push("screen.root: missing explicit composition reference".into());
            String::new()
        }
        Some(root) if !composition_ids.contains(root) => {
            errors.push(format!("screen.root: '{root}' is not a composition"));
            String::new()
        }
        Some(root) => root.to_owned(),
    };
    let mut visiting = HashSet::new();
    let mut visited = HashSet::new();
    fn visit(
        id: &str,
        all: &HashMap<&str, &ResolvedComposition>,
        visiting: &mut HashSet<String>,
        visited: &mut HashSet<String>,
        errors: &mut Vec<String>,
    ) {
        if visiting.contains(id) {
            errors.push(format!("composition cycle detected at '{id}'"));
            return;
        }
        if !visited.insert(id.to_owned()) {
            return;
        }
        visiting.insert(id.to_owned());
        if let Some(c) = all.get(id) {
            for child in &c.children {
                if let CompositionChild::Composition(child) = child {
                    visit(child, all, visiting, visited, errors);
                }
            }
        }
        visiting.remove(id);
    }
    let by_id: HashMap<_, _> = compositions.iter().map(|c| (c.id.as_str(), c)).collect();
    if !root.is_empty() {
        visit(&root, &by_id, &mut visiting, &mut visited, &mut errors);
    }
    let mut resolved_fixtures = Vec::new();
    for f in &source.fixture {
        add_id(&mut ids, &mut errors, &f.id, "fixture");
        let mut seen_content = HashSet::new();
        let mut content = Vec::new();
        for record in &f.content {
            if !seen_content.insert(record.element.clone()) {
                errors.push(format!(
                    "fixture '{}': duplicate content for element '{}'",
                    f.id, record.element
                ));
                continue;
            }
            let Some(element) = elements.iter().find(|e| e.id == record.element) else {
                errors.push(format!(
                    "fixture '{}': content references missing element '{}'",
                    f.id, record.element
                ));
                continue;
            };
            let families = usize::from(record.items.is_some())
                + usize::from(record.properties.is_some())
                + usize::from(record.text.is_some())
                + usize::from(record.nodes.is_some())
                + usize::from(record.document.is_some())
                + usize::from(record.command.is_some());
            if families != 1 {
                errors.push(format!(
                    "fixture '{}': content for '{}' must contain exactly one payload family",
                    f.id, record.element
                ));
                continue;
            }
            if record.selected.is_some() && record.items.is_none() && record.nodes.is_none() {
                errors.push(format!(
                    "fixture '{}': selected is only valid with collection or tree content for '{}'",
                    f.id, record.element
                ));
                continue;
            }
            if let Some(items) = &record.items {
                if element.kind != ElementKind::Collection {
                    errors.push(format!(
                        "fixture '{}': collection content on '{}' requires a collection element",
                        f.id, record.element
                    ));
                    continue;
                }
                let mut item_ids = HashSet::new();
                let resolved_items = items
                    .iter()
                    .filter_map(|item| {
                        if !item_ids.insert(item.id.clone()) {
                            errors.push(format!(
                                "fixture '{}': duplicate collection item id '{}' for '{}'",
                                f.id, item.id, record.element
                            ));
                            None
                        } else {
                            Some(ResolvedCollectionItem {
                                id: item.id.clone(),
                                label: item.label.clone(),
                            })
                        }
                    })
                    .collect();
                if let Some(selected) = &record.selected {
                    if !item_ids.contains(selected) {
                        errors.push(format!(
                            "fixture '{}': selected item '{}' is absent from collection '{}'",
                            f.id, selected, record.element
                        ));
                    }
                }
                content.push(ResolvedFixtureContent::Collection {
                    element: record.element.clone(),
                    items: resolved_items,
                    selected: record.selected.clone(),
                });
            } else if let Some(properties) = &record.properties {
                if element.kind != ElementKind::PropertySheet {
                    errors.push(format!(
                        "fixture '{}': property content on '{}' requires a property_sheet element",
                        f.id, record.element
                    ));
                    continue;
                }
                content.push(ResolvedFixtureContent::Properties {
                    element: record.element.clone(),
                    properties: properties
                        .iter()
                        .map(|p| ResolvedProperty {
                            name: p.name.clone(),
                            value: p.value.clone(),
                        })
                        .collect(),
                });
            } else if let Some(text) = &record.text {
                if element.kind != ElementKind::Status {
                    errors.push(format!(
                        "fixture '{}': text content on '{}' requires a status element",
                        f.id, record.element
                    ));
                    continue;
                }
                content.push(ResolvedFixtureContent::Text {
                    element: record.element.clone(),
                    text: text.clone(),
                });
            } else if let Some(nodes) = &record.nodes {
                if element.kind != ElementKind::Tree {
                    errors.push(format!(
                        "fixture '{}': tree content on '{}' requires a tree element",
                        f.id, record.element
                    ));
                    continue;
                }
                let mut node_ids = HashSet::new();
                let resolved_nodes: Vec<_> = nodes
                    .iter()
                    .filter_map(|node| {
                        if !node_ids.insert(node.id.clone()) {
                            errors.push(format!(
                                "fixture '{}': duplicate tree node id '{}' for '{}'",
                                f.id, node.id, record.element
                            ));
                            None
                        } else {
                            Some(ResolvedTreeNode {
                                id: node.id.clone(),
                                label: node.label.clone(),
                                parent: node.parent.clone(),
                            })
                        }
                    })
                    .collect();
                for node in &resolved_nodes {
                    if let Some(parent) = &node.parent {
                        if !node_ids.contains(parent) {
                            errors.push(format!(
                                "fixture '{}': tree node '{}' references missing parent '{}' for '{}'",
                                f.id, node.id, parent, record.element
                            ));
                        }
                        if parent == &node.id {
                            errors.push(format!(
                                "fixture '{}': tree node '{}' cannot parent itself for '{}'",
                                f.id, node.id, record.element
                            ));
                        }
                    }
                }
                if let Some(selected) = &record.selected {
                    if !node_ids.contains(selected) {
                        errors.push(format!(
                            "fixture '{}': selected tree node '{}' is absent from '{}'",
                            f.id, selected, record.element
                        ));
                    }
                }
                let parents: HashMap<_, _> = resolved_nodes
                    .iter()
                    .map(|node| (node.id.as_str(), node.parent.as_deref()))
                    .collect();
                for node in &resolved_nodes {
                    let mut seen = HashSet::new();
                    let mut current = Some(node.id.as_str());
                    while let Some(id) = current {
                        if !seen.insert(id) {
                            errors.push(format!(
                                "fixture '{}': tree parent cycle includes node '{}' for '{}'",
                                f.id, id, record.element
                            ));
                            break;
                        }
                        current = parents.get(id).copied().flatten();
                    }
                }
                content.push(ResolvedFixtureContent::Tree {
                    element: record.element.clone(),
                    nodes: resolved_nodes,
                    selected: record.selected.clone(),
                });
            } else if let Some(document) = &record.document {
                if element.kind != ElementKind::Document {
                    errors.push(format!(
                        "fixture '{}': document content on '{}' requires a document element",
                        f.id, record.element
                    ));
                    continue;
                }
                content.push(ResolvedFixtureContent::Document {
                    element: record.element.clone(),
                    title: document.title.clone(),
                    paragraphs: document.paragraphs.clone(),
                });
            } else if let Some(command) = &record.command {
                if element.kind != ElementKind::Command {
                    errors.push(format!(
                        "fixture '{}': command content on '{}' requires a command element",
                        f.id, record.element
                    ));
                    continue;
                }
                content.push(ResolvedFixtureContent::Command {
                    element: record.element.clone(),
                    enabled: command.enabled,
                    reason: command.reason.clone(),
                });
            }
        }
        resolved_fixtures.push(ResolvedFixture {
            id: f.id.clone(),
            state: f.state.clone(),
            content,
        });
    }
    if let Some(d) = &source.design.dominant {
        if !region_ids.contains(d.as_str()) && !element_ids.contains(d.as_str()) {
            errors.push(format!("design.dominant: missing reference '{d}'"));
        }
    }
    if !errors.is_empty() {
        return Err(BlueprintError::Validation(errors.join("\n")));
    }
    let visual = resolve_visual(source.visual.as_ref(), &source.tokens, &mut errors);
    if !errors.is_empty() {
        return Err(BlueprintError::Validation(errors.join("\n")));
    }
    Ok(ResolvedBlueprint {
        screen: ResolvedScreen {
            id: source.screen.id,
            purpose: source.screen.purpose,
            density,
        },
        design: source.design,
        root,
        regions,
        compositions,
        elements,
        fixtures: resolved_fixtures,
        visual,
    })
}

impl ResolvedBlueprint {
    pub fn semantic_tree(&self) -> String {
        fn walk(
            id: &str,
            b: &ResolvedBlueprint,
            out: &mut String,
            indent: usize,
            seen: &mut HashSet<String>,
        ) {
            if !seen.insert(id.to_owned()) {
                out.push_str(&format!("{:indent$}↻ {id}\n", "", indent = indent));
                return;
            }
            let c = b.compositions.iter().find(|c| c.id == id).unwrap();
            out.push_str(&format!(
                "{:indent$}composition {} ({}, {})\n",
                "",
                c.id,
                c.kind.as_str(),
                c.axis.as_str(),
                indent = indent
            ));
            for child in &c.children {
                match child {
                    CompositionChild::Composition(child) => walk(child, b, out, indent + 2, seen),
                    CompositionChild::Region(region) => {
                        out.push_str(&format!(
                            "{:indent$}region {} (role {})\n",
                            "",
                            region,
                            b.regions
                                .iter()
                                .find(|candidate| candidate.id == *region)
                                .map(|candidate| candidate.role.as_str())
                                .unwrap_or("unknown"),
                            indent = indent + 2
                        ));
                        for e in b.elements.iter().filter(|e| e.region == *region) {
                            out.push_str(&format!(
                                "{:indent$}element {} ({:?}{}{})\n",
                                "",
                                e.id,
                                e.kind,
                                e.action
                                    .as_ref()
                                    .map(|action| format!(", action {}", action.as_str()))
                                    .unwrap_or_default(),
                                e.presentation
                                    .map(|presentation| format!(", presentation {presentation:?}"))
                                    .unwrap_or_default(),
                                indent = indent + 4
                            ));
                        }
                    }
                }
            }
        }
        let mut out = format!(
            "screen {} — {} (density {:?})\n",
            self.screen.id, self.screen.purpose, self.screen.density
        );
        walk(&self.root, self, &mut out, 2, &mut HashSet::new());
        for f in &self.fixtures {
            out.push_str(&format!("  fixture {} [{}]\n", f.id, f.state));
            for content in &f.content {
                match content {
                    ResolvedFixtureContent::Collection {
                        element,
                        items,
                        selected,
                    } => out.push_str(&format!(
                        "    content {element}: collection {} items, selected {:?}\n",
                        items.len(),
                        selected
                    )),
                    ResolvedFixtureContent::Properties {
                        element,
                        properties,
                    } => out.push_str(&format!(
                        "    content {element}: {} properties\n",
                        properties.len()
                    )),
                    ResolvedFixtureContent::Text { element, text } => {
                        out.push_str(&format!("    content {element}: {text}\n"))
                    }
                    ResolvedFixtureContent::Tree {
                        element,
                        nodes,
                        selected,
                    } => out.push_str(&format!(
                        "    content {element}: tree {} nodes, selected {:?}\n",
                        nodes.len(),
                        selected
                    )),
                    ResolvedFixtureContent::Document {
                        element,
                        title,
                        paragraphs,
                    } => out.push_str(&format!(
                        "    content {element}: document '{title}', {} paragraphs\n",
                        paragraphs.len()
                    )),
                    ResolvedFixtureContent::Command {
                        element,
                        enabled,
                        reason,
                    } => out.push_str(&format!(
                        "    content {element}: command enabled={enabled}, reason {:?}\n",
                        reason
                    )),
                }
            }
        }
        out
    }

    pub fn command_state<'a>(&'a self, fixture: &str, element: &str) -> ResolvedCommandState<'a> {
        self.fixtures
            .iter()
            .find(|fixture_value| fixture_value.id == fixture)
            .and_then(|fixture_value| {
                fixture_value
                    .content
                    .iter()
                    .find_map(|content| match content {
                        ResolvedFixtureContent::Command {
                            element: content_element,
                            enabled,
                            reason,
                        } if content_element == element => Some(ResolvedCommandState {
                            enabled: *enabled,
                            reason: reason.as_deref(),
                        }),
                        _ => None,
                    })
            })
            .unwrap_or(ResolvedCommandState {
                enabled: true,
                reason: None,
            })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ResolvedCommandState<'a> {
    pub enabled: bool,
    pub reason: Option<&'a str>,
}

#[cfg(test)]
mod tests {
    use super::*;
    fn project() -> &'static str {
        include_str!("../../../examples/project-browser.toml")
    }
    fn reader() -> &'static str {
        include_str!("../../../specimens/reader-workspace.toml")
    }
    #[test]
    fn project_browser_resolves() {
        assert_eq!(parse_and_resolve(project()).unwrap().root, "workspace");
    }

    #[test]
    fn all_supported_region_roles_resolve_to_typed_values() {
        let cases = [
            ("commands", RegionRole::Commands),
            ("controls", RegionRole::Controls),
            ("navigation", RegionRole::Navigation),
            ("primary_content", RegionRole::PrimaryContent),
            ("inspector", RegionRole::Inspector),
            ("status", RegionRole::Status),
        ];
        for (source_role, expected) in cases {
            let source = format!(
                "[screen]\nid='x'\npurpose='x'\nroot='root'\n[[region]]\nid='region'\nrole='{source_role}'\nimportance='primary'\n[[region]]\nid='other'\nrole='primary_content'\nimportance='secondary'\n[[composition]]\nid='root'\nkind='split'\nchildren=['region','other']"
            );
            let blueprint = parse_and_resolve(&source).unwrap();
            assert_eq!(blueprint.regions[0].role, expected);
            assert!(blueprint
                .semantic_tree()
                .contains(&format!("region region (role {source_role})")));
        }
    }

    #[test]
    fn unknown_region_role_is_rejected_with_region_and_value() {
        let source = "[screen]\nid='x'\npurpose='x'\nroot='root'\n[[region]]\nid='library'\nrole='sidebar'\nimportance='primary'\n[[composition]]\nid='root'\nkind='split'\nchildren=['library']";
        let error = parse_and_resolve(source).unwrap_err().to_string();
        assert!(error.contains("region 'library'") && error.contains("unknown role 'sidebar'"));
    }

    fn composition_source(kind: &str, axis: Option<&str>) -> String {
        let axis = axis
            .map(|axis| format!("axis='{axis}'\n"))
            .unwrap_or_default();
        format!(
            "[screen]\nid='x'\npurpose='x'\nroot='root'\n[[region]]\nid='first'\nrole='navigation'\nimportance='primary'\n[[region]]\nid='second'\nrole='inspector'\nimportance='secondary'\n[[composition]]\nid='root'\nkind='{kind}'\n{axis}children=['first','second']"
        )
    }

    #[test]
    fn composition_kinds_and_axes_resolve_with_compatibility_rules() {
        let resolved = |kind, axis| parse_and_resolve(&composition_source(kind, axis)).unwrap();
        for (kind, axis, expected_kind, expected_axis) in [
            (
                "split",
                Some("horizontal"),
                CompositionKind::Split,
                Axis::Horizontal,
            ),
            (
                "split",
                Some("vertical"),
                CompositionKind::Split,
                Axis::Vertical,
            ),
            ("split", None, CompositionKind::Split, Axis::Horizontal),
            ("row", None, CompositionKind::Row, Axis::Horizontal),
            (
                "row",
                Some("horizontal"),
                CompositionKind::Row,
                Axis::Horizontal,
            ),
            ("column", None, CompositionKind::Column, Axis::Vertical),
            (
                "column",
                Some("vertical"),
                CompositionKind::Column,
                Axis::Vertical,
            ),
        ] {
            let composition = &resolved(kind, axis).compositions[0];
            assert_eq!(composition.kind, expected_kind);
            assert_eq!(composition.axis, expected_axis);
        }
        let vertical = resolved("column", None).semantic_tree();
        assert!(vertical.contains("composition root (column, vertical)"));
    }

    #[test]
    fn unsupported_and_contradictory_compositions_are_rejected() {
        for kind in ["stack", "overlay", "grid"] {
            let error = parse_and_resolve(&composition_source(kind, None))
                .unwrap_err()
                .to_string();
            assert!(error.contains("composition 'root'") && error.contains(kind));
        }
        let row_error = parse_and_resolve(&composition_source("row", Some("vertical")))
            .unwrap_err()
            .to_string();
        assert!(row_error.contains("composition 'root': row requires horizontal axis"));
        let column_error = parse_and_resolve(&composition_source("column", Some("horizontal")))
            .unwrap_err()
            .to_string();
        assert!(column_error.contains("composition 'root': column requires vertical axis"));
        let axis_error = parse_and_resolve(&composition_source("split", Some("diagonal")))
            .unwrap_err()
            .to_string();
        assert!(axis_error.contains("composition 'root': unknown axis 'diagonal'"));
    }
    #[test]
    fn collection_presentations_are_typed_validated_and_defaulted() {
        let project_blueprint = parse_and_resolve(project()).unwrap();
        assert_eq!(
            project_blueprint
                .elements
                .iter()
                .find(|element| element.id == "navigation_items")
                .unwrap()
                .presentation,
            Some(CollectionPresentation::List)
        );
        assert_eq!(
            project_blueprint
                .elements
                .iter()
                .find(|element| element.id == "project_collection")
                .unwrap()
                .presentation,
            Some(CollectionPresentation::AdaptiveCards)
        );
        let semantic = project_blueprint.semantic_tree();
        assert!(semantic.contains("navigation_items (Collection, presentation List)"));
        assert!(semantic.contains("project_collection (Collection, presentation AdaptiveCards)"));
        let dependency =
            parse_and_resolve(include_str!("../../../specimens/dependency-workbench.toml"))
                .unwrap();
        assert!(dependency
            .elements
            .iter()
            .filter(|element| element.kind == ElementKind::Collection)
            .all(|element| element.presentation == Some(CollectionPresentation::List)));
        let omitted = project()
            .replace("presentation = \"list\"\n", "")
            .replace("presentation = \"adaptive_cards\"\n", "");
        assert!(parse_and_resolve(&omitted)
            .unwrap()
            .elements
            .iter()
            .filter(|element| element.kind == ElementKind::Collection)
            .all(|element| element.presentation == Some(CollectionPresentation::List)));
        let unknown = project().replace("adaptive_cards", "grid");
        assert!(parse_and_resolve(&unknown)
            .unwrap_err()
            .to_string()
            .contains("unknown collection presentation 'grid'"));
        let command = reader().replace(
            "kind = \"command\"\nimportance = \"primary\"",
            "kind = \"command\"\nimportance = \"primary\"\npresentation = \"list\"",
        );
        assert!(parse_and_resolve(&command)
            .unwrap_err()
            .to_string()
            .contains("only valid for collection elements"));
        let search = project().replace(
            "kind = \"search\"\nimportance = \"secondary\"",
            "kind = \"search\"\nimportance = \"secondary\"\npresentation = \"list\"",
        );
        assert!(parse_and_resolve(&search)
            .unwrap_err()
            .to_string()
            .contains("only valid for collection elements"));
    }
    #[test]
    fn screen_density_is_typed_defaulted_and_validated() {
        assert_eq!(
            parse_and_resolve(project()).unwrap().screen.density,
            Density::Comfortable
        );
        assert_eq!(
            parse_and_resolve(include_str!(
                "../../../specimens/reader-workspace-visual.toml"
            ))
            .unwrap()
            .screen
            .density,
            Density::Comfortable
        );
        assert_eq!(
            parse_and_resolve(include_str!("../../../specimens/dependency-workbench.toml"))
                .unwrap()
                .screen
                .density,
            Density::Dense
        );
        let omitted = "[screen]\nid='x'\npurpose='x'\nroot='root'\n[[region]]\nid='a'\nrole='navigation'\nimportance='primary'\n[[region]]\nid='b'\nrole='inspector'\nimportance='secondary'\n[[composition]]\nid='root'\nkind='split'\nchildren=['a','b']";
        assert_eq!(
            parse_and_resolve(omitted).unwrap().screen.density,
            Density::Comfortable
        );
        let unknown = omitted.replace("purpose='x'", "purpose='x'\ndensity='compact'");
        assert!(parse_and_resolve(&unknown)
            .unwrap_err()
            .to_string()
            .contains("screen.density: unknown density 'compact'"));
        let semantic =
            parse_and_resolve(include_str!("../../../specimens/dependency-workbench.toml"))
                .unwrap()
                .semantic_tree();
        assert!(semantic.contains("density Dense"));
    }
    #[test]
    fn reader_resolves_nested_tree() {
        let b = parse_and_resolve(reader()).unwrap();
        assert_eq!(b.root, "workspace");
        assert!(matches!(
            b.compositions[0].children[1],
            CompositionChild::Composition(_)
        ));
        assert_eq!(
            b.regions
                .iter()
                .find(|r| r.id == "app_commands")
                .unwrap()
                .height,
            Some(52)
        );
        assert!(b.elements.iter().any(|e| e.kind == ElementKind::Document));
    }
    #[test]
    fn invalid_root_and_missing_child_are_reported() {
        let e = parse_and_resolve("[screen]\nid='x'\npurpose='x'\nroot='bad'\n[[composition]]\nid='root'\nkind='split'\nchildren=['missing','also_missing']").unwrap_err().to_string();
        assert!(e.contains("screen.root") && e.contains("missing child"));
    }
    #[test]
    fn cycle_is_rejected() {
        let e = parse_and_resolve("[screen]\nid='x'\npurpose='x'\nroot='a'\n[[composition]]\nid='a'\nkind='split'\nchildren=['b','r']\n[[composition]]\nid='b'\nkind='split'\nchildren=['a','r']\n[[region]]\nid='r'\nrole='primary_content'\nimportance='primary'").unwrap_err().to_string();
        assert!(e.contains("composition cycle detected"));
    }
    #[test]
    fn malformed_height_is_rejected() {
        let e = parse_and_resolve("[screen]\nid='x'\npurpose='x'\nroot='root'\n[[composition]]\nid='root'\nkind='split'\nchildren=['a','b']\n[[region]]\nid='a'\nrole='navigation'\nimportance='primary'\nheight='tall'\n[[region]]\nid='b'\nrole='inspector'\nimportance='secondary'").unwrap_err().to_string();
        assert!(e.contains("height 'a': size must be"));
    }
    #[test]
    fn existing_bad_diagnostics_remain() {
        let e = parse_and_resolve("[screen]\nid='x'\npurpose='x'\nroot='root'\n[[region]]\nid='same'\nrole='navigation'\nimportance='primary'\n[[region]]\nid='same'\nrole='inspector'\nimportance='secondary'\n[[composition]]\nid='root'\nkind='split'\nchildren=['same','nope']\ngap='md'").unwrap_err().to_string();
        assert!(
            e.contains("duplicate id")
                && e.contains("missing child")
                && e.contains("missing spacing token")
        );
    }
    #[test]
    fn dependency_workbench_fixture_content_resolves_and_differs() {
        let b = parse_and_resolve(include_str!("../../../specimens/dependency-workbench.toml"))
            .unwrap();
        let healthy = b.fixtures.iter().find(|f| f.id == "healthy").unwrap();
        let advisory = b.fixtures.iter().find(|f| f.id == "advisory").unwrap();
        assert!(healthy.content.iter().any(|c| matches!(c, ResolvedFixtureContent::Text { text, .. } if text.contains("0 advisories"))));
        assert!(advisory.content.iter().any(
            |c| matches!(c, ResolvedFixtureContent::Text { text, .. } if text.contains("advisory"))
        ));
        assert_ne!(format!("{healthy:?}"), format!("{advisory:?}"));
    }
    #[test]
    fn fixture_content_validation_rejects_invalid_records() {
        let base = "[screen]\nid='x'\npurpose='x'\nroot='root'\n[[region]]\nid='r'\nrole='primary_content'\nimportance='primary'\n[[composition]]\nid='root'\nkind='split'\nchildren=['r','r']\n[[element]]\nid='list'\nregion='r'\nkind='collection'\nimportance='primary'\n[[fixture]]\nid='f'\nstate='x'\n";
        let missing = format!("{base}[[fixture.content]]\nelement='nope'\ntext='x'");
        assert!(parse_and_resolve(&missing)
            .unwrap_err()
            .to_string()
            .contains("missing element"));
        let selected = format!(
            "{base}[[fixture.content]]\nelement='list'\nitems=[{{id='a',label='A'}}]\nselected='b'"
        );
        assert!(parse_and_resolve(&selected)
            .unwrap_err()
            .to_string()
            .contains("selected item"));
        let incompatible = format!(
            "{base}[[fixture.content]]\nelement='list'\nproperties=[{{name='x',value='y'}}]"
        );
        assert!(parse_and_resolve(&incompatible)
            .unwrap_err()
            .to_string()
            .contains("property content"));
        let mixed = format!("{base}[[fixture.content]]\nelement='list'\nitems=[]\ntext='x'");
        assert!(parse_and_resolve(&mixed)
            .unwrap_err()
            .to_string()
            .contains("exactly one payload"));
        let text = format!("{base}[[fixture.content]]\nelement='list'\ntext='x'");
        assert!(parse_and_resolve(&text)
            .unwrap_err()
            .to_string()
            .contains("text content"));
    }
    #[test]
    fn fixture_content_unknown_and_duplicate_items_are_rejected() {
        let base = "[screen]\nid='x'\npurpose='x'\nroot='root'\n[[region]]\nid='r'\nrole='primary_content'\nimportance='primary'\n[[composition]]\nid='root'\nkind='split'\nchildren=['r','r']\n[[element]]\nid='list'\nregion='r'\nkind='collection'\nimportance='primary'\n[[fixture]]\nid='f'\nstate='x'\n[[fixture.content]]\nelement='list'\nitems=[{id='a',label='A'},{id='a',label='A2'}]\n";
        assert!(parse_and_resolve(base)
            .unwrap_err()
            .to_string()
            .contains("duplicate collection item"));
        let unknown = base.replace(
            "items=[{id='a',label='A'},{id='a',label='A2'}]",
            "items=[]\nwat='x'",
        );
        assert!(parse_and_resolve(&unknown)
            .unwrap_err()
            .to_string()
            .contains("unknown field"));
    }
    #[test]
    fn visual_reader_resolves_concrete_palette_and_profile() {
        let b = parse_and_resolve(include_str!(
            "../../../specimens/reader-workspace-visual.toml"
        ))
        .unwrap();
        let v = b.visual.unwrap();
        assert_eq!(
            v.palette.canvas,
            Color {
                r: 38,
                g: 43,
                b: 51,
                a: 255
            }
        );
        assert_eq!(v.corner_radius, 7);
    }
    #[test]
    fn malformed_visual_inputs_and_unknown_fields_are_rejected() {
        let bad = "[screen]\nid='x'\npurpose='x'\nroot='root'\n[tokens.color]\ncanvas='#nope'\n[visual]\ncanvas='canvas'\nsurface='canvas'\nsurface_raised='canvas'\ntext='canvas'\ntext_muted='canvas'\naccent='canvas'\nborder='canvas'\ncorner='missing'\nborder_policy='wrong'\n[[composition]]\nid='root'\nkind='split'\nchildren=['a','b']\n[[region]]\nid='a'\nrole='navigation'\nimportance='primary'\n[[region]]\nid='b'\nrole='inspector'\nimportance='secondary'";
        let e = parse_and_resolve(bad).unwrap_err().to_string();
        assert!(
            e.contains("expected #RRGGBB")
                && e.contains("missing corner token")
                && e.contains("invalid policy")
        );
        let unknown = "[screen]\nid='x'\npurpose='x'\nroot='root'\nunknown='x'";
        assert!(parse_and_resolve(unknown)
            .unwrap_err()
            .to_string()
            .contains("unknown field"));
    }
    #[test]
    fn missing_visual_color_reference_is_rejected() {
        let source = include_str!("../../../specimens/reader-workspace-visual.toml")
            .replace("border = \"border\"", "border = \"missing_border\"");
        let error = parse_and_resolve(&source).unwrap_err().to_string();
        assert!(error.contains("visual.border: missing color token 'missing_border'"));
    }
    #[test]
    fn invalid_region_surface_is_rejected() {
        let source = include_str!("../../../specimens/reader-workspace-visual.toml").replace(
            "surface = \"canvas\"\ngrow = 1",
            "surface = \"glass\"\ngrow = 1",
        );
        let error = parse_and_resolve(&source).unwrap_err().to_string();
        assert!(error.contains("invalid surface role 'glass'"));
    }

    #[test]
    fn reader_fixtures_resolve_typed_tree_document_and_existing_content() {
        let b = parse_and_resolve(include_str!(
            "../../../specimens/reader-workspace-visual.toml"
        ))
        .unwrap();
        let reading = b.fixtures.iter().find(|f| f.id == "reading").unwrap();
        assert!(reading.content.iter().any(|content| matches!(
            content,
            ResolvedFixtureContent::Tree { nodes, selected, .. }
                if nodes.len() == 5 && selected.as_deref() == Some("chapter_two")
        )));
        assert!(reading.content.iter().any(|content| matches!(
            content,
            ResolvedFixtureContent::Document { title, paragraphs, .. }
                if title == "The Quiet Machine" && paragraphs.len() == 3
        )));
        assert!(reading.content.iter().any(|content| matches!(
            content,
            ResolvedFixtureContent::Properties { properties, .. } if properties.len() == 3
        )));
        assert!(reading.content.iter().any(|content| matches!(
            content,
            ResolvedFixtureContent::Text { text, .. } if text.contains("Ready to read")
        )));
        let empty = b.fixtures.iter().find(|f| f.id == "empty").unwrap();
        assert!(empty.content.iter().any(|content| matches!(
            content,
            ResolvedFixtureContent::Tree { nodes, .. } if nodes.is_empty()
        )));
        assert!(empty.content.iter().any(|content| matches!(
            content,
            ResolvedFixtureContent::Document { title, paragraphs, .. }
                if title == "No document loaded" && paragraphs.is_empty()
        )));
        let debug = b.semantic_tree();
        assert!(debug.contains("document_outline: tree 5 nodes"));
        assert!(debug.contains("document_surface: document 'The Quiet Machine', 3 paragraphs"));
        assert!(debug.contains("region library (role navigation)"));
    }

    #[test]
    fn m7_pressure_and_canonical_commands_resolve_with_stable_actions() {
        let pressure = parse_and_resolve(include_str!(
            "../../../specimens/reader-workspace-visual-m7.toml"
        ))
        .unwrap();
        assert_eq!(
            pressure
                .elements
                .iter()
                .find(|element| element.id == "play_pause")
                .unwrap()
                .action
                .as_ref()
                .unwrap()
                .as_str(),
            "reader.play_pause"
        );
        assert_eq!(
            pressure.command_state("reading", "play_pause").enabled,
            true
        );
        assert_eq!(
            pressure.command_state("empty", "play_pause"),
            ResolvedCommandState {
                enabled: false,
                reason: Some("Open a document first")
            }
        );
        assert!(pressure
            .semantic_tree()
            .contains("action reader.play_pause"));
        assert!(pressure.semantic_tree().contains("enabled=false"));

        for source in [
            include_str!("../../../specimens/reader-workspace.toml"),
            include_str!("../../../specimens/reader-workspace-visual.toml"),
            include_str!("../../../specimens/dependency-workbench.toml"),
        ] {
            let blueprint = parse_and_resolve(source).unwrap();
            assert!(blueprint
                .elements
                .iter()
                .filter(|element| element.kind == ElementKind::Command)
                .all(|element| element.action.is_some()));
        }
        let duplicate_actions = include_str!("../../../specimens/reader-workspace.toml")
            .replace(
                "[[element]]\nid = \"reading_status\"",
                "[[element]]\nid = \"speed_duplicate\"\nregion = \"transport\"\nkind = \"command\"\nimportance = \"secondary\"\nlabel = \"Speed duplicate\"\naction = \"reader.choose_speed\"\n[[element]]\nid = \"reading_status\"",
            );
        assert!(parse_and_resolve(&duplicate_actions).is_ok());
    }

    #[test]
    fn action_identifier_and_command_fixture_validation_are_strict() {
        let base = "[screen]\nid='x'\npurpose='x'\nroot='root'\n[[region]]\nid='r'\nrole='commands'\nimportance='primary'\n[[region]]\nid='other'\nrole='primary_content'\nimportance='secondary'\n[[element]]\nid='command'\nregion='r'\nkind='command'\nimportance='primary'\nlabel='Run'\naction='run.now'\n[[element]]\nid='text'\nregion='r'\nkind='text'\nimportance='secondary'\nlabel='Text'\n[[composition]]\nid='root'\nkind='split'\nchildren=['r','other']\n[[fixture]]\nid='f'\nstate='x'\n";
        let valid = base;
        let malformed_base = valid.replace("\naction='run.now'", "");
        assert!(
            parse_and_resolve(&valid).is_ok(),
            "valid action fixture failed: {:?}",
            parse_and_resolve(&valid).unwrap_err()
        );
        for action in ["run", "", ".foo", "foo.", "foo..bar", "foo bar", "foo/$bar"] {
            let source =
                malformed_base.replace("label='Run'", &format!("label='Run'\naction='{action}'"));
            assert!(
                parse_and_resolve(&source).is_err(),
                "accepted invalid action {action:?}"
            );
        }
        assert!(parse_and_resolve(&malformed_base)
            .unwrap_err()
            .to_string()
            .contains("require an action"));
        let non_command = base.replace("kind='text'", "kind='text'\naction='text.inspect'");
        assert!(parse_and_resolve(&non_command)
            .unwrap_err()
            .to_string()
            .contains("only valid for command"));
        let disabled = valid.replace(
            "[[fixture]]\nid='f'\nstate='x'\n",
            "[[fixture]]\nid='f'\nstate='x'\n[[fixture.content]]\nelement='command'\ncommand={enabled=false,reason='Not ready'}\n",
        );
        let resolved = parse_and_resolve(&disabled).unwrap();
        assert!(!resolved.command_state("f", "command").enabled);
        assert_eq!(
            resolved.command_state("f", "command").reason,
            Some("Not ready")
        );
        let wrong_element = valid.replace(
            "[[fixture]]\nid='f'\nstate='x'\n",
            "[[fixture]]\nid='f'\nstate='x'\n[[fixture.content]]\nelement='text'\ncommand={enabled=true}\n",
        );
        assert!(parse_and_resolve(&wrong_element)
            .unwrap_err()
            .to_string()
            .contains("requires a command element"));
        let mixed = valid.replace(
            "[[fixture]]\nid='f'\nstate='x'\n",
            "[[fixture]]\nid='f'\nstate='x'\n[[fixture.content]]\nelement='command'\ntext='bad'\ncommand={enabled=true}\n",
        );
        assert!(parse_and_resolve(&mixed)
            .unwrap_err()
            .to_string()
            .contains("exactly one payload"));
    }

    #[test]
    fn m5_pressure_specimens_parse_and_resolve() {
        for source in [
            include_str!("../../../specimens/reader-workspace-m5.toml"),
            include_str!("../../../specimens/reader-workspace-visual-m5.toml"),
        ] {
            let blueprint = parse_and_resolve(source).unwrap();
            assert!(blueprint.fixtures.iter().any(|fixture| fixture
                .content
                .iter()
                .any(|content| matches!(content, ResolvedFixtureContent::Tree { .. }))));
            assert!(blueprint.fixtures.iter().any(|fixture| fixture
                .content
                .iter()
                .any(|content| matches!(content, ResolvedFixtureContent::Document { .. }))));
        }
    }

    #[test]
    fn tree_and_document_fixture_validation_is_actionable() {
        let base = "[screen]\nid='x'\npurpose='x'\nroot='root'\n[[region]]\nid='r'\nrole='primary_content'\nimportance='primary'\n[[composition]]\nid='root'\nkind='split'\nchildren=['r','r']\n[[element]]\nid='tree'\nregion='r'\nkind='tree'\nimportance='primary'\n[[element]]\nid='document'\nregion='r'\nkind='document'\nimportance='primary'\n[[fixture]]\nid='f'\nstate='x'\n";
        let check = |suffix: &str, expected: &str| {
            assert!(parse_and_resolve(&(base.to_owned() + suffix))
                .unwrap_err()
                .to_string()
                .contains(expected));
        };
        check(
            "[[fixture.content]]\nelement='tree'\nnodes=[{id='a',label='A'},{id='a',label='A2'}]",
            "duplicate tree node id",
        );
        check(
            "[[fixture.content]]\nelement='tree'\nnodes=[{id='a',label='A',parent='missing'}]",
            "missing parent",
        );
        check(
            "[[fixture.content]]\nelement='tree'\nnodes=[{id='a',label='A',parent='a'}]",
            "cannot parent itself",
        );
        check(
            "[[fixture.content]]\nelement='tree'\nnodes=[{id='a',label='A',parent='b'},{id='b',label='B',parent='a'}]",
            "parent cycle",
        );
        check(
            "[[fixture.content]]\nelement='tree'\nnodes=[]\nselected='missing'",
            "selected tree node",
        );
        check(
            "[[fixture.content]]\nelement='tree'\ndocument={title='x',paragraphs=[]}",
            "document content",
        );
        check(
            "[[fixture.content]]\nelement='document'\nnodes=[]",
            "tree content",
        );
        check(
            "[[fixture.content]]\nelement='tree'\nnodes=[]\ntext='x'",
            "exactly one payload",
        );
        check(
            "[[fixture.content]]\nelement='document'\ndocument={title='x',paragraphs=[]}\nwat='x'",
            "unknown field",
        );
        check(
            "[[fixture.content]]\nelement='tree'\nnodes=[{id='a',label='A',wat='x'}]",
            "unknown field",
        );
    }

    #[test]
    fn invalid_growth_is_rejected_for_regions_and_compositions() {
        let source = "[screen]\nid='x'\npurpose='x'\nroot='root'\n[[region]]\nid='a'\nrole='navigation'\nimportance='primary'\ngrow=-1\n[[region]]\nid='b'\nrole='inspector'\nimportance='secondary'\n[[composition]]\nid='root'\nkind='split'\nchildren=['a','b']\ngrow=-2";
        let error = parse_and_resolve(source).unwrap_err().to_string();
        assert!(error.contains("region 'a': grow must be finite and non-negative"));
        assert!(error.contains("composition 'root': grow must be finite and non-negative"));
    }

    #[test]
    fn non_finite_growth_is_rejected_for_regions_and_compositions() {
        let source = "[screen]\nid='x'\npurpose='x'\nroot='root'\n[[region]]\nid='a'\nrole='navigation'\nimportance='primary'\n[[region]]\nid='b'\nrole='inspector'\nimportance='secondary'\n[[composition]]\nid='root'\nkind='split'\nchildren=['a','b']";
        let mut source: SourceBlueprint = toml::from_str(source).unwrap();
        source.region[0].grow = Some(f32::NAN);
        source.composition[0].grow = Some(f32::INFINITY);
        let error = resolve(source).unwrap_err().to_string();
        assert!(error.contains("region 'a': grow must be finite and non-negative"));
        assert!(error.contains("composition 'root': grow must be finite and non-negative"));
    }

    #[test]
    fn zero_and_positive_composition_growth_resolve() {
        let source = "[screen]\nid='x'\npurpose='x'\nroot='root'\n[[region]]\nid='a'\nrole='navigation'\nimportance='primary'\n[[region]]\nid='b'\nrole='inspector'\nimportance='secondary'\n[[composition]]\nid='root'\nkind='split'\nchildren=['a','b']\ngrow=0";
        let b = parse_and_resolve(source).unwrap();
        assert_eq!(b.compositions[0].grow, 0.0);
        let source = source.replace("grow=0", "grow=2.5");
        assert_eq!(
            parse_and_resolve(&source).unwrap().compositions[0].grow,
            2.5
        );
    }
}
