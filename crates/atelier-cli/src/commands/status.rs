//! The session orientation view (ADR 0022): checkout and owner, work counts,
//! missions, and other local branches. It is also the SessionStart hook.
use anyhow::{Context, Result};
use atelier_app::record_index::RecordIndex;
use atelier_app::workflow_policy::WorkflowPolicy;
use serde_json::json;
use std::collections::BTreeSet;
use std::path::{Path, PathBuf};

use crate::commands::{self, git};
use crate::human_output::{
    path_summary, render_lines_section, render_metadata, render_page, render_text_section,
    IssueListPanel, IssueListRow, RenderContext,
};
use atelier_app::work_classification::StatusView;

const PRIME_DEPRECATION: &str = "`atelier prime` is deprecated: run `atelier status`, and rerun `atelier setup claude` (with your `--binary <path>`, if you installed with one) so the SessionStart hook calls it.";
/// Ready items listed under the ready count when the checkout has no owner.
const READY_SHOWN: usize = 5;
/// Newest reviews and evidence shown for an owner.
const RECORDS_SHOWN: usize = 8;

/// `prime` is the deprecated alias: the same view after a deprecation line.
pub fn run(hook: bool, quiet: bool, prime: bool) -> Result<()> {
    let note = prime.then_some(PRIME_DEPRECATION);
    if hook {
        let context = commands::session::read_hook_payload("SessionStart")
            .and_then(|_| text(false, RenderContext::plain()))
            .unwrap_or_else(|error| {
                format!("Atelier context unavailable: {error:#}. Run atelier check and repair the named records.")
            });
        let context = match note {
            Some(note) => format!("{note}\n{context}"),
            None => context,
        };
        let output = json!({"hookSpecificOutput": {"hookEventName": "SessionStart", "additionalContext": context}});
        println!("{}", serde_json::to_string(&output)?);
        return Ok(());
    }
    if let Some(note) = note {
        // Quiet output stays parseable.
        if quiet {
            eprintln!("{note}");
        } else {
            println!("{note}");
        }
    }
    println!("{}", text(quiet, RenderContext::for_stdout())?);
    Ok(())
}

fn text(quiet: bool, context: RenderContext) -> Result<String> {
    let layout = atelier_app::storage_layout::StorageLayout::discover()?;
    let _lock =
        atelier_records::mutation_lock::CanonicalMutationLock::shared(&layout.canonical_dir())?;
    let (_, index) = atelier_app::use_cases::load_repository()?;
    let repo = layout.repo_root();
    // A malformed working policy falls back to the last committed one; the
    // page says so, because hook context never shows stderr.
    let (policy, diagnostics) = if repo
        .join(atelier_app::workflow_policy::WORKFLOW_POLICY_PATH)
        .exists()
    {
        let source = atelier_app::base_policy::working(repo)?;
        (Some(source.policy), source.diagnostics)
    } else {
        (None, Vec::new())
    };
    let view = atelier_app::work_classification::status_view(&index, policy.as_ref())?;
    if quiet {
        return Ok(format!(
            "work={} current_missions={} ready={} human={}",
            if view.work.active.is_empty() {
                "none".to_string()
            } else {
                view.work.active.len().to_string()
            },
            view.current_mission_count,
            view.work.ready.len(),
            view.work.human.len(),
        ));
    }
    let checkout = Checkout::read(repo);
    let owner = match (&checkout.branch, &policy) {
        (Ok(branch), Some(policy)) => {
            match commands::session::branch_owners(&index, branch).as_slice() {
                [] => Ok(None),
                [id] => Owner::read(repo, &index, policy, branch, &checkout, id)
                    .with_context(|| format!("owner {id} context unavailable"))
                    .map(Some),
                owners => Err(anyhow::anyhow!(
                    "{branch} has several owners: {}",
                    owners.join(", ")
                )),
            }
        }
        _ => Ok(None),
    };
    let mut page = render_status_page(&view, &diagnostics, &checkout, &owner, context);
    let branches = git::branch_lines(repo, &layout.canonical_dir(), &index)
        .unwrap_or_else(|error| vec![format!("Branch orientation unavailable: {error}")]);
    if !branches.is_empty() {
        page.push_str("\n\n");
        page.push_str(&render_page(
            "Other local work branches",
            [render_lines_section("", branches)],
            context,
        ));
    }
    Ok(page)
}

/// Git facts about the checkout; each is reported unavailable on its own,
/// so status still orients outside a Git repository.
struct Checkout {
    branch: Result<String>,
    head: Result<String>,
    /// Includes files hidden by Git index flags, which make proof stale.
    dirty: Result<Vec<PathBuf>>,
    /// How the branch differs from its upstream, when it has one and differs.
    upstream: Option<String>,
}

impl Checkout {
    fn read(repo: &Path) -> Self {
        Self {
            branch: git::current_branch(repo)
                .map(|branch| branch.unwrap_or_else(|| "detached HEAD".into())),
            head: atelier_app::git_freshness::head_commit(repo),
            dirty: atelier_app::git_freshness::working_paths(repo),
            upstream: upstream_divergence(repo),
        }
    }
}

/// `ahead 1 of origin/feat`, `behind 2 of …`, or `ahead 1, behind 2 of …`.
fn upstream_divergence(repo: &Path) -> Option<String> {
    let upstream = git::text(
        repo,
        &[
            "rev-parse",
            "--abbrev-ref",
            "--symbolic-full-name",
            "@{upstream}",
        ],
    )
    .ok()?;
    let counts = git::text(
        repo,
        &["rev-list", "--left-right", "--count", "HEAD...@{upstream}"],
    )
    .ok()?;
    let (ahead, behind) = counts.split_once('\t')?;
    let parts = [("ahead", ahead), ("behind", behind)]
        .into_iter()
        .filter(|(_, count)| *count != "0")
        .map(|(label, count)| format!("{label} {count}"))
        .collect::<Vec<_>>();
    (!parts.is_empty()).then(|| format!("{} of {upstream}", parts.join(", ")))
}

/// What a session on the owner's branch needs: its record, mission, scope
/// work, reviews, and evidence freshness.
struct Owner {
    id: String,
    heading: String,
    base: Option<String>,
    body: String,
    missions: Vec<(String, Option<String>)>,
    work: Vec<String>,
    reviews: Vec<String>,
    evidence: Vec<String>,
    inspect: String,
}

impl Owner {
    fn read(
        repo: &Path,
        index: &RecordIndex,
        policy: &WorkflowPolicy,
        branch: &str,
        checkout: &Checkout,
        id: &str,
    ) -> Result<Self> {
        let record = index.issue_record(id).context("owner record missing")?;
        let scope = atelier_app::review_gate::scope(index, id)?;
        let status = if policy.status_category(&record.issue.status) == Some("done") {
            git::unmerged_done_label(
                repo,
                id,
                &record.issue.status,
                branch,
                &policy.branch_policy.base_branch,
            )
        } else {
            record.issue.status.clone()
        };
        let content = atelier_records::render_issue_record(record)?;
        let (_, body) =
            atelier_records::split_document(&content).context("owner has no record body")?;
        let mut missions = Vec::new();
        for mission in index.issues().filter(|issue| issue.issue_type == "mission") {
            if atelier_app::objective_graph::mission_issue_ids(index, &mission.id)?.contains(id) {
                missions.push((
                    format!("{} [{}] {}", mission.id, mission.status, mission.title),
                    mission
                        .strategy
                        .as_ref()
                        .map(|strategy| strategy.path.clone()),
                ));
            }
        }
        let mut work = Vec::new();
        let human_children =
            atelier_app::work_classification::open_human_children(index, policy, id);
        if !human_children.is_empty() {
            work.push(format!(
                "Needs a human: {}",
                atelier_app::work_classification::human_children_note(id, &human_children)
            ));
        }
        let buckets = atelier_app::work_classification::work_buckets_for(
            index,
            Some(policy),
            index.issues().filter(|issue| scope.contains(&issue.id)),
        )?;
        work.extend(
            buckets
                .active
                .iter()
                .chain(&buckets.blocked)
                .take(RECORDS_SHOWN)
                .map(|row| {
                    format!(
                        "{} [{}] {} — blockers: {}",
                        row.id,
                        row.status,
                        row.title,
                        row.open_blockers.join(", ")
                    )
                }),
        );
        Ok(Self {
            id: id.to_owned(),
            heading: format!("{id} [{status}] {}", record.issue.title),
            base: record
                .issue
                .workflow_branch
                .as_ref()
                .map(|ownership| ownership.branch_base.clone()),
            body: crate::utils::truncate(body, 3000),
            missions,
            work,
            reviews: reviews(index, &scope),
            evidence: evidence(repo, index, &scope, checkout)?,
            inspect: if index.get_subissues(id).is_empty() {
                format!("atelier issue show {id}")
            } else {
                format!("atelier work parent {id}")
            },
        })
    }
}

fn reviews(index: &RecordIndex, scope: &BTreeSet<String>) -> Vec<String> {
    let mut reviews = index
        .reviews()
        .filter(|review| scope.contains(&review.target))
        .collect::<Vec<_>>();
    reviews.sort_by_key(|review| {
        std::cmp::Reverse((review.header.created_at, review.header.id.clone()))
    });
    reviews
        .into_iter()
        .take(RECORDS_SHOWN)
        .map(|review| {
            format!(
                "{} for {} — {:?}: {}",
                review.header.id, review.target, review.event, review.summary
            )
        })
        .collect()
}

/// The newest evidence about the owner's scope, each marked fresh only when
/// it is fresh for every scoped issue it supports.
fn evidence(
    repo: &Path,
    index: &RecordIndex,
    scope: &BTreeSet<String>,
    checkout: &Checkout,
) -> Result<Vec<String>> {
    let head = checkout
        .head
        .as_ref()
        .map_err(|error| anyhow::anyhow!("{error:#}"))?;
    let dirty = checkout
        .dirty
        .as_ref()
        .map_err(|error| anyhow::anyhow!("{error:#}"))?;
    let mut proof =
        atelier_app::git_freshness::ReadContext::from_snapshot(repo, head.clone(), dirty);
    let mut records = index.evidence_records();
    records.sort_by_key(|record| {
        std::cmp::Reverse((record.data.captured_at, record.header.id.clone()))
    });
    let mut lines = Vec::new();
    for evidence in records {
        let targets = index
            .list_record_links("evidence", &evidence.header.id)?
            .into_iter()
            .flat_map(|link| [link.source_id, link.target_id])
            .filter(|id| scope.contains(id))
            .collect::<Vec<_>>();
        if targets.is_empty() {
            continue;
        }
        let fresh = targets.iter().all(|id| {
            index
                .issue_record(id)
                .is_some_and(|issue| proof.evidence_is_fresh(issue, evidence).unwrap_or(false))
        });
        lines.push(format!(
            "{} — recorded {:?} @ {} ({})",
            evidence.header.id,
            evidence.data.result,
            &evidence.data.commit[..8.min(evidence.data.commit.len())],
            if fresh {
                "fresh"
            } else {
                "stale against current work"
            }
        ));
        if lines.len() == RECORDS_SHOWN {
            break;
        }
    }
    Ok(lines)
}

fn render_status_page(
    view: &StatusView,
    diagnostics: &[String],
    checkout: &Checkout,
    owner: &Result<Option<Owner>>,
    context: RenderContext,
) -> String {
    let current_work = if view.work.active.is_empty() {
        "none".to_string()
    } else {
        format!("{} issue(s)", view.work.active.len())
    };
    let summary_lines = vec![
        format!("Ready work:    {}", view.work.ready.len()),
        format!("Needs a human: {}", view.work.human.len()),
        format!("Current work:  {current_work}"),
        format!("Current missions: {}", view.current_mission_count),
    ];
    let owned = matches!(owner, Ok(Some(_)));
    // Without an owner, the session's next step is choosing ready work.
    let ready_panel = if owned || view.work.ready.is_empty() {
        Vec::new()
    } else {
        IssueListPanel::from_work_rows("Ready Work", &view.work.ready)
            .limit(READY_SHOWN)
            .render(context)
    };

    let active_rows = view
        .work
        .active
        .iter()
        .map(|issue| IssueListRow {
            role: crate::human_output::DisplayRole::Executable,
            id: issue.id.clone(),
            status: issue.status_category.clone(),
            priority: issue.priority.clone(),
            title: issue.title.clone(),
            blockers: 0,
            depth: 1,
        })
        .collect::<Vec<_>>();

    let mut local_state = Vec::new();
    match (&checkout.branch, &checkout.head) {
        (Ok(branch), Ok(head)) => local_state.push((
            "Branch",
            match &checkout.upstream {
                Some(upstream) => format!("{branch} @ {} ({upstream})", &head[..8]),
                None => format!("{branch} @ {}", &head[..8]),
            },
        )),
        (Ok(branch), Err(_)) => local_state.push(("Branch", branch.clone())),
        _ => {}
    }
    if checkout.branch.is_ok() {
        match owner {
            Ok(Some(owner)) => {
                local_state.push(("Owner", owner.heading.clone()));
                if let Some(base) = &owner.base {
                    local_state.push(("Integration base", base.clone()));
                }
                for (mission, strategy) in &owner.missions {
                    local_state.push(("Mission", mission.clone()));
                    if let Some(strategy) = strategy {
                        local_state.push(("Strategy", strategy.clone()));
                    }
                }
            }
            Ok(None) => local_state.push(("Owner", "none".into())),
            Err(error) => local_state.push(("Owner", format!("unavailable - {error:#}"))),
        }
    }
    local_state.push((
        "Checkout",
        match &checkout.dirty {
            Ok(dirty) if dirty.is_empty() => "clean".to_string(),
            Ok(dirty) => format!(
                "dirty ({})",
                path_summary(
                    &dirty
                        .iter()
                        .map(|path| path.display().to_string())
                        .collect::<Vec<_>>(),
                    3
                )
            ),
            Err(error) => format!("unavailable - {error:#}"),
        },
    ));

    let ready = view.work.ready.len();
    let next_actions = if let Ok(Some(owner)) = owner {
        vec![
            format!("  Inspect owner work: {}", owner.inspect),
            format!("  Check handoff: atelier land {}", owner.id),
            format!(
                "  Check completed-owner merge readiness: atelier merge {} --check",
                owner.id
            ),
        ]
    } else if ready > 0 {
        vec![
            format!("  Choose ready work ({ready} ready issue(s) available): atelier work ready"),
            "  Inspect selected work transitions: atelier issue transition <issue-id>".to_string(),
        ]
    } else if !view.work.human.is_empty() {
        vec![format!(
            "  Only work for the human is ready ({} item(s)): atelier work human",
            view.work.human.len()
        )]
    } else if !view.work.blocked.is_empty() {
        vec![
            "  Inspect blocked work (no ready work is available): atelier work blocked".to_string(),
        ]
    } else {
        vec!["  No specific next action is available from checkout state.".to_string()]
    };

    let mut panels = vec![
        if diagnostics.is_empty() {
            Vec::new()
        } else {
            render_lines_section("Policy Diagnostics", diagnostics.to_vec())
        },
        render_lines_section("", summary_lines),
        ready_panel,
        IssueListPanel::new("Current Work", active_rows)
            .limit(8)
            .render(context),
        render_metadata(Some("Local State"), &local_state),
    ];
    if let Ok(Some(owner)) = owner {
        panels.push(render_text_section("Owner Record", Some(&owner.body)));
        if !owner.work.is_empty() {
            panels.push(render_lines_section("Owner Work", owner.work.clone()));
        }
        panels.push(render_lines_section("Reviews", owner.reviews.clone()));
        panels.push(render_lines_section("Evidence", owner.evidence.clone()));
    }
    panels.push(render_lines_section("Next Actions", next_actions));
    render_page("Atelier Status", panels, context)
}
