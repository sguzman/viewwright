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
pub struct SourceBlueprint {
    pub screen: ScreenSource,
    #[serde(default)]
    pub design: DesignSource,
    #[serde(default)]
    pub tokens: TokensSource,
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
pub struct ScreenSource {
    pub id: String,
    pub purpose: String,
    #[serde(default = "default_density")]
    pub density: String,
}
fn default_density() -> String {
    "comfortable".into()
}
#[derive(Debug, Clone, Default, Deserialize)]
pub struct DesignSource {
    #[serde(default)]
    pub character: Vec<String>,
    pub dominant: Option<String>,
    #[serde(default)]
    pub avoid: Vec<String>,
}
#[derive(Debug, Clone, Default, Deserialize)]
pub struct TokensSource {
    #[serde(default)]
    pub spacing: HashMap<String, u32>,
    #[serde(default)]
    pub corners: HashMap<String, u32>,
}
#[derive(Debug, Clone, Deserialize)]
pub struct RegionSource {
    pub id: String,
    pub role: String,
    pub importance: String,
    pub width: Option<String>,
    pub grow: Option<f32>,
}
#[derive(Debug, Clone, Deserialize)]
pub struct CompositionSource {
    pub id: String,
    pub kind: String,
    pub axis: Option<String>,
    pub children: Vec<String>,
    pub gap: Option<String>,
    pub padding: Option<String>,
}
#[derive(Debug, Clone, Deserialize)]
pub struct ElementSource {
    pub id: String,
    pub region: String,
    pub kind: String,
    pub importance: String,
    pub label: Option<String>,
    pub presentation: Option<String>,
}
#[derive(Debug, Clone, Deserialize)]
pub struct FixtureSource {
    pub id: String,
    pub state: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Importance {
    Primary,
    Secondary,
    Tertiary,
}
fn importance(s: &str, w: &str, e: &mut Vec<String>) -> Importance {
    match s {
        "primary" => Importance::Primary,
        "secondary" => Importance::Secondary,
        "tertiary" => Importance::Tertiary,
        _ => {
            e.push(format!("{w}: unknown importance '{s}'"));
            Importance::Tertiary
        }
    }
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
}
fn kind(s: &str, w: &str, e: &mut Vec<String>) -> ElementKind {
    match s {
        "text" => ElementKind::Text,
        "command" => ElementKind::Command,
        "search" => ElementKind::Search,
        "collection" => ElementKind::Collection,
        "property_sheet" => ElementKind::PropertySheet,
        "tree" => ElementKind::Tree,
        "preview" => ElementKind::Preview,
        "status" => ElementKind::Status,
        _ => {
            e.push(format!("{w}: unknown element kind '{s}'"));
            ElementKind::Text
        }
    }
}
#[derive(Debug, Clone)]
pub struct ResolvedRegion {
    pub id: String,
    pub role: String,
    pub importance: Importance,
    pub width: Option<u32>,
    pub grow: f32,
}
#[derive(Debug, Clone)]
pub struct ResolvedElement {
    pub id: String,
    pub region: String,
    pub kind: ElementKind,
    pub importance: Importance,
    pub label: String,
    pub presentation: Option<String>,
}
#[derive(Debug, Clone)]
pub struct ResolvedComposition {
    pub id: String,
    pub kind: String,
    pub axis: String,
    pub children: Vec<String>,
    pub gap: u32,
    pub padding: u32,
}
#[derive(Debug, Clone)]
pub struct ResolvedFixture {
    pub id: String,
    pub state: String,
}
#[derive(Debug, Clone)]
pub struct ResolvedBlueprint {
    pub screen: ScreenSource,
    pub design: DesignSource,
    pub regions: Vec<ResolvedRegion>,
    pub compositions: Vec<ResolvedComposition>,
    pub elements: Vec<ResolvedElement>,
    pub fixtures: Vec<ResolvedFixture>,
}

pub fn parse_and_resolve(s: &str) -> Result<ResolvedBlueprint, BlueprintError> {
    resolve(toml::from_str(s)?)
}
fn add_id(ids: &mut HashSet<String>, errors: &mut Vec<String>, id: &str, kind: &str) {
    if !ids.insert(id.to_owned()) {
        errors.push(format!("duplicate id '{id}' ({kind})"));
    }
}
pub fn resolve(s: SourceBlueprint) -> Result<ResolvedBlueprint, BlueprintError> {
    let mut errs = Vec::new();
    let mut ids = HashSet::new();
    let spacing = &s.tokens.spacing;
    let regions = s
        .region
        .iter()
        .map(|r| {
            add_id(&mut ids, &mut errs, &r.id, "region");
            let width = r
                .width
                .as_deref()
                .and_then(|w| w.strip_suffix("px").and_then(|n| n.parse().ok()));
            if r.width.is_some() && width.is_none() {
                errs.push(format!("region '{}': width must be pixels", r.id));
            }
            ResolvedRegion {
                id: r.id.clone(),
                role: r.role.clone(),
                importance: importance(&r.importance, &format!("region '{}'", r.id), &mut errs),
                width,
                grow: r.grow.unwrap_or(0.0),
            }
        })
        .collect::<Vec<_>>();
    let rids: HashSet<_> = regions.iter().map(|r| r.id.as_str()).collect();
    let elements = s
        .element
        .iter()
        .map(|x| {
            add_id(&mut ids, &mut errs, &x.id, "element");
            if !rids.contains(x.region.as_str()) {
                errs.push(format!("element '{}': missing region '{}'", x.id, x.region));
            }
            ResolvedElement {
                id: x.id.clone(),
                region: x.region.clone(),
                kind: kind(&x.kind, &format!("element '{}'", x.id), &mut errs),
                importance: importance(&x.importance, &format!("element '{}'", x.id), &mut errs),
                label: x.label.clone().unwrap_or_else(|| x.id.replace('_', " ")),
                presentation: x.presentation.clone(),
            }
        })
        .collect::<Vec<_>>();
    let eids: HashSet<_> = elements.iter().map(|x| x.id.as_str()).collect();
    let compositions = s
        .composition
        .iter()
        .map(|c| {
            add_id(&mut ids, &mut errs, &c.id, "composition");
            if c.kind != "row"
                && c.kind != "column"
                && c.kind != "stack"
                && c.kind != "split"
                && c.kind != "overlay"
            {
                errs.push(format!(
                    "composition '{}': unknown composition kind '{}'",
                    c.id, c.kind
                ));
            }
            if let Some(axis) = &c.axis {
                if axis != "horizontal" && axis != "vertical" {
                    errs.push(format!(
                        "composition '{}': axis must be horizontal or vertical",
                        c.id
                    ));
                }
            }
            for child in &c.children {
                if !rids.contains(child.as_str()) && !eids.contains(child.as_str()) {
                    errs.push(format!(
                        "composition '{}': missing child reference '{}'",
                        c.id, child
                    ));
                }
            }
            let gap = c
                .gap
                .as_deref()
                .and_then(|g| spacing.get(g))
                .copied()
                .unwrap_or(0);
            if c.gap.is_some() && !spacing.contains_key(c.gap.as_ref().unwrap()) {
                errs.push(format!("composition '{}': missing spacing token", c.id));
            }
            let padding = c
                .padding
                .as_deref()
                .and_then(|g| spacing.get(g))
                .copied()
                .unwrap_or(0);
            ResolvedComposition {
                id: c.id.clone(),
                kind: c.kind.clone(),
                axis: c.axis.clone().unwrap_or_else(|| "horizontal".into()),
                children: c.children.clone(),
                gap,
                padding,
            }
        })
        .collect::<Vec<_>>();
    for c in &compositions {
        if c.children.len() < 2 {
            errs.push(format!("composition '{}': requires two children", c.id));
        }
    }
    let fixtures = s
        .fixture
        .iter()
        .map(|f| {
            add_id(&mut ids, &mut errs, &f.id, "fixture");
            ResolvedFixture {
                id: f.id.clone(),
                state: f.state.clone(),
            }
        })
        .collect();
    if let Some(d) = &s.design.dominant {
        if !rids.contains(d.as_str()) && !eids.contains(d.as_str()) {
            errs.push(format!("design.dominant: missing reference '{d}'"));
        }
    }
    if !errs.is_empty() {
        return Err(BlueprintError::Validation(errs.join("\n")));
    }
    Ok(ResolvedBlueprint {
        screen: s.screen,
        design: s.design,
        regions,
        compositions,
        elements,
        fixtures,
    })
}
impl ResolvedBlueprint {
    pub fn semantic_tree(&self) -> String {
        let mut o = format!("screen {} — {}\n", self.screen.id, self.screen.purpose);
        for r in &self.regions {
            o.push_str(&format!(
                "  region {} [{}] {:?}\n",
                r.id, r.role, r.importance
            ));
            for e in self.elements.iter().filter(|e| e.region == r.id) {
                o.push_str(&format!("    element {} ({:?})\n", e.id, e.kind));
            }
        }
        for f in &self.fixtures {
            o.push_str(&format!("  fixture {} [{}]\n", f.id, f.state));
        }
        o
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn specimen() {
        let b = parse_and_resolve(include_str!("../../../examples/project-browser.toml")).unwrap();
        assert_eq!(b.compositions[0].gap, 12);
    }
    #[test]
    fn bad_ref() {
        let e=parse_and_resolve("[screen]\nid='x'\npurpose='x'\n[[element]]\nid='a'\nregion='nope'\nkind='search'\nimportance='secondary'").unwrap_err().to_string();
        assert!(e.contains("missing region"));
    }

    #[test]
    fn duplicate_and_missing_token_are_reported_together() {
        let e = parse_and_resolve(
            "[screen]\nid='x'\npurpose='x'\n\
             [[region]]\nid='same'\nrole='navigation'\nimportance='secondary'\n\
             [[region]]\nid='same'\nrole='inspector'\nimportance='secondary'\n\
             [[composition]]\nid='root'\nkind='split'\nchildren=['same','nope']\ngap='md'",
        )
        .unwrap_err()
        .to_string();
        assert!(e.contains("duplicate id 'same'"));
        assert!(e.contains("missing child reference 'nope'"));
        assert!(e.contains("missing spacing token"));
    }
}
