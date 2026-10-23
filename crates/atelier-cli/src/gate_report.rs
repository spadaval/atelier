//! The gate lines `land` and `merge` print (atelier-h1nc): each failed check
//! first, one line per record with its cause and the command that fixes it,
//! then the passes collapsed, then the verdict line. One renderer serves
//! both commands, so a check reads the same wherever it fails.
use crate::human_output::{count_noun, ItemLine};
use atelier_app::{
    record_index::RecordIndex,
    review_gate::{Gap, GateNode},
    workflow_policy::WorkflowPolicy,
};
use atelier_core::{FindingDecision, ReviewEvent};

/// What the renderer reads besides the nodes.
pub(crate) struct Gates<'a> {
    pub index: &'a RecordIndex,
    pub policy: &'a WorkflowPolicy,
    /// The work branch's owner: its checks are named without a record in the
    /// verdict, and its passes are listed by name.
    pub owner: Option<&'a str>,
}

/// A rendered gate run.
#[derive(Debug, Default)]
pub(crate) struct GateReport {
    /// One line per failed check and record, each ending in its fix.
    pub failures: Vec<String>,
    /// The passes, collapsed to a line for the owner's checks and a count of
    /// the other done claims that hold.
    pub passes: Vec<String>,
    /// Every failed check as the verdict names it, including those whose
    /// line another failure's line absorbed.
    pub failed: Vec<String>,
}

/// A refusal by failed gates. `land` returns it from its checks, and its
/// text is what the Stop hook reports; `merge` prints its lines as it goes
/// and returns it for the verdict.
#[derive(Debug)]
pub(crate) struct GatesRefused {
    pub report: GateReport,
    /// Facts that bear on the failures, such as uncommitted changes that make
    /// passing proof stale; printed after the passes.
    pub notes: Vec<String>,
}
impl std::fmt::Display for GatesRefused {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(
            &self
                .report
                .failures
                .iter()
                .chain(&self.notes)
                .cloned()
                .collect::<Vec<_>>()
                .join("\n"),
        )
    }
}
impl std::error::Error for GatesRefused {}

/// `<command>: passed`, or `<command>: refused — N checks (a, b)` naming
/// every failed check.
pub(crate) fn verdict(command: &str, failed: &[String]) -> String {
    if failed.is_empty() {
        format!("{command}: passed")
    } else {
        format!(
            "{command}: refused — {} ({})",
            count_noun(failed.len(), "check"),
            failed.join(", ")
        )
    }
}

/// One failure line before it is written out.
struct Line {
    record: String,
    check: String,
    cause: String,
    fix: Option<String>,
    /// The blocker a blocker line is about, so a repeated blocker check on
    /// another record can be folded into it.
    blocker: Option<String>,
}
impl Line {
    fn new(record: &str, node: &GateNode, cause: String, fix: Option<String>) -> Self {
        Self {
            record: record.to_owned(),
            check: node.check.clone(),
            cause,
            fix,
            blocker: None,
        }
    }
    fn render(&self, also: &[String]) -> String {
        let mut text = format!(
            "⚠ {} {}: {}",
            self.record,
            self.check,
            self.cause.trim_end().trim_end_matches('.')
        );
        if !also.is_empty() {
            text.push_str(&format!(" (also {})", also.join(", ")));
        }
        if let Some(fix) = &self.fix {
            text.push_str(&format!(". Next: {fix}"));
        }
        text
    }
}

impl Gates<'_> {
    /// The verdict's name for a node's check: bare for the owner (and for
    /// findings judged across claims), `<record> <check>` otherwise.
    fn label(&self, node: &GateNode) -> String {
        if Some(node.target.as_str()) == self.owner || node.target == "session" {
            node.check.clone()
        } else {
            format!("{} {}", node.target, node.check)
        }
    }

    fn is_owner(&self, node: &GateNode) -> bool {
        Some(node.target.as_str()) == self.owner
    }

    /// Whether `check` failed on the owner.
    fn owner_failed(&self, nodes: &[GateNode], check: &str) -> bool {
        nodes
            .iter()
            .any(|node| !node.passed && node.check == check && self.is_owner(node))
    }

    /// The item line of a record, or its ID when it is not an issue.
    fn item(&self, id: &str) -> String {
        match self.index.get_issue(id) {
            Some(issue) => ItemLine::new(id, &issue.status, &issue.priority, &issue.title)
                .state(self.policy.status_category(&issue.status), None)
                .text(),
            None => id.to_owned(),
        }
    }

    /// The transition that closes `id` from its status, or the command that
    /// lists its transitions when none does.
    fn close_command(&self, id: &str) -> String {
        let Some(issue) = self.index.get_issue(id) else {
            return format!("atelier issue show {id}");
        };
        let mut closing = self
            .policy
            .workflow_for_issue_type(&issue.issue_type)
            .map(|workflow| {
                workflow
                    .transitions
                    .iter()
                    .filter(|(_, transition)| {
                        transition.from.contains(&issue.status)
                            && transition.to != "superseded"
                            && self
                                .policy
                                .issue_status_is_terminal(&issue.issue_type, &transition.to)
                                .unwrap_or(false)
                    })
                    .collect::<Vec<_>>()
            })
            .unwrap_or_default();
        closing.sort_by_key(|(name, _)| name.as_str() != "close");
        match closing.first() {
            Some((name, transition)) => {
                let reason = transition
                    .required_fields
                    .iter()
                    .any(|field| field == "close_reason" || field.starts_with("report."));
                format!(
                    "atelier issue transition {id} {name}{}",
                    if reason { " --reason \"…\"" } else { "" }
                )
            }
            None => format!("atelier issue transition {id}"),
        }
    }

    /// The owner whose review covers `id`: its parent, or itself.
    fn reviewed_owner(&self, id: &str) -> String {
        self.index
            .get_issue(id)
            .and_then(|issue| issue.parent_id)
            .unwrap_or_else(|| id.to_owned())
    }

    /// A finding as `<id> <recommendation> finding "<first line>"`, with
    /// its target and recommendation.
    fn finding(&self, id: &str) -> (String, String, Option<FindingDecision>) {
        let Ok(record) = self.index.require_review(id) else {
            return (id.to_owned(), id.to_owned(), None);
        };
        let recommendation = match &record.event {
            ReviewEvent::Finding { recommendation } => Some(*recommendation),
            _ => None,
        };
        let summary = record.summary.lines().next().unwrap_or_default().trim();
        const LONGEST: usize = 100;
        let summary = if summary.chars().count() > LONGEST {
            format!("{}…", summary.chars().take(LONGEST).collect::<String>())
        } else {
            summary.to_owned()
        };
        (
            record.target.clone(),
            format!(
                "{id} {}finding \"{summary}\"",
                recommendation.map_or(String::new(), |decision| format!("{} ", decision.as_str()))
            ),
            recommendation,
        )
    }

    /// Fresh proof for `target`: the failing receipt's command again when
    /// it captured one.
    fn rerun_command(&self, target: &str, evidence: &str) -> String {
        match self.index.evidence(evidence) {
            Some(record) => match &record.data.command {
                Some(command) => format!(
                    "atelier evidence record --target issue/{target} --kind {} -- {command}",
                    record.data.evidence_type
                ),
                None => format!(
                    "atelier evidence record --target issue/{target} --kind {} --result pass \"…\"",
                    record.data.evidence_type
                ),
            },
            None => format!("atelier evidence show {evidence}"),
        }
    }

    /// The latest command `target`'s proof captured, recorded again.
    fn recapture_command(&self, target: &str) -> Option<String> {
        let record = self
            .index
            .evidence_records()
            .into_iter()
            .filter(|record| {
                record.data.command.is_some()
                    && record
                        .header
                        .relationships
                        .attachments
                        .iter()
                        .any(|link| link.kind == "issue" && link.id == target)
            })
            .max_by_key(|record| record.data.captured_at)?;
        Some(format!(
            "atelier evidence record --target issue/{target} --kind {} -- {}",
            record.data.evidence_type,
            record.data.command.as_deref()?
        ))
    }

    /// Whether the owner's review passes only by trivial exemption.
    fn exempted_only(&self, nodes: &[GateNode]) -> bool {
        nodes
            .iter()
            .find(|node| node.check == "review.current" && self.is_owner(node) && node.passed)
            .is_some_and(|node| {
                !node.supports.is_empty()
                    && node.supports.iter().all(|id| {
                        self.index.require_review(id).is_ok_and(|record| {
                            matches!(record.event, ReviewEvent::TrivialExemption)
                        })
                    })
            })
    }

    fn finding_lines(&self, node: &GateNode, nodes: &[GateNode]) -> Vec<Line> {
        let mut lines = Vec::new();
        let mut awaiting = Vec::new();
        for gap in &node.gaps {
            match gap {
                Gap::Undisposed(id) => {
                    let (target, shown, recommendation) = self.finding(id);
                    let fix = match recommendation.unwrap_or(FindingDecision::FixNow) {
                        FindingDecision::FixNow => format!("atelier review disposition {id} --decision fix_now --resolution-commit <commit> --identity <identity> \"…\""),
                        FindingDecision::Defer => format!("atelier review disposition {id} --decision defer --follow-up <issue> --identity <reviewer> \"…\""),
                        FindingDecision::NoAction => format!("atelier review disposition {id} --decision no_action --identity <reviewer> \"…\""),
                    };
                    lines.push(Line::new(
                        &target,
                        node,
                        format!("{shown} has no disposition"),
                        Some(fix),
                    ));
                }
                Gap::Unresolved {
                    finding,
                    disposition,
                } => {
                    let (target, shown, _) = self.finding(finding);
                    lines.push(Line::new(
                        &target,
                        node,
                        format!("{shown}: fix_now disposition {disposition} names no resolution commit"),
                        Some(format!("atelier review disposition {finding} --decision fix_now --resolution-commit <commit> --identity <identity> \"…\"")),
                    ));
                }
                Gap::AwaitingApproval(id) => awaiting.push(id.clone()),
                Gap::Unaccepted {
                    finding,
                    disposition,
                } => {
                    let (target, shown, _) = self.finding(finding);
                    let decision = self
                        .index
                        .require_review(disposition)
                        .ok()
                        .and_then(|record| match &record.event {
                            ReviewEvent::Disposition { decision, .. } => Some(decision.as_str()),
                            _ => None,
                        })
                        .unwrap_or("its");
                    lines.push(Line::new(
                        &target,
                        node,
                        format!("{shown}: {decision} disposition {disposition} needs acceptance by a non-author"),
                        Some(format!(
                            "atelier review request {}",
                            self.reviewed_owner(&target)
                        )),
                    ));
                }
                Gap::Untraced {
                    finding,
                    disposition,
                    error,
                } => {
                    let (target, shown, _) = self.finding(finding);
                    lines.push(Line::new(
                        &target,
                        node,
                        format!("{shown}: disposition {disposition} could not be traced to the commit that introduced it: {error}"),
                        Some(format!("atelier review show {disposition}")),
                    ));
                }
                _ => {}
            }
        }
        if !awaiting.is_empty() {
            let mut targets = awaiting
                .iter()
                .map(|id| self.finding(id).0)
                .collect::<Vec<_>>();
            targets.sort();
            targets.dedup();
            let record = if node.target == "session" {
                targets.join(", ")
            } else {
                node.target.clone()
            };
            let owner = self.reviewed_owner(targets.first().map_or(&record, |target| target));
            let count = count_noun(awaiting.len(), "fix_now finding");
            let they = if awaiting.len() == 1 {
                "it needs"
            } else {
                "they need"
            };
            let cause = if self.owner_failed(nodes, "review.current") {
                format!("{count} resolved; {they} a fresh approval (see review.current)")
            } else if self.exempted_only(nodes) {
                format!("{count} resolved; a fix_now resolution needs an independent approval, which a trivial exemption is not")
            } else {
                format!("{count} resolved after the current approval; {they} a fresh approval")
            };
            lines.push(Line::new(
                &record,
                node,
                cause,
                Some(format!("atelier review request {owner}")),
            ));
        }
        lines
    }

    /// The lines for one failed node.
    fn lines(&self, node: &GateNode, nodes: &[GateNode]) -> Vec<Line> {
        let target = node.target.as_str();
        let first_line = || node.reason.lines().next().unwrap_or_default().to_owned();
        let lines = match node.check.as_str() {
            "work.terminal" => node
                .gaps
                .iter()
                .filter_map(|gap| match gap {
                    Gap::Open(id) => {
                        let status = self
                            .index
                            .get_issue(id)
                            .map_or_else(|| "unknown".to_owned(), |issue| issue.status);
                        Some(Line::new(
                            id,
                            node,
                            if id == target {
                                format!("{status}, not closed")
                            } else {
                                format!("{status}, open in {target}'s scope")
                            },
                            Some(self.close_command(id)),
                        ))
                    }
                    _ => None,
                })
                .collect(),
            "blockers.none_open" | "blockers.transitive_none_open" => {
                let done = self.index.get_issue(target).is_some_and(|issue| {
                    self.policy
                        .issue_status_is_terminal(&issue.issue_type, &issue.status)
                        .unwrap_or(false)
                });
                node.gaps
                    .iter()
                    .filter_map(|gap| match gap {
                        Gap::Blocker(id) => Some(Line {
                            blocker: Some(id.clone()),
                            ..Line::new(
                                target,
                                node,
                                format!(
                                    "{}blocked by {}",
                                    if done { "done but " } else { "" },
                                    self.item(id)
                                ),
                                Some(self.close_command(id)),
                            )
                        }),
                        _ => None,
                    })
                    .collect()
            }
            "review.current" => vec![Line::new(
                target,
                node,
                node.reason.clone(),
                Some(format!("atelier review request {target}")),
            )],
            "findings.disposed" => self.finding_lines(node, nodes),
            "evidence.no_newer_failure" => node
                .gaps
                .iter()
                .filter_map(|gap| match gap {
                    Gap::FailingEvidence(id) => Some(Line::new(
                        target,
                        node,
                        node.reason.clone(),
                        Some(self.rerun_command(target, id)),
                    )),
                    _ => None,
                })
                .collect(),
            "shell.check" => node
                .gaps
                .iter()
                .filter_map(|gap| match gap {
                    Gap::FailedCommand { command, output } => Some(Line::new(
                        target,
                        node,
                        if output.is_empty() {
                            format!("`{command}` failed")
                        } else {
                            format!("`{command}` failed: {output}")
                        },
                        Some(command.clone()),
                    )),
                    _ => None,
                })
                .collect(),
            "evidence.attached" => vec![Line::new(
                target,
                node,
                first_line(),
                self.recapture_command(target).or_else(|| node.fix.clone()),
            )],
            check if check.starts_with("required_field.") => {
                let field = &check["required_field.".len()..];
                let path = format!(".atelier/issues/{target}.md");
                let fix = match atelier_core::ReportPart::from_field(field) {
                    Some(part) => format!(
                        "add a `### {}` subsection to the Close Reason in {path}, then commit",
                        part.title()
                    ),
                    None => format!("write the Close Reason in {path}, then commit"),
                };
                vec![Line::new(
                    target,
                    node,
                    format!("committed claim is missing required field {field}"),
                    Some(fix),
                )]
            }
            _ => Vec::new(),
        };
        if lines.is_empty() {
            vec![Line::new(target, node, first_line(), node.fix.clone())]
        } else {
            lines
        }
    }

    /// The node whose line absorbs `node`'s, when both have one cause: a
    /// done claim outside the approval when the owner's review is not
    /// current or its findings are not settled, the merged tree when a shell check failed on it, and a
    /// transitive blocker check whose blockers a direct one already names.
    fn host(&self, node: &GateNode, nodes: &[GateNode]) -> Option<usize> {
        let failed = |check: &str| {
            nodes
                .iter()
                .position(|other| !other.passed && other.check == check && self.is_owner(other))
        };
        match node.check.as_str() {
            "claims.reviewed" => {
                let in_scope = node.gaps.iter().all(|gap| match gap {
                    Gap::UnreviewedClaim { claim, .. } => self.owner.is_some_and(|owner| {
                        atelier_app::review_gate::scope(self.index, owner)
                            .is_ok_and(|scope| scope.contains(claim))
                    }),
                    _ => false,
                });
                // An approval covers a claim only once every finding before
                // it is settled, so unsettled findings are also a cause.
                if !in_scope || node.gaps.is_empty() {
                    return None;
                }
                failed("review.current").or_else(|| failed("findings.disposed"))
            }
            "merge.merged_tree" => failed("shell.check"),
            "blockers.none_open" | "blockers.transitive_none_open" => {
                let blockers = node
                    .gaps
                    .iter()
                    .filter(|gap| matches!(gap, Gap::Blocker(_)))
                    .collect::<Vec<_>>();
                if blockers.is_empty() {
                    return None;
                }
                // A direct check folds only into the owner's; a transitive one
                // into the owner's first, else into its record's direct one.
                let covers = |other: &GateNode| {
                    !other.passed
                        && other.check == "blockers.none_open"
                        && !std::ptr::eq(other, node)
                        && blockers.iter().all(|gap| other.gaps.contains(gap))
                };
                let owner = nodes
                    .iter()
                    .position(|other| covers(other) && self.is_owner(other));
                if node.check == "blockers.none_open" {
                    owner.filter(|_| !self.is_owner(node))
                } else {
                    owner.or_else(|| {
                        nodes
                            .iter()
                            .position(|other| covers(other) && other.target == node.target)
                    })
                }
            }
            _ => None,
        }
    }

    /// The pass label of an owner check; `review.current` names the review
    /// that carries it, an independent approval first.
    fn pass_label(&self, node: &GateNode) -> String {
        if node.check != "review.current" {
            return node.check.clone();
        }
        let approval = node.supports.iter().find(|id| {
            self.index
                .require_review(id)
                .is_ok_and(|record| matches!(record.event, ReviewEvent::Approval))
        });
        match approval.or(node.supports.first()) {
            Some(id) => format!("review.current via {id}"),
            None => node.check.clone(),
        }
    }

    pub(crate) fn report(&self, nodes: &[GateNode]) -> GateReport {
        let mut report = GateReport::default();
        // The owner's failures lead, its review first since other lines
        // point to it, then the other records', each in the order the gates
        // ran.
        let mut order = (0..nodes.len())
            .filter(|&at| !nodes[at].passed)
            .collect::<Vec<_>>();
        order.sort_by_key(|&at| {
            (
                !self.is_owner(&nodes[at]),
                nodes[at].check != "review.current",
            )
        });
        for &at in &order {
            let label = self.label(&nodes[at]);
            if !report.failed.contains(&label) {
                report.failed.push(label);
            }
        }
        let hosts = order
            .iter()
            .map(|&at| (at, self.host(&nodes[at], nodes)))
            .collect::<Vec<_>>();
        for &at in &order {
            if hosts
                .iter()
                .any(|(folded, host)| *folded == at && host.is_some())
            {
                continue;
            }
            // A folded check is named on the line about its blocker, or
            // else on its host's first line.
            for (number, line) in self.lines(&nodes[at], nodes).into_iter().enumerate() {
                let also = hosts
                    .iter()
                    .filter(|(folded, host)| {
                        *host == Some(at)
                            && match &line.blocker {
                                Some(blocker) => {
                                    nodes[*folded].gaps.contains(&Gap::Blocker(blocker.clone()))
                                }
                                None => number == 0,
                            }
                    })
                    .map(|(folded, _)| self.label(&nodes[*folded]))
                    .collect::<Vec<_>>();
                report.failures.push(line.render(&also));
            }
        }
        let owner_passes = nodes
            .iter()
            .filter(|node| node.passed && self.is_owner(node))
            .map(|node| self.pass_label(node))
            .collect::<Vec<_>>();
        if !owner_passes.is_empty() {
            report.passes.push(format!(
                "✓ {} {} ({})",
                count_noun(owner_passes.len(), "check"),
                if owner_passes.len() == 1 {
                    "passes"
                } else {
                    "pass"
                },
                owner_passes.join(", ")
            ));
        }
        let mut claims = nodes
            .iter()
            .filter(|node| !self.is_owner(node) && self.index.get_issue(&node.target).is_some())
            .map(|node| node.target.as_str())
            .collect::<Vec<_>>();
        claims.sort();
        claims.dedup();
        let holding = claims
            .iter()
            .filter(|target| {
                nodes
                    .iter()
                    .all(|node| node.target != **target || node.passed)
            })
            .count();
        if holding > 0 {
            report.passes.push(format!(
                "✓ {} {}",
                count_noun(holding, "done claim"),
                if holding == 1 { "holds" } else { "hold" }
            ));
        }
        report
    }
}
