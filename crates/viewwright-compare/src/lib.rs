//! Strict comparison of intended ViewWright expectations with observed ViewWitness data.

use std::collections::HashMap;

use thiserror::Error;
use viewwitness::{Node, Rect, Witness};
use viewwright_expectation::{
    Bounds, ElementExpectation, RegionExpectation, ViewWrightExpectation,
};

/// The only ViewWitness wire/model version whose semantics M35 supports.
pub const SUPPORTED_VIEWWITNESS_VERSION: &str = "0.1";

/// Identifies the kind of expected object whose authored ID was checked.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExpectedObjectKind {
    Screen,
    Region,
    Element,
}

/// An observed fact contradicts or fails an exact expected fact.
#[derive(Debug, Clone, PartialEq)]
pub enum Mismatch {
    ViewportWidth {
        expected: f32,
        observed: f32,
    },
    ViewportHeight {
        expected: f32,
        observed: f32,
    },
    MissingExpectedAuthorId {
        object: ExpectedObjectKind,
        author_id: String,
    },
    AmbiguousObservedAuthorId {
        object: ExpectedObjectKind,
        author_id: String,
        observed_count: usize,
    },
    RegionBounds {
        region_author_id: String,
        expected: Bounds,
        observed: Rect,
    },
    ElementOwner {
        element_author_id: String,
        expected_region_author_id: String,
        observed_region_author_id: String,
    },
}

/// The witness lacks enough evidence to evaluate an otherwise comparable fact.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EvidenceGap {
    RegionBoundsUnavailable { region_author_id: String },
    ElementParentUnavailable { element_author_id: String },
    ElementParentIdentityUnavailable { element_author_id: String },
    ElementParentAuthorIdUnavailable { element_author_id: String },
}

/// A malformed or unsupported witness cannot be compared truthfully.
#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum ComparisonError {
    #[error("unsupported ViewWitness version '{found}'; expected '{supported}'")]
    UnsupportedViewWitnessVersion {
        found: String,
        supported: &'static str,
    },
    #[error("ViewWitness validation failed: {issues:?}")]
    InvalidWitness { issues: Vec<String> },
}

/// Ordered findings from one expectation/witness comparison.
#[derive(Debug, Clone, PartialEq)]
pub struct ComparisonReport {
    pub mismatches: Vec<Mismatch>,
    pub evidence_gaps: Vec<EvidenceGap>,
}

impl ComparisonReport {
    /// True only when the witness exactly satisfies every comparable fact and supplies all evidence.
    #[must_use]
    pub fn is_exact_match(&self) -> bool {
        self.mismatches.is_empty() && self.evidence_gaps.is_empty()
    }
}

/// Compares only exact shared evidence represented by the M33 and ViewWitness 0.1 models.
pub fn compare(
    expectation: &ViewWrightExpectation,
    witness: &Witness,
) -> Result<ComparisonReport, ComparisonError> {
    if witness.viewwitness_version != SUPPORTED_VIEWWITNESS_VERSION {
        return Err(ComparisonError::UnsupportedViewWitnessVersion {
            found: witness.viewwitness_version.clone(),
            supported: SUPPORTED_VIEWWITNESS_VERSION,
        });
    }

    let issues = witness.validation_issues();
    if !issues.is_empty() {
        return Err(ComparisonError::InvalidWitness { issues });
    }

    let author_nodes = author_id_index(&witness.nodes);
    let nodes_by_id = witness
        .nodes
        .iter()
        .map(|node| (node.id.as_str(), node))
        .collect::<HashMap<_, _>>();
    let mut report = ComparisonReport {
        mismatches: Vec::new(),
        evidence_gaps: Vec::new(),
    };

    if expectation.viewport.width != witness.capture.viewport.width {
        report.mismatches.push(Mismatch::ViewportWidth {
            expected: expectation.viewport.width,
            observed: witness.capture.viewport.width,
        });
    }
    if expectation.viewport.height != witness.capture.viewport.height {
        report.mismatches.push(Mismatch::ViewportHeight {
            expected: expectation.viewport.height,
            observed: witness.capture.viewport.height,
        });
    }

    let _screen_node = match match_author_id(
        &mut report.mismatches,
        &author_nodes,
        ExpectedObjectKind::Screen,
        &expectation.screen.author_id,
    ) {
        IdentityMatch::Unique(node) => Some(node),
        IdentityMatch::Missing | IdentityMatch::Ambiguous => None,
    };

    for region in &expectation.regions {
        let IdentityMatch::Unique(node) = match_author_id(
            &mut report.mismatches,
            &author_nodes,
            ExpectedObjectKind::Region,
            &region.author_id,
        ) else {
            continue;
        };

        compare_region_bounds(region, node, &mut report);
    }

    for element in &expectation.elements {
        let IdentityMatch::Unique(node) = match_author_id(
            &mut report.mismatches,
            &author_nodes,
            ExpectedObjectKind::Element,
            &element.author_id,
        ) else {
            continue;
        };

        compare_element_owner(element, node, &nodes_by_id, &mut report);
    }

    Ok(report)
}

fn author_id_index<'a>(nodes: &'a [Node]) -> HashMap<&'a str, Vec<&'a Node>> {
    let mut index: HashMap<&str, Vec<&Node>> = HashMap::new();
    for node in nodes {
        if let Some(author_id) = node
            .identity
            .as_ref()
            .and_then(|identity| identity.author_id.as_deref())
        {
            index.entry(author_id).or_default().push(node);
        }
    }
    index
}

enum IdentityMatch<'a> {
    Missing,
    Ambiguous,
    Unique(&'a Node),
}

fn match_author_id<'a>(
    mismatches: &mut Vec<Mismatch>,
    index: &'a HashMap<&str, Vec<&'a Node>>,
    object: ExpectedObjectKind,
    author_id: &str,
) -> IdentityMatch<'a> {
    match index.get(author_id).map(Vec::as_slice).unwrap_or_default() {
        [] => {
            mismatches.push(Mismatch::MissingExpectedAuthorId {
                object,
                author_id: author_id.to_owned(),
            });
            IdentityMatch::Missing
        }
        [node] => IdentityMatch::Unique(node),
        matches => {
            mismatches.push(Mismatch::AmbiguousObservedAuthorId {
                object,
                author_id: author_id.to_owned(),
                observed_count: matches.len(),
            });
            IdentityMatch::Ambiguous
        }
    }
}

fn compare_region_bounds(region: &RegionExpectation, node: &Node, report: &mut ComparisonReport) {
    let Some(observed) = node.bounds else {
        report
            .evidence_gaps
            .push(EvidenceGap::RegionBoundsUnavailable {
                region_author_id: region.author_id.clone(),
            });
        return;
    };

    if !rect_matches(region.bounds, observed) {
        report.mismatches.push(Mismatch::RegionBounds {
            region_author_id: region.author_id.clone(),
            expected: region.bounds,
            observed,
        });
    }
}

fn rect_matches(expected: Bounds, observed: Rect) -> bool {
    expected.x == observed.x
        && expected.y == observed.y
        && expected.width == observed.width
        && expected.height == observed.height
}

fn compare_element_owner(
    element: &ElementExpectation,
    node: &Node,
    nodes_by_id: &HashMap<&str, &Node>,
    report: &mut ComparisonReport,
) {
    let Some(parent_id) = node.parent.as_deref() else {
        report
            .evidence_gaps
            .push(EvidenceGap::ElementParentUnavailable {
                element_author_id: element.author_id.clone(),
            });
        return;
    };
    let parent = nodes_by_id
        .get(parent_id)
        .expect("witness validation guarantees referenced parent exists");
    let Some(identity) = parent.identity.as_ref() else {
        report
            .evidence_gaps
            .push(EvidenceGap::ElementParentIdentityUnavailable {
                element_author_id: element.author_id.clone(),
            });
        return;
    };
    let Some(observed_region_author_id) = identity.author_id.as_deref() else {
        report
            .evidence_gaps
            .push(EvidenceGap::ElementParentAuthorIdUnavailable {
                element_author_id: element.author_id.clone(),
            });
        return;
    };

    if observed_region_author_id != element.region_author_id {
        report.mismatches.push(Mismatch::ElementOwner {
            element_author_id: element.author_id.clone(),
            expected_region_author_id: element.region_author_id.clone(),
            observed_region_author_id: observed_region_author_id.to_owned(),
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use viewwitness::{Capture, NodeIdentity, Viewport};
    use viewwright_expectation::{build_expectation, LogicalViewport};
    use viewwright_model::parse_and_resolve;

    fn expectation() -> ViewWrightExpectation {
        let blueprint =
            parse_and_resolve(include_str!("../../../examples/project-browser.toml")).unwrap();
        build_expectation(&blueprint, 1440.0, 900.0).unwrap()
    }

    fn identity(author_id: Option<&str>) -> Option<NodeIdentity> {
        author_id.map(|author_id| NodeIdentity {
            provenance: "synthetic_test".into(),
            stability: "stable_for_test".into(),
            author_id: Some(author_id.into()),
        })
    }

    fn node(
        id: impl Into<String>,
        author_id: Option<&str>,
        parent: Option<&str>,
        bounds: Option<Rect>,
    ) -> Node {
        Node {
            id: id.into(),
            role: "generic_container".into(),
            parent: parent.map(str::to_owned),
            identity: identity(author_id),
            name: None,
            bounds,
            visible: None,
            enabled: None,
            focused: None,
            selected: None,
            text: None,
            value: None,
            actions: Vec::new(),
            properties: Default::default(),
        }
    }

    fn witness(expectation: &ViewWrightExpectation) -> Witness {
        let mut nodes = vec![node(
            "screen-node",
            Some(&expectation.screen.author_id),
            None,
            None,
        )];
        for (index, region) in expectation.regions.iter().enumerate() {
            nodes.push(node(
                format!("region-node-{index}"),
                Some(&region.author_id),
                Some("screen-node"),
                Some(Rect {
                    x: region.bounds.x,
                    y: region.bounds.y,
                    width: region.bounds.width,
                    height: region.bounds.height,
                }),
            ));
        }
        for (index, element) in expectation.elements.iter().enumerate() {
            let parent = expectation
                .regions
                .iter()
                .position(|region| region.author_id == element.region_author_id)
                .map(|i| format!("region-node-{i}"))
                .expect("expected element owner exists");
            nodes.push(node(
                format!("element-node-{index}"),
                Some(&element.author_id),
                Some(&parent),
                None,
            ));
        }
        Witness {
            viewwitness_version: SUPPORTED_VIEWWITNESS_VERSION.into(),
            capture: Capture {
                source: "synthetic".into(),
                frame: None,
                viewport: Viewport {
                    width: expectation.viewport.width,
                    height: expectation.viewport.height,
                    scale_factor: 1.0,
                },
                metadata: Default::default(),
            },
            nodes,
            relations: Vec::new(),
        }
    }

    fn node_with_author_id<'a>(witness: &'a Witness, author_id: &str) -> &'a Node {
        witness
            .nodes
            .iter()
            .find(|node| {
                node.identity
                    .as_ref()
                    .and_then(|identity| identity.author_id.as_deref())
                    == Some(author_id)
            })
            .unwrap()
    }

    fn node_with_author_id_mut<'a>(witness: &'a mut Witness, author_id: &str) -> &'a mut Node {
        witness
            .nodes
            .iter_mut()
            .find(|node| {
                node.identity
                    .as_ref()
                    .and_then(|identity| identity.author_id.as_deref())
                    == Some(author_id)
            })
            .unwrap()
    }

    fn remove_author_id(witness: &mut Witness, author_id: &str) {
        node_with_author_id_mut(witness, author_id).identity = None;
    }

    #[test]
    fn project_browser_expectation_and_exact_witness_match() {
        let expected = expectation();
        let observed = witness(&expected);
        let report = compare(&expected, &observed).unwrap();
        assert!(report.mismatches.is_empty());
        assert!(report.evidence_gaps.is_empty());
        assert!(report.is_exact_match());
    }

    #[test]
    fn viewport_width_and_height_are_independent_exact_comparisons() {
        let expected = expectation();
        let mut observed = witness(&expected);
        observed.capture.viewport.width += 1.0;
        observed.capture.viewport.height -= 1.0;
        observed.capture.viewport.scale_factor = 2.5;
        let report = compare(&expected, &observed).unwrap();
        assert_eq!(
            report.mismatches,
            vec![
                Mismatch::ViewportWidth {
                    expected: 1440.0,
                    observed: 1441.0,
                },
                Mismatch::ViewportHeight {
                    expected: 900.0,
                    observed: 899.0,
                },
            ]
        );

        observed.capture.viewport.width = 1440.0;
        observed.capture.viewport.height = 900.0;
        let report = compare(&expected, &observed).unwrap();
        assert!(
            report.is_exact_match(),
            "scale factor is intentionally ignored"
        );
    }

    #[test]
    fn missing_expected_screen_region_and_element_ids_are_typed_mismatches() {
        let expected = expectation();
        let mut observed = witness(&expected);
        for author_id in ["project_browser", "navigation", "project_search"] {
            remove_author_id(&mut observed, author_id);
        }
        let report = compare(&expected, &observed).unwrap();
        assert_eq!(
            report.mismatches,
            vec![
                Mismatch::MissingExpectedAuthorId {
                    object: ExpectedObjectKind::Screen,
                    author_id: "project_browser".into(),
                },
                Mismatch::MissingExpectedAuthorId {
                    object: ExpectedObjectKind::Region,
                    author_id: "navigation".into(),
                },
                Mismatch::MissingExpectedAuthorId {
                    object: ExpectedObjectKind::Element,
                    author_id: "project_search".into(),
                },
            ]
        );
    }

    #[test]
    fn duplicate_expected_ids_are_ambiguous_and_skip_dependent_checks() {
        let expected = expectation();
        let mut observed = witness(&expected);
        let original = node_with_author_id(&observed, "projects").clone();
        let mut duplicate = original.clone();
        duplicate.id = "duplicate-projects-region".into();
        duplicate.bounds = Some(Rect {
            x: -999.0,
            y: -999.0,
            width: 1.0,
            height: 1.0,
        });
        observed.nodes.push(duplicate);
        let report = compare(&expected, &observed).unwrap();
        assert_eq!(
            report.mismatches,
            vec![Mismatch::AmbiguousObservedAuthorId {
                object: ExpectedObjectKind::Region,
                author_id: "projects".into(),
                observed_count: 2,
            }]
        );
        assert!(!report
            .mismatches
            .iter()
            .any(|mismatch| matches!(mismatch, Mismatch::RegionBounds { .. })));
    }

    #[test]
    fn duplicate_screen_and_element_ids_are_ambiguous_without_guessing() {
        let expected = expectation();
        for (author_id, object) in [
            ("project_browser", ExpectedObjectKind::Screen),
            ("project_search", ExpectedObjectKind::Element),
        ] {
            let mut observed = witness(&expected);
            let mut duplicate = node_with_author_id(&observed, author_id).clone();
            duplicate.id = format!("duplicate-{author_id}");
            observed.nodes.push(duplicate);
            let report = compare(&expected, &observed).unwrap();
            assert_eq!(
                report.mismatches,
                vec![Mismatch::AmbiguousObservedAuthorId {
                    object,
                    author_id: author_id.into(),
                    observed_count: 2,
                }]
            );
            if object == ExpectedObjectKind::Element {
                assert!(!report.mismatches.iter().any(|mismatch| matches!(
                    mismatch,
                    Mismatch::ElementOwner {
                        element_author_id,
                        ..
                    } if element_author_id == author_id
                )));
            }
        }
    }

    #[test]
    fn extra_and_unidentified_nodes_are_ignored() {
        let expected = expectation();
        let mut observed = witness(&expected);
        observed
            .nodes
            .push(node("extra-authored", Some("future.extra"), None, None));
        observed.nodes.push(node("unidentified", None, None, None));
        assert!(compare(&expected, &observed).unwrap().is_exact_match());
    }

    #[test]
    fn exact_region_bounds_pass_and_each_bound_difference_is_reported_once() {
        let expected = expectation();
        let baseline = witness(&expected);
        let report = compare(&expected, &baseline).unwrap();
        assert!(report.is_exact_match());

        for (axis, update) in [
            (
                "x",
                Rect {
                    x: 1.0,
                    y: 0.0,
                    width: 0.0,
                    height: 0.0,
                },
            ),
            (
                "y",
                Rect {
                    x: 0.0,
                    y: 1.0,
                    width: 0.0,
                    height: 0.0,
                },
            ),
            (
                "width",
                Rect {
                    x: 0.0,
                    y: 0.0,
                    width: 1.0,
                    height: 0.0,
                },
            ),
            (
                "height",
                Rect {
                    x: 0.0,
                    y: 0.0,
                    width: 0.0,
                    height: 1.0,
                },
            ),
        ] {
            let mut observed = baseline.clone();
            let original = node_with_author_id(&observed, "navigation").bounds.unwrap();
            node_with_author_id_mut(&mut observed, "navigation").bounds = Some(Rect {
                x: original.x + update.x,
                y: original.y + update.y,
                width: original.width + update.width,
                height: original.height + update.height,
            });
            let report = compare(&expected, &observed).unwrap();
            assert_eq!(report.mismatches.len(), 1, "axis {axis}");
            assert!(matches!(
                report.mismatches[0],
                Mismatch::RegionBounds { .. }
            ));
        }

        let mut observed = baseline;
        node_with_author_id_mut(&mut observed, "navigation").bounds = Some(Rect {
            x: -0.0,
            y: 5.0,
            width: 321.0,
            height: 901.0,
        });
        let report = compare(&expected, &observed).unwrap();
        assert_eq!(
            report.mismatches,
            vec![Mismatch::RegionBounds {
                region_author_id: "navigation".into(),
                expected: expected.regions[0].bounds,
                observed: Rect {
                    x: -0.0,
                    y: 5.0,
                    width: 321.0,
                    height: 901.0,
                },
            }]
        );
    }

    #[test]
    fn missing_region_bounds_is_an_evidence_gap() {
        let expected = expectation();
        let mut observed = witness(&expected);
        node_with_author_id_mut(&mut observed, "navigation").bounds = None;
        let report = compare(&expected, &observed).unwrap();
        assert!(report.mismatches.is_empty());
        assert_eq!(
            report.evidence_gaps,
            vec![EvidenceGap::RegionBoundsUnavailable {
                region_author_id: "navigation".into(),
            }]
        );
        assert!(!report.is_exact_match());
    }

    #[test]
    fn element_ownership_uses_only_immediate_parent_author_id() {
        let expected = expectation();
        let mut observed = witness(&expected);
        assert!(compare(&expected, &observed).unwrap().is_exact_match());

        let navigation_node_id = node_with_author_id(&observed, "navigation").id.clone();
        observed.nodes.push(node(
            "intermediate-node",
            None,
            Some(&navigation_node_id),
            None,
        ));
        let project_search = node_with_author_id_mut(&mut observed, "project_search");
        project_search.parent = Some("intermediate-node".into());
        let report = compare(&expected, &observed).unwrap();
        assert!(report.mismatches.is_empty());
        assert_eq!(
            report.evidence_gaps,
            vec![EvidenceGap::ElementParentIdentityUnavailable {
                element_author_id: "project_search".into(),
            }],
            "ownership must not climb to the matching region ancestor"
        );
    }

    #[test]
    fn missing_element_parent_and_parent_identity_are_separate_evidence_gaps() {
        let expected = expectation();
        let mut observed = witness(&expected);
        node_with_author_id_mut(&mut observed, "project_search").parent = None;
        let report = compare(&expected, &observed).unwrap();
        assert_eq!(
            report.evidence_gaps,
            vec![EvidenceGap::ElementParentUnavailable {
                element_author_id: "project_search".into(),
            }]
        );

        let mut observed = witness(&expected);
        observed.nodes.push(node(
            "parent-without-identity",
            None,
            Some("region-node-0"),
            None,
        ));
        node_with_author_id_mut(&mut observed, "project_search").parent =
            Some("parent-without-identity".into());
        let report = compare(&expected, &observed).unwrap();
        assert_eq!(
            report.evidence_gaps,
            vec![EvidenceGap::ElementParentIdentityUnavailable {
                element_author_id: "project_search".into(),
            }]
        );
    }

    #[test]
    fn parent_identity_without_author_id_is_an_evidence_gap() {
        let expected = expectation();
        let mut observed = witness(&expected);
        observed.nodes.push(Node {
            identity: Some(NodeIdentity {
                provenance: "synthetic_test".into(),
                stability: "stable_for_test".into(),
                author_id: None,
            }),
            ..node(
                "parent-without-author-id",
                None,
                Some("region-node-0"),
                None,
            )
        });
        node_with_author_id_mut(&mut observed, "project_search").parent =
            Some("parent-without-author-id".into());
        let report = compare(&expected, &observed).unwrap();
        assert_eq!(
            report.evidence_gaps,
            vec![EvidenceGap::ElementParentAuthorIdUnavailable {
                element_author_id: "project_search".into(),
            }]
        );
    }

    #[test]
    fn explicit_wrong_region_owner_is_a_mismatch() {
        let expected = expectation();
        let mut observed = witness(&expected);
        let projects_node_id = node_with_author_id(&observed, "projects").id.clone();
        node_with_author_id_mut(&mut observed, "project_search").parent = Some(projects_node_id);
        let report = compare(&expected, &observed).unwrap();
        assert_eq!(
            report.mismatches,
            vec![Mismatch::ElementOwner {
                element_author_id: "project_search".into(),
                expected_region_author_id: "navigation".into(),
                observed_region_author_id: "projects".into(),
            }]
        );
    }

    #[test]
    fn unsupported_version_and_structural_issues_are_errors_before_findings() {
        let expected = expectation();
        let mut observed = witness(&expected);
        observed.viewwitness_version = "0.2".into();
        observed.capture.viewport.width += 10.0;
        assert_eq!(
            compare(&expected, &observed),
            Err(ComparisonError::UnsupportedViewWitnessVersion {
                found: "0.2".into(),
                supported: SUPPORTED_VIEWWITNESS_VERSION,
            })
        );

        observed.viewwitness_version = SUPPORTED_VIEWWITNESS_VERSION.into();
        observed.nodes[1].id = observed.nodes[0].id.clone();
        assert!(matches!(
            compare(&expected, &observed),
            Err(ComparisonError::InvalidWitness { issues })
                if issues.iter().any(|issue| issue.contains("duplicate node id"))
        ));

        observed.nodes[1].id = "region-node-0".into();
        observed.nodes[1].parent = Some("missing-parent".into());
        assert!(matches!(
            compare(&expected, &observed),
            Err(ComparisonError::InvalidWitness { issues })
                if issues.iter().any(|issue| issue.contains("references missing parent"))
        ));
    }

    #[test]
    fn finding_order_is_stable_and_follows_expected_traversal() {
        let expected = expectation();
        let mut observed = witness(&expected);
        observed.capture.viewport.width += 1.0;
        observed.capture.viewport.height += 2.0;
        for author_id in [
            "project_browser",
            "navigation",
            "projects",
            "project_search",
        ] {
            remove_author_id(&mut observed, author_id);
        }
        node_with_author_id_mut(&mut observed, "inspector").bounds = None;
        observed.nodes.push(node(
            "inspector-intermediate",
            None,
            Some("region-node-2"),
            None,
        ));
        node_with_author_id_mut(&mut observed, "project_inspector").parent =
            Some("inspector-intermediate".into());
        let first = compare(&expected, &observed).unwrap();
        for _ in 0..10 {
            assert_eq!(compare(&expected, &observed).unwrap(), first);
        }
        assert!(matches!(
            first.mismatches[0],
            Mismatch::ViewportWidth { .. }
        ));
        assert!(matches!(
            first.mismatches[1],
            Mismatch::ViewportHeight { .. }
        ));
        assert!(matches!(
            first.mismatches[2],
            Mismatch::MissingExpectedAuthorId {
                object: ExpectedObjectKind::Screen,
                ..
            }
        ));
        assert_eq!(
            first.mismatches[3],
            Mismatch::MissingExpectedAuthorId {
                object: ExpectedObjectKind::Region,
                author_id: "navigation".into(),
            }
        );
        assert_eq!(
            first.mismatches[4],
            Mismatch::MissingExpectedAuthorId {
                object: ExpectedObjectKind::Region,
                author_id: "projects".into(),
            }
        );
        assert_eq!(
            first.evidence_gaps,
            vec![
                EvidenceGap::RegionBoundsUnavailable {
                    region_author_id: "inspector".into(),
                },
                EvidenceGap::ElementParentIdentityUnavailable {
                    element_author_id: "navigation_items".into(),
                },
                EvidenceGap::ElementParentIdentityUnavailable {
                    element_author_id: "project_collection".into(),
                },
                EvidenceGap::ElementParentIdentityUnavailable {
                    element_author_id: "project_inspector".into(),
                },
            ]
        );
    }

    #[test]
    fn deferred_observed_fields_and_element_bounds_do_not_change_report() {
        let expected = expectation();
        let observed = witness(&expected);
        let baseline = compare(&expected, &observed).unwrap();
        let mut changed = observed.clone();
        let element = node_with_author_id_mut(&mut changed, "project_search");
        element.role = "different-role".into();
        element.name = Some("different name".into());
        element.visible = Some(false);
        element.enabled = Some(false);
        element.focused = Some(true);
        element.selected = Some(true);
        element.text = Some("different text".into());
        element.value = Some(serde_json::json!("different value"));
        element.actions.push("different.action".into());
        element
            .properties
            .insert("changed".into(), serde_json::json!(true));
        element.bounds = Some(Rect {
            x: 999.0,
            y: 999.0,
            width: 1.0,
            height: 1.0,
        });
        changed.capture.viewport.scale_factor = 3.0;
        assert_eq!(compare(&expected, &changed).unwrap(), baseline);

        let mut changed_expectation = expected.clone();
        changed_expectation.regions[0].role = viewwright_expectation::RegionRole::Inspector;
        changed_expectation.regions[0].importance = viewwright_expectation::Importance::Tertiary;
        changed_expectation.regions[0].overflow = viewwright_expectation::Overflow::ScrollY;
        changed_expectation.elements[0].kind = viewwright_expectation::ElementKind::Command;
        changed_expectation.elements[0].importance = viewwright_expectation::Importance::Tertiary;
        changed_expectation.elements[0].label = "ignored label".into();
        changed_expectation.elements[0].action = Some("ignored.action".into());
        changed_expectation.dominant = None;
        assert_eq!(compare(&changed_expectation, &observed).unwrap(), baseline);
    }

    #[test]
    fn public_exact_match_definition_requires_no_gaps_or_mismatches() {
        assert!(ComparisonReport {
            mismatches: vec![],
            evidence_gaps: vec![],
        }
        .is_exact_match());
        assert!(!ComparisonReport {
            mismatches: vec![Mismatch::ViewportWidth {
                expected: 1.0,
                observed: 2.0,
            }],
            evidence_gaps: vec![],
        }
        .is_exact_match());
        assert!(!ComparisonReport {
            mismatches: vec![],
            evidence_gaps: vec![EvidenceGap::RegionBoundsUnavailable {
                region_author_id: "region".into(),
            }],
        }
        .is_exact_match());
    }

    #[test]
    fn logical_viewport_expectation_remains_explicit_and_unchanged() {
        assert_eq!(
            expectation().viewport,
            LogicalViewport {
                width: 1440.0,
                height: 900.0,
            }
        );
    }
}
