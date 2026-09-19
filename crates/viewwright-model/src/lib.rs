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
    pub responsive: Option<ResponsiveSource>,
    #[serde(default)]
    pub region: Vec<RegionSource>,
    #[serde(default)]
    pub composition: Vec<CompositionSource>,
    #[serde(default)]
    pub furnishing: Vec<FurnishingSource>,
    #[serde(default)]
    pub element: Vec<ElementSource>,
    #[serde(default)]
    pub fixture: Vec<FixtureSource>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ResponsiveSource {
    pub default: String,
    pub variant: Vec<ResponsiveVariantSource>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ResponsiveVariantSource {
    pub id: String,
    pub root: String,
    pub min_width: Option<u32>,
    pub max_width: Option<u32>,
    #[serde(default)]
    pub region: Vec<ResponsiveRegionOverrideSource>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ResponsiveRegionOverrideSource {
    pub id: String,
    pub width: Option<String>,
    pub height: Option<String>,
    pub grow: Option<f32>,
    pub furnishing: Option<String>,
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
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ResolvedDesign {
    pub character: Vec<String>,
    pub dominant: Option<DominantTarget>,
    pub avoid: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DominantTarget {
    Region(String),
    Element(String),
}

impl DominantTarget {
    pub fn id(&self) -> &str {
        match self {
            Self::Region(id) | Self::Element(id) => id,
        }
    }
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
    pub overflow: Option<String>,
    pub furnishing: Option<String>,
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
pub struct FurnishingSource {
    pub id: String,
    pub kind: String,
    pub children: Vec<String>,
    pub gap: Option<String>,
    pub padding: Option<String>,
    pub width: Option<String>,
    pub height: Option<String>,
    pub grow: Option<f32>,
    pub overflow: Option<String>,
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
    pub choice: Option<ChoiceSource>,
    pub scalar: Option<ScalarSource>,
}
#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ChoiceSource {
    pub presentation: String,
    pub options: Vec<ChoiceOptionSource>,
}
#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ChoiceOptionSource {
    pub id: String,
    pub label: String,
}
#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ScalarSource {
    pub min: f32,
    pub max: f32,
    pub step: f32,
    pub unit: Option<String>,
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
    pub choice: Option<ChoiceStateSource>,
    pub boolean: Option<BooleanStateSource>,
    pub scalar: Option<ScalarStateSource>,
}
#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ChoiceStateSource {
    pub selected: String,
}
#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct BooleanStateSource {
    pub value: bool,
}
#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ScalarStateSource {
    pub value: f32,
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
#[serde(untagged)]
pub enum DocumentSource {
    Legacy(LegacyDocumentSource),
    Rich(RichDocumentSource),
}
#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct LegacyDocumentSource {
    pub title: String,
    pub paragraphs: Vec<String>,
}
#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RichDocumentSource {
    pub blocks: Vec<DocumentBlockSource>,
    pub spoken: Option<SpokenRangeSource>,
}
#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct DocumentBlockSource {
    pub id: String,
    pub kind: String,
    pub text: Option<String>,
    pub level: Option<i64>,
}
#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SpokenRangeSource {
    pub block: String,
    pub start: usize,
    pub end: usize,
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
pub enum OverflowPolicy {
    Clip,
    ScrollY,
}

impl OverflowPolicy {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Clip => "clip",
            Self::ScrollY => "scroll_y",
        }
    }
}

impl std::fmt::Display for OverflowPolicy {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
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

fn overflow(value: Option<&str>, id: &str, errors: &mut Vec<String>) -> OverflowPolicy {
    match value.unwrap_or("clip") {
        "clip" => OverflowPolicy::Clip,
        "scroll_y" => OverflowPolicy::ScrollY,
        other => {
            errors.push(format!(
                "region '{id}': unknown overflow '{other}' (expected clip or scroll_y)"
            ));
            OverflowPolicy::Clip
        }
    }
}

fn furnishing_overflow(value: Option<&str>, id: &str, errors: &mut Vec<String>) -> OverflowPolicy {
    match value.unwrap_or("clip") {
        "clip" => OverflowPolicy::Clip,
        "scroll_y" => OverflowPolicy::ScrollY,
        other => {
            errors.push(format!(
                "furnishing '{id}': unknown overflow '{other}' (expected clip or scroll_y)"
            ));
            OverflowPolicy::Clip
        }
    }
}

fn composition_kind(value: &str, id: &str, errors: &mut Vec<String>) -> CompositionKind {
    match value {
        "split" => CompositionKind::Split,
        "row" => CompositionKind::Row,
        "column" => CompositionKind::Column,
        "overlay" => CompositionKind::Overlay,
        other => {
            errors.push(format!(
                "composition '{id}': unknown composition kind '{other}'"
            ));
            CompositionKind::Split
        }
    }
}

fn furnishing_kind(value: &str, id: &str, errors: &mut Vec<String>) -> FurnishingKind {
    match value {
        "row" => FurnishingKind::Row,
        "column" => FurnishingKind::Column,
        other => {
            errors.push(format!(
                "furnishing '{id}': unknown kind '{other}' (expected row or column)"
            ));
            FurnishingKind::Column
        }
    }
}

fn axis(
    value: Option<&str>,
    kind: CompositionKind,
    id: &str,
    errors: &mut Vec<String>,
) -> Option<Axis> {
    if kind == CompositionKind::Overlay {
        if let Some(value) = value {
            errors.push(format!(
                "composition '{id}': overlay does not support authored axis '{value}'"
            ));
        }
        return None;
    }
    let implied = match kind {
        CompositionKind::Column => Axis::Vertical,
        CompositionKind::Split | CompositionKind::Row => Axis::Horizontal,
        CompositionKind::Overlay => unreachable!(),
    };
    let Some(value) = value else {
        return Some(implied);
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
        return Some(implied);
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
    Some(axis)
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
    Choice,
    Boolean,
    Scalar,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CollectionPresentation {
    List,
    AdaptiveCards,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ChoicePresentation {
    Select,
    Segmented,
}
impl ChoicePresentation {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Select => "select",
            Self::Segmented => "segmented",
        }
    }
}
#[derive(Debug, Clone)]
pub struct ResolvedChoiceOption {
    pub id: String,
    pub label: String,
}
#[derive(Debug, Clone)]
pub struct ResolvedChoice {
    pub presentation: ChoicePresentation,
    pub options: Vec<ResolvedChoiceOption>,
}
#[derive(Debug, Clone)]
pub struct ResolvedScalar {
    pub min: f32,
    pub max: f32,
    pub step: f32,
    pub unit: Option<String>,
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
        "choice" => ElementKind::Choice,
        "boolean" => ElementKind::Boolean,
        "scalar" => ElementKind::Scalar,
        _ => {
            errors.push(format!("{where_}: unknown element kind '{s}'"));
            ElementKind::Text
        }
    }
}
fn element_label(value: Option<&String>, id: &str, errors: &mut Vec<String>) -> String {
    match value {
        Some(label) if !label.trim().is_empty() => label.clone(),
        Some(_) => {
            errors.push(format!(
                "element '{id}': label must contain at least one non-whitespace character"
            ));
            String::new()
        }
        None => {
            errors.push(format!("element '{id}': label is required"));
            String::new()
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
fn choice_config(
    source: Option<&ChoiceSource>,
    element: &str,
    element_kind: ElementKind,
    errors: &mut Vec<String>,
) -> Option<ResolvedChoice> {
    if element_kind != ElementKind::Choice {
        if source.is_some() {
            errors.push(format!(
                "element '{element}': choice configuration is only valid for choice elements"
            ));
        }
        return None;
    }
    let Some(source) = source else {
        errors.push(format!(
            "element '{element}': choice elements require choice configuration"
        ));
        return None;
    };
    let presentation = match source.presentation.as_str() {
        "select" => Some(ChoicePresentation::Select),
        "segmented" => Some(ChoicePresentation::Segmented),
        other => {
            errors.push(format!(
                "element '{element}': unknown choice presentation '{other}'"
            ));
            None
        }
    };
    if source.options.len() < 2 {
        errors.push(format!(
            "element '{element}': choice requires at least two options"
        ));
    }
    let mut ids = HashSet::new();
    let options = source
        .options
        .iter()
        .filter_map(|option| {
            if option.id.trim().is_empty() {
                errors.push(format!(
                    "element '{element}': choice option id must be nonblank"
                ));
                return None;
            }
            if option.label.trim().is_empty() {
                errors.push(format!(
                    "element '{element}': choice option '{}' label must be nonblank",
                    option.id
                ));
            }
            if !ids.insert(option.id.clone()) {
                errors.push(format!(
                    "element '{element}': duplicate choice option id '{}'",
                    option.id
                ));
                return None;
            }
            Some(ResolvedChoiceOption {
                id: option.id.clone(),
                label: option.label.clone(),
            })
        })
        .collect();
    presentation.map(|presentation| ResolvedChoice {
        presentation,
        options,
    })
}

fn scalar_config(
    source: Option<&ScalarSource>,
    element: &str,
    element_kind: ElementKind,
    errors: &mut Vec<String>,
) -> Option<ResolvedScalar> {
    if element_kind != ElementKind::Scalar {
        if source.is_some() {
            errors.push(format!(
                "element '{element}': scalar configuration is only valid for scalar elements"
            ));
        }
        return None;
    }
    let Some(source) = source else {
        errors.push(format!(
            "element '{element}': scalar elements require scalar configuration"
        ));
        return None;
    };
    if !source.min.is_finite() {
        errors.push(format!("element '{element}': scalar min must be finite"));
    }
    if !source.max.is_finite() {
        errors.push(format!("element '{element}': scalar max must be finite"));
    }
    if source.min.is_finite() && source.max.is_finite() && source.min >= source.max {
        errors.push(format!(
            "element '{element}': scalar min must be less than max"
        ));
    }
    if !source.step.is_finite() || source.step <= 0.0 {
        errors.push(format!(
            "element '{element}': scalar step must be finite and positive"
        ));
    }
    if source
        .unit
        .as_ref()
        .is_some_and(|unit| unit.trim().is_empty())
    {
        errors.push(format!(
            "element '{element}': scalar unit must be nonblank when present"
        ));
    }
    Some(ResolvedScalar {
        min: source.min,
        max: source.max,
        step: source.step,
        unit: source.unit.clone(),
    })
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
    pub overflow: OverflowPolicy,
    pub furnishing: Option<String>,
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
    pub choice: Option<ResolvedChoice>,
    pub scalar: Option<ResolvedScalar>,
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
    pub axis: Option<Axis>,
    pub children: Vec<CompositionChild>,
    pub gap: u32,
    pub padding: u32,
    pub grow: f32,
}

#[derive(Debug, Clone)]
pub struct ResolvedFurnishing {
    pub id: String,
    pub kind: FurnishingKind,
    pub children: Vec<FurnishingChild>,
    pub gap: u32,
    pub padding: u32,
    pub width: Option<u32>,
    pub height: Option<u32>,
    pub grow: f32,
    pub overflow: OverflowPolicy,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FurnishingKind {
    Row,
    Column,
}

impl FurnishingKind {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Row => "row",
            Self::Column => "column",
        }
    }
}

impl std::fmt::Display for FurnishingKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[derive(Debug, Clone)]
pub enum FurnishingChild {
    Furnishing(String),
    Element(String),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CompositionKind {
    Split,
    Row,
    Column,
    Overlay,
}

impl CompositionKind {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Split => "split",
            Self::Row => "row",
            Self::Column => "column",
            Self::Overlay => "overlay",
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
        document: ResolvedDocument,
    },
    Command {
        element: String,
        enabled: bool,
        reason: Option<String>,
    },
    Choice {
        element: String,
        selected: String,
    },
    Boolean {
        element: String,
        value: bool,
    },
    Scalar {
        element: String,
        value: f32,
    },
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ResolvedDocument {
    Legacy {
        title: String,
        paragraphs: Vec<String>,
    },
    Rich {
        blocks: Vec<ResolvedDocumentBlock>,
        spoken: Option<ResolvedSpokenRange>,
    },
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DocumentBlockKind {
    Eyebrow,
    Heading,
    Paragraph,
    Quote,
    Divider,
}
impl DocumentBlockKind {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Eyebrow => "eyebrow",
            Self::Heading => "heading",
            Self::Paragraph => "paragraph",
            Self::Quote => "quote",
            Self::Divider => "divider",
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ResolvedDocumentBlock {
    Eyebrow { id: String, text: String },
    Heading { id: String, level: u8, text: String },
    Paragraph { id: String, text: String },
    Quote { id: String, text: String },
    Divider { id: String },
}
impl ResolvedDocumentBlock {
    pub fn id(&self) -> &str {
        match self {
            Self::Eyebrow { id, .. }
            | Self::Heading { id, .. }
            | Self::Paragraph { id, .. }
            | Self::Quote { id, .. }
            | Self::Divider { id } => id,
        }
    }

    pub fn kind(&self) -> DocumentBlockKind {
        match self {
            Self::Eyebrow { .. } => DocumentBlockKind::Eyebrow,
            Self::Heading { .. } => DocumentBlockKind::Heading,
            Self::Paragraph { .. } => DocumentBlockKind::Paragraph,
            Self::Quote { .. } => DocumentBlockKind::Quote,
            Self::Divider { .. } => DocumentBlockKind::Divider,
        }
    }

    pub fn text(&self) -> Option<&str> {
        match self {
            Self::Eyebrow { text, .. }
            | Self::Heading { text, .. }
            | Self::Paragraph { text, .. }
            | Self::Quote { text, .. } => Some(text),
            Self::Divider { .. } => None,
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ResolvedSpokenRange {
    pub block: String,
    pub start: usize,
    pub end: usize,
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
    pub design: ResolvedDesign,
    pub root: String,
    pub regions: Vec<ResolvedRegion>,
    pub compositions: Vec<ResolvedComposition>,
    pub furnishings: Vec<ResolvedFurnishing>,
    pub elements: Vec<ResolvedElement>,
    pub fixtures: Vec<ResolvedFixture>,
    pub visual: Option<ResolvedVisual>,
    pub responsive: Option<ResolvedResponsive>,
}

#[derive(Debug, Clone)]
pub struct ResolvedResponsive {
    pub default: String,
    pub variants: Vec<ResolvedResponsiveVariant>,
}

#[derive(Debug, Clone)]
pub struct ResolvedResponsiveVariant {
    pub id: String,
    pub min_width: u32,
    pub max_width: Option<u32>,
    pub root: String,
    pub regions: HashMap<String, ResolvedRegionOverride>,
}

#[derive(Debug, Clone)]
pub struct ResolvedRegionOverride {
    pub width: Option<u32>,
    pub height: Option<u32>,
    pub grow: Option<f32>,
    pub furnishing: Option<String>,
}

impl ResolvedResponsive {
    pub fn select(&self, width: f32) -> Option<&ResolvedResponsiveVariant> {
        self.variants.iter().find(|variant| {
            width >= variant.min_width as f32
                && variant.max_width.is_none_or(|max| width < max as f32)
        })
    }
}

impl ResolvedBlueprint {
    pub fn responsive_variant(&self, width: f32) -> Option<&ResolvedResponsiveVariant> {
        self.responsive.as_ref()?.select(width)
    }

    pub fn active_root(&self, width: f32) -> &str {
        self.responsive_variant(width)
            .map(|variant| variant.root.as_str())
            .unwrap_or(&self.root)
    }

    pub fn default_variant(&self) -> Option<&ResolvedResponsiveVariant> {
        let responsive = self.responsive.as_ref()?;
        responsive
            .variants
            .iter()
            .find(|variant| variant.id == responsive.default)
    }
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
fn validate_nonblank_id(id: &str, context: &str, errors: &mut Vec<String>) {
    if id.trim().is_empty() {
        errors.push(format!(
            "{context}: must contain at least one non-whitespace character"
        ));
    }
}

fn resolve_document_source(
    fixture_id: &str,
    element_id: &str,
    source: &DocumentSource,
    errors: &mut Vec<String>,
) -> ResolvedDocument {
    match source {
        DocumentSource::Legacy(document) => {
            if document.title.trim().is_empty() {
                errors.push(format!(
                    "fixture '{fixture_id}': document for '{element_id}' title must contain at least one non-whitespace character"
                ));
            }
            ResolvedDocument::Legacy {
                title: document.title.clone(),
                paragraphs: document.paragraphs.clone(),
            }
        }
        DocumentSource::Rich(document) => {
            if document.blocks.is_empty() {
                errors.push(format!(
                    "fixture '{fixture_id}': rich document for '{element_id}' must contain at least one block"
                ));
            }

            let mut seen_ids = HashSet::new();
            let mut blocks = Vec::with_capacity(document.blocks.len());
            for block in &document.blocks {
                let context = format!("fixture '{fixture_id}': document block id '{}'", block.id);
                validate_nonblank_id(&block.id, &context, errors);
                if !seen_ids.insert(block.id.as_str()) {
                    errors.push(format!(
                        "fixture '{fixture_id}': duplicate document block id '{}' for '{element_id}'",
                        block.id
                    ));
                }

                let resolved = match block.kind.as_str() {
                    "eyebrow" => {
                        if block.level.is_some() {
                            errors.push(format!(
                                "fixture '{fixture_id}': eyebrow block '{}' for '{element_id}' must not have a level",
                                block.id
                            ));
                        }
                        resolve_document_block_text(fixture_id, element_id, block, errors).map(
                            |text| ResolvedDocumentBlock::Eyebrow {
                                id: block.id.clone(),
                                text,
                            },
                        )
                    }
                    "heading" => {
                        let level = match block.level {
                            Some(1..=6) => block.level.map(|level| level as u8),
                            Some(level) => {
                                errors.push(format!(
                                    "fixture '{fixture_id}': heading block '{}' for '{element_id}' level must be between 1 and 6, got {level}",
                                    block.id
                                ));
                                None
                            }
                            None => {
                                errors.push(format!(
                                    "fixture '{fixture_id}': heading block '{}' for '{element_id}' requires a level from 1 through 6",
                                    block.id
                                ));
                                None
                            }
                        };
                        let text =
                            resolve_document_block_text(fixture_id, element_id, block, errors);
                        match (level, text) {
                            (Some(level), Some(text)) => Some(ResolvedDocumentBlock::Heading {
                                id: block.id.clone(),
                                level,
                                text,
                            }),
                            _ => None,
                        }
                    }
                    "paragraph" => {
                        if block.level.is_some() {
                            errors.push(format!(
                                "fixture '{fixture_id}': paragraph block '{}' for '{element_id}' must not have a level",
                                block.id
                            ));
                        }
                        resolve_document_block_text(fixture_id, element_id, block, errors).map(
                            |text| ResolvedDocumentBlock::Paragraph {
                                id: block.id.clone(),
                                text,
                            },
                        )
                    }
                    "quote" => {
                        if block.level.is_some() {
                            errors.push(format!(
                                "fixture '{fixture_id}': quote block '{}' for '{element_id}' must not have a level",
                                block.id
                            ));
                        }
                        resolve_document_block_text(fixture_id, element_id, block, errors).map(
                            |text| ResolvedDocumentBlock::Quote {
                                id: block.id.clone(),
                                text,
                            },
                        )
                    }
                    "divider" => {
                        if block.text.is_some() {
                            errors.push(format!(
                                "fixture '{fixture_id}': divider block '{}' for '{element_id}' must not have text",
                                block.id
                            ));
                        }
                        if block.level.is_some() {
                            errors.push(format!(
                                "fixture '{fixture_id}': divider block '{}' for '{element_id}' must not have a level",
                                block.id
                            ));
                        }
                        Some(ResolvedDocumentBlock::Divider {
                            id: block.id.clone(),
                        })
                    }
                    kind => {
                        errors.push(format!(
                            "fixture '{fixture_id}': document block '{}' for '{element_id}' has unsupported kind '{kind}'",
                            block.id
                        ));
                        None
                    }
                };
                if let Some(block) = resolved {
                    blocks.push(block);
                }
            }

            let spoken = document.spoken.as_ref().and_then(|spoken| {
                let block = document.blocks.iter().find(|block| block.id == spoken.block);
                let Some(block) = block else {
                    errors.push(format!(
                        "fixture '{fixture_id}': spoken range for '{element_id}' references missing document block '{}'",
                        spoken.block
                    ));
                    return None;
                };
                if spoken.start >= spoken.end {
                    errors.push(format!(
                        "fixture '{fixture_id}': spoken range for block '{}' must satisfy start < end",
                        spoken.block
                    ));
                }
                let textual = matches!(
                    block.kind.as_str(),
                    "eyebrow" | "heading" | "paragraph" | "quote"
                );
                if !textual {
                    errors.push(format!(
                        "fixture '{fixture_id}': spoken range for '{element_id}' must target a textual document block, not '{}'",
                        block.kind
                    ));
                    return None;
                }
                if let Some(text) = &block.text {
                    let scalar_len = text.chars().count();
                    if spoken.end > scalar_len {
                        errors.push(format!(
                            "fixture '{fixture_id}': spoken range {}..{} exceeds document block '{}' Unicode scalar length {scalar_len}",
                            spoken.start, spoken.end, spoken.block
                        ));
                    }
                }
                Some(ResolvedSpokenRange {
                    block: spoken.block.clone(),
                    start: spoken.start,
                    end: spoken.end,
                })
            });

            ResolvedDocument::Rich { blocks, spoken }
        }
    }
}

fn resolve_document_block_text(
    fixture_id: &str,
    element_id: &str,
    block: &DocumentBlockSource,
    errors: &mut Vec<String>,
) -> Option<String> {
    let Some(text) = block.text.as_deref() else {
        errors.push(format!(
            "fixture '{fixture_id}': {} block '{}' for '{element_id}' requires text",
            block.kind, block.id
        ));
        return None;
    };
    if text.trim().is_empty() {
        errors.push(format!(
            "fixture '{fixture_id}': {} block '{}' for '{element_id}' text must contain at least one non-whitespace character",
            block.kind, block.id
        ));
        return None;
    }
    Some(text.to_owned())
}

fn add_id(ids: &mut HashSet<String>, errors: &mut Vec<String>, id: &str, kind: &str) {
    validate_nonblank_id(id, &format!("{kind}.id"), errors);
    if !ids.insert(id.to_owned()) {
        errors.push(format!("duplicate id '{id}' ({kind})"));
    }
}
fn validate_screen_author_id_collision(
    screen_id: &str,
    declaration_id: &str,
    kind: &str,
    errors: &mut Vec<String>,
) {
    if screen_id == declaration_id {
        errors.push(format!(
            "screen.id '{screen_id}' conflicts with {kind} id '{declaration_id}' in observable author-id namespace"
        ));
    }
}
fn logical_size(
    value: Option<&String>,
    label: &str,
    id: &str,
    errors: &mut Vec<String>,
) -> Option<u32> {
    let value = value?;
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

#[allow(clippy::too_many_arguments)]
fn resolve_furnishings(
    source_regions: &[RegionSource],
    source_furnishings: &[FurnishingSource],
    regions: &mut [ResolvedRegion],
    elements: &[ResolvedElement],
    ids: &mut HashSet<String>,
    spacing: &HashMap<String, u32>,
    responsive: bool,
    errors: &mut Vec<String>,
) -> Vec<ResolvedFurnishing> {
    let furnishing_ids: HashSet<_> = source_furnishings
        .iter()
        .filter(|furnishing| !furnishing.id.trim().is_empty())
        .map(|furnishing| furnishing.id.as_str())
        .collect();
    let element_ids: HashSet<_> = elements
        .iter()
        .filter(|element| !element.id.trim().is_empty())
        .map(|element| element.id.as_str())
        .collect();
    let element_by_id: HashMap<_, _> = elements
        .iter()
        .map(|element| (element.id.as_str(), element))
        .collect();

    let mut furnished_roots: HashMap<String, Vec<String>> = HashMap::new();
    for (source, region) in source_regions.iter().zip(regions.iter_mut()) {
        region.furnishing = source.furnishing.clone();
        if let Some(root) = &source.furnishing {
            if !furnishing_ids.contains(root.as_str()) {
                errors.push(format!(
                    "region '{}': missing furnishing root '{}'",
                    region.id, root
                ));
            } else {
                furnished_roots
                    .entry(root.clone())
                    .or_default()
                    .push(region.id.clone());
            }
        }
    }

    let mut resolved = Vec::with_capacity(source_furnishings.len());
    for source in source_furnishings {
        add_id(ids, errors, &source.id, "furnishing");
        let kind = furnishing_kind(&source.kind, &source.id, errors);
        let mut children = Vec::with_capacity(source.children.len());
        let mut seen_children = HashSet::new();
        let mut child_types = HashSet::new();
        for child in &source.children {
            if !seen_children.insert(child.as_str()) {
                errors.push(format!(
                    "furnishing '{}': child '{}' is duplicated within its children",
                    source.id, child
                ));
                continue;
            }
            if furnishing_ids.contains(child.as_str()) {
                child_types.insert("furnishing");
                children.push(FurnishingChild::Furnishing(child.clone()));
            } else if element_ids.contains(child.as_str()) {
                child_types.insert("element");
                children.push(FurnishingChild::Element(child.clone()));
            } else {
                errors.push(format!(
                    "furnishing '{}': missing child reference '{}'",
                    source.id, child
                ));
            }
        }
        if child_types.len() > 1 {
            errors.push(format!(
                "furnishing '{}': direct children must be either all furnishings or all elements",
                source.id
            ));
        }

        let gap = source
            .gap
            .as_ref()
            .and_then(|token| spacing.get(token))
            .copied()
            .unwrap_or(0);
        if let Some(token) = &source.gap {
            if !spacing.contains_key(token) {
                errors.push(format!(
                    "furnishing '{}': missing spacing token '{}' for gap",
                    source.id, token
                ));
            }
        }
        let padding = source
            .padding
            .as_ref()
            .and_then(|token| spacing.get(token))
            .copied()
            .unwrap_or(0);
        if let Some(token) = &source.padding {
            if !spacing.contains_key(token) {
                errors.push(format!(
                    "furnishing '{}': missing spacing token '{}' for padding",
                    source.id, token
                ));
            }
        }
        let width = logical_size(
            source.width.as_ref(),
            "furnishing width",
            &source.id,
            errors,
        );
        let height = logical_size(
            source.height.as_ref(),
            "furnishing height",
            &source.id,
            errors,
        );
        let grow = source.grow.unwrap_or(0.0);
        if !grow.is_finite() || grow < 0.0 {
            errors.push(format!(
                "furnishing '{}': grow must be finite and non-negative",
                source.id
            ));
        }
        if furnished_roots.contains_key(&source.id)
            && (source.width.is_some() || source.height.is_some() || source.grow.is_some())
        {
            errors.push(format!(
                "furnishing '{}': a region furnishing root must not author width, height, or grow",
                source.id
            ));
        }
        resolved.push(ResolvedFurnishing {
            id: source.id.clone(),
            kind,
            children,
            gap,
            padding,
            width,
            height,
            grow: if grow.is_finite() && grow >= 0.0 {
                grow
            } else {
                0.0
            },
            overflow: furnishing_overflow(source.overflow.as_deref(), &source.id, errors),
        });
    }

    let by_id: HashMap<_, _> = resolved
        .iter()
        .map(|furnishing| (furnishing.id.as_str(), furnishing))
        .collect();
    let source_by_id: HashMap<_, _> = source_furnishings
        .iter()
        .map(|furnishing| (furnishing.id.as_str(), furnishing))
        .collect();
    let mut furnishing_parents: HashMap<&str, Vec<&str>> = HashMap::new();
    let mut element_parents: HashMap<&str, Vec<&str>> = HashMap::new();
    for parent in &resolved {
        for child in &parent.children {
            match child {
                FurnishingChild::Furnishing(id) => furnishing_parents
                    .entry(id.as_str())
                    .or_default()
                    .push(parent.id.as_str()),
                FurnishingChild::Element(id) => element_parents
                    .entry(id.as_str())
                    .or_default()
                    .push(parent.id.as_str()),
            }
        }
    }
    for (child, parents) in &furnishing_parents {
        if !responsive && parents.len() > 1 {
            errors.push(format!(
                "furnishing '{child}' has multiple furnishing parents: {}",
                parents.join(" and ")
            ));
        }
        if !responsive && furnished_roots.contains_key(*child) {
            errors.push(format!(
                "furnishing root '{child}' cannot have a furnishing parent"
            ));
        }
    }
    for (child, parents) in &element_parents {
        if !responsive && parents.len() > 1 {
            errors.push(format!(
                "element '{child}' has multiple furnishing parents: {}",
                parents.join(" and ")
            ));
        }
    }
    for (root, owners) in &furnished_roots {
        if !responsive && owners.len() > 1 {
            errors.push(format!(
                "furnishing root '{root}' is assigned to multiple regions: {}",
                owners.join(" and ")
            ));
        }
    }

    for parent in &resolved {
        for child in &parent.children {
            let FurnishingChild::Furnishing(child_id) = child else {
                continue;
            };
            let Some(child_furnishing) = by_id.get(child_id.as_str()) else {
                continue;
            };
            let Some(source_child) = source_by_id.get(child_id.as_str()) else {
                continue;
            };
            let has_main_axis_slot = match parent.kind {
                FurnishingKind::Row => {
                    child_furnishing.width.is_some() || child_furnishing.grow > 0.0
                }
                FurnishingKind::Column => {
                    child_furnishing.height.is_some() || child_furnishing.grow > 0.0
                }
            };
            if !has_main_axis_slot {
                let axis = match parent.kind {
                    FurnishingKind::Row => "width",
                    FurnishingKind::Column => "height",
                };
                errors.push(format!(
                    "furnishing '{}' child '{}': requires {axis} or positive grow for deterministic allocation",
                    parent.id, source_child.id
                ));
            }
        }
    }

    let mut visiting = HashSet::new();
    let mut visited = HashSet::new();
    fn visit(
        id: &str,
        furnishings: &HashMap<&str, &ResolvedFurnishing>,
        visiting: &mut HashSet<String>,
        visited: &mut HashSet<String>,
        errors: &mut Vec<String>,
    ) {
        if visiting.contains(id) {
            errors.push(format!("furnishing cycle detected at '{id}'"));
            return;
        }
        if !visited.insert(id.to_owned()) {
            return;
        }
        visiting.insert(id.to_owned());
        if let Some(furnishing) = furnishings.get(id) {
            for child in &furnishing.children {
                if let FurnishingChild::Furnishing(child) = child {
                    visit(child, furnishings, visiting, visited, errors);
                }
            }
        }
        visiting.remove(id);
    }
    for furnishing in &resolved {
        if !visited.contains(&furnishing.id) {
            visit(&furnishing.id, &by_id, &mut visiting, &mut visited, errors);
        }
    }

    #[allow(clippy::too_many_arguments)]
    fn collect_reachable(
        id: &str,
        owner_region: &str,
        furnishings: &HashMap<&str, &ResolvedFurnishing>,
        elements: &HashMap<&str, &ResolvedElement>,
        stack: &mut HashSet<String>,
        furnishing_counts: &mut HashMap<String, usize>,
        element_counts: &mut HashMap<(String, String), usize>,
        errors: &mut Vec<String>,
    ) {
        if !stack.insert(id.to_owned()) {
            return;
        }
        *furnishing_counts.entry(id.to_owned()).or_default() += 1;
        if let Some(furnishing) = furnishings.get(id) {
            for child in &furnishing.children {
                match child {
                    FurnishingChild::Furnishing(child) => collect_reachable(
                        child,
                        owner_region,
                        furnishings,
                        elements,
                        stack,
                        furnishing_counts,
                        element_counts,
                        errors,
                    ),
                    FurnishingChild::Element(child) => {
                        if let Some(element) = elements.get(child.as_str()) {
                            if element.region != owner_region {
                                errors.push(format!(
                                    "furnishing '{}' in region '{}' contains element '{}' owned by region '{}'",
                                    furnishing.id, owner_region, child, element.region
                                ));
                            }
                            *element_counts
                                .entry((owner_region.to_owned(), child.clone()))
                                .or_default() += 1;
                        }
                    }
                }
            }
        }
        stack.remove(id);
    }

    if responsive {
        return resolved;
    }

    let mut furnishing_counts = HashMap::new();
    let mut element_counts = HashMap::new();
    for (root, owners) in &furnished_roots {
        for owner in owners {
            collect_reachable(
                root,
                owner,
                &by_id,
                &element_by_id,
                &mut HashSet::new(),
                &mut furnishing_counts,
                &mut element_counts,
                errors,
            );
        }
    }
    for furnishing in &resolved {
        let count = furnishing_counts.get(&furnishing.id).copied().unwrap_or(0);
        if count != 1 {
            errors.push(format!(
                "furnishing '{}': must be reachable from exactly one region furnishing root (found {count})",
                furnishing.id
            ));
        }
    }
    for region in regions.iter().filter(|region| region.furnishing.is_some()) {
        for element in elements
            .iter()
            .filter(|element| element.region == region.id)
        {
            let count = element_counts
                .get(&(region.id.clone(), element.id.clone()))
                .copied()
                .unwrap_or(0);
            if count != 1 {
                errors.push(format!(
                    "furnished region '{}': element '{}' must be reachable exactly once from its furnishing root (found {count})",
                    region.id, element.id
                ));
            }
        }
    }

    resolved
}

fn parse_color(value: &str) -> Option<Color> {
    let valid = value
        .strip_prefix('#')
        .filter(|v| v.len() == 6)
        .and_then(|v| u32::from_str_radix(v, 16).ok());
    valid.map(|v| Color {
        r: (v >> 16) as u8,
        g: (v >> 8) as u8,
        b: v as u8,
        a: 255,
    })
}
fn validate_token_names<T>(tokens: &HashMap<String, T>, family: &str, errors: &mut Vec<String>) {
    let mut names: Vec<_> = tokens.keys().map(String::as_str).collect();
    names.sort_unstable();
    for name in names {
        if name.trim().is_empty() {
            errors.push(format!(
                "{family}: token name '{name}' must contain at least one non-whitespace character"
            ));
        }
    }
}
fn validate_color_values(tokens: &ColorTokensSource, errors: &mut Vec<String>) {
    let mut color_names: Vec<_> = tokens.values.keys().collect();
    color_names.sort_unstable();
    for name in color_names {
        if parse_color(&tokens.values[name]).is_none() {
            errors.push(format!("color token '{name}': expected #RRGGBB"));
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
        Some(value) => parse_color(value).unwrap_or(Color {
            r: 0,
            g: 0,
            b: 0,
            a: 255,
        }),
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
    let v = source?;
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

fn collect_variant_furnishing_elements(
    id: &str,
    owner_region: &str,
    furnishings: &HashMap<&str, &ResolvedFurnishing>,
    elements: &HashMap<&str, &ResolvedElement>,
    stack: &mut HashSet<String>,
    counts: &mut HashMap<String, usize>,
    errors: &mut Vec<String>,
) {
    if !stack.insert(id.to_owned()) {
        return;
    }
    if let Some(furnishing) = furnishings.get(id) {
        for child in &furnishing.children {
            match child {
                FurnishingChild::Furnishing(child) => collect_variant_furnishing_elements(
                    child,
                    owner_region,
                    furnishings,
                    elements,
                    stack,
                    counts,
                    errors,
                ),
                FurnishingChild::Element(child) => {
                    if let Some(element) = elements.get(child.as_str()) {
                        if element.region != owner_region {
                            errors.push(format!(
                                "responsive furnishing '{}' in region '{}' contains element '{}' owned by region '{}'",
                                furnishing.id, owner_region, child, element.region
                            ));
                        }
                        *counts.entry(child.clone()).or_default() += 1;
                    }
                }
            }
        }
    }
    stack.remove(id);
}

fn reachable_regions_for_root(root: &str, compositions: &[ResolvedComposition]) -> HashSet<String> {
    let by_id: HashMap<_, _> = compositions
        .iter()
        .map(|composition| (composition.id.as_str(), composition))
        .collect();
    let mut regions = HashSet::new();
    let mut seen = HashSet::new();
    let mut pending = vec![root.to_owned()];
    while let Some(id) = pending.pop() {
        if !seen.insert(id.clone()) {
            continue;
        }
        if let Some(composition) = by_id.get(id.as_str()) {
            for child in &composition.children {
                match child {
                    CompositionChild::Composition(child) => pending.push(child.clone()),
                    CompositionChild::Region(region) => {
                        regions.insert(region.clone());
                    }
                }
            }
        }
    }
    regions
}

#[allow(clippy::too_many_arguments)]
fn resolve_responsive(
    source: Option<&ResponsiveSource>,
    screen_root: &str,
    dominant: Option<&str>,
    composition_ids: &HashSet<&str>,
    compositions: &[ResolvedComposition],
    regions: &[ResolvedRegion],
    elements: &[ResolvedElement],
    furnishings: &[ResolvedFurnishing],
    errors: &mut Vec<String>,
) -> (Option<ResolvedResponsive>, HashSet<String>) {
    let Some(source) = source else {
        return (None, reachable_regions_for_root(screen_root, compositions));
    };
    if source.variant.len() < 2 {
        errors.push("responsive: requires at least two variants".into());
    }
    let mut ids = HashSet::new();
    let mut resolved = Vec::with_capacity(source.variant.len());
    let region_ids: HashSet<_> = regions.iter().map(|region| region.id.as_str()).collect();
    let furnishing_ids: HashSet<_> = furnishings
        .iter()
        .map(|furnishing| furnishing.id.as_str())
        .collect();
    for variant in &source.variant {
        validate_nonblank_id(&variant.id, "responsive.variant.id", errors);
        if !ids.insert(variant.id.clone()) {
            errors.push(format!("responsive: duplicate variant id '{}'", variant.id));
        }
        if !composition_ids.contains(variant.root.as_str()) {
            errors.push(format!(
                "responsive variant '{}': missing root composition '{}'",
                variant.id, variant.root
            ));
        }
        if variant
            .max_width
            .is_some_and(|max| variant.min_width.unwrap_or(0) >= max)
        {
            errors.push(format!(
                "responsive variant '{}': interval must be non-empty",
                variant.id
            ));
        }
        let reachable = reachable_regions_for_root(&variant.root, compositions);
        let mut overrides = HashMap::new();
        for override_source in &variant.region {
            if !region_ids.contains(override_source.id.as_str()) {
                errors.push(format!(
                    "responsive variant '{}': override references missing region '{}'",
                    variant.id, override_source.id
                ));
            }
            if !reachable.contains(&override_source.id) {
                errors.push(format!(
                    "responsive variant '{}': override region '{}' is not reachable from root '{}'",
                    variant.id, override_source.id, variant.root
                ));
            }
            if override_source.width.is_none()
                && override_source.height.is_none()
                && override_source.grow.is_none()
                && override_source.furnishing.is_none()
            {
                errors.push(format!(
                    "responsive variant '{}': region override '{}' must specify at least one field",
                    variant.id, override_source.id
                ));
            }
            if overrides.contains_key(&override_source.id) {
                errors.push(format!(
                    "responsive variant '{}': duplicate region override '{}'",
                    variant.id, override_source.id
                ));
            }
            let width = override_source.width.as_ref().and_then(|value| {
                let parsed = logical_size(
                    Some(value),
                    "responsive region width",
                    &override_source.id,
                    errors,
                );
                if parsed == Some(0) {
                    errors.push(format!(
                        "responsive variant '{}': region '{}' width must be positive",
                        variant.id, override_source.id
                    ));
                }
                parsed
            });
            let height = override_source.height.as_ref().and_then(|value| {
                let parsed = logical_size(
                    Some(value),
                    "responsive region height",
                    &override_source.id,
                    errors,
                );
                if parsed == Some(0) {
                    errors.push(format!(
                        "responsive variant '{}': region '{}' height must be positive",
                        variant.id, override_source.id
                    ));
                }
                parsed
            });
            let grow = override_source.grow.map(|grow| {
                if !grow.is_finite() || grow < 0.0 {
                    errors.push(format!(
                        "responsive variant '{}': region '{}' grow must be finite and non-negative",
                        variant.id, override_source.id
                    ));
                }
                if grow.is_finite() && grow >= 0.0 {
                    grow
                } else {
                    0.0
                }
            });
            if let Some(furnishing) = &override_source.furnishing {
                if !furnishing_ids.contains(furnishing.as_str()) {
                    errors.push(format!(
                        "responsive variant '{}': region '{}' references missing furnishing '{}'",
                        variant.id, override_source.id, furnishing
                    ));
                }
            }
            overrides.insert(
                override_source.id.clone(),
                ResolvedRegionOverride {
                    width,
                    height,
                    grow,
                    furnishing: override_source.furnishing.clone(),
                },
            );
        }
        resolved.push(ResolvedResponsiveVariant {
            id: variant.id.clone(),
            min_width: variant.min_width.unwrap_or(0),
            max_width: variant.max_width,
            root: variant.root.clone(),
            regions: overrides,
        });
    }
    let mut ordered: Vec<_> = resolved.iter().collect();
    ordered.sort_by_key(|variant| (variant.min_width, variant.max_width));
    let mut cursor = 0_u32;
    for variant in &ordered {
        if variant.min_width != cursor {
            errors.push(format!(
                "responsive: interval coverage gap or overlap before variant '{}' at {}px",
                variant.id, variant.min_width
            ));
        }
        if let Some(max) = variant.max_width {
            cursor = max;
        } else {
            if variant.id
                != ordered
                    .last()
                    .map(|last| last.id.as_str())
                    .unwrap_or_default()
            {
                errors.push(format!(
                    "responsive: unbounded variant '{}' must be final",
                    variant.id
                ));
            }
            cursor = u32::MAX;
        }
    }
    if ordered
        .last()
        .is_none_or(|variant| variant.max_width.is_some())
    {
        errors.push("responsive: intervals must cover through unbounded infinity".into());
    }
    let default = resolved.iter().find(|variant| variant.id == source.default);
    if default.is_none() {
        errors.push(format!(
            "responsive.default: unknown variant '{}'",
            source.default
        ));
    } else if default.is_some_and(|variant| variant.root != screen_root) {
        errors.push(format!(
            "responsive.default: variant '{}' root must equal screen.root '{}'",
            source.default, screen_root
        ));
    }
    let mut union = HashSet::new();
    for variant in &resolved {
        let reachable = reachable_regions_for_root(&variant.root, compositions);
        if let Some(dominant) = dominant {
            let dominant_region = regions
                .iter()
                .find(|region| region.id == dominant)
                .map(|region| region.id.as_str())
                .or_else(|| {
                    elements
                        .iter()
                        .find(|element| element.id == dominant)
                        .map(|element| element.region.as_str())
                });
            if let Some(region) = dominant_region {
                if !reachable.contains(region) {
                    errors.push(format!(
                        "responsive variant '{}': dominant target '{}' is not reachable",
                        variant.id, dominant
                    ));
                }
            }
        }
        union.extend(reachable);
    }
    let furnishing_by_id: HashMap<_, _> = furnishings
        .iter()
        .map(|furnishing| (furnishing.id.as_str(), furnishing))
        .collect();
    let element_by_id: HashMap<_, _> = elements
        .iter()
        .map(|element| (element.id.as_str(), element))
        .collect();
    let region_by_id: HashMap<_, _> = regions
        .iter()
        .map(|region| (region.id.as_str(), region))
        .collect();
    let mut furnishing_owner: HashMap<String, String> = HashMap::new();
    for variant in &resolved {
        for region_id in reachable_regions_for_root(&variant.root, compositions) {
            let Some(region) = region_by_id.get(region_id.as_str()) else {
                continue;
            };
            let furnishing = variant
                .regions
                .get(region_id.as_str())
                .and_then(|override_| override_.furnishing.as_deref())
                .or(region.furnishing.as_deref());
            let Some(furnishing) = furnishing else {
                continue;
            };
            let previous = furnishing_owner.insert(furnishing.to_owned(), region_id.clone());
            if let Some(previous) = previous {
                if previous != region_id {
                    errors.push(format!(
                        "furnishing root '{}' is assigned to regions '{}' and '{}' across responsive variants",
                        furnishing, previous, region_id
                    ));
                }
            }
            if !furnishing_by_id.contains_key(furnishing) {
                continue;
            }
            let mut counts = HashMap::new();
            collect_variant_furnishing_elements(
                furnishing,
                region_id.as_str(),
                &furnishing_by_id,
                &element_by_id,
                &mut HashSet::new(),
                &mut counts,
                errors,
            );
            for element in elements
                .iter()
                .filter(|element| element.region == region_id)
            {
                let count = counts.get(&element.id).copied().unwrap_or(0);
                if count != 1 {
                    errors.push(format!(
                        "responsive variant '{}': furnished region '{}' element '{}' must be reachable exactly once (found {count})",
                        variant.id, region_id, element.id
                    ));
                }
            }
        }
    }
    (
        Some(ResolvedResponsive {
            default: source.default.clone(),
            variants: resolved,
        }),
        union,
    )
}

pub fn resolve(source: SourceBlueprint) -> Result<ResolvedBlueprint, BlueprintError> {
    let mut errors = Vec::new();
    let responsive_enabled = source.responsive.is_some();
    let mut ids = HashSet::new();
    validate_token_names(&source.tokens.spacing, "tokens.spacing", &mut errors);
    validate_token_names(&source.tokens.corners, "tokens.corners", &mut errors);
    validate_token_names(&source.tokens.color.values, "tokens.color", &mut errors);
    validate_color_values(&source.tokens.color, &mut errors);
    let token_validation_errors = errors.len();
    for (index, value) in source.design.character.iter().enumerate() {
        if value.trim().is_empty() {
            errors.push(format!(
                "design.character[{index}]: must contain at least one non-whitespace character"
            ));
        }
    }
    for (index, value) in source.design.avoid.iter().enumerate() {
        if value.trim().is_empty() {
            errors.push(format!(
                "design.avoid[{index}]: must contain at least one non-whitespace character"
            ));
        }
    }
    validate_nonblank_id(&source.screen.id, "screen.id", &mut errors);
    if source.screen.purpose.trim().is_empty() {
        errors.push("screen.purpose: must contain at least one non-whitespace character".into());
    }
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
        validate_screen_author_id_collision(&source.screen.id, &r.id, "region", &mut errors);
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
            overflow: overflow(r.overflow.as_deref(), &r.id, &mut errors),
            furnishing: r.furnishing.clone(),
        });
    }
    let region_ids: HashSet<_> = regions
        .iter()
        .filter(|r| !r.id.trim().is_empty())
        .map(|r| r.id.clone())
        .collect();
    let mut elements = Vec::new();
    for e in &source.element {
        validate_screen_author_id_collision(&source.screen.id, &e.id, "element", &mut errors);
        add_id(&mut ids, &mut errors, &e.id, "element");
        if !region_ids.contains(&e.region) {
            errors.push(format!("element '{}': missing region '{}'", e.id, e.region));
        }
        let element_kind = kind(&e.kind, &format!("element '{}'", e.id), &mut errors);
        let presentation =
            collection_presentation(e.presentation.as_deref(), &e.id, element_kind, &mut errors);
        let choice = choice_config(e.choice.as_ref(), &e.id, element_kind, &mut errors);
        let scalar = scalar_config(e.scalar.as_ref(), &e.id, element_kind, &mut errors);
        let action = match (element_kind, e.action.as_deref()) {
            (
                ElementKind::Command
                | ElementKind::Choice
                | ElementKind::Boolean
                | ElementKind::Scalar,
                Some(action),
            ) => action_id(action, &e.id, &mut errors),
            (ElementKind::Command, None) => {
                errors.push(format!(
                    "element '{}': command elements require an action",
                    e.id
                ));
                None
            }
            (ElementKind::Choice | ElementKind::Boolean | ElementKind::Scalar, None) => {
                errors.push(format!(
                    "element '{}': {} elements require an action",
                    e.id, e.kind
                ));
                None
            }
            (_kind, Some(_)) => {
                errors.push(format!(
                    "element '{}': action is only valid for command and value-control elements",
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
            label: element_label(e.label.as_ref(), &e.id, &mut errors),
            presentation,
            action,
            choice,
            scalar,
        });
    }
    let composition_ids: HashSet<_> = source
        .composition
        .iter()
        .filter(|c| !c.id.trim().is_empty())
        .map(|c| c.id.as_str())
        .collect();
    let element_ids: HashSet<_> = elements
        .iter()
        .filter(|e| !e.id.trim().is_empty())
        .map(|e| e.id.as_str())
        .collect();
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
            if region_ids.contains(child) {
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
        if let Some(gap_token) = &c.gap {
            if !spacing.contains_key(gap_token) {
                errors.push(format!(
                    "composition '{}': missing spacing token '{}'",
                    c.id, gap_token
                ));
            }
        }
        if composition_kind == CompositionKind::Overlay {
            if c.gap.is_some() {
                errors.push(format!(
                    "composition '{}': overlay does not support authored gap",
                    c.id
                ));
            }
            if children.len() != 2 {
                errors.push(format!(
                    "composition '{}': overlay requires exactly two children",
                    c.id
                ));
            } else {
                if !matches!(children[0], CompositionChild::Composition(_)) {
                    errors.push(format!(
                        "composition '{}': overlay child 0 must be a composition",
                        c.id
                    ));
                }
                if !matches!(children[1], CompositionChild::Region(_)) {
                    errors.push(format!(
                        "composition '{}': overlay child 1 must be a region",
                        c.id
                    ));
                }
                if let CompositionChild::Region(id) = &children[1] {
                    let floating = regions.iter().find(|region| region.id == *id).unwrap();
                    if floating.width.is_none() {
                        errors.push(format!(
                            "composition '{}': overlay floating region '{}' requires fixed width",
                            c.id, id
                        ));
                    }
                    if floating.height.is_none() {
                        errors.push(format!(
                            "composition '{}': overlay floating region '{}' requires fixed height",
                            c.id, id
                        ));
                    }
                    if floating.grow > 0.0 {
                        errors.push(format!(
                            "composition '{}': overlay floating region '{}' cannot have positive grow",
                            c.id, id
                        ));
                    }
                }
            }
        }
        let padding = c
            .padding
            .as_ref()
            .and_then(|g| spacing.get(g))
            .copied()
            .unwrap_or(0);
        if let Some(padding_token) = &c.padding {
            if !spacing.contains_key(padding_token) {
                errors.push(format!(
                    "composition '{}': missing spacing token '{}'",
                    c.id, padding_token
                ));
            }
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
    let furnishings = resolve_furnishings(
        &source.region,
        &source.furnishing,
        &mut regions,
        &elements,
        &mut ids,
        spacing,
        source.responsive.is_some(),
        &mut errors,
    );
    let root = match source.screen.root.as_deref() {
        None => {
            errors.push("screen.root: missing explicit composition reference".into());
            None
        }
        Some(root) if !composition_ids.contains(root) => {
            errors.push(format!("screen.root: '{root}' is not a composition"));
            None
        }
        Some(root) => Some(root.to_owned()),
    };
    let (responsive, responsive_reachable_regions) = resolve_responsive(
        source.responsive.as_ref(),
        root.as_deref().unwrap_or_default(),
        source.design.dominant.as_deref(),
        &composition_ids,
        &compositions,
        &regions,
        &elements,
        &furnishings,
        &mut errors,
    );

    let mut owners: HashMap<&str, &str> = HashMap::new();
    for composition in &compositions {
        let mut siblings = HashSet::new();
        for child in &composition.children {
            let (child_id, child_kind) = match child {
                CompositionChild::Region(id) => (id.as_str(), "region"),
                CompositionChild::Composition(id) => (id.as_str(), "composition"),
            };
            if !siblings.insert(child_id) {
                errors.push(format!(
                    "composition '{}': child '{}' is duplicated within its children",
                    composition.id, child_id
                ));
                continue;
            }
            if let Some(first_parent) = owners.get(child_id) {
                if !responsive_enabled && *first_parent != composition.id {
                    errors.push(format!(
                        "{} '{}' has multiple composition parents: '{}' and '{}'",
                        child_kind, child_id, first_parent, composition.id
                    ));
                }
            } else {
                owners.insert(child_id, composition.id.as_str());
            }
            if !responsive_enabled
                && matches!(child, CompositionChild::Composition(id) if Some(id.as_str()) == root.as_deref())
            {
                errors.push(format!(
                    "composition '{}': root composition '{}' cannot be a child",
                    composition.id,
                    root.as_deref().unwrap_or_default()
                ));
            }
        }
    }

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
    for composition in &compositions {
        if !visited.contains(&composition.id) {
            visit(
                &composition.id,
                &by_id,
                &mut visiting,
                &mut visited,
                &mut errors,
            );
        }
    }
    let mut reachable_regions = HashSet::new();
    if let Some(root_id) = root.as_deref() {
        let mut reachable_compositions = HashSet::new();
        let mut pending = vec![root_id.to_owned()];
        while let Some(id) = pending.pop() {
            if !reachable_compositions.insert(id.clone()) {
                continue;
            }
            if let Some(composition) = by_id.get(id.as_str()) {
                for child in &composition.children {
                    match child {
                        CompositionChild::Composition(child) => pending.push(child.clone()),
                        CompositionChild::Region(region) => {
                            reachable_regions.insert(region.clone());
                        }
                    }
                }
            }
        }
    }
    let fixture_reachable_regions = if responsive_enabled {
        &responsive_reachable_regions
    } else {
        &reachable_regions
    };
    let mut resolved_fixtures = Vec::new();
    for f in &source.fixture {
        add_id(&mut ids, &mut errors, &f.id, "fixture");
        if f.state.trim().is_empty() {
            errors.push(format!(
                "fixture '{}': state must contain at least one non-whitespace character",
                f.id
            ));
        }
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
            if root.is_some() && !fixture_reachable_regions.contains(element.region.as_str()) {
                errors.push(format!(
                    "fixture '{}': content target '{}' is not reachable because owning region '{}' is outside screen.root '{}'",
                    f.id,
                    record.element,
                    element.region,
                    root.as_deref().unwrap_or_default()
                ));
                continue;
            }
            let families = usize::from(record.items.is_some())
                + usize::from(record.properties.is_some())
                + usize::from(record.text.is_some())
                + usize::from(record.nodes.is_some())
                + usize::from(record.document.is_some())
                + usize::from(record.command.is_some())
                + usize::from(record.choice.is_some())
                + usize::from(record.boolean.is_some())
                + usize::from(record.scalar.is_some());
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
                        validate_nonblank_id(
                            &item.id,
                            &format!("fixture '{}': collection item id '{}'", f.id, item.id),
                            &mut errors,
                        );
                        if item.id.trim().is_empty() {
                            return None;
                        }
                        if item.label.trim().is_empty() {
                            errors.push(format!(
                                "fixture '{}': collection item '{}' label must contain at least one non-whitespace character",
                                f.id, item.id
                            ));
                        }
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
                        .enumerate()
                        .map(|(index, p)| {
                            if p.name.trim().is_empty() {
                                errors.push(format!(
                                    "fixture '{}': property[{}] for '{}' name must contain at least one non-whitespace character",
                                    f.id, index, record.element
                                ));
                            }
                            ResolvedProperty {
                                name: p.name.clone(),
                                value: p.value.clone(),
                            }
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
                if text.trim().is_empty() {
                    errors.push(format!(
                        "fixture '{}': text for '{}' must contain at least one non-whitespace character",
                        f.id, record.element
                    ));
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
                        validate_nonblank_id(
                            &node.id,
                            &format!("fixture '{}': tree node id '{}'", f.id, node.id),
                            &mut errors,
                        );
                        if node.id.trim().is_empty() {
                            return None;
                        }
                        if node.label.trim().is_empty() {
                            errors.push(format!(
                                "fixture '{}': tree node '{}' label must contain at least one non-whitespace character",
                                f.id, node.id
                            ));
                        }
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
                let resolved =
                    resolve_document_source(&f.id, &record.element, document, &mut errors);
                content.push(ResolvedFixtureContent::Document {
                    element: record.element.clone(),
                    document: resolved,
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
            } else if let Some(choice_state) = &record.choice {
                if element.kind != ElementKind::Choice {
                    errors.push(format!(
                        "fixture '{}': choice state on '{}' requires a choice element",
                        f.id, record.element
                    ));
                    continue;
                }
                let option_exists = element.choice.as_ref().is_some_and(|choice| {
                    choice
                        .options
                        .iter()
                        .any(|option| option.id == choice_state.selected)
                });
                if !option_exists {
                    errors.push(format!(
                        "fixture '{}': selected choice '{}' is absent from options for '{}'",
                        f.id, choice_state.selected, record.element
                    ));
                }
                content.push(ResolvedFixtureContent::Choice {
                    element: record.element.clone(),
                    selected: choice_state.selected.clone(),
                });
            } else if let Some(boolean_state) = &record.boolean {
                if element.kind != ElementKind::Boolean {
                    errors.push(format!(
                        "fixture '{}': boolean state on '{}' requires a boolean element",
                        f.id, record.element
                    ));
                    continue;
                }
                content.push(ResolvedFixtureContent::Boolean {
                    element: record.element.clone(),
                    value: boolean_state.value,
                });
            } else if let Some(scalar_state) = &record.scalar {
                if element.kind != ElementKind::Scalar {
                    errors.push(format!(
                        "fixture '{}': scalar state on '{}' requires a scalar element",
                        f.id, record.element
                    ));
                    continue;
                }
                if !scalar_state.value.is_finite() {
                    errors.push(format!(
                        "fixture '{}': scalar value for '{}' must be finite",
                        f.id, record.element
                    ));
                }
                if let Some(scalar) = &element.scalar {
                    if scalar_state.value.is_finite()
                        && (scalar_state.value < scalar.min || scalar_state.value > scalar.max)
                    {
                        errors.push(format!(
                            "fixture '{}': scalar value {} for '{}' must be within {}..{}",
                            f.id, scalar_state.value, record.element, scalar.min, scalar.max
                        ));
                    }
                }
                content.push(ResolvedFixtureContent::Scalar {
                    element: record.element.clone(),
                    value: scalar_state.value,
                });
            }
        }
        for element in elements.iter().filter(|element| {
            matches!(
                element.kind,
                ElementKind::Choice | ElementKind::Boolean | ElementKind::Scalar
            )
        }) {
            if !seen_content.contains(&element.id) {
                errors.push(format!(
                    "fixture '{}': missing representative state for control '{}'",
                    f.id, element.id
                ));
            }
        }
        resolved_fixtures.push(ResolvedFixture {
            id: f.id.clone(),
            state: f.state.clone(),
            content,
        });
    }
    let dominant = source.design.dominant.as_ref().and_then(|d| {
        if region_ids.contains(d) {
            if root.is_some() && !reachable_regions.contains(d) {
                errors.push(format!(
                    "design.dominant: region '{}' is not reachable from screen.root '{}'",
                    d,
                    root.as_deref().unwrap_or_default()
                ));
            }
            Some(DominantTarget::Region(d.clone()))
        } else if element_ids.contains(d.as_str()) {
            let element = elements.iter().find(|element| element.id == *d).unwrap();
            if root.is_some() && !reachable_regions.contains(element.region.as_str()) {
                errors.push(format!(
                    "design.dominant: element '{}' is not reachable because owning region '{}' is outside screen.root '{}'",
                    d,
                    element.region,
                    root.as_deref().unwrap_or_default()
                ));
            }
            Some(DominantTarget::Element(d.clone()))
        } else {
            errors.push(format!("design.dominant: missing reference '{d}'"));
            None
        }
    });
    if errors.len() > token_validation_errors {
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
        design: ResolvedDesign {
            character: source.design.character,
            dominant,
            avoid: source.design.avoid,
        },
        root: root.expect("validated root"),
        regions,
        compositions,
        furnishings,
        elements,
        fixtures: resolved_fixtures,
        visual,
        responsive,
    })
}

impl ResolvedBlueprint {
    pub fn semantic_tree(&self) -> String {
        fn render_element(element: &ResolvedElement, out: &mut String, indent: usize) {
            let mut semantics = Vec::new();
            if let Some(action) = &element.action {
                semantics.push(format!("action {}", action.as_str()));
            }
            if let Some(presentation) = element.presentation {
                semantics.push(format!("presentation {presentation:?}"));
            }
            if let Some(choice) = &element.choice {
                semantics.push(format!(
                    "choice/{} options [{}]",
                    choice.presentation.as_str(),
                    choice
                        .options
                        .iter()
                        .map(|option| format!("{}={}", option.id, option.label))
                        .collect::<Vec<_>>()
                        .join(", ")
                ));
            }
            if let Some(scalar) = &element.scalar {
                semantics.push(format!(
                    "scalar {}..{} step {}{}",
                    scalar.min,
                    scalar.max,
                    scalar.step,
                    scalar
                        .unit
                        .as_deref()
                        .map(|unit| format!(" {unit}"))
                        .unwrap_or_default()
                ));
            }
            out.push_str(&format!(
                "{:indent$}element {} ({:?}{})\n",
                "",
                element.id,
                element.kind,
                if semantics.is_empty() {
                    String::new()
                } else {
                    format!(", {}", semantics.join(", "))
                },
                indent = indent
            ));
        }
        fn walk_furnishing(id: &str, b: &ResolvedBlueprint, out: &mut String, indent: usize) {
            let Some(furnishing) = b.furnishings.iter().find(|furnishing| furnishing.id == id)
            else {
                return;
            };
            let mut attributes = vec![
                format!("gap {}px", furnishing.gap),
                format!("padding {}px", furnishing.padding),
                format!("overflow {}", furnishing.overflow.as_str()),
            ];
            if let Some(width) = furnishing.width {
                attributes.push(format!("width {width}px"));
            }
            if let Some(height) = furnishing.height {
                attributes.push(format!("height {height}px"));
            }
            if furnishing.grow > 0.0 {
                attributes.push(format!("grow {}", furnishing.grow));
            }
            out.push_str(&format!(
                "{:indent$}furnishing {} — {} ({})\n",
                "",
                furnishing.id,
                furnishing.kind,
                attributes.join(", "),
                indent = indent
            ));
            for child in &furnishing.children {
                match child {
                    FurnishingChild::Furnishing(child) => {
                        walk_furnishing(child, b, out, indent + 2)
                    }
                    FurnishingChild::Element(id) => {
                        if let Some(element) = b.elements.iter().find(|element| element.id == *id) {
                            render_element(element, out, indent + 2);
                        }
                    }
                }
            }
        }
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
            if let Some(axis) = c.axis {
                out.push_str(&format!(
                    "{:indent$}composition {} ({}, {})\n",
                    "",
                    c.id,
                    c.kind.as_str(),
                    axis.as_str(),
                    indent = indent
                ));
            } else {
                out.push_str(&format!(
                    "{:indent$}composition {} ({}, axisless)\n",
                    "",
                    c.id,
                    c.kind.as_str(),
                    indent = indent
                ));
            }
            for (index, child) in c.children.iter().enumerate() {
                let layer = if c.kind == CompositionKind::Overlay {
                    Some(if index == 0 { "base" } else { "floating" })
                } else {
                    None
                };
                match child {
                    CompositionChild::Composition(child) => {
                        if let Some(layer) = layer {
                            out.push_str(&format!(
                                "{:indent$}{layer} layer\n",
                                "",
                                indent = indent + 2
                            ));
                        }
                        walk(
                            child,
                            b,
                            out,
                            indent + if layer.is_some() { 4 } else { 2 },
                            seen,
                        )
                    }
                    CompositionChild::Region(region) => {
                        let resolved = b.regions.iter().find(|candidate| candidate.id == *region);
                        let overflow = resolved
                            .map(|candidate| candidate.overflow)
                            .unwrap_or(OverflowPolicy::Clip);
                        let annotation = if overflow == OverflowPolicy::ScrollY {
                            format!(", overflow {}", overflow.as_str())
                        } else {
                            String::new()
                        };
                        out.push_str(&format!(
                            "{:indent$}{}region {} (role {}{}{})\n",
                            "",
                            layer.map(|layer| format!("{layer} ")).unwrap_or_default(),
                            region,
                            resolved
                                .map(|candidate| candidate.role.as_str())
                                .unwrap_or("unknown"),
                            annotation,
                            resolved
                                .and_then(|candidate| candidate.furnishing.as_deref())
                                .map(|root| format!(", furnishing {root}"))
                                .unwrap_or_default(),
                            indent = indent + 2
                        ));
                        if let Some(root) =
                            resolved.and_then(|candidate| candidate.furnishing.as_deref())
                        {
                            walk_furnishing(root, b, out, indent + 4);
                        } else {
                            for element in b
                                .elements
                                .iter()
                                .filter(|element| element.region == *region)
                            {
                                render_element(element, out, indent + 4);
                            }
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
                    ResolvedFixtureContent::Document { element, document } => match document {
                        ResolvedDocument::Legacy { title, paragraphs } => out.push_str(&format!(
                            "    content {element}: document '{title}', {} paragraphs\n",
                            paragraphs.len()
                        )),
                        ResolvedDocument::Rich { blocks, spoken } => {
                            out.push_str(&format!(
                                "    content {element}: rich document, {} blocks\n",
                                blocks.len()
                            ));
                            for block in blocks {
                                match block {
                                    ResolvedDocumentBlock::Heading { id, level, text } => out
                                        .push_str(&format!(
                                            "      block {id}: heading level {level} — {text}\n"
                                        )),
                                    ResolvedDocumentBlock::Eyebrow { id, text }
                                    | ResolvedDocumentBlock::Paragraph { id, text }
                                    | ResolvedDocumentBlock::Quote { id, text } => {
                                        out.push_str(&format!(
                                            "      block {id}: {} — {text}\n",
                                            block.kind().as_str()
                                        ))
                                    }
                                    ResolvedDocumentBlock::Divider { id } => {
                                        out.push_str(&format!("      block {id}: divider\n"))
                                    }
                                }
                            }
                            if let Some(spoken) = spoken {
                                out.push_str(&format!(
                                    "      spoken: block {}, range {}..{}\n",
                                    spoken.block, spoken.start, spoken.end
                                ));
                            }
                        }
                    },
                    ResolvedFixtureContent::Command {
                        element,
                        enabled,
                        reason,
                    } => out.push_str(&format!(
                        "    content {element}: command enabled={enabled}, reason {:?}\n",
                        reason
                    )),
                    ResolvedFixtureContent::Choice { element, selected } => out.push_str(&format!(
                        "    content {element}: choice selected={selected}\n"
                    )),
                    ResolvedFixtureContent::Boolean { element, value } => {
                        out.push_str(&format!("    content {element}: boolean value={value}\n"))
                    }
                    ResolvedFixtureContent::Scalar { element, value } => {
                        out.push_str(&format!("    content {element}: scalar value={value}\n"))
                    }
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

    fn control_source() -> String {
        r#"
[screen]
id = "controls"
purpose = "Test value controls"
root = "root"

[[region]]
id = "inspector"
role = "inspector"
importance = "secondary"

[[region]]
id = "reader"
role = "primary_content"
importance = "primary"

[[composition]]
id = "root"
kind = "column"
axis = "vertical"
children = ["inspector", "reader"]

[[element]]
id = "font"
region = "inspector"
kind = "choice"
importance = "secondary"
label = "Font"
action = "reader.font.set"
choice = { presentation = "select", options = [{ id = "a", label = "Alpha" }, { id = "b", label = "Beta" }] }

[[element]]
id = "enabled"
region = "inspector"
kind = "boolean"
importance = "secondary"
label = "Enabled"
action = "reader.enabled.set"

[[element]]
id = "size"
region = "inspector"
kind = "scalar"
importance = "secondary"
label = "Size"
action = "reader.size.set"
scalar = { min = 10.0, max = 30.0, step = 1.0, unit = "px" }

[[fixture]]
id = "first"
state = "representative"
[[fixture.content]]
element = "font"
choice = { selected = "a" }
[[fixture.content]]
element = "enabled"
boolean = { value = true }
[[fixture.content]]
element = "size"
scalar = { value = 18.0 }
"#.to_owned()
    }

    fn control_validation_error(source: &str, expected: &str) {
        let error = parse_and_resolve(source).unwrap_err().to_string();
        assert!(
            error.contains(expected),
            "expected {expected:?} in {error:?}"
        );
    }

    #[test]
    fn m42_typed_controls_resolve_with_explicit_fixture_state_and_inspectable_semantics() {
        let blueprint = parse_and_resolve(&control_source()).unwrap();
        assert!(blueprint.elements.iter().any(|element| {
            element.id == "font"
                && element.kind == ElementKind::Choice
                && element.choice.as_ref().is_some_and(|choice| {
                    choice.presentation == ChoicePresentation::Select && choice.options.len() == 2
                })
        }));
        assert!(blueprint
            .elements
            .iter()
            .any(|element| element.kind == ElementKind::Boolean));
        assert!(blueprint.elements.iter().any(|element| {
            element.kind == ElementKind::Scalar
                && element
                    .scalar
                    .as_ref()
                    .is_some_and(|scalar| scalar.min == 10.0 && scalar.max == 30.0)
        }));
        let semantic = blueprint.semantic_tree();
        assert!(semantic.contains("choice/select options [a=Alpha, b=Beta]"));
        assert!(semantic.contains("scalar 10..30 step 1 px"));
        assert!(semantic.contains("content enabled: boolean value=true"));
    }

    #[test]
    fn m42_element_configuration_validation_rejects_invalid_control_schemas() {
        let source = control_source();
        let cases = [
            (source.replace("choice = { presentation = \"select\", options = [{ id = \"a\", label = \"Alpha\" }, { id = \"b\", label = \"Beta\" }] }\n", ""), "choice elements require choice configuration"),
            (source.replacen("kind = \"choice\"", "kind = \"text\"", 1), "choice configuration is only valid for choice elements"),
            (source.replace("{ id = \"b\", label = \"Beta\" }]", "{ id = \"a\", label = \"Beta\" }]"), "duplicate choice option id 'a'"),
            (source.replace("{ id = \"a\", label = \"Alpha\" }", "{ id = \"  \", label = \"Alpha\" }"), "choice option id must be nonblank"),
            (source.replace("label = \"Alpha\"", "label = \"  \""), "choice option 'a' label must be nonblank"),
            (source.replace("presentation = \"select\"", "presentation = \"tabs\""), "unknown choice presentation 'tabs'"),
            (source.replace("options = [{ id = \"a\", label = \"Alpha\" }, { id = \"b\", label = \"Beta\" }]", "options = [{ id = \"a\", label = \"Alpha\" }]"), "choice requires at least two options"),
            (source.replacen("kind = \"scalar\"", "kind = \"text\"", 1), "scalar configuration is only valid for scalar elements"),
            (source.replace("scalar = { min = 10.0, max = 30.0, step = 1.0, unit = \"px\" }\n", ""), "scalar elements require scalar configuration"),
            (source.replace("min = 10.0", "min = nan"), "scalar min must be finite"),
            (source.replace("max = 30.0", "max = nan"), "scalar max must be finite"),
            (source.replace("step = 1.0", "step = nan"), "scalar step must be finite and positive"),
            (source.replace("min = 10.0, max = 30.0", "min = 30.0, max = 10.0"), "scalar min must be less than max"),
            (source.replace("step = 1.0", "step = 0.0"), "scalar step must be finite and positive"),
            (source.replace("unit = \"px\"", "unit = \"  \""), "scalar unit must be nonblank"),
            (source.replace("action = \"reader.font.set\"", ""), "choice elements require an action"),
            (source.replace("reader.font.set", "bad action"), "action must be a non-empty dot-separated identifier"),
        ];
        for (invalid, expected) in cases {
            control_validation_error(&invalid, expected);
        }
    }

    #[test]
    fn m42_fixture_state_validation_rejects_cross_kind_missing_duplicate_and_invalid_values() {
        let source = control_source();
        let cases = [
            (
                source.replace("selected = \"a\"", "selected = \"missing\""),
                "selected choice 'missing' is absent from options",
            ),
            (
                source.replacen("kind = \"boolean\"", "kind = \"text\"", 1),
                "boolean state on 'enabled' requires a boolean element",
            ),
            (
                source.replacen("kind = \"choice\"", "kind = \"text\"", 1),
                "choice state on 'font' requires a choice element",
            ),
            (
                source.replacen("kind = \"scalar\"", "kind = \"text\"", 1),
                "scalar state on 'size' requires a scalar element",
            ),
            (
                source.replace("value = 18.0", "value = nan"),
                "scalar value for 'size' must be finite",
            ),
            (
                source.replace("value = 18.0", "value = 31.0"),
                "scalar value 31 for 'size' must be within 10..30",
            ),
            (
                source.replace(
                    "[[fixture.content]]\nelement = \"size\"\nscalar = { value = 18.0 }\n",
                    "",
                ),
                "missing representative state for control 'size'",
            ),
            (
                format!(
                    "{}\n[[fixture.content]]\nelement = \"size\"\nscalar = {{ value = 20.0 }}\n",
                    source
                ),
                "duplicate content for element 'size'",
            ),
        ];
        for (invalid, expected) in cases {
            control_validation_error(&invalid, expected);
        }
    }
    fn reader() -> &'static str {
        include_str!("../../../specimens/reader-workspace.toml")
    }
    #[test]
    fn project_browser_resolves() {
        let blueprint = parse_and_resolve(project()).unwrap();
        assert_eq!(blueprint.root, "workspace");
        assert_eq!(
            blueprint
                .elements
                .iter()
                .find(|element| element.id == "navigation_items")
                .unwrap()
                .label,
            "navigation items"
        );
        assert_eq!(
            blueprint
                .elements
                .iter()
                .find(|element| element.id == "project_collection")
                .unwrap()
                .label,
            "project collection"
        );
        assert_eq!(
            blueprint
                .elements
                .iter()
                .find(|element| element.id == "project_inspector")
                .unwrap()
                .label,
            "project inspector"
        );
    }

    #[test]
    fn canonical_dominant_targets_resolve_as_regions() {
        for source in [
            project(),
            reader(),
            include_str!("../../../specimens/reader-workspace-visual.toml"),
            include_str!("../../../specimens/dependency-workbench.toml"),
            include_str!("../../../specimens/density-pressure-comfortable.toml"),
            include_str!("../../../specimens/density-pressure-dense.toml"),
        ] {
            let blueprint = parse_and_resolve(source).unwrap();
            assert!(matches!(
                blueprint.design.dominant,
                Some(DominantTarget::Region(_))
            ));
        }
    }

    fn nested_dominant_source(dominant: &str) -> String {
        format!(
            "[screen]\nid='x'\npurpose='x'\nroot='workspace'\n[design]\ndominant='{dominant}'\n[[region]]\nid='navigation'\nrole='navigation'\nimportance='secondary'\n[[region]]\nid='nested_target'\nrole='primary_content'\nimportance='primary'\n[[region]]\nid='other'\nrole='status'\nimportance='tertiary'\n[[element]]\nid='target_element'\nregion='nested_target'\nkind='text'\nimportance='primary'\nlabel='Target'\n[[composition]]\nid='workspace'\nkind='split'\nchildren=['body','other']\n[[composition]]\nid='body'\nkind='split'\nchildren=['navigation','nested_target']"
        )
    }

    #[test]
    fn reachable_dominant_regions_and_elements_preserve_typed_identity() {
        let direct = parse_and_resolve(project()).unwrap();
        assert_eq!(
            direct.design.dominant,
            Some(DominantTarget::Region("projects".into()))
        );

        let nested_region = parse_and_resolve(&nested_dominant_source("nested_target")).unwrap();
        assert_eq!(
            nested_region.design.dominant,
            Some(DominantTarget::Region("nested_target".into()))
        );

        let nested_element = parse_and_resolve(&nested_dominant_source("target_element")).unwrap();
        assert_eq!(
            nested_element.design.dominant,
            Some(DominantTarget::Element("target_element".into()))
        );
    }

    #[test]
    fn unreachable_dominant_region_is_rejected_without_global_reachability_rule() {
        let source = "[screen]\nid='x'\npurpose='x'\nroot='workspace'\n[design]\ndominant='unused_region'\n[[region]]\nid='visible'\nrole='primary_content'\nimportance='primary'\n[[region]]\nid='other'\nrole='status'\nimportance='secondary'\n[[region]]\nid='unused_region'\nrole='inspector'\nimportance='tertiary'\n[[composition]]\nid='workspace'\nkind='split'\nchildren=['visible','other']";
        let error = parse_and_resolve(source).unwrap_err().to_string();
        assert!(
            error.contains("design.dominant")
                && error.contains("unused_region")
                && error.contains("not reachable")
                && error.contains("workspace")
        );
    }

    #[test]
    fn unreachable_dominant_element_is_rejected_through_its_owning_region() {
        let source = "[screen]\nid='x'\npurpose='x'\nroot='workspace'\n[design]\ndominant='unused_details'\n[[region]]\nid='visible'\nrole='primary_content'\nimportance='primary'\n[[region]]\nid='other'\nrole='status'\nimportance='secondary'\n[[region]]\nid='unused_inspector'\nrole='inspector'\nimportance='tertiary'\n[[element]]\nid='unused_details'\nregion='unused_inspector'\nkind='text'\nimportance='tertiary'\nlabel='Details'\n[[composition]]\nid='workspace'\nkind='split'\nchildren=['visible','other']";
        let error = parse_and_resolve(source).unwrap_err().to_string();
        assert!(
            error.contains("design.dominant")
                && error.contains("unused_details")
                && error.contains("unused_inspector")
                && error.contains("not reachable")
                && error.contains("workspace")
        );
    }

    #[test]
    fn canonical_dominant_targets_are_root_reachable() {
        for source in [
            project(),
            reader(),
            include_str!("../../../specimens/reader-workspace-visual.toml"),
            include_str!("../../../specimens/reader-workspace-visual-m7.toml"),
            include_str!("../../../specimens/dependency-workbench.toml"),
            include_str!("../../../specimens/density-pressure-comfortable.toml"),
            include_str!("../../../specimens/density-pressure-dense.toml"),
        ] {
            assert!(parse_and_resolve(source).is_ok());
        }
    }

    fn dominant_element_source(dominant: Option<&str>) -> String {
        let dominant = dominant
            .map(|dominant| format!("dominant='{dominant}'\n"))
            .unwrap_or_default();
        format!(
            "[screen]\nid='x'\npurpose='x'\nroot='root'\n[design]\ncharacter=['Zed','alpha']\n{dominant}avoid=['z_avoid','A_avoid']\n[[region]]\nid='navigation'\nrole='navigation'\nimportance='secondary'\n[[region]]\nid='content'\nrole='primary_content'\nimportance='primary'\n[[element]]\nid='target'\nregion='navigation'\nkind='text'\nimportance='tertiary'\nlabel='Target copy'\n[[composition]]\nid='root'\nkind='split'\nchildren=['navigation','content']"
        )
    }

    #[test]
    fn dominant_targets_distinguish_elements_and_preserve_design_metadata() {
        let blueprint = parse_and_resolve(&dominant_element_source(Some("target"))).unwrap();
        assert_eq!(
            blueprint.design.dominant,
            Some(DominantTarget::Element("target".into()))
        );
        assert_eq!(blueprint.design.character, ["Zed", "alpha"]);
        assert_eq!(blueprint.design.avoid, ["z_avoid", "A_avoid"]);

        let omitted = parse_and_resolve(&dominant_element_source(None)).unwrap();
        assert_eq!(omitted.design.dominant, None);
    }

    fn design_entries_source(character: &str, avoid: &str) -> String {
        format!(
            "[screen]\nid='x'\npurpose='x'\nroot='root'\n[design]\ncharacter={character}\navoid={avoid}\n[[region]]\nid='a'\nrole='navigation'\nimportance='primary'\n[[region]]\nid='b'\nrole='inspector'\nimportance='secondary'\n[[composition]]\nid='root'\nkind='split'\nchildren=['a','b']"
        )
    }

    #[test]
    fn design_vocabulary_entries_must_be_nonblank() {
        for value in ["", "   ", "\t", " \t "] {
            let character = format!("[{value:?}]");
            let error = parse_and_resolve(&design_entries_source(&character, "['valid']"))
                .unwrap_err()
                .to_string();
            assert!(error.contains("design.character[0]") && error.contains("non-whitespace"));

            let avoid = format!("[{value:?}]");
            let error = parse_and_resolve(&design_entries_source("['valid']", &avoid))
                .unwrap_err()
                .to_string();
            assert!(error.contains("design.avoid[0]") && error.contains("non-whitespace"));
        }
    }

    #[test]
    fn design_vocabulary_preserves_order_text_and_duplicates() {
        let blueprint = parse_and_resolve(&design_entries_source(
            "['  first  ','second','second']",
            "['  avoid  ','avoid','avoid']",
        ))
        .unwrap();
        assert_eq!(
            blueprint.design.character,
            ["  first  ", "second", "second"]
        );
        assert_eq!(blueprint.design.avoid, ["  avoid  ", "avoid", "avoid"]);
    }

    #[test]
    fn omitted_and_explicitly_empty_design_vocabulary_remain_legal() {
        let omitted = parse_and_resolve(&token_maps_source("", "", "")).unwrap();
        assert!(omitted.design.character.is_empty());
        assert!(omitted.design.avoid.is_empty());
        assert_eq!(omitted.design.dominant, None);

        let empty = parse_and_resolve(&design_entries_source("[]", "[]")).unwrap();
        assert!(empty.design.character.is_empty());
        assert!(empty.design.avoid.is_empty());
    }

    #[test]
    fn canonical_sources_resolve_with_nonblank_design_vocabulary() {
        for source in [
            project(),
            reader(),
            include_str!("../../../specimens/reader-workspace-visual.toml"),
            include_str!("../../../specimens/reader-workspace-visual-m7.toml"),
            include_str!("../../../specimens/dependency-workbench.toml"),
            include_str!("../../../specimens/density-pressure-comfortable.toml"),
            include_str!("../../../specimens/density-pressure-dense.toml"),
        ] {
            let parsed: SourceBlueprint = toml::from_str(source).unwrap();
            assert!(parsed
                .design
                .character
                .iter()
                .all(|value| !value.trim().is_empty()));
            assert!(parsed
                .design
                .avoid
                .iter()
                .all(|value| !value.trim().is_empty()));
            parse_and_resolve(source).unwrap();
        }
    }

    #[test]
    fn invalid_dominant_namespaces_remain_rejected() {
        for (target, replacement) in [
            ("missing", "missing"),
            ("workspace", "workspace"),
            ("many_projects", "many_projects"),
        ] {
            let source = project().replace(
                "dominant = \"projects\"",
                &format!("dominant = \"{replacement}\""),
            );
            let error = parse_and_resolve(&source).unwrap_err().to_string();
            assert!(error.contains("design.dominant") && error.contains(target));
        }
    }

    fn labeled_element_source(label: Option<&str>) -> String {
        let label = label
            .map(|label| format!("label='{label}'\n"))
            .unwrap_or_default();
        format!(
            "[screen]\nid='x'\npurpose='x'\nroot='root'\n[[region]]\nid='navigation'\nrole='navigation'\nimportance='primary'\n[[region]]\nid='content'\nrole='primary_content'\nimportance='secondary'\n[[element]]\nid='internal_search_control'\nregion='navigation'\nkind='search'\nimportance='secondary'\n{label}[[composition]]\nid='root'\nkind='split'\nchildren=['navigation','content']"
        )
    }

    #[test]
    fn element_labels_require_authored_nonblank_text() {
        let missing = parse_and_resolve(&labeled_element_source(None))
            .unwrap_err()
            .to_string();
        assert!(missing.contains("element 'internal_search_control': label is required"));

        for blank in ["", "   "] {
            let error = parse_and_resolve(&labeled_element_source(Some(blank)))
                .unwrap_err()
                .to_string();
            assert!(
                error.contains("element 'internal_search_control'")
                    && error.contains("non-whitespace")
            );
        }
    }

    #[test]
    fn authored_element_label_is_preserved_exactly() {
        let blueprint = parse_and_resolve(&labeled_element_source(Some("Find projects…"))).unwrap();
        assert_eq!(blueprint.elements[0].label, "Find projects…");
    }

    fn purpose_source(purpose: &str) -> String {
        format!(
            "[screen]\nid='x'\npurpose='{purpose}'\nroot='root'\n[[region]]\nid='a'\nrole='navigation'\nimportance='primary'\n[[region]]\nid='b'\nrole='inspector'\nimportance='secondary'\n[[composition]]\nid='root'\nkind='split'\nchildren=['a','b']"
        )
    }

    #[test]
    fn screen_purpose_must_be_nonblank() {
        for purpose in ["", "   ", "\t", " \t "] {
            let error = parse_and_resolve(&purpose_source(purpose))
                .unwrap_err()
                .to_string();
            assert!(error.contains("screen.purpose") && error.contains("non-whitespace"));
        }
    }

    #[test]
    fn authored_screen_purpose_is_preserved_exactly() {
        let purpose = "  Read documents deliberately  ";
        let blueprint = parse_and_resolve(&purpose_source(purpose)).unwrap();
        assert_eq!(blueprint.screen.purpose, purpose);
    }

    #[test]
    fn missing_screen_purpose_remains_a_parse_error() {
        let source = "[screen]\nid='x'\nroot='root'\n[[region]]\nid='a'\nrole='navigation'\nimportance='primary'\n[[region]]\nid='b'\nrole='inspector'\nimportance='secondary'\n[[composition]]\nid='root'\nkind='split'\nchildren=['a','b']";
        let error = parse_and_resolve(source).unwrap_err().to_string();
        assert!(
            error.contains("missing field `purpose`") || error.contains("missing field 'purpose'")
        );
    }

    fn global_id_source(
        screen: &str,
        region: &str,
        element: &str,
        composition: &str,
        fixture: &str,
    ) -> String {
        format!(
            "[screen]\nid='{screen}'\npurpose='x'\nroot='root'\n[[region]]\nid='{region}'\nrole='navigation'\nimportance='primary'\n[[region]]\nid='other'\nrole='inspector'\nimportance='secondary'\n[[element]]\nid='{element}'\nregion='{region}'\nkind='text'\nimportance='primary'\nlabel='Text'\n[[composition]]\nid='{composition}'\nkind='split'\nchildren=['{region}','other']\n[[fixture]]\nid='{fixture}'\nstate='ready'"
        )
    }

    #[test]
    fn structural_global_ids_must_be_nonblank() {
        let cases = [
            ("screen.id", "", "screen"),
            ("screen.id", "   ", "screen"),
            ("region.id", "", "region"),
            ("region.id", "   ", "region"),
            ("element.id", "", "element"),
            ("element.id", "   ", "element"),
            ("composition.id", "", "composition"),
            ("composition.id", "   ", "composition"),
            ("fixture.id", "", "fixture"),
            ("fixture.id", "   ", "fixture"),
        ];
        for (context, value, field) in cases {
            let source = match field {
                "screen" => global_id_source(value, "region", "element", "root", "fixture"),
                "region" => global_id_source("screen", value, "element", "root", "fixture"),
                "element" => global_id_source("screen", "region", value, "root", "fixture"),
                "composition" => global_id_source("screen", "region", "element", value, "fixture"),
                "fixture" => global_id_source("screen", "region", "element", "root", value),
                _ => unreachable!(),
            };
            let error = parse_and_resolve(&source).unwrap_err().to_string();
            assert!(
                error.contains(context),
                "{context} was not diagnosed: {error}"
            );
        }
    }

    fn collection_id_source(id: &str) -> String {
        format!(
            "[screen]\nid='screen'\npurpose='x'\nroot='root'\n[[region]]\nid='content'\nrole='primary_content'\nimportance='primary'\n[[region]]\nid='other'\nrole='status'\nimportance='secondary'\n[[element]]\nid='collection'\nregion='content'\nkind='collection'\nimportance='primary'\nlabel='Items'\n[[composition]]\nid='root'\nkind='split'\nchildren=['content','other']\n[[fixture]]\nid='fixture'\nstate='ready'\n[[fixture.content]]\nelement='collection'\nitems=[{{id='{id}',label='X'}}]"
        )
    }

    #[test]
    fn collection_item_ids_must_be_nonblank() {
        for id in ["", "   "] {
            let error = parse_and_resolve(&collection_id_source(id))
                .unwrap_err()
                .to_string();
            assert!(error.contains("collection item id") && error.contains("non-whitespace"));
        }
    }

    fn collection_label_source(label: &str) -> String {
        collection_id_source("item").replace(
            "items=[{id='item',label='X'}]",
            &format!("items=[{{id='item',label='{label}'}}]"),
        )
    }

    #[test]
    fn collection_item_labels_must_be_nonblank() {
        for label in ["", "   ", "\t", " \t "] {
            let error = parse_and_resolve(&collection_label_source(label))
                .unwrap_err()
                .to_string();
            assert!(
                error.contains("fixture 'fixture'")
                    && error.contains("collection item 'item'")
                    && error.contains("label")
                    && error.contains("non-whitespace")
            );
        }
    }

    #[test]
    fn collection_item_labels_are_preserved_and_duplicates_are_allowed() {
        let source = collection_label_source("  Project A  ").replace(
            "items=[{id='item',label='  Project A  '}]",
            "items=[{id='item',label='  Project A  '},{id='second',label='  Project A  '}]",
        );
        let blueprint = parse_and_resolve(&source).unwrap();
        let items = match &blueprint.fixtures[0].content[0] {
            ResolvedFixtureContent::Collection { items, .. } => items,
            other => panic!("unexpected fixture content: {other:?}"),
        };
        assert_eq!(items[0].label, "  Project A  ");
        assert_eq!(items[1].label, "  Project A  ");
    }

    #[test]
    fn missing_collection_item_label_remains_a_parse_error() {
        let source = collection_id_source("item").replace("label='X'", "");
        let error = parse_and_resolve(&source).unwrap_err().to_string();
        assert!(error.contains("TOML parse error"));
    }

    fn tree_id_source(id: &str) -> String {
        format!(
            "[screen]\nid='screen'\npurpose='x'\nroot='root'\n[[region]]\nid='content'\nrole='primary_content'\nimportance='primary'\n[[region]]\nid='other'\nrole='status'\nimportance='secondary'\n[[element]]\nid='tree'\nregion='content'\nkind='tree'\nimportance='primary'\nlabel='Outline'\n[[composition]]\nid='root'\nkind='split'\nchildren=['content','other']\n[[fixture]]\nid='fixture'\nstate='ready'\n[[fixture.content]]\nelement='tree'\nnodes=[{{id='{id}',label='Node'}}]"
        )
    }

    #[test]
    fn tree_node_ids_must_be_nonblank() {
        for id in ["", "   "] {
            let error = parse_and_resolve(&tree_id_source(id))
                .unwrap_err()
                .to_string();
            assert!(error.contains("tree node id") && error.contains("non-whitespace"));
        }
    }

    fn tree_label_source(label: &str) -> String {
        tree_id_source("node").replace(
            "nodes=[{id='node',label='Node'}]",
            &format!("nodes=[{{id='node',label='{label}'}}]"),
        )
    }

    #[test]
    fn tree_node_labels_must_be_nonblank() {
        for label in ["", "   ", "\t", " \t "] {
            let error = parse_and_resolve(&tree_label_source(label))
                .unwrap_err()
                .to_string();
            assert!(
                error.contains("fixture 'fixture'")
                    && error.contains("tree node 'node'")
                    && error.contains("label")
                    && error.contains("non-whitespace")
            );
        }
    }

    #[test]
    fn tree_node_labels_are_preserved_and_duplicates_are_allowed() {
        let source = tree_label_source("  Chapter One  ").replace(
            "nodes=[{id='node',label='  Chapter One  '}]",
            "nodes=[{id='node',label='  Chapter One  '},{id='second',label='  Chapter One  '}]",
        );
        let blueprint = parse_and_resolve(&source).unwrap();
        let nodes = match &blueprint.fixtures[0].content[0] {
            ResolvedFixtureContent::Tree { nodes, .. } => nodes,
            other => panic!("unexpected fixture content: {other:?}"),
        };
        assert_eq!(nodes[0].label, "  Chapter One  ");
        assert_eq!(nodes[1].label, "  Chapter One  ");
    }

    #[test]
    fn missing_tree_node_label_remains_a_parse_error() {
        let source = tree_id_source("node").replace("label='Node'", "");
        let error = parse_and_resolve(&source).unwrap_err().to_string();
        assert!(error.contains("TOML parse error"));
    }

    fn fixture_state_source(state: &str) -> String {
        format!(
            "[screen]\nid='screen'\npurpose='x'\nroot='root'\n[[region]]\nid='a'\nrole='navigation'\nimportance='primary'\n[[region]]\nid='b'\nrole='inspector'\nimportance='secondary'\n[[composition]]\nid='root'\nkind='split'\nchildren=['a','b']\n[[fixture]]\nid='fixture'\nstate='{state}'"
        )
    }

    #[test]
    fn fixture_state_must_be_nonblank() {
        for state in ["", "   ", "\t", " \t "] {
            let error = parse_and_resolve(&fixture_state_source(state))
                .unwrap_err()
                .to_string();
            assert!(
                error.contains("fixture 'fixture'")
                    && error.contains("state")
                    && error.contains("non-whitespace")
            );
        }
    }

    #[test]
    fn authored_fixture_state_is_preserved_and_duplicate_states_are_allowed() {
        let state = "  loaded document  ";
        let source = fixture_state_source(state).replace(
            "[[fixture]]\nid='fixture'\nstate='  loaded document  '",
            "[[fixture]]\nid='fixture'\nstate='  loaded document  '\n[[fixture]]\nid='second'\nstate='  loaded document  '",
        );
        let blueprint = parse_and_resolve(&source).unwrap();
        assert_eq!(blueprint.fixtures[0].state, state);
        assert_eq!(blueprint.fixtures[1].state, state);
    }

    #[test]
    fn missing_fixture_state_remains_a_parse_error() {
        let source = fixture_state_source("ready").replace("\nstate='ready'", "");
        let error = parse_and_resolve(&source).unwrap_err().to_string();
        assert!(error.contains("missing field `state`") || error.contains("missing field 'state'"));
    }

    #[test]
    fn valid_structural_ids_and_root_are_preserved_exactly() {
        let source = global_id_source(
            " screen ",
            " reader panel ",
            "element with spaces",
            "root",
            "fixture",
        );
        let blueprint = parse_and_resolve(&source).unwrap();
        assert_eq!(blueprint.screen.id, " screen ");
        assert_eq!(blueprint.regions[0].id, " reader panel ");
        assert_eq!(blueprint.elements[0].id, "element with spaces");
        assert_eq!(blueprint.elements[0].region, " reader panel ");
        assert_eq!(blueprint.root, "root");
        assert_eq!(blueprint.fixtures[0].id, "fixture");
    }

    #[test]
    fn screen_id_collisions_with_reachable_regions_and_elements_are_rejected() {
        let region_source =
            global_id_source("workspace", "workspace", "element", "root", "fixture");
        let region_error = parse_and_resolve(&region_source).unwrap_err().to_string();
        assert!(region_error.contains("screen.id 'workspace'"));
        assert!(region_error.contains("region id 'workspace'"));
        assert!(region_error.contains("observable author-id namespace"));

        let element_source =
            global_id_source("workspace", "region", "workspace", "root", "fixture");
        let element_error = parse_and_resolve(&element_source).unwrap_err().to_string();
        assert!(element_error.contains("screen.id 'workspace'"));
        assert!(element_error.contains("element id 'workspace'"));
        assert!(element_error.contains("observable author-id namespace"));
    }

    #[test]
    fn screen_id_collisions_with_unused_regions_and_elements_are_rejected() {
        let unused_region = format!(
            "{}\n[[region]]\nid='workspace'\nrole='inspector'\nimportance='tertiary'\n",
            global_id_source("workspace", "region", "element", "root", "fixture")
        );
        let region_error = parse_and_resolve(&unused_region).unwrap_err().to_string();
        assert!(region_error.contains("screen.id 'workspace'"));
        assert!(region_error.contains("region id 'workspace'"));

        let unused_element = format!(
            "{}\n[[region]]\nid='unused_region'\nrole='inspector'\nimportance='tertiary'\n[[element]]\nid='workspace'\nregion='unused_region'\nkind='text'\nimportance='tertiary'\nlabel='Unused text'\n",
            global_id_source("workspace", "region", "element", "root", "fixture")
        );
        let element_error = parse_and_resolve(&unused_element).unwrap_err().to_string();
        assert!(element_error.contains("screen.id 'workspace'"));
        assert!(element_error.contains("element id 'workspace'"));
    }

    #[test]
    fn screen_composition_and_fixture_namespaces_remain_separate() {
        let same_composition = global_id_source("root", "region", "element", "root", "fixture");
        assert!(parse_and_resolve(&same_composition).is_ok());

        let same_fixture = global_id_source("fixture", "region", "element", "root", "fixture");
        assert!(parse_and_resolve(&same_fixture).is_ok());
    }

    #[test]
    fn local_collection_and_tree_ids_remain_separate_from_screen_identity() {
        assert!(parse_and_resolve(&collection_id_source("screen")).is_ok());
        assert!(parse_and_resolve(&tree_id_source("screen")).is_ok());
    }

    #[test]
    fn observable_namespace_equality_is_exact_and_authored_ids_are_preserved() {
        let case_distinct = global_id_source("Reader", "reader", "element", "root", "fixture");
        let blueprint = parse_and_resolve(&case_distinct).unwrap();
        assert_eq!(blueprint.screen.id, "Reader");
        assert_eq!(blueprint.regions[0].id, "reader");

        let composed = "cafe\u{301}";
        let unicode_distinct = global_id_source("café", composed, "element", "root", "fixture");
        let blueprint = parse_and_resolve(&unicode_distinct).unwrap();
        assert_eq!(blueprint.screen.id, "café");
        assert_eq!(blueprint.regions[0].id, composed);
        assert_ne!(blueprint.screen.id, blueprint.regions[0].id);
    }

    #[test]
    fn accepted_canonical_and_pressure_sources_resolve_without_identity_migration() {
        for source in [
            project(),
            reader(),
            include_str!("../../../specimens/reader-workspace-m5.toml"),
            include_str!("../../../specimens/reader-workspace-visual.toml"),
            include_str!("../../../specimens/reader-workspace-visual-m5.toml"),
            include_str!("../../../specimens/reader-workspace-visual-m7.toml"),
            include_str!("../../../specimens/reader-workspace-visual-crushed.toml"),
            include_str!("../../../specimens/dependency-workbench.toml"),
            include_str!("../../../specimens/density-pressure-comfortable.toml"),
            include_str!("../../../specimens/density-pressure-dense.toml"),
            include_str!("../../../specimens/overlay-command-palette-pressure.toml"),
            include_str!("../../../specimens/reader-overflow-pressure.toml"),
        ] {
            parse_and_resolve(source).unwrap();
        }
    }

    #[test]
    fn blank_composition_and_root_references_do_not_use_empty_sentinel() {
        let empty = "[screen]\nid='screen'\npurpose='x'\nroot=''\n[[region]]\nid='a'\nrole='navigation'\nimportance='primary'\n[[region]]\nid='b'\nrole='inspector'\nimportance='secondary'\n[[composition]]\nid=''\nkind='split'\nchildren=['a','b']";
        let error = parse_and_resolve(empty).unwrap_err().to_string();
        assert!(error.contains("composition.id") && error.contains("screen.root"));

        for root in ["", "   "] {
            let source = format!(
                "[screen]\nid='screen'\npurpose='x'\nroot='{root}'\n[[region]]\nid='a'\nrole='navigation'\nimportance='primary'\n[[region]]\nid='b'\nrole='inspector'\nimportance='secondary'\n[[composition]]\nid='root'\nkind='split'\nchildren=['a','b']"
            );
            let error = parse_and_resolve(&source).unwrap_err().to_string();
            assert!(error.contains("screen.root"));
        }
    }

    #[test]
    fn dominant_reachability_requires_a_validated_root() {
        let cases = [
            (
                "root='missing'\n",
                "existing_region",
                "region",
                "design.dominant: region 'existing_region' is not reachable",
            ),
            (
                "root='missing'\n",
                "existing_element",
                "element",
                "design.dominant: element 'existing_element' is not reachable",
            ),
            (
                "",
                "existing_region",
                "region",
                "design.dominant: region 'existing_region' is not reachable",
            ),
        ];

        for (root, dominant, kind, dependent_diagnostic) in cases {
            let element = if kind == "element" {
                "[[element]]\nid='existing_element'\nregion='existing_region'\nkind='text'\nimportance='primary'\nlabel='Existing element'\n"
            } else {
                ""
            };
            let source = format!(
                "[screen]\nid='x'\npurpose='x'\n{root}[design]\ndominant='{dominant}'\n[[region]]\nid='existing_region'\nrole='primary_content'\nimportance='primary'\n[[region]]\nid='other'\nrole='status'\nimportance='secondary'\n{element}[[composition]]\nid='workspace'\nkind='split'\nchildren=['existing_region','other']"
            );
            let error = parse_and_resolve(&source).unwrap_err().to_string();
            assert!(error.contains("screen.root"));
            assert!(!error.contains(dependent_diagnostic));
            assert!(!error.contains("screen.root ''"));
        }
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
                Some(Axis::Horizontal),
            ),
            (
                "split",
                Some("vertical"),
                CompositionKind::Split,
                Some(Axis::Vertical),
            ),
            (
                "split",
                None,
                CompositionKind::Split,
                Some(Axis::Horizontal),
            ),
            ("row", None, CompositionKind::Row, Some(Axis::Horizontal)),
            (
                "row",
                Some("horizontal"),
                CompositionKind::Row,
                Some(Axis::Horizontal),
            ),
            (
                "column",
                None,
                CompositionKind::Column,
                Some(Axis::Vertical),
            ),
            (
                "column",
                Some("vertical"),
                CompositionKind::Column,
                Some(Axis::Vertical),
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
        for kind in ["stack", "grid"] {
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
    fn overlay_resolves_as_axisless_two_layer_topology() {
        let blueprint = parse_and_resolve(include_str!(
            "../../../specimens/overlay-command-palette-pressure.toml"
        ))
        .unwrap();
        let overlay = blueprint
            .compositions
            .iter()
            .find(|composition| composition.id == "root_overlay")
            .unwrap();
        assert_eq!(overlay.kind, CompositionKind::Overlay);
        assert_eq!(overlay.axis, None);
        assert!(matches!(
            overlay.children.as_slice(),
            [CompositionChild::Composition(base), CompositionChild::Region(floating)]
                if base == "workspace" && floating == "palette_surface"
        ));
        let floating = blueprint
            .regions
            .iter()
            .find(|region| region.id == "palette_surface")
            .unwrap();
        assert_eq!(floating.width, Some(520));
        assert_eq!(floating.height, Some(300));
        assert_eq!(floating.grow, 0.0);
    }

    #[test]
    fn overlay_constraints_remain_narrow_and_explicit() {
        let pressure = include_str!("../../../specimens/overlay-command-palette-pressure.toml");
        let cases = [
            (
                pressure.replace(
                    "kind = \"overlay\"\nchildren",
                    "kind = \"overlay\"\naxis = \"horizontal\"\nchildren",
                ),
                "overlay does not support authored axis",
            ),
            (
                pressure.replace(
                    "kind = \"overlay\"\nchildren",
                    "kind = \"overlay\"\ngap = \"md\"\nchildren",
                ),
                "overlay does not support authored gap",
            ),
            (
                pressure.replace(
                    "children = [\"workspace\", \"palette_surface\"]",
                    "children = [\"workspace\"]",
                ),
                "overlay requires exactly two children",
            ),
            (
                pressure.replace(
                    "children = [\"workspace\", \"palette_surface\"]",
                    "children = [\"workspace\", \"palette_surface\", \"palette_surface\"]",
                ),
                "overlay requires exactly two children",
            ),
            (
                pressure.replace(
                    "children = [\"workspace\", \"palette_surface\"]",
                    "children = [\"navigation\", \"palette_surface\"]",
                ),
                "overlay child 0 must be a composition",
            ),
            (
                pressure.replace(
                    "children = [\"workspace\", \"palette_surface\"]",
                    "children = [\"workspace\", \"workspace\"]",
                ),
                "overlay child 1 must be a region",
            ),
            (
                pressure.replace("width = \"520px\"\n", ""),
                "overlay floating region 'palette_surface' requires fixed width",
            ),
            (
                pressure.replace("height = \"300px\"\n", ""),
                "overlay floating region 'palette_surface' requires fixed height",
            ),
            (
                pressure.replace(
                    "width = \"520px\"\nheight = \"300px\"",
                    "width = \"520px\"\ngrow = 1\nheight = \"300px\"",
                ),
                "overlay floating region 'palette_surface' cannot have positive grow",
            ),
        ];
        for (source, diagnostic) in cases {
            let error = parse_and_resolve(&source).unwrap_err().to_string();
            assert!(error.contains(diagnostic), "missing {diagnostic}: {error}");
        }
        let stack = pressure.replace("kind = \"overlay\"", "kind = \"stack\"");
        let error = parse_and_resolve(&stack).unwrap_err().to_string();
        assert!(error.contains("unknown composition kind 'stack'"));
    }

    #[test]
    fn overlay_projections_preserve_layers_without_fake_axis() {
        let blueprint = parse_and_resolve(include_str!(
            "../../../specimens/overlay-command-palette-pressure.toml"
        ))
        .unwrap();
        let semantic = blueprint.semantic_tree();
        assert!(semantic.contains("composition root_overlay (overlay, axisless)"));
        assert!(semantic.contains("base layer"));
        assert!(semantic.contains("floating region palette_surface"));
        assert!(!semantic.contains("root_overlay (overlay, horizontal)"));
    }

    fn overflow_source(value: Option<&str>) -> String {
        let overflow = value.map_or_else(String::new, |value| format!("overflow='{value}'\n"));
        format!(
            "[screen]\nid='x'\npurpose='x'\nroot='root'\n[[region]]\nid='content'\nrole='primary_content'\nimportance='primary'\n{overflow}[[region]]\nid='other'\nrole='status'\nimportance='secondary'\n[[composition]]\nid='root'\nkind='split'\nchildren=['content','other']"
        )
    }

    #[test]
    fn region_overflow_defaults_and_validates_as_typed_policy() {
        assert_eq!(
            parse_and_resolve(&overflow_source(None)).unwrap().regions[0].overflow,
            OverflowPolicy::Clip
        );
        assert_eq!(
            parse_and_resolve(&overflow_source(Some("clip")))
                .unwrap()
                .regions[0]
                .overflow,
            OverflowPolicy::Clip
        );
        assert_eq!(
            parse_and_resolve(&overflow_source(Some("scroll_y")))
                .unwrap()
                .regions[0]
                .overflow,
            OverflowPolicy::ScrollY
        );
        let scroll = parse_and_resolve(&overflow_source(Some("scroll_y"))).unwrap();
        assert!(scroll
            .semantic_tree()
            .contains("region content (role primary_content, overflow scroll_y)"));
        let error = parse_and_resolve(&overflow_source(Some("scroll")))
            .unwrap_err()
            .to_string();
        assert!(error.contains("region 'content'") && error.contains("unknown overflow 'scroll'"));
    }

    #[test]
    fn canonical_regions_without_overflow_remain_clip() {
        for source in [
            include_str!("../../../examples/project-browser.toml"),
            include_str!("../../../specimens/reader-workspace.toml"),
            include_str!("../../../specimens/reader-workspace-m5.toml"),
            include_str!("../../../specimens/reader-workspace-visual.toml"),
            include_str!("../../../specimens/reader-workspace-visual-m5.toml"),
            include_str!("../../../specimens/reader-workspace-visual-m7.toml"),
            include_str!("../../../specimens/dependency-workbench.toml"),
            include_str!("../../../specimens/density-pressure-comfortable.toml"),
            include_str!("../../../specimens/density-pressure-dense.toml"),
            include_str!("../../../specimens/overlay-command-palette-pressure.toml"),
        ] {
            let blueprint = parse_and_resolve(source).unwrap();
            assert!(blueprint
                .regions
                .iter()
                .all(|region| region.overflow == OverflowPolicy::Clip));
        }
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
    fn disconnected_composition_source(regions: &str, compositions: &str) -> String {
        format!(
            "[screen]\nid='x'\npurpose='x'\nroot='root'\n{regions}\n[[composition]]\nid='root'\nkind='split'\nchildren=['main','other']\n{compositions}"
        )
    }
    #[test]
    fn unreachable_two_node_composition_cycle_is_rejected() {
        let source = disconnected_composition_source(
            "[[region]]\nid='main'\nrole='primary_content'\nimportance='primary'\n[[region]]\nid='other'\nrole='status'\nimportance='secondary'\n[[region]]\nid='a'\nrole='navigation'\nimportance='tertiary'\n[[region]]\nid='b'\nrole='inspector'\nimportance='tertiary'",
            "[[composition]]\nid='orphan_a'\nkind='split'\nchildren=['orphan_b','a']\n[[composition]]\nid='orphan_b'\nkind='split'\nchildren=['orphan_a','b']",
        );
        let error = parse_and_resolve(&source).unwrap_err().to_string();
        assert!(error.contains("composition cycle detected") && error.contains("orphan_a"));
    }
    #[test]
    fn unreachable_self_cycle_is_rejected() {
        let source = disconnected_composition_source(
            "[[region]]\nid='main'\nrole='primary_content'\nimportance='primary'\n[[region]]\nid='other'\nrole='status'\nimportance='secondary'\n[[region]]\nid='unused'\nrole='inspector'\nimportance='tertiary'",
            "[[composition]]\nid='orphan'\nkind='split'\nchildren=['orphan','unused']",
        );
        let error = parse_and_resolve(&source).unwrap_err().to_string();
        assert!(error.contains("composition cycle detected") && error.contains("orphan"));
    }
    #[test]
    fn unreachable_longer_composition_cycle_is_rejected() {
        let source = disconnected_composition_source(
            "[[region]]\nid='main'\nrole='primary_content'\nimportance='primary'\n[[region]]\nid='other'\nrole='status'\nimportance='secondary'\n[[region]]\nid='a'\nrole='navigation'\nimportance='tertiary'\n[[region]]\nid='b'\nrole='inspector'\nimportance='tertiary'\n[[region]]\nid='c'\nrole='controls'\nimportance='tertiary'",
            "[[composition]]\nid='a_parent'\nkind='split'\nchildren=['b_parent','a']\n[[composition]]\nid='b_parent'\nkind='split'\nchildren=['c_parent','b']\n[[composition]]\nid='c_parent'\nkind='split'\nchildren=['a_parent','c']",
        );
        let error = parse_and_resolve(&source).unwrap_err().to_string();
        assert!(error.contains("composition cycle detected") && error.contains("a_parent"));
    }
    #[test]
    fn disconnected_acyclic_composition_subtree_remains_valid() {
        let source = disconnected_composition_source(
            "[[region]]\nid='main'\nrole='primary_content'\nimportance='primary'\n[[region]]\nid='other'\nrole='status'\nimportance='secondary'\n[[region]]\nid='unused_a'\nrole='navigation'\nimportance='tertiary'\n[[region]]\nid='unused_b'\nrole='inspector'\nimportance='tertiary'\n[[region]]\nid='unused_c'\nrole='controls'\nimportance='tertiary'",
            "[[composition]]\nid='unused_parent'\nkind='split'\nchildren=['unused_child','unused_a']\n[[composition]]\nid='unused_child'\nkind='split'\nchildren=['unused_b','unused_c']",
        );
        assert!(parse_and_resolve(&source).is_ok());
    }
    #[test]
    fn duplicate_region_sibling_is_rejected() {
        let e = parse_and_resolve("[screen]\nid='x'\npurpose='x'\nroot='root'\n[[region]]\nid='a'\nrole='navigation'\nimportance='primary'\n[[composition]]\nid='root'\nkind='split'\nchildren=['a','a']").unwrap_err().to_string();
        assert!(
            e.contains("composition 'root'") && e.contains("child 'a'") && e.contains("duplicated")
        );
    }
    #[test]
    fn duplicate_composition_sibling_is_rejected() {
        let e = parse_and_resolve("[screen]\nid='x'\npurpose='x'\nroot='root'\n[[region]]\nid='a'\nrole='navigation'\nimportance='primary'\n[[region]]\nid='b'\nrole='inspector'\nimportance='secondary'\n[[composition]]\nid='root'\nkind='split'\nchildren=['body','body']\n[[composition]]\nid='body'\nkind='split'\nchildren=['a','b']").unwrap_err().to_string();
        assert!(
            e.contains("composition 'root'")
                && e.contains("child 'body'")
                && e.contains("duplicated")
        );
    }
    #[test]
    fn region_multiple_parents_are_rejected() {
        let e = parse_and_resolve("[screen]\nid='x'\npurpose='x'\nroot='root'\n[[region]]\nid='shared_region'\nrole='navigation'\nimportance='primary'\n[[region]]\nid='a'\nrole='inspector'\nimportance='secondary'\n[[region]]\nid='b'\nrole='status'\nimportance='tertiary'\n[[composition]]\nid='root'\nkind='split'\nchildren=['parent_a','parent_b']\n[[composition]]\nid='parent_a'\nkind='split'\nchildren=['shared_region','a']\n[[composition]]\nid='parent_b'\nkind='split'\nchildren=['shared_region','b']").unwrap_err().to_string();
        assert!(e.contains("shared_region") && e.contains("parent_a") && e.contains("parent_b"));
    }
    #[test]
    fn composition_multiple_parents_are_rejected() {
        let e = parse_and_resolve("[screen]\nid='x'\npurpose='x'\nroot='root'\n[[region]]\nid='a'\nrole='navigation'\nimportance='primary'\n[[region]]\nid='b'\nrole='inspector'\nimportance='secondary'\n[[region]]\nid='c'\nrole='status'\nimportance='tertiary'\n[[region]]\nid='d'\nrole='primary_content'\nimportance='secondary'\n[[composition]]\nid='root'\nkind='split'\nchildren=['parent_a','parent_b']\n[[composition]]\nid='parent_a'\nkind='split'\nchildren=['shared_body','a']\n[[composition]]\nid='parent_b'\nkind='split'\nchildren=['shared_body','b']\n[[composition]]\nid='shared_body'\nkind='split'\nchildren=['c','d']").unwrap_err().to_string();
        assert!(e.contains("shared_body") && e.contains("parent_a") && e.contains("parent_b"));
    }
    #[test]
    fn root_as_child_is_rejected_globally() {
        let e = parse_and_resolve("[screen]\nid='x'\npurpose='x'\nroot='workspace'\n[[region]]\nid='a'\nrole='navigation'\nimportance='primary'\n[[region]]\nid='b'\nrole='inspector'\nimportance='secondary'\n[[region]]\nid='other'\nrole='status'\nimportance='tertiary'\n[[composition]]\nid='workspace'\nkind='split'\nchildren=['a','b']\n[[composition]]\nid='orphan'\nkind='split'\nchildren=['workspace','other']").unwrap_err().to_string();
        assert!(e.contains("root composition 'workspace'") && e.contains("composition 'orphan'"));
    }
    #[test]
    fn unused_declarations_without_ownership_conflicts_remain_allowed() {
        let source = "[screen]\nid='x'\npurpose='x'\nroot='root'\n[[region]]\nid='a'\nrole='navigation'\nimportance='primary'\n[[region]]\nid='b'\nrole='inspector'\nimportance='secondary'\n[[region]]\nid='unused_a'\nrole='status'\nimportance='tertiary'\n[[region]]\nid='unused_b'\nrole='primary_content'\nimportance='secondary'\n[[composition]]\nid='root'\nkind='split'\nchildren=['a','b']\n[[composition]]\nid='unused'\nkind='split'\nchildren=['unused_a','unused_b']";
        assert!(parse_and_resolve(source).is_ok());
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
        let base = "[screen]\nid='x'\npurpose='x'\nroot='root'\n[[region]]\nid='r'\nrole='primary_content'\nimportance='primary'\n[[region]]\nid='other'\nrole='status'\nimportance='secondary'\n[[composition]]\nid='root'\nkind='split'\nchildren=['r','other']\n[[element]]\nid='list'\nregion='r'\nkind='collection'\nimportance='primary'\n[[fixture]]\nid='f'\nstate='x'\n";
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

    fn fixture_content_reachability_source(root: Option<&str>, nested: bool) -> String {
        let root = root.map_or_else(String::new, |root| format!("root='{root}'\n"));
        let compositions = if nested {
            "[[composition]]\nid='root'\nkind='split'\nchildren=['nested','root_status']\n[[composition]]\nid='nested'\nkind='split'\nchildren=['content','nested_status']"
        } else {
            "[[composition]]\nid='root'\nkind='split'\nchildren=['content','root_status']"
        };
        format!(
            "[screen]\nid='x'\npurpose='x'\n{root}[[region]]\nid='content'\nrole='primary_content'\nimportance='primary'\n[[region]]\nid='root_status'\nrole='status'\nimportance='secondary'\n[[region]]\nid='nested_status'\nrole='status'\nimportance='tertiary'\n[[element]]\nid='target_status'\nregion='content'\nkind='status'\nimportance='primary'\nlabel='Target status'\n{compositions}\n[[fixture]]\nid='fixture'\nstate='ready'\n[[fixture.content]]\nelement='target_status'\ntext='Ready'"
        )
    }

    #[test]
    fn fixture_content_targets_direct_and_nested_reachable_regions() {
        for source in [
            fixture_content_reachability_source(Some("root"), false),
            fixture_content_reachability_source(Some("root"), true),
        ] {
            assert!(parse_and_resolve(&source).is_ok());
        }
    }

    #[test]
    fn fixture_content_targeting_unreachable_region_is_rejected() {
        let source = fixture_content_reachability_source(Some("root"), false)
            .replace("region='content'", "region='unused'")
            .replace(
                "[[element]]",
                "[[region]]\nid='unused'\nrole='inspector'\nimportance='tertiary'\n[[element]]",
            );
        let error = parse_and_resolve(&source).unwrap_err().to_string();
        assert!(error.contains("fixture 'fixture'"));
        assert!(error.contains("target 'target_status'"));
        assert!(error.contains("owning region 'unused'"));
        assert!(error.contains("outside screen.root 'root'"));
    }

    #[test]
    fn fixture_content_reachability_is_gated_by_valid_root() {
        for source in [
            fixture_content_reachability_source(Some("missing"), false),
            fixture_content_reachability_source(None, false),
        ] {
            let error = parse_and_resolve(&source).unwrap_err().to_string();
            assert!(error.contains("screen.root"));
            assert!(!error.contains("content target 'target_status'"));
            assert!(!error.contains("screen.root ''"));
        }
    }

    #[test]
    fn missing_fixture_content_element_remains_a_missing_element_error() {
        let source = fixture_content_reachability_source(Some("root"), false)
            .replace("element='target_status'", "element='does_not_exist'");
        let error = parse_and_resolve(&source).unwrap_err().to_string();
        assert!(error.contains("content references missing element 'does_not_exist'"));
        assert!(!error.contains("not reachable"));
    }

    #[test]
    fn unused_non_target_elements_and_fixture_omission_remain_legal() {
        let source = fixture_content_reachability_source(Some("root"), false)
            .replace(
                "[[fixture.content]]\nelement='target_status'\ntext='Ready'",
                "",
            )
            .replace(
                "[[element]]",
                "[[region]]\nid='unused'\nrole='inspector'\nimportance='tertiary'\n[[element]]",
            )
            .replace(
                "id='target_status'\nregion='content'",
                "id='unused_element'\nregion='unused'",
            );
        assert!(parse_and_resolve(&source).is_ok());
    }

    #[test]
    fn canonical_fixture_content_targets_remain_compatible() {
        for source in [
            include_str!("../../../examples/project-browser.toml"),
            include_str!("../../../specimens/reader-workspace.toml"),
            include_str!("../../../specimens/reader-workspace-m5.toml"),
            include_str!("../../../specimens/reader-workspace-visual.toml"),
            include_str!("../../../specimens/reader-workspace-visual-m5.toml"),
            include_str!("../../../specimens/reader-workspace-visual-m7.toml"),
            include_str!("../../../specimens/reader-workspace-visual-crushed.toml"),
            include_str!("../../../specimens/dependency-workbench.toml"),
            include_str!("../../../specimens/density-pressure-comfortable.toml"),
            include_str!("../../../specimens/density-pressure-dense.toml"),
        ] {
            parse_and_resolve(source).unwrap();
        }
    }

    fn property_source(properties: &str) -> String {
        format!(
            "[screen]\nid='x'\npurpose='x'\nroot='root'\n[[region]]\nid='content'\nrole='primary_content'\nimportance='primary'\n[[region]]\nid='other'\nrole='status'\nimportance='secondary'\n[[element]]\nid='details'\nregion='content'\nkind='property_sheet'\nimportance='primary'\nlabel='Details'\n[[composition]]\nid='root'\nkind='split'\nchildren=['content','other']\n[[fixture]]\nid='fixture'\nstate='ready'\n[[fixture.content]]\nelement='details'\nproperties={properties}"
        )
    }

    #[test]
    fn property_names_must_be_nonblank() {
        for name in ["", "   ", "\t", " \t "] {
            let quoted = format!("{name:?}");
            let properties = format!("[{{name={quoted},value='x'}}]");
            let error = parse_and_resolve(&property_source(&properties))
                .unwrap_err()
                .to_string();
            assert!(
                error.contains("fixture 'fixture'")
                    && error.contains("property[0]")
                    && error.contains("details")
                    && error.contains("name")
                    && error.contains("non-whitespace")
            );
        }
    }

    #[test]
    fn property_names_and_values_preserve_order_exactly_and_allow_duplicates() {
        let blueprint = parse_and_resolve(&property_source(
            "[{name='  Status Label  ',value=''}, {name='Status Label',value='   '}, {name='Status Label',value='Secondary'}]",
        ))
        .unwrap();
        let properties = match &blueprint.fixtures[0].content[0] {
            ResolvedFixtureContent::Properties { properties, .. } => properties,
            other => panic!("unexpected fixture content: {other:?}"),
        };
        assert_eq!(properties[0].name, "  Status Label  ");
        assert_eq!(properties[0].value, "");
        assert_eq!(properties[1].name, "Status Label");
        assert_eq!(properties[1].value, "   ");
        assert_eq!(properties[2].name, "Status Label");
        assert_eq!(properties[2].value, "Secondary");
    }

    #[test]
    fn empty_property_lists_remain_legal() {
        let blueprint = parse_and_resolve(&property_source("[]")).unwrap();
        assert!(matches!(
            &blueprint.fixtures[0].content[0],
            ResolvedFixtureContent::Properties { properties, .. } if properties.is_empty()
        ));
    }

    #[test]
    fn missing_property_name_remains_a_parse_error() {
        let source = property_source("[{value='x'}]");
        let error = parse_and_resolve(&source).unwrap_err().to_string();
        assert!(error.contains("TOML parse error"));
    }

    #[test]
    fn canonical_sources_resolve_with_nonblank_property_names() {
        for source in [
            project(),
            reader(),
            include_str!("../../../specimens/reader-workspace-visual.toml"),
            include_str!("../../../specimens/reader-workspace-visual-m7.toml"),
            include_str!("../../../specimens/dependency-workbench.toml"),
            include_str!("../../../specimens/density-pressure-comfortable.toml"),
            include_str!("../../../specimens/density-pressure-dense.toml"),
        ] {
            let parsed: SourceBlueprint = toml::from_str(source).unwrap();
            for fixture in &parsed.fixture {
                for content in &fixture.content {
                    if let Some(properties) = &content.properties {
                        assert!(properties
                            .iter()
                            .all(|property| !property.name.trim().is_empty()));
                    }
                }
            }
            parse_and_resolve(source).unwrap();
        }
    }

    fn document_source(title: &str, paragraphs: &str) -> String {
        let title = format!("{title:?}");
        format!(
            "[screen]\nid='x'\npurpose='x'\nroot='root'\n[[region]]\nid='content'\nrole='primary_content'\nimportance='primary'\n[[region]]\nid='other'\nrole='status'\nimportance='secondary'\n[[element]]\nid='document_surface'\nregion='content'\nkind='document'\nimportance='primary'\nlabel='Document'\n[[composition]]\nid='root'\nkind='split'\nchildren=['content','other']\n[[fixture]]\nid='fixture'\nstate='ready'\n[[fixture.content]]\nelement='document_surface'\ndocument={{title={title},paragraphs={paragraphs}}}"
        )
    }

    fn rich_document_source(blocks: &str, spoken: &str) -> String {
        format!(
            "[screen]\nid='x'\npurpose='x'\nroot='root'\n[[region]]\nid='content'\nrole='primary_content'\nimportance='primary'\n[[region]]\nid='other'\nrole='status'\nimportance='secondary'\n[[element]]\nid='document_surface'\nregion='content'\nkind='document'\nimportance='primary'\nlabel='Document'\n[[composition]]\nid='root'\nkind='split'\nchildren=['content','other']\n[[fixture]]\nid='fixture'\nstate='ready'\n[[fixture.content]]\nelement='document_surface'\ndocument={{blocks=[{blocks}]{spoken}}}"
        )
    }

    #[test]
    fn document_titles_must_be_nonblank() {
        for title in ["", "   ", "\t", " \t "] {
            let error = parse_and_resolve(&document_source(title, "['paragraph']"))
                .unwrap_err()
                .to_string();
            assert!(
                error.contains("fixture 'fixture'")
                    && error.contains("document_surface")
                    && error.contains("title")
                    && error.contains("non-whitespace")
            );
        }
    }

    #[test]
    fn document_title_and_paragraphs_preserve_exact_order_and_blank_content() {
        let blueprint = parse_and_resolve(&document_source(
            "  The Quiet Machine  ",
            "['first','','   ','third']",
        ))
        .unwrap();
        let (title, paragraphs) = match &blueprint.fixtures[0].content[0] {
            ResolvedFixtureContent::Document {
                document: ResolvedDocument::Legacy { title, paragraphs },
                ..
            } => (title, paragraphs),
            other => panic!("unexpected fixture content: {other:?}"),
        };
        assert_eq!(title, "  The Quiet Machine  ");
        assert_eq!(paragraphs, &["first", "", "   ", "third"]);
    }

    #[test]
    fn empty_document_paragraph_lists_remain_legal() {
        let blueprint = parse_and_resolve(&document_source("No document loaded", "[]")).unwrap();
        assert!(matches!(
            &blueprint.fixtures[0].content[0],
            ResolvedFixtureContent::Document {
                document: ResolvedDocument::Legacy { title, paragraphs },
                ..
            } if title == "No document loaded" && paragraphs.is_empty()
        ));
    }

    #[test]
    fn missing_document_title_remains_a_parse_error() {
        let source = document_source("No document loaded", "[]").replace(
            "document={title=\"No document loaded\",paragraphs=[]}",
            "document={paragraphs=[]}",
        );
        let error = parse_and_resolve(&source).unwrap_err().to_string();
        assert!(error.contains("TOML parse error"));
    }

    #[test]
    fn rich_document_blocks_resolve_with_unicode_scalar_spoken_range() {
        let blueprint = parse_and_resolve(&rich_document_source(
            "{id='eyebrow',kind='eyebrow',text='CHAPTER'},{id='heading',kind='heading',level=6,text='Café'},{id='p1',kind='paragraph',text='naïve café'}",
            ",spoken={block='p1',start=2,end=7}",
        ))
        .unwrap();
        let ResolvedFixtureContent::Document {
            document: ResolvedDocument::Rich { blocks, spoken },
            ..
        } = &blueprint.fixtures[0].content[0]
        else {
            panic!("expected rich document");
        };
        assert_eq!(blocks.len(), 3);
        assert_eq!(blocks[1].kind(), DocumentBlockKind::Heading);
        assert_eq!(spoken.as_ref().unwrap().start, 2);
        assert_eq!(spoken.as_ref().unwrap().end, 7);
    }

    #[test]
    fn rich_document_validation_rejects_invalid_block_shapes_and_spoken_ranges() {
        let cases = [
            ("", "", "at least one block"),
            ("{id='',kind='paragraph',text='x'}", "", "non-whitespace"),
            (
                "{id='x',kind='paragraph',text='x'},{id='x',kind='divider'}",
                "",
                "duplicate document block",
            ),
            ("{id='x',kind='unknown',text='x'}", "", "unsupported kind"),
            ("{id='x',kind='heading',text='x'}", "", "requires a level"),
            (
                "{id='x',kind='heading',level=7,text='x'}",
                "",
                "between 1 and 6",
            ),
            (
                "{id='x',kind='paragraph',level=1,text='x'}",
                "",
                "must not have a level",
            ),
            ("{id='x',kind='divider',text='x'}", "", "must not have text"),
            (
                "{id='x',kind='divider'}",
                ",spoken={block='x',start=0,end=1}",
                "textual document block",
            ),
            (
                "{id='x',kind='paragraph',text='abc'}",
                ",spoken={block='missing',start=0,end=1}",
                "missing document block",
            ),
            (
                "{id='x',kind='paragraph',text='abc'}",
                ",spoken={block='x',start=1,end=1}",
                "start < end",
            ),
            (
                "{id='x',kind='paragraph',text='abc'}",
                ",spoken={block='x',start=0,end=4}",
                "scalar length",
            ),
        ];
        for (blocks, spoken, expected) in cases {
            let error = parse_and_resolve(&rich_document_source(blocks, spoken))
                .unwrap_err()
                .to_string();
            assert!(error.contains(expected), "expected {expected:?}: {error}");
        }
    }

    #[test]
    fn document_forms_are_exclusive() {
        for source in [
            document_source("Only title", "[]").replace(
                "document={title=\"Only title\",paragraphs=[]}",
                "document={title=\"Only title\"}",
            ),
            document_source("Ignored", "['paragraph']").replace(
                "document={title=\"Ignored\",paragraphs=['paragraph']}",
                "document={paragraphs=['paragraph']}",
            ),
            rich_document_source("{id='x',kind='paragraph',text='x'}", "").replace(
                "document={blocks=[",
                "document={title='mixed',paragraphs=[],blocks=[",
            ),
        ] {
            assert!(
                parse_and_resolve(&source).is_err(),
                "source should be rejected: {source}"
            );
        }
    }

    #[test]
    fn canonical_sources_resolve_with_nonblank_document_titles() {
        for source in [
            reader(),
            include_str!("../../../specimens/reader-workspace-visual.toml"),
            include_str!("../../../specimens/reader-workspace-visual-m7.toml"),
        ] {
            let parsed: SourceBlueprint = toml::from_str(source).unwrap();
            for fixture in &parsed.fixture {
                for content in &fixture.content {
                    if let Some(DocumentSource::Legacy(document)) = &content.document {
                        assert!(!document.title.trim().is_empty());
                    }
                }
            }
            parse_and_resolve(source).unwrap();
        }
    }

    fn status_source(text: &str, with_content: bool) -> String {
        let content = if with_content {
            format!("[[fixture.content]]\nelement='workspace_status'\ntext={text:?}")
        } else {
            String::new()
        };
        format!(
            "[screen]\nid='x'\npurpose='x'\nroot='root'\n[[region]]\nid='content'\nrole='primary_content'\nimportance='primary'\n[[region]]\nid='other'\nrole='status'\nimportance='secondary'\n[[element]]\nid='workspace_status'\nregion='other'\nkind='status'\nimportance='secondary'\nlabel='Status'\n[[composition]]\nid='root'\nkind='split'\nchildren=['content','other']\n[[fixture]]\nid='fixture'\nstate='ready'\n{content}"
        )
    }

    #[test]
    fn status_text_payloads_must_be_nonblank() {
        for text in ["", "   ", "\t", " \t "] {
            let error = parse_and_resolve(&status_source(text, true))
                .unwrap_err()
                .to_string();
            assert!(
                error.contains("fixture 'fixture'")
                    && error.contains("workspace_status")
                    && error.contains("text")
                    && error.contains("non-whitespace")
            );
        }
    }

    #[test]
    fn status_text_is_preserved_exactly_and_omission_remains_legal() {
        let blueprint = parse_and_resolve(&status_source("  Ready · now  ", true)).unwrap();
        assert!(matches!(
            &blueprint.fixtures[0].content[0],
            ResolvedFixtureContent::Text { text, .. } if text == "  Ready · now  "
        ));

        let omitted = parse_and_resolve(&status_source("ignored", false)).unwrap();
        assert!(omitted.fixtures[0].content.is_empty());
    }

    #[test]
    fn canonical_sources_resolve_with_nonblank_status_text() {
        for source in [
            reader(),
            include_str!("../../../specimens/reader-workspace-visual.toml"),
            include_str!("../../../specimens/reader-workspace-visual-m7.toml"),
            include_str!("../../../specimens/dependency-workbench.toml"),
        ] {
            let parsed: SourceBlueprint = toml::from_str(source).unwrap();
            for fixture in &parsed.fixture {
                for content in &fixture.content {
                    if let Some(text) = &content.text {
                        assert!(!text.trim().is_empty());
                    }
                }
            }
            parse_and_resolve(source).unwrap();
        }
    }

    #[test]
    fn fixture_content_unknown_and_duplicate_items_are_rejected() {
        let base = "[screen]\nid='x'\npurpose='x'\nroot='root'\n[[region]]\nid='r'\nrole='primary_content'\nimportance='primary'\n[[region]]\nid='other'\nrole='status'\nimportance='secondary'\n[[composition]]\nid='root'\nkind='split'\nchildren=['r','other']\n[[element]]\nid='list'\nregion='r'\nkind='collection'\nimportance='primary'\n[[fixture]]\nid='f'\nstate='x'\n[[fixture.content]]\nelement='list'\nitems=[{id='a',label='A'},{id='a',label='A2'}]\n";
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
    fn color_tokens_source(tokens: &str) -> String {
        format!(
            "[screen]\nid='x'\npurpose='x'\nroot='root'\n[tokens.color]\n{tokens}\n[[region]]\nid='a'\nrole='navigation'\nimportance='primary'\n[[region]]\nid='b'\nrole='inspector'\nimportance='secondary'\n[[composition]]\nid='root'\nkind='split'\nchildren=['a','b']"
        )
    }

    fn token_maps_source(spacing: &str, corners: &str, color: &str) -> String {
        format!(
            "[screen]\nid='x'\npurpose='x'\nroot='root'\n[tokens.spacing]\n{spacing}\n[tokens.corners]\n{corners}\n[tokens.color]\n{color}\n[[region]]\nid='a'\nrole='navigation'\nimportance='primary'\n[[region]]\nid='b'\nrole='inspector'\nimportance='secondary'\n[[composition]]\nid='root'\nkind='split'\nchildren=['a','b']"
        )
    }

    #[test]
    fn blank_spacing_and_corner_token_names_are_rejected() {
        for name in ["", "   ", "\t", " \t "] {
            let spacing = format!("{name:?} = 8");
            let error = parse_and_resolve(&token_maps_source(
                &spacing,
                "valid = 4",
                "valid = '#123456'",
            ))
            .unwrap_err()
            .to_string();
            assert!(error.contains("tokens.spacing") && error.contains("non-whitespace"));

            let corners = format!("{name:?} = 4");
            let error = parse_and_resolve(&token_maps_source(
                "valid = 8",
                &corners,
                "valid = '#123456'",
            ))
            .unwrap_err()
            .to_string();
            assert!(error.contains("tokens.corners") && error.contains("non-whitespace"));
        }
    }

    #[test]
    fn blank_color_token_names_are_rejected_even_when_unused_and_without_visual() {
        for name in ["", "   ", "\t", " \t "] {
            let color = format!("{name:?} = '#FFFFFF'");
            let error = parse_and_resolve(&token_maps_source("valid = 8", "valid = 4", &color))
                .unwrap_err()
                .to_string();
            assert!(error.contains("tokens.color") && error.contains("non-whitespace"));
        }
    }

    #[test]
    fn invalid_color_name_and_value_report_both_diagnostics() {
        let error = parse_and_resolve(&token_maps_source(
            "valid = 8",
            "valid = 4",
            "\"\" = 'garbage'",
        ))
        .unwrap_err()
        .to_string();
        let name_offset = error.find("tokens.color: token name ''").unwrap();
        let value_offset = error.find("color token '': expected #RRGGBB").unwrap();
        assert!(name_offset < value_offset);
    }

    #[test]
    fn token_name_diagnostics_are_sorted_within_each_family() {
        let source = token_maps_source(
            "\"   \" = 8\n\"\" = 9",
            "\"   \" = 4\n\"\" = 5",
            "\"   \" = '#FFFFFF'\n\"\" = '#000000'",
        );
        let error = parse_and_resolve(&source).unwrap_err().to_string();
        assert!(
            error.find("tokens.spacing: token name ''").unwrap()
                < error.find("tokens.spacing: token name '   '").unwrap()
        );
        assert!(
            error.find("tokens.corners: token name ''").unwrap()
                < error.find("tokens.corners: token name '   '").unwrap()
        );
        assert!(
            error.find("tokens.color: token name ''").unwrap()
                < error.find("tokens.color: token name '   '").unwrap()
        );
    }

    #[test]
    fn valid_token_names_and_exact_references_are_preserved() {
        let source = "[screen]\nid='x'\npurpose='x'\nroot='root'\n[tokens.spacing]\n\"  custom gap  \" = 12\n[tokens.corners]\n\"  custom corner  \" = 6\n[tokens.color]\n\"  custom color  \" = '#123456'\n[[region]]\nid='a'\nrole='navigation'\nimportance='primary'\n[[region]]\nid='b'\nrole='inspector'\nimportance='secondary'\n[[composition]]\nid='root'\nkind='split'\nchildren=['a','b']\ngap='  custom gap  '\n";
        let blueprint = parse_and_resolve(source).unwrap();
        assert_eq!(blueprint.compositions[0].gap, 12);
        let mismatch = parse_and_resolve(&source.replace("'  custom gap  '", "'custom gap'"))
            .unwrap_err()
            .to_string();
        assert!(mismatch.contains("missing spacing token 'custom gap'"));
        let source_blueprint: SourceBlueprint = toml::from_str(source).unwrap();
        assert!(source_blueprint
            .tokens
            .spacing
            .contains_key("  custom gap  "));
        assert!(source_blueprint
            .tokens
            .corners
            .contains_key("  custom corner  "));
        assert!(source_blueprint
            .tokens
            .color
            .values
            .contains_key("  custom color  "));
    }

    #[test]
    fn valid_unused_spacing_corner_and_color_tokens_remain_legal() {
        let source = token_maps_source(
            "\"future spacing\" = 18",
            "\"future corner\" = 9",
            "\"future accent\" = '#123456'",
        );
        assert!(parse_and_resolve(&source).is_ok());
    }

    #[test]
    fn canonical_sources_resolve_with_nonblank_token_names() {
        for source in [
            project(),
            reader(),
            include_str!("../../../specimens/reader-workspace-visual.toml"),
            include_str!("../../../specimens/reader-workspace-visual-m7.toml"),
            include_str!("../../../specimens/dependency-workbench.toml"),
            include_str!("../../../specimens/density-pressure-comfortable.toml"),
            include_str!("../../../specimens/density-pressure-dense.toml"),
        ] {
            let parsed: SourceBlueprint = toml::from_str(source).unwrap();
            assert!(parsed
                .tokens
                .spacing
                .keys()
                .all(|name| !name.trim().is_empty()));
            assert!(parsed
                .tokens
                .corners
                .keys()
                .all(|name| !name.trim().is_empty()));
            assert!(parsed
                .tokens
                .color
                .values
                .keys()
                .all(|name| !name.trim().is_empty()));
            parse_and_resolve(source).unwrap();
        }
    }

    #[test]
    fn every_authored_color_token_is_validated_deterministically() {
        let invalid_with_visual = include_str!("../../../specimens/reader-workspace-visual.toml")
            .replace(
                "canvas = \"#262B33\"",
                "canvas = \"#262B33\"\nunused_bad = \"garbage\"",
            );
        let error = parse_and_resolve(&invalid_with_visual)
            .unwrap_err()
            .to_string();
        assert!(error.contains("color token 'unused_bad': expected #RRGGBB"));

        let invalid_without_visual = color_tokens_source("future = 'garbage'");
        let error = parse_and_resolve(&invalid_without_visual)
            .unwrap_err()
            .to_string();
        assert!(error.contains("color token 'future': expected #RRGGBB"));

        let valid_unused = color_tokens_source("future = '#123456'");
        assert!(parse_and_resolve(&valid_unused).is_ok());

        let ordered = color_tokens_source("z_bad = 'garbage'\na_bad = 'garbage'");
        let error = parse_and_resolve(&ordered).unwrap_err().to_string();
        assert!(error.find("a_bad").unwrap() < error.find("z_bad").unwrap());
    }

    #[test]
    fn referenced_malformed_and_missing_color_tokens_remain_rejected() {
        let malformed = include_str!("../../../specimens/reader-workspace-visual.toml")
            .replace("canvas = \"#262B33\"", "canvas = \"invalid\"");
        let error = parse_and_resolve(&malformed).unwrap_err().to_string();
        assert!(error.contains("color token 'canvas': expected #RRGGBB"));

        let missing = include_str!("../../../specimens/reader-workspace-visual.toml")
            .replace("canvas = \"canvas\"", "canvas = \"does_not_exist\"");
        let error = parse_and_resolve(&missing).unwrap_err().to_string();
        assert!(error.contains("visual.canvas: missing color token 'does_not_exist'"));
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
            ResolvedFixtureContent::Document {
                document: ResolvedDocument::Legacy { title, paragraphs },
                ..
            } if title == "The Quiet Machine" && paragraphs.len() == 3
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
            ResolvedFixtureContent::Document {
                document: ResolvedDocument::Legacy { title, paragraphs },
                ..
            } if title == "No document loaded" && paragraphs.is_empty()
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
        assert!(pressure.command_state("reading", "play_pause").enabled);
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
            parse_and_resolve(valid).is_ok(),
            "valid action fixture failed: {:?}",
            parse_and_resolve(valid).unwrap_err()
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
        let base = "[screen]\nid='x'\npurpose='x'\nroot='root'\n[[region]]\nid='r'\nrole='primary_content'\nimportance='primary'\n[[region]]\nid='other'\nrole='status'\nimportance='secondary'\n[[composition]]\nid='root'\nkind='split'\nchildren=['r','other']\n[[element]]\nid='tree'\nregion='r'\nkind='tree'\nimportance='primary'\n[[element]]\nid='document'\nregion='r'\nkind='document'\nimportance='primary'\n[[fixture]]\nid='f'\nstate='x'\n";
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

    fn furnishing_source() -> String {
        r#"
[screen]
id = "furnishing_root"
purpose = "Inspect nested furnishing layout"
root = "workspace"

[tokens.spacing]
sm = 8
md = 12

[[region]]
id = "reader"
role = "primary_content"
importance = "primary"
furnishing = "furnishing_root"

[[region]]
id = "other"
role = "status"
importance = "secondary"

[[element]]
id = "search"
region = "reader"
kind = "search"
importance = "secondary"
label = "Search"

[[element]]
id = "previous"
region = "reader"
kind = "command"
importance = "secondary"
label = "Previous"
action = "chapter.previous"

[[element]]
id = "chapter_status"
region = "reader"
kind = "status"
importance = "tertiary"
label = "Chapter position"

[[element]]
id = "chapter"
region = "reader"
kind = "document"
importance = "primary"
label = "Chapter"

[[composition]]
id = "workspace"
kind = "split"
children = ["reader", "other"]

[[furnishing]]
id = "furnishing_root"
kind = "column"
children = ["toolbar", "document_slot"]
gap = "md"
padding = "sm"

[[furnishing]]
id = "toolbar"
kind = "row"
children = ["search_slot", "actions_slot"]
height = "52px"
gap = "sm"

[[furnishing]]
id = "search_slot"
kind = "row"
children = ["search"]
grow = 1

[[furnishing]]
id = "actions_slot"
kind = "row"
children = ["previous", "chapter_status"]
width = "320px"
grow = 0.5
gap = "sm"

[[furnishing]]
id = "document_slot"
kind = "column"
children = ["chapter"]
grow = 1
overflow = "scroll_y"
"#
        .to_owned()
    }

    fn assert_furnishing_error(source: &str, expected: &str) {
        let error = parse_and_resolve(source).unwrap_err().to_string();
        assert!(
            error.contains(expected),
            "expected diagnostic {expected:?}, got:\n{error}"
        );
    }

    fn with_extra_furnishing(source: &str, block: &str) -> String {
        format!("{source}\n{block}")
    }

    #[test]
    fn nested_furnishing_resolves_typed_tree_and_covers_region_elements_once() {
        let blueprint = parse_and_resolve(&furnishing_source()).unwrap();
        assert_eq!(blueprint.screen.id, "furnishing_root");
        let reader = blueprint
            .regions
            .iter()
            .find(|region| region.id == "reader")
            .unwrap();
        assert_eq!(reader.furnishing.as_deref(), Some("furnishing_root"));
        assert_eq!(blueprint.furnishings.len(), 5);

        let root = blueprint
            .furnishings
            .iter()
            .find(|furnishing| furnishing.id == "furnishing_root")
            .unwrap();
        assert_eq!(root.kind, FurnishingKind::Column);
        assert_eq!(root.gap, 12);
        assert_eq!(root.padding, 8);
        assert!(root.width.is_none() && root.height.is_none());
        assert!(matches!(
            root.children.as_slice(),
            [FurnishingChild::Furnishing(toolbar), FurnishingChild::Furnishing(document)]
                if toolbar == "toolbar" && document == "document_slot"
        ));

        let actions = blueprint
            .furnishings
            .iter()
            .find(|furnishing| furnishing.id == "actions_slot")
            .unwrap();
        assert_eq!(actions.kind, FurnishingKind::Row);
        assert_eq!(actions.width, Some(320));
        assert_eq!(actions.grow, 0.5);
        assert_eq!(actions.gap, 8);
        let document = blueprint
            .furnishings
            .iter()
            .find(|furnishing| furnishing.id == "document_slot")
            .unwrap();
        assert_eq!(document.overflow, OverflowPolicy::ScrollY);
        assert!(matches!(
            document.children.as_slice(),
            [FurnishingChild::Element(chapter)] if chapter == "chapter"
        ));
    }

    #[test]
    fn furnished_lantern_leaf_specimen_resolves_without_a_fake_toolbar_region() {
        let blueprint = parse_and_resolve(include_str!(
            "../../../specimens/lantern-leaf-reader-furnished.toml"
        ))
        .unwrap();
        assert_eq!(blueprint.screen.id, "lantern_leaf_reader_furnished");
        assert!(blueprint.regions.iter().any(|region| region.id == "reader"));
        assert!(!blueprint
            .regions
            .iter()
            .any(|region| region.id == "reader_toolbar"));
        assert_eq!(blueprint.furnishings.len(), 15);
        for region in blueprint
            .regions
            .iter()
            .filter(|region| region.furnishing.is_some())
        {
            let root = region.furnishing.as_deref().unwrap();
            let root_node = blueprint
                .furnishings
                .iter()
                .find(|node| node.id == root)
                .unwrap();
            assert!(root_node.width.is_none());
            assert!(root_node.height.is_none());
            assert_eq!(root_node.grow, 0.0);
        }
        let reader = blueprint
            .regions
            .iter()
            .find(|region| region.id == "reader")
            .unwrap();
        assert_eq!(reader.furnishing.as_deref(), Some("reader_furnishing"));
        let fixtures = &blueprint.fixtures;
        assert!(fixtures.iter().any(|fixture| fixture.id == "reading"));
        let semantic = blueprint.semantic_tree();
        assert!(semantic.contains("region reader"));
        assert!(semantic.contains("furnishing reader_furnishing — column"));
        assert!(semantic.contains("furnishing toolbar_slot — row"));
        assert!(semantic.contains("furnishing document_slot — column"));
        assert!(semantic.contains("element chapter_document"));
    }

    #[test]
    fn furnishing_validation_rejects_missing_root_cycles_and_duplicate_children() {
        let source = furnishing_source();
        assert_furnishing_error(
            &source.replace(
                "furnishing = \"furnishing_root\"",
                "furnishing = \"missing_root\"",
            ),
            "missing furnishing root 'missing_root'",
        );
        assert_furnishing_error(
            &source.replace(
                "children = [\"search_slot\", \"actions_slot\"]",
                "children = [\"search_slot\", \"actions_slot\", \"toolbar\"]",
            ),
            "furnishing cycle detected",
        );
        assert_furnishing_error(
            &source.replace(
                "children = [\"search\"]",
                "children = [\"search\", \"search\"]",
            ),
            "child 'search' is duplicated",
        );
    }

    #[test]
    fn furnishing_validation_rejects_multiple_parents_and_mixed_children() {
        let source = furnishing_source();
        let two_furnishing_parents = with_extra_furnishing(
            &source,
            "[[furnishing]]\nid = \"extra\"\nkind = \"column\"\nchildren = [\"search_slot\"]\nheight = \"40px\"",
        );
        assert_furnishing_error(
            &two_furnishing_parents,
            "furnishing 'search_slot' has multiple furnishing parents",
        );

        let two_element_parents = with_extra_furnishing(
            &source,
            "[[furnishing]]\nid = \"extra\"\nkind = \"row\"\nchildren = [\"search\"]\nwidth = \"100px\"",
        );
        assert_furnishing_error(
            &two_element_parents,
            "element 'search' has multiple furnishing parents",
        );

        let mixed = source.replace(
            "children = [\"toolbar\", \"document_slot\"]",
            "children = [\"toolbar\", \"document_slot\", \"search\"]",
        );
        assert_furnishing_error(
            &mixed,
            "direct children must be either all furnishings or all elements",
        );
    }

    #[test]
    fn furnishing_validation_rejects_cross_region_elements_and_orphans() {
        let source = furnishing_source();
        assert_furnishing_error(
            &source.replace(
                "id = \"search\"\nregion = \"reader\"",
                "id = \"search\"\nregion = \"other\"",
            ),
            "contains element 'search' owned by region 'other'",
        );
        let orphan = source.replace("children = [\"search\"]", "children = []");
        assert_furnishing_error(&orphan, "element 'search' must be reachable exactly once");
    }

    #[test]
    fn furnishing_validation_rejects_bad_spacing_dimensions_growth_and_slots() {
        let source = furnishing_source();
        assert_furnishing_error(
            &source.replace(
                "gap = \"md\"\npadding = \"sm\"",
                "gap = \"absent\"\npadding = \"sm\"",
            ),
            "missing spacing token 'absent' for gap",
        );
        assert_furnishing_error(
            &source.replace("padding = \"sm\"", "padding = \"absent\""),
            "missing spacing token 'absent' for padding",
        );
        assert_furnishing_error(
            &source.replace("height = \"52px\"", "height = \"52\""),
            "furnishing height 'toolbar': size must be a pixel value",
        );
        assert_furnishing_error(
            &source.replace("width = \"320px\"", "width = \"wide\""),
            "furnishing width 'actions_slot': size must be a pixel value",
        );
        assert_furnishing_error(
            &source.replace("grow = 0.5", "grow = -1.0"),
            "furnishing 'actions_slot': grow must be finite and non-negative",
        );
        assert_furnishing_error(
            &source.replace("grow = 0.5", "grow = nan"),
            "furnishing 'actions_slot': grow must be finite and non-negative",
        );
        assert_furnishing_error(
            &source.replace("width = \"320px\"\ngrow = 0.5\n", ""),
            "child 'actions_slot': requires width or positive grow",
        );
        assert_furnishing_error(
            &source.replace(
                "id = \"furnishing_root\"\nkind = \"column\"",
                "id = \"furnishing_root\"\nkind = \"column\"\nwidth = \"300px\"",
            ),
            "region furnishing root must not author width, height, or grow",
        );
    }

    #[test]
    fn furnishing_validation_rejects_multiple_roots_unrooted_nodes_and_unknown_fields() {
        let source = furnishing_source();
        let multiple_roots = source.replace(
            "id = \"other\"\nrole = \"status\"",
            "id = \"other\"\nrole = \"status\"\nfurnishing = \"furnishing_root\"",
        );
        assert_furnishing_error(
            &multiple_roots,
            "furnishing root 'furnishing_root' is assigned to multiple regions",
        );
        let unrooted = with_extra_furnishing(
            &source,
            "[[furnishing]]\nid = \"unrooted\"\nkind = \"row\"\nchildren = []",
        );
        assert_furnishing_error(
            &unrooted,
            "furnishing 'unrooted': must be reachable from exactly one region furnishing root (found 0)",
        );
        assert_furnishing_error(
            &source.replace(
                "id = \"search_slot\"\nkind = \"row\"",
                "id = \"search_slot\"\nkind = \"row\"\nrole = \"commands\"",
            ),
            "unknown field `role`",
        );
    }

    #[test]
    fn m44_responsive_specimen_selects_exhaustive_authored_variants() {
        let blueprint = parse_and_resolve(include_str!(
            "../../../specimens/lantern-leaf-reader-responsive.toml"
        ))
        .expect("responsive specimen must resolve");
        let responsive = blueprint.responsive.as_ref().expect("responsive model");
        assert_eq!(responsive.variants.len(), 3);
        assert_eq!(blueprint.responsive_variant(1440.0).unwrap().id, "wide");
        assert_eq!(blueprint.responsive_variant(1100.0).unwrap().id, "compact");
        assert_eq!(blueprint.responsive_variant(800.0).unwrap().id, "narrow");
        assert_eq!(blueprint.active_root(1100.0), "workspace_compact");
        assert_eq!(blueprint.active_root(800.0), "workspace_narrow");
    }
}
