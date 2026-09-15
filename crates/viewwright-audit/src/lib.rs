use viewwright_model::{Color, ResolvedBlueprint, SurfaceRole};

/// Practical ViewWright lint heuristics, not a complete WCAG conformance claim.
pub const MIN_FOREGROUND_CONTRAST: f32 = 4.5;
/// A small surface contrast below this value is treated as visually compressed.
pub const MIN_SURFACE_SEPARATION: f32 = 1.15;
/// A palette must be both very dark and tightly clustered to trigger this warning.
pub const VERY_DARK_MAX_LUMINANCE: f32 = 0.08;
pub const COMPRESSED_STRUCTURAL_SPAN: f32 = 0.025;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PaletteLuminance {
    pub canvas: f32,
    pub panel: f32,
    pub raised: f32,
    pub text: f32,
    pub text_muted: f32,
    pub accent: f32,
    pub border: f32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ForegroundRole {
    Text,
    TextMuted,
    Accent,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ForegroundContrast {
    pub foreground: ForegroundRole,
    pub surface: SurfaceRole,
    pub ratio: f32,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SurfaceSeparation {
    pub first: SurfaceRole,
    pub second: SurfaceRole,
    pub luminance_difference: f32,
    pub ratio: f32,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum AuditFinding {
    WeakForegroundContrast {
        foreground: ForegroundRole,
        surface: SurfaceRole,
        ratio: f32,
    },
    WeakSurfaceSeparation {
        first: SurfaceRole,
        second: SurfaceRole,
        ratio: f32,
    },
    CompressedVeryDarkRange {
        minimum: f32,
        maximum: f32,
        span: f32,
    },
}

#[derive(Debug, Clone, PartialEq)]
pub struct VisualAudit {
    pub screen_id: String,
    pub luminance: PaletteLuminance,
    pub used_surfaces: Vec<SurfaceRole>,
    pub foreground_contrast: Vec<ForegroundContrast>,
    pub surface_separation: Vec<SurfaceSeparation>,
    pub findings: Vec<AuditFinding>,
}

pub fn audit(blueprint: &ResolvedBlueprint) -> Option<VisualAudit> {
    let visual = blueprint.visual.as_ref()?;
    let luminance = PaletteLuminance {
        canvas: relative_luminance(visual.palette.canvas),
        panel: relative_luminance(visual.palette.surface),
        raised: relative_luminance(visual.palette.surface_raised),
        text: relative_luminance(visual.palette.text),
        text_muted: relative_luminance(visual.palette.text_muted),
        accent: relative_luminance(visual.palette.accent),
        border: relative_luminance(visual.palette.border),
    };
    let used_surfaces = [SurfaceRole::Canvas, SurfaceRole::Panel, SurfaceRole::Raised]
        .into_iter()
        .filter(|surface| {
            blueprint
                .regions
                .iter()
                .any(|region| region.surface == *surface)
        })
        .collect::<Vec<_>>();
    let foregrounds = [
        (ForegroundRole::Text, luminance.text),
        (ForegroundRole::TextMuted, luminance.text_muted),
        (ForegroundRole::Accent, luminance.accent),
    ];
    let foreground_contrast = foregrounds
        .into_iter()
        .flat_map(|(foreground, value)| {
            used_surfaces.iter().map(move |surface| ForegroundContrast {
                foreground,
                surface: *surface,
                ratio: contrast_ratio(value, surface_luminance(luminance, *surface)),
            })
        })
        .collect::<Vec<_>>();
    let surface_pairs = [
        (SurfaceRole::Canvas, SurfaceRole::Panel),
        (SurfaceRole::Panel, SurfaceRole::Raised),
        (SurfaceRole::Canvas, SurfaceRole::Raised),
    ];
    let surface_separation = surface_pairs
        .into_iter()
        .filter(|(first, second)| used_surfaces.contains(first) && used_surfaces.contains(second))
        .map(|(first, second)| {
            let first_l = surface_luminance(luminance, first);
            let second_l = surface_luminance(luminance, second);
            SurfaceSeparation {
                first,
                second,
                luminance_difference: (first_l - second_l).abs(),
                ratio: contrast_ratio(first_l, second_l),
            }
        })
        .collect::<Vec<_>>();
    let mut findings = foreground_contrast
        .iter()
        .filter(|metric| metric.ratio < MIN_FOREGROUND_CONTRAST)
        .map(|metric| AuditFinding::WeakForegroundContrast {
            foreground: metric.foreground,
            surface: metric.surface,
            ratio: metric.ratio,
        })
        .collect::<Vec<_>>();
    findings.extend(
        surface_separation
            .iter()
            .filter(|metric| metric.ratio < MIN_SURFACE_SEPARATION)
            .map(|metric| AuditFinding::WeakSurfaceSeparation {
                first: metric.first,
                second: metric.second,
                ratio: metric.ratio,
            }),
    );
    let structural = used_surfaces
        .iter()
        .map(|surface| surface_luminance(luminance, *surface))
        .collect::<Vec<_>>();
    if let (Some(minimum), Some(maximum)) = (
        structural.iter().copied().reduce(f32::min),
        structural.iter().copied().reduce(f32::max),
    ) {
        let span = maximum - minimum;
        if maximum <= VERY_DARK_MAX_LUMINANCE && span <= COMPRESSED_STRUCTURAL_SPAN {
            findings.push(AuditFinding::CompressedVeryDarkRange {
                minimum,
                maximum,
                span,
            });
        }
    }
    Some(VisualAudit {
        screen_id: blueprint.screen.id.clone(),
        luminance,
        used_surfaces,
        foreground_contrast,
        surface_separation,
        findings,
    })
}

pub fn relative_luminance(color: Color) -> f32 {
    fn linear(channel: u8) -> f32 {
        let value = channel as f32 / 255.0;
        if value <= 0.04045 {
            value / 12.92
        } else {
            ((value + 0.055) / 1.055).powf(2.4)
        }
    }
    0.2126 * linear(color.r) + 0.7152 * linear(color.g) + 0.0722 * linear(color.b)
}

pub fn contrast_ratio(first: f32, second: f32) -> f32 {
    let (lighter, darker) = if first >= second {
        (first, second)
    } else {
        (second, first)
    };
    (lighter + 0.05) / (darker + 0.05)
}

fn surface_luminance(luminance: PaletteLuminance, surface: SurfaceRole) -> f32 {
    match surface {
        SurfaceRole::Canvas => luminance.canvas,
        SurfaceRole::Panel => luminance.panel,
        SurfaceRole::Raised => luminance.raised,
        SurfaceRole::Transparent => 0.0,
    }
}

fn foreground_name(role: ForegroundRole) -> &'static str {
    match role {
        ForegroundRole::Text => "text",
        ForegroundRole::TextMuted => "text_muted",
        ForegroundRole::Accent => "accent",
    }
}

fn surface_name(role: SurfaceRole) -> &'static str {
    match role {
        SurfaceRole::Canvas => "canvas",
        SurfaceRole::Panel => "panel",
        SurfaceRole::Raised => "raised",
        SurfaceRole::Transparent => "transparent",
    }
}

impl VisualAudit {
    pub fn summary(&self) -> String {
        let mut out = format!("Visual audit — {}\n\n", self.screen_id);
        out.push_str("surface luminance\n");
        for (role, value) in [
            ("canvas", self.luminance.canvas),
            ("panel", self.luminance.panel),
            ("raised", self.luminance.raised),
            ("text", self.luminance.text),
            ("text_muted", self.luminance.text_muted),
            ("accent", self.luminance.accent),
            ("border", self.luminance.border),
        ] {
            out.push_str(&format!("  {role:<10} {value:.4}\n"));
        }
        out.push_str("\nforeground contrast\n");
        for metric in &self.foreground_contrast {
            out.push_str(&format!(
                "  {}/{}      {:.2}:1\n",
                foreground_name(metric.foreground),
                surface_name(metric.surface),
                metric.ratio
            ));
        }
        out.push_str("\nsurface separation\n");
        for metric in &self.surface_separation {
            out.push_str(&format!(
                "  {}/{}      Δ{:.4}, {:.2}:1\n",
                surface_name(metric.first),
                surface_name(metric.second),
                metric.luminance_difference,
                metric.ratio
            ));
        }
        out.push_str("\nwarnings\n");
        if self.findings.is_empty() {
            out.push_str("  - none\n");
        } else {
            for finding in &self.findings {
                out.push_str("  - ");
                match finding {
                    AuditFinding::WeakForegroundContrast {
                        foreground,
                        surface,
                        ratio,
                    } => out.push_str(&format!(
                        "weak {}/{} foreground contrast ({ratio:.2}:1)",
                        foreground_name(*foreground),
                        surface_name(*surface)
                    )),
                    AuditFinding::WeakSurfaceSeparation { first, second, ratio } => out
                        .push_str(&format!(
                            "weak {}/{} surface separation ({ratio:.2}:1)",
                            surface_name(*first),
                            surface_name(*second)
                        )),
                    AuditFinding::CompressedVeryDarkRange {
                        minimum,
                        maximum,
                        span,
                    } => out.push_str(&format!(
                        "structural palette is very dark and compressed (min {minimum:.4}, max {maximum:.4}, span {span:.4})"
                    )),
                }
                out.push('\n');
            }
        }
        out
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use viewwright_model::parse_and_resolve;

    fn visual_reader() -> ResolvedBlueprint {
        parse_and_resolve(include_str!(
            "../../../specimens/reader-workspace-visual.toml"
        ))
        .unwrap()
    }

    fn crushed_reader() -> ResolvedBlueprint {
        parse_and_resolve(include_str!(
            "../../../specimens/reader-workspace-visual-crushed.toml"
        ))
        .unwrap()
    }

    #[test]
    fn srgb_luminance_and_contrast_are_standard() {
        assert_eq!(
            relative_luminance(Color {
                r: 0,
                g: 0,
                b: 0,
                a: 255
            }),
            0.0
        );
        assert!(
            (relative_luminance(Color {
                r: 255,
                g: 255,
                b: 255,
                a: 255
            }) - 1.0)
                .abs()
                < 1e-6
        );
        assert!(
            (relative_luminance(Color {
                r: 119,
                g: 119,
                b: 119,
                a: 255
            }) - 0.1845)
                .abs()
                < 0.001
        );
        assert!((contrast_ratio(1.0, 0.0) - 21.0).abs() < 1e-5);
        assert!((contrast_ratio(0.1845, 0.0) - 4.69).abs() < 0.02);
    }

    #[test]
    fn canonical_reader_reports_strong_foreground_and_accepted_surfaces() {
        let result = audit(&visual_reader()).unwrap();
        assert!(result.used_surfaces.contains(&SurfaceRole::Canvas));
        assert!(result.used_surfaces.contains(&SurfaceRole::Panel));
        assert!(result.used_surfaces.contains(&SurfaceRole::Raised));
        assert!(result.foreground_contrast.iter().any(|metric| {
            metric.foreground == ForegroundRole::Text
                && metric.surface == SurfaceRole::Canvas
                && metric.ratio > 4.5
        }));
        assert!(result
            .findings
            .iter()
            .all(|finding| !matches!(finding, AuditFinding::CompressedVeryDarkRange { .. })));
        assert!(result.luminance.canvas > 0.02);
    }

    #[test]
    fn crushed_regression_preserves_structural_detector_and_summary_is_stable() {
        let canonical = audit(&visual_reader()).unwrap();
        let crushed = audit(&crushed_reader()).unwrap();
        assert!(crushed
            .findings
            .iter()
            .any(|finding| matches!(finding, AuditFinding::CompressedVeryDarkRange { .. })));
        assert!(crushed.findings.iter().any(|finding| matches!(
            finding,
            AuditFinding::WeakSurfaceSeparation {
                first: SurfaceRole::Canvas,
                second: SurfaceRole::Panel,
                ..
            }
        )));
        assert!(canonical.findings.len() < crushed.findings.len());
        assert_eq!(crushed.summary(), crushed.summary());
    }

    #[test]
    fn unused_surfaces_and_non_visual_blueprints_are_handled_cleanly() {
        let source = "[screen]\nid='x'\npurpose='x'\nroot='root'\n[[region]]\nid='r'\nrole='primary_content'\nimportance='primary'\n[[composition]]\nid='root'\nkind='split'\nchildren=['r','r']";
        let blueprint = parse_and_resolve(source).unwrap();
        assert!(audit(&blueprint).is_none());
        let mut blueprint = visual_reader();
        blueprint
            .regions
            .retain(|region| region.surface != SurfaceRole::Raised);
        let result = audit(&blueprint).unwrap();
        assert!(!result.used_surfaces.contains(&SurfaceRole::Raised));
        assert!(result
            .surface_separation
            .iter()
            .all(|pair| pair.first != SurfaceRole::Panel || pair.second != SurfaceRole::Raised));
    }
}
