//! Pinned, read-only branch snapshots shared by start and orientation.
use anyhow::{bail, Context, Result};
use atelier_app::{git_freshness, record_index::RecordIndex, review_gate, workflow_policy};
use std::{
    collections::{BTreeMap, BTreeSet},
    path::{Path, PathBuf},
    process::{Command, Stdio},
};

pub(crate) fn text(repo: &Path, args: &[&str]) -> Result<String> {
    Ok(String::from_utf8(git_freshness::git_output(repo, args)?)?
        .trim()
        .into())
}
pub(crate) fn current_branch(repo: &Path) -> Result<Option<String>> {
    Ok(checkout_branch(repo)?.map(|branch| branch.trim_end().to_owned()))
}
/// The recursive checkout branch identity, preserving trailing whitespace.
pub(crate) fn checkout_branch(repo: &Path) -> Result<Option<String>> {
    let output = atelier_app::git_freshness::command(repo)
        .args(["symbolic-ref", "--quiet", "HEAD"])
        .output()?;
    if output.status.code() == Some(1) {
        return Ok(None);
    }
    if !output.status.success() {
        bail!(
            "cannot resolve checkout branch: {}",
            String::from_utf8_lossy(&output.stderr)
        );
    }
    let reference = String::from_utf8(output.stdout)?;
    Ok(Some(
        reference
            .strip_suffix('\n')
            .unwrap_or(&reference)
            .strip_prefix("refs/heads/")
            .context("checkout HEAD does not reference a local branch")?
            .to_owned(),
    ))
}
pub(crate) fn resolve(repo: &Path, reference: &str) -> Result<String> {
    text(
        repo,
        &["rev-parse", "--verify", &format!("{reference}^{{commit}}")],
    )
}
/// Prefer a local head; a unique remote-tracking head is an existing branch too.
pub(crate) fn work_ref(repo: &Path, branch: &str) -> Result<Option<(String, String)>> {
    let local = format!("refs/heads/{branch}");
    if let Ok(commit) = resolve(repo, &local) {
        return Ok(Some((local, commit)));
    }
    let refs = text(
        repo,
        &["for-each-ref", "--format=%(refname)", "refs/remotes/"],
    )?;
    let matches = refs
        .lines()
        .filter(|reference| {
            reference
                .strip_prefix("refs/remotes/")
                .and_then(|rest| rest.split_once('/'))
                .is_some_and(|(_, name)| name == branch)
        })
        .collect::<Vec<_>>();
    match matches.as_slice() {
        [] => Ok(None),
        [reference] => Ok(Some(((*reference).into(), resolve(repo, reference)?))),
        _ => bail!("start_remote_ambiguous: several remote-tracking refs own {branch}: {}. Create the local tracking branch explicitly, then retry.", matches.join(", ")),
    }
}

pub(crate) struct Snapshot {
    dir: tempfile::TempDir,
    pub index: RecordIndex,
}
impl Snapshot {
    pub fn at(repo: &Path, state: &Path, commit: &str) -> Result<Self> {
        let relative = state
            .strip_prefix(repo)
            .context("tracker is outside repository")?;
        let dir = tempfile::tempdir()?;
        let mut archive = atelier_app::git_freshness::command(repo)
            .args(["archive", "--format=tar", commit, "--"])
            .arg(relative)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()?;
        let unpack = Command::new("tar")
            .current_dir(dir.path())
            .args(["-xf", "-"])
            .stdin(archive.stdout.take().context("archive has no stdout")?)
            .output()?;
        let archive = archive.wait_with_output()?;
        if !archive.status.success() || !unpack.status.success() {
            bail!(
                "branch_snapshot: cannot read tracker at {commit}: {}{}",
                String::from_utf8_lossy(&archive.stderr),
                String::from_utf8_lossy(&unpack.stderr)
            );
        }
        let index = RecordIndex::load_structural(&dir.path().join(relative))?;
        Ok(Self { dir, index })
    }
    pub fn root(&self) -> &Path {
        self.dir.path()
    }
}

/// Gates read custom context link types from the project config,
/// so a guard refuses one that differs from HEAD, even if Git ignores it
/// (atelier-v44v).
pub(crate) fn require_committed_config(repo: &Path, head: &str, command: &str) -> Result<()> {
    let path = Path::new(".atelier/config.toml");
    let committed = git_freshness::blob(repo, head, path).ok();
    let working = std::fs::read_to_string(repo.join(path)).ok();
    if committed != working {
        bail!(
            "{command}_config_uncommitted: .atelier/config.toml differs from HEAD, and gates read its custom context link types; commit it with the work it describes, or restore it with `git restore .atelier/config.toml`"
        );
    }
    Ok(())
}

/// Record-only changes may be committed on the base branch directly; any
/// other unpushed base commit must be covered by an approval or trivial
/// exemption of a completed owner that branched from this base, which is what
/// `atelier merge` integrates. Commits that install the workflow policy
/// predate the gates and are exempt.
pub(crate) fn check_base_commits(
    repo: &Path,
    index: &RecordIndex,
    policy: &atelier_workflow::WorkflowPolicy,
    tip: &str,
) -> Result<String> {
    let base = &policy.branch_policy.base_branch;
    let Ok(upstream) = text(
        repo,
        &[
            "rev-parse",
            "--verify",
            "--quiet",
            &format!("{base}@{{upstream}}"),
        ],
    ) else {
        return Ok(format!(
            "Base commits unchecked: {base} has no upstream, so there is no pushed point to check from."
        ));
    };
    // History reads need the tip as a full commit ID.
    let tip = &resolve(repo, tip)?;
    let mut read = git_freshness::ReadContext::from_head(repo, tip.clone());
    let reviews = BaseReviews::new(repo, index, policy, &mut read, &upstream)?;
    let unreviewed = text(repo, &["rev-list", tip, "--not", &upstream])?
        .lines()
        .filter(|commit| !reviews.covered.contains(*commit))
        .map(str::to_owned)
        .collect::<Vec<_>>();
    let normalized = |path: &Path| {
        path.components()
            .filter(|part| !matches!(part, std::path::Component::CurDir))
            .collect::<std::path::PathBuf>()
    };
    let strategies = index
        .issues()
        .filter(|issue| issue.issue_type == "mission")
        .filter_map(|issue| issue.strategy.as_ref())
        .map(|strategy| normalized(Path::new(&strategy.path)))
        .collect::<BTreeSet<_>>();
    let mut checked = 0;
    // The install predates the gates: the first commit on the base's
    // first-parent line that adds the policy, with everything it merges in
    // (an install branch and its review fixes).
    let has_policy = |commit: &str| {
        git_freshness::git_output(
            repo,
            &[
                "ls-tree",
                commit,
                "--",
                workflow_policy::WORKFLOW_POLICY_PATH,
            ],
        )
        .is_ok_and(|listed| !listed.is_empty())
    };
    let mut installed = BTreeSet::new();
    let first_parent = text(
        repo,
        &[
            "rev-list",
            "--first-parent",
            "--reverse",
            tip,
            "--not",
            &upstream,
        ],
    )?;
    if let Some(install) = first_parent.lines().find(|commit| {
        has_policy(commit)
            && !resolve(repo, &format!("{commit}^")).is_ok_and(|parent| has_policy(&parent))
    }) {
        installed.insert(install.to_owned());
        let parent = format!("{install}^");
        let mut merged = vec!["rev-list", install, "--not", upstream.as_str()];
        if resolve(repo, &parent).is_ok() {
            merged.push(&parent);
        }
        installed.extend(text(repo, &merged)?.lines().map(str::to_owned));
    }
    for commit in &unreviewed {
        if installed.contains(commit) {
            continue;
        }
        // Commits made before the policy existed predate the gates too.
        let parent = format!("{commit}^");
        if !resolve(repo, &parent).is_ok_and(|parent| has_policy(&parent)) {
            continue;
        }
        checked += 1;
        let Some(changed) = introduced_paths(repo, commit)? else {
            continue;
        };
        let unreviewed_claim =
            reviews.unreviewed_claim(repo, &mut read, policy, commit, tip, &upstream, &changed)?;
        if let Some(id) = unreviewed_claim {
            bail!(
                "land_base_done_claim: commit {} on {base} marks {id} done outside review; a done claim is committed on the work branch with the work it describes. Undo it on {base} with `git revert {}`, or edit {id} back to its earlier status and commit, then run `atelier start {id}` and make the claim on the work branch. To retire work that will not be done, supersede it instead of closing it.",
                &commit[..8.min(commit.len())],
                &commit[..8.min(commit.len())],
            );
        }
        let other = changed
            .into_iter()
            .filter(|path| !git_freshness::is_record_path(path) && !strategies.contains(path))
            .collect::<Vec<_>>();
        if let Some(first) = other.first() {
            bail!(
                "land_base_unreviewed: commit {} on {base} changes {}{} without recorded review; only records and mission strategies may be committed on {base} directly. Move it to a work branch: `git switch -c <issue_type>/<id>`, `git branch -f {base} {base}@{{upstream}}`, then `atelier start <id>` there; get it reviewed and integrate it with `atelier merge`.",
                &commit[..8.min(commit.len())],
                first.display(),
                match other.len() {
                    1 => String::new(),
                    count => format!(
                        " and {}",
                        crate::human_output::count_noun(count - 1, "other path")
                    ),
                },
            );
        }
    }
    Ok(match checked {
        0 => "Base commits: none unpushed outside recorded review.".to_owned(),
        count => format!(
            "Base commits: {} outside review.",
            crate::human_output::count_noun(count, "unpushed record-only commit")
        ),
    })
}

/// The approvals and trivial exemptions that cover commits not yet on a
/// trusted point of the base (its upstream for `land`, its head for
/// `merge`): those of completed owners that branched from this base and are
/// not done at the trusted point, that `merge` would accept (atelier-n90q).
struct BaseReviews {
    /// Commits a covering pin contains.
    covered: BTreeSet<String>,
    /// Each covering pin with its approved scope, for done claims after it.
    approvals: Vec<(String, BTreeSet<String>)>,
}

impl BaseReviews {
    fn new(
        repo: &Path,
        index: &RecordIndex,
        policy: &atelier_workflow::WorkflowPolicy,
        read: &mut git_freshness::ReadContext<'_>,
        trusted: &str,
    ) -> Result<Self> {
        let base = &policy.branch_policy.base_branch;
        let mut reviews = Self {
            covered: BTreeSet::new(),
            approvals: Vec::new(),
        };
        for review in index.reviews().filter(|review| {
            matches!(
                review.event,
                atelier_core::ReviewEvent::Approval | atelier_core::ReviewEvent::TrivialExemption
            )
        }) {
            let Some(owner) = index.get_issue(&review.target) else {
                continue;
            };
            if !owner
                .workflow_branch
                .as_ref()
                .is_some_and(|branch| branch.branch_base == *base)
                || policy.status_category(&owner.status) != Some("done")
                || resolve(repo, &review.commit).is_err()
            {
                continue;
            }
            let path = format!(".atelier/issues/{}.md", owner.id);
            // An owner already done at the trusted point was integrated
            // before; a review recorded for it now covers nothing new.
            let pushed_done = git_freshness::blob(repo, trusted, Path::new(&path))
                .ok()
                .and_then(|blob| {
                    atelier_records::parse_issue_record(
                        &blob,
                        Path::new(&path).strip_prefix(".atelier").ok()?,
                    )
                    .ok()
                })
                .is_some_and(|record| policy.status_category(&record.issue.status) == Some("done"));
            if pushed_done || !review_gate::approval_accepted(index, read, review)? {
                continue;
            }
            let range = ["rev-list", review.commit.as_str(), "--not", trusted];
            reviews
                .covered
                .extend(text(repo, &range)?.lines().map(str::to_owned));
            reviews
                .approvals
                .push((review.commit.clone(), review_gate::scope(index, &owner.id)?));
        }
        Ok(reviews)
    }

    /// The first work item `commit` takes to done that no covering approval
    /// reviewed. A claim after a pin is reviewed while that approval, for a
    /// scope holding the item, is still current at the claim: the rule
    /// `merge`'s review.current applies (atelier-n642, atelier-uw4x). The tip
    /// stands for the base, so a clean merge of base commits keeps it current.
    #[allow(clippy::too_many_arguments)]
    fn unreviewed_claim(
        &self,
        repo: &Path,
        read: &mut git_freshness::ReadContext<'_>,
        policy: &atelier_workflow::WorkflowPolicy,
        commit: &str,
        tip: &str,
        trusted: &str,
        changed: &[std::path::PathBuf],
    ) -> Result<Option<String>> {
        'claims: for id in base_done_claims(repo, policy, commit, tip, trusted, changed)? {
            for (pin, ids) in self.approvals.iter().filter(|(_, ids)| ids.contains(&id)) {
                let follows =
                    git_freshness::git_output(repo, &["merge-base", "--is-ancestor", pin, commit])
                        .is_ok();
                if follows && review_gate::approval_is_current_to(read, pin, ids, commit, tip)? {
                    continue 'claims;
                }
            }
            return Ok(Some(id));
        }
        Ok(None)
    }
}

/// The paths `commit` introduces, or `None` for a clean merge, which
/// introduces nothing beyond its parents (atelier-n7g1). A merge (such as a
/// non-rebasing pull) introduces only the paths that differ from every parent.
fn introduced_paths(repo: &Path, commit: &str) -> Result<Option<Vec<std::path::PathBuf>>> {
    if git_freshness::clean_merge(repo, commit)? {
        return Ok(None);
    }
    if text(repo, &["rev-list", "--parents", "-n1", commit])?
        .split(' ')
        .count()
        > 2
    {
        let changed = text(
            repo,
            &[
                "diff-tree",
                "-r",
                "-c",
                "--no-commit-id",
                "--name-only",
                "-z",
                commit,
            ],
        )?;
        return Ok(Some(
            changed
                .split('\0')
                .filter(|path| !path.is_empty())
                .map(std::path::PathBuf::from)
                .collect(),
        ));
    }
    Ok(Some(git_freshness::changed_paths(
        repo,
        &format!("{commit}^"),
        commit,
    )?))
}

/// The first done claim among `tip`'s commits not on `trusted` that no
/// approval `merge` accepts covers, with its commit: what `land` on the base
/// would refuse once `tip` is there. `merge` asks this of the commit it would
/// put on the base, so it refuses before integrating (atelier-uw4x).
pub(crate) fn first_unreviewed_claim(
    repo: &Path,
    index: &RecordIndex,
    policy: &atelier_workflow::WorkflowPolicy,
    tip: &str,
    trusted: &str,
) -> Result<Option<(String, String)>> {
    let mut read = git_freshness::ReadContext::from_head(repo, tip.to_owned());
    let reviews = BaseReviews::new(repo, index, policy, &mut read, trusted)?;
    for commit in text(repo, &["rev-list", tip, "--not", trusted])?.lines() {
        if reviews.covered.contains(commit) {
            continue;
        }
        let Some(changed) = introduced_paths(repo, commit)? else {
            continue;
        };
        if let Some(id) =
            reviews.unreviewed_claim(repo, &mut read, policy, commit, tip, trusted, &changed)?
        {
            return Ok(Some((commit.to_owned(), id)));
        }
    }
    Ok(None)
}

/// The work items `commit` takes to done, compared with every parent, that
/// are still done at `tip` (a later revert withdraws the claim).
/// Missions close and decisions are decided on the base as planning, and
/// superseding retires work without a claim; anything else reaches done
/// through a reviewed work branch (atelier-fj1t).
fn base_done_claims(
    repo: &Path,
    policy: &atelier_workflow::WorkflowPolicy,
    commit: &str,
    tip: &str,
    upstream: &str,
    changed: &[std::path::PathBuf],
) -> Result<Vec<String>> {
    let mut claims = Vec::new();
    let parents = text(repo, &["rev-list", "--parents", "-n1", commit])?
        .split(' ')
        .skip(1)
        .map(str::to_owned)
        .collect::<Vec<_>>();
    // `None` when the record is absent, `Some(None)` when it does not parse.
    // A record from before the epic migration is read as migrated.
    let status_at = |revision: &str, path: &Path| -> Option<Option<(String, String)>> {
        let blob = git_freshness::blob(repo, revision, path).ok()?;
        let relative = path.strip_prefix(".atelier").ok()?;
        let parsed = atelier_records::parse_issue_record(&blob, relative)
            .ok()
            .or_else(|| {
                let migrated =
                    atelier_app::epic_migration::migrate_epic_text(&blob, "feature", "draft")
                        .ok()??;
                atelier_records::parse_issue_record(&migrated.text, relative).ok()
            });
        Some(parsed.map(|record| (record.issue.issue_type, record.issue.status)))
    };
    for path in changed {
        if path.parent() != Some(Path::new(".atelier/issues"))
            || path.extension().is_none_or(|ext| ext != "md")
        {
            continue;
        }
        let Some(Some((issue_type, status))) = status_at(commit, path) else {
            continue;
        };
        if matches!(issue_type.as_str(), "mission" | "decision")
            || status == "superseded"
            || policy.status_category(&status) != Some("done")
        {
            continue;
        }
        // Only a record that was absent or had another status is a new claim.
        // An earlier copy that does not parse cannot show the claim was
        // already made, so the pushed copy decides (atelier-hyew).
        let had_status = |revision: &str| match status_at(revision, path) {
            Some(Some((_, before))) => Some(before == status),
            Some(None) => None,
            None => Some(false),
        };
        // The nearest earlier copy that parses decides: unpushed commits
        // touching the record first, then the pushed copy.
        let earlier = |parent: &str| {
            text(
                repo,
                &[
                    "rev-list",
                    parent,
                    "--not",
                    upstream,
                    "--",
                    &path.to_string_lossy(),
                ],
            )
            .unwrap_or_default()
            .lines()
            .find_map(had_status)
            .or_else(|| had_status(upstream))
        };
        let new_claim = parents.iter().all(|parent| {
            had_status(parent)
                .or_else(|| earlier(parent))
                .is_none_or(|done| !done)
        });
        if new_claim
            && status_at(tip, path).is_some_and(|now| now.is_some_and(|(_, now)| now == status))
        {
            claims.extend(
                path.file_stem()
                    .and_then(|stem| stem.to_str())
                    .map(str::to_owned),
            );
        }
    }
    Ok(claims)
}

/// A done claim on a work branch is not base completion until the base
/// branch's own copy of the record carries it. Once prune removes that copy,
/// the claim counts as merged when the branch's committed record has it and
/// the branch tip is in the base (atelier-lwdu).
///
/// The cue after a done owner's item line: `merged into <base>` or `not
/// merged into <base>`.
pub(crate) fn done_merge_cue(
    repo: &Path,
    id: &str,
    status: &str,
    branch: &str,
    base_branch: &str,
) -> String {
    if done_merged(repo, id, status, branch, base_branch) {
        format!("merged into {base_branch}")
    } else {
        format!("not merged into {base_branch}")
    }
}

fn done_merged(repo: &Path, id: &str, status: &str, branch: &str, base_branch: &str) -> bool {
    let path = format!(".atelier/issues/{id}.md");
    let status_at = |commit: &str| {
        git_freshness::blob(repo, commit, Path::new(&path))
            .ok()
            .map(|blob| {
                atelier_records::parse_issue_record(
                    &blob,
                    Path::new(&path).strip_prefix(".atelier").ok()?,
                )
                .ok()
                .map(|record| record.issue.status)
            })
    };
    let base = resolve(repo, &format!("refs/heads/{base_branch}")).ok();
    base.as_deref().is_some_and(|base| match status_at(base) {
        Some(on_base) => on_base.as_deref() == Some(status),
        None => resolve(repo, &format!("refs/heads/{branch}")).is_ok_and(|tip| {
            status_at(&tip).flatten().as_deref() == Some(status)
                && git_freshness::git_output(repo, &["merge-base", "--is-ancestor", &tip, base])
                    .is_ok()
        }),
    })
}

/// An owner as its branch's committed records describe it.
struct BranchOwner {
    branch: String,
    id: String,
    title: String,
    status: String,
    /// The status's category under the branch's own policy.
    category: Option<String>,
    /// Set for a done owner: whether its claim is merged into the base.
    merged: Option<bool>,
    /// For an owner that is not done: how many of its direct children are
    /// active and done, so a parent-owned branch does not read as idle.
    children: Children,
    /// Set for a done owner not merged into the base: how it would merge
    /// now, or why that could not be read.
    readiness: Option<Result<Readiness, String>>,
}

#[derive(Default)]
struct Children {
    total: usize,
    active: usize,
    done: usize,
}

/// A waiting branch's readiness to merge into the current base
/// (atelier-tciw), read without changing any Git state.
struct Readiness {
    approval: review_gate::ApprovalState,
    /// Base commits the branch does not have.
    behind: usize,
    /// Every done item in the owner's scope, except superseded ones, has
    /// fresh passing proof at the branch head.
    proof: bool,
    /// The paths `git merge-tree` cannot merge into the current base.
    conflicts: Vec<PathBuf>,
}

impl Readiness {
    /// `, ✓ approved, behind 3, proof at head, merges cleanly`: the facts
    /// for the row's status part, the conflict list last so its commas end
    /// the part.
    fn facts(&self) -> String {
        let approval = match self.approval {
            review_gate::ApprovalState::Current => "✓ approved",
            review_gate::ApprovalState::Stale => "⚠ approval stale",
            review_gate::ApprovalState::Missing => "⚠ not approved",
        };
        let proof = if self.proof {
            "proof at head"
        } else {
            "no proof at head"
        };
        let merge = if self.conflicts.is_empty() {
            "merges cleanly".to_owned()
        } else {
            format!("conflicts: {}", display_paths(&self.conflicts))
        };
        format!(", {approval}, behind {}, {proof}, {merge}", self.behind)
    }
}

fn display_paths(paths: &[PathBuf]) -> String {
    paths
        .iter()
        .map(|path| path.display().to_string())
        .collect::<Vec<_>>()
        .join(", ")
}

impl BranchOwner {
    /// The status word with its ✓ or ⚠, as the checkout's own rows mark it.
    fn cue(&self) -> String {
        crate::human_output::status_cue(&self.status, self.category.as_deref(), None)
    }

    /// `<branch>  <id> <status>[, merged into <base>|, not merged[,
    /// <readiness>]][, <children>]  <title>`: one dense line. Merge-readiness
    /// facts belong in the status part, before the free-text title, and the
    /// merge wording matches the `↳` in-flight mark for the same item.
    fn line(&self, base: &str) -> String {
        let mut facts = match self.merged {
            Some(true) => format!(", merged into {base}"),
            Some(false) => ", not merged".to_owned(),
            None => String::new(),
        };
        match &self.readiness {
            Some(Ok(readiness)) => facts.push_str(&readiness.facts()),
            Some(Err(reason)) => facts.push_str(&format!(", readiness unknown: {reason}")),
            None => {}
        }
        let children = &self.children;
        if children.active + children.done > 0 {
            facts.push_str(&format!(
                ", {} of {} children done",
                children.done, children.total
            ));
            if children.active > 0 {
                facts.push_str(&format!(", {} active", children.active));
            }
        }
        format!(
            "{}  {} {}{facts}  {}",
            self.branch,
            self.id,
            self.cue(),
            self.title
        )
    }
}

/// A branch whose committed records could not be summarized.
struct UnreadableBranch {
    branch: String,
    /// Owners found before the failure, so their ready rows can still say so.
    owners: Vec<String>,
    error: anyhow::Error,
}

/// The committed state of the other local work branches, read once per
/// command and shared by the status branch section and the in-flight marks on
/// ready work. The current checkout's records are never overlaid.
pub(crate) struct BranchSummaries {
    base_branch: String,
    /// In branch order.
    entries: Vec<Result<BranchOwner, UnreadableBranch>>,
    /// Branches with no started work or no tracker, in branch order.
    unowned: Vec<String>,
    /// Each pair of waiting branches that change the same files, with them.
    shared: Vec<(String, String, Vec<PathBuf>)>,
}

/// Marks for ready rows owned on other branches, and notices for unreadable
/// branches whose owners are unknown.
#[derive(Default)]
pub(crate) struct InFlight {
    pub(crate) marks: BTreeMap<String, String>,
    pub(crate) notices: Vec<String>,
}

impl BranchSummaries {
    /// The status section's lines.
    pub(crate) fn lines(&self) -> Vec<String> {
        let mut lines = self
            .entries
            .iter()
            .map(|owner| match owner {
                Ok(owner) => owner.line(&self.base_branch),
                Err(unreadable) => {
                    format!(
                        "{}: unavailable — {:#}",
                        unreadable.branch, unreadable.error
                    )
                }
            })
            .collect::<Vec<_>>();
        if !self.unowned.is_empty() {
            lines.push(format!("Unowned: {}", self.unowned.join(", ")));
        }
        for (first, second, paths) in &self.shared {
            const SHOWN: usize = 5;
            let mut touched = display_paths(&paths[..paths.len().min(SHOWN)]);
            if paths.len() > SHOWN {
                touched.push_str(&format!(" and {} more", paths.len() - SHOWN));
            }
            lines.push(format!(
                "Shared files: {first} and {second} both touch {touched}"
            ));
        }
        lines
    }

    /// Each owner's state on its branches, keyed by ID: `in_progress on
    /// bug/atelier-j48s`, `done on bug/atelier-fj1t, not merged`, or `state
    /// unknown on task/x: <reason>` when the branch cannot be read.
    pub(crate) fn in_flight(&self) -> InFlight {
        let mut in_flight = InFlight::default();
        let mut mark = |id: &str, mark: String| {
            in_flight
                .marks
                .entry(id.to_owned())
                .and_modify(|marks| {
                    marks.push_str("; ");
                    marks.push_str(&mark);
                })
                .or_insert(mark);
        };
        let mut notices = Vec::new();
        for entry in &self.entries {
            match entry {
                Ok(owner) => mark(
                    &owner.id,
                    match owner.merged {
                        Some(true) => format!(
                            "{} on {}, merged into {}",
                            owner.cue(),
                            owner.branch,
                            self.base_branch
                        ),
                        Some(false) => format!("{} on {}, not merged", owner.cue(), owner.branch),
                        None => format!("{} on {}", owner.cue(), owner.branch),
                    },
                ),
                Err(unreadable) => {
                    let reason = short_reason(&unreadable.error);
                    if unreadable.owners.is_empty() {
                        notices.push(format!(
                            "State unknown on {}, owner not found: {reason}",
                            unreadable.branch
                        ));
                    }
                    for id in &unreadable.owners {
                        mark(
                            id,
                            format!("state unknown on {}: {reason}", unreadable.branch),
                        );
                    }
                }
            }
        }
        in_flight.notices = notices;
        in_flight
    }
}

pub(crate) fn branch_summaries(
    repo: &Path,
    state: &Path,
    live: &RecordIndex,
) -> Result<BranchSummaries> {
    let current = current_branch(repo)?.unwrap_or_default();
    let policy = workflow_policy::load(repo)?;
    branch_summaries_for(
        repo,
        state,
        live,
        &current,
        &policy.branch_policy.base_branch,
    )
}

fn branch_summaries_for(
    repo: &Path,
    state: &Path,
    live: &RecordIndex,
    current: &str,
    base_branch: &str,
) -> Result<BranchSummaries> {
    let heads = text(
        repo,
        &[
            "for-each-ref",
            "--format=%(refname) %(objectname)",
            "refs/heads/",
        ],
    )?;
    let base_head = resolve(repo, &format!("refs/heads/{base_branch}")).ok();
    let mut entries = Vec::new();
    let mut unowned = Vec::new();
    // The files each waiting branch changes since it left the base.
    let mut waiting = Vec::<(String, Vec<PathBuf>)>::new();
    for head in heads.lines() {
        let Some((reference, commit)) = head.split_once(' ') else {
            continue;
        };
        let branch = reference.trim_start_matches("refs/heads/");
        if branch == current || branch == base_branch {
            continue;
        }
        // Branches from before Atelier was initialised have no tracker to read.
        // `ls-tree` prints nothing for an absent path but still fails on
        // missing objects, which are reported below as real errors.
        let listed = git_freshness::git_output(
            repo,
            &[
                "ls-tree",
                commit,
                "--",
                workflow_policy::WORKFLOW_POLICY_PATH,
            ],
        );
        if listed.as_ref().is_ok_and(|output| output.is_empty()) {
            unowned.push(branch.to_string());
            continue;
        }
        // Legacy branches can contain thousands of obsolete records. Inspect
        // the small compatibility/owner blobs before materializing any tree.
        // Owners are found first, so a later failure can still name them.
        let owners = branch_owner_ids(repo, live, branch, commit);
        let view = (|| -> Result<Option<Vec<BranchOwner>>> {
            let yaml = git_freshness::blob(
                repo,
                commit,
                Path::new(workflow_policy::WORKFLOW_POLICY_PATH),
            )?;
            let branch_policy =
                atelier_workflow::parse_policy_text(&yaml, workflow_policy::WORKFLOW_POLICY_PATH)?;
            if let Ok(config) = git_freshness::blob(repo, commit, Path::new(".atelier/config.toml"))
            {
                atelier_app::project_config::parse_project_config(
                    &config,
                    Path::new(".atelier/config.toml"),
                )?;
            }
            let owners = match &owners {
                Ok(owners) if owners.is_empty() => return Ok(None),
                Ok(owners) => owners.clone(),
                Err(error) => bail!("{error:#}"),
            };
            let snapshot = Snapshot::at(repo, state, commit)?;
            snapshot
                .index
                .validate_policy(&branch_policy, snapshot.root())?;
            let mut rows = Vec::new();
            // Read once per branch, for its first waiting owner.
            let mut merge = None;
            for id in owners {
                let owner = snapshot.index.require_issue(&id)?;
                let category = branch_policy.status_category(&owner.status);
                let merged = (category == Some("done"))
                    .then(|| done_merged(repo, &id, &owner.status, branch, base_branch));
                let mut children = Children::default();
                if category != Some("done") {
                    for child in snapshot.index.child_ids(&id) {
                        let child = snapshot.index.require_issue(child)?;
                        children.total += 1;
                        match branch_policy.status_category(&child.status) {
                            Some("active") => children.active += 1,
                            Some("done") => children.done += 1,
                            _ => {}
                        }
                    }
                }
                let readiness = match (merged, &base_head) {
                    (Some(false), Some(base_head)) => Some(
                        (|| -> Result<Readiness> {
                            if merge.is_none() {
                                merge = Some(branch_merge(repo, commit, base_head)?);
                            }
                            let merge = merge.as_ref().expect("branch merge read");
                            readiness(repo, &snapshot.index, &branch_policy, &id, commit, merge)
                        })()
                        .map_err(|error| short_reason(&error)),
                    ),
                    _ => None,
                };
                rows.push(BranchOwner {
                    branch: branch.to_owned(),
                    title: owner.title.clone(),
                    status: owner.status.clone(),
                    category: category.map(str::to_owned),
                    id,
                    merged,
                    children,
                    readiness,
                });
            }
            if let Some(merge) = merge {
                waiting.push((branch.to_owned(), merge.changed));
            }
            Ok(Some(rows))
        })();
        match view {
            Ok(Some(rows)) => entries.extend(rows.into_iter().map(Ok)),
            Ok(None) => unowned.push(branch.to_string()),
            Err(error) => entries.push(Err(UnreadableBranch {
                branch: branch.to_string(),
                owners: owners.unwrap_or_default(),
                error,
            })),
        }
    }
    let mut shared = Vec::new();
    for (index, (first, first_paths)) in waiting.iter().enumerate() {
        for (second, second_paths) in &waiting[index + 1..] {
            let both = first_paths
                .iter()
                .filter(|path| second_paths.binary_search(path).is_ok())
                .cloned()
                .collect::<Vec<_>>();
            if !both.is_empty() {
                shared.push((first.clone(), second.clone(), both));
            }
        }
    }
    Ok(BranchSummaries {
        base_branch: base_branch.to_owned(),
        entries,
        unowned,
        shared,
    })
}

/// How a branch at `tip` meets the base at `base_head`; the same for every
/// owner of the branch.
struct BranchMerge {
    /// Where the branch left the base.
    fork_point: String,
    behind: usize,
    conflicts: Vec<PathBuf>,
    /// The paths the branch changes since `fork_point`, sorted.
    changed: Vec<PathBuf>,
}

/// Reads the branch's merge into the base with `git merge-tree`, which
/// writes only unreachable objects, so no ref, index or checkout changes.
fn branch_merge(repo: &Path, tip: &str, base_head: &str) -> Result<BranchMerge> {
    let fork_point = text(repo, &["merge-base", base_head, tip])?;
    let behind = text(
        repo,
        &["rev-list", "--count", &format!("{tip}..{base_head}")],
    )?
    .parse()
    .context("git rev-list --count printed no count")?;
    let conflicts = if behind == 0 {
        Vec::new()
    } else {
        git_freshness::merge_tree(repo, base_head, tip)?
            .context("git merge-tree could not merge the branch into the base")?
            .1
    };
    let changed = git_freshness::changed_paths(repo, &fork_point, tip)?;
    Ok(BranchMerge {
        fork_point,
        behind,
        conflicts,
        changed,
    })
}

/// The owner's readiness on a branch at `tip`, judged from the branch's
/// committed records: approval as `merge`'s `review.current` judges it, and
/// whether every done item in the owner's scope has fresh passing proof at
/// the tip. Superseded work needs no proof, as `merge` asks none of it.
fn readiness(
    repo: &Path,
    index: &RecordIndex,
    policy: &atelier_workflow::WorkflowPolicy,
    owner: &str,
    tip: &str,
    merge: &BranchMerge,
) -> Result<Readiness> {
    let mut read = git_freshness::ReadContext::from_snapshot(repo, tip.to_owned(), &[]);
    let approval = review_gate::approval_state(index, &mut read, owner, &merge.fork_point)?;
    let mut proof = true;
    for id in review_gate::scope(index, owner)? {
        let issue = index.require_issue(&id)?;
        if policy.issue_status_is_terminal(&issue.issue_type, &issue.status)?
            && issue.status != "superseded"
            && !read.issue_gate(index, &id, 1, None)?.passed
        {
            proof = false;
            break;
        }
    }
    Ok(Readiness {
        approval,
        behind: merge.behind,
        proof,
        conflicts: merge.conflicts.clone(),
    })
}

/// The issues a branch's committed records say own it, read from their small
/// record blobs without materializing the tracker.
fn branch_owner_ids(
    repo: &Path,
    live: &RecordIndex,
    branch: &str,
    commit: &str,
) -> Result<Vec<String>> {
    let mut paths = live
        .issues()
        .filter(|issue| {
            issue
                .workflow_branch
                .as_ref()
                .is_some_and(|owner| owner.work_branch == branch)
        })
        .map(|issue| format!(".atelier/issues/{}.md", issue.id))
        .collect::<BTreeSet<_>>();
    if let Some(id) = branch
        .rsplit('/')
        .next()
        .filter(|id| atelier_records::validate_record_id(id).is_ok())
    {
        paths.insert(format!(".atelier/issues/{id}.md"));
    }
    let mut owners = Vec::new();
    for attempt in 0..2 {
        for path in &paths {
            if let Ok(blob) = git_freshness::blob(repo, commit, Path::new(path)) {
                let record = atelier_records::parse_issue_record(
                    &blob,
                    Path::new(path).strip_prefix(".atelier")?,
                )?;
                if record
                    .issue
                    .workflow_branch
                    .as_ref()
                    .is_some_and(|owner| owner.work_branch == branch)
                {
                    owners.push(record.issue.id);
                }
            }
        }
        if !owners.is_empty() || attempt == 1 {
            break;
        }
        // A custom branch can introduce its owner only on that branch.
        let found = text(
            repo,
            &[
                "grep",
                "-l",
                "^workflow_branch:",
                commit,
                "--",
                ".atelier/issues/",
            ],
        )
        .unwrap_or_default();
        paths = found
            .lines()
            .filter_map(|line| line.split_once(':').map(|(_, path)| path.into()))
            .collect();
    }
    Ok(owners)
}

/// The first line of an error's outermost context, for a one-line mark.
fn short_reason(error: &anyhow::Error) -> String {
    let reason = error.to_string();
    crate::utils::truncate(reason.lines().next().unwrap_or_default(), 100)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    fn git(repo: &Path, args: &[&str]) -> String {
        text(repo, args).unwrap()
    }

    #[test]
    fn native_checkout_queries_preserve_recursive_heads_and_refresh_linked_checkouts() {
        let repo = tempfile::tempdir().unwrap();
        for args in [
            vec!["init", "-qb", "main"],
            vec!["config", "user.name", "Author"],
            vec!["config", "user.email", "author@example.test"],
        ] {
            git(repo.path(), &args);
        }
        assert!(git_freshness::head_commit(repo.path()).is_err());
        fs::write(repo.path().join("code"), "base").unwrap();
        git(repo.path(), &["add", "."]);
        git(repo.path(), &["commit", "-qm", "base"]);
        let base = git_freshness::head_commit(repo.path()).unwrap();
        git(repo.path(), &["checkout", "-qb", "task/source"]);
        fs::write(repo.path().join("code"), "owner").unwrap();
        git(repo.path(), &["commit", "-qam", "owner"]);
        let original = git_freshness::head_commit(repo.path()).unwrap();
        git(
            repo.path(),
            &["tag", "-am", "shadow source", "task/source", &base],
        );
        // Git 2.53+ refuses `git tag HEAD`; older repositories may still hold
        // this ref, so write it directly.
        git(repo.path(), &["update-ref", "refs/tags/HEAD", &base]);
        git(
            repo.path(),
            &["symbolic-ref", "refs/heads/alias", "refs/heads/task/source"],
        );
        git(repo.path(), &["symbolic-ref", "HEAD", "refs/heads/alias"]);
        assert_eq!(
            current_branch(repo.path()).unwrap().as_deref(),
            Some("task/source")
        );
        assert_eq!(git_freshness::head_commit(repo.path()).unwrap(), original);
        fs::write(
            repo.path().join(".git/refs/heads/broken"),
            format!("{}\n", "0".repeat(40)),
        )
        .unwrap();
        assert_eq!(git_freshness::head_commit(repo.path()).unwrap(), original);
        assert!(resolve(repo.path(), "refs/heads/broken").is_err());
        fs::remove_file(repo.path().join(".git/refs/heads/broken")).unwrap();
        #[cfg(unix)]
        {
            use std::os::unix::ffi::OsStringExt;
            let opaque = repo
                .path()
                .join(".git/refs/heads")
                .join(std::ffi::OsString::from_vec(vec![0xff]));
            fs::write(opaque, format!("{base}\n")).unwrap();
            assert_eq!(git_freshness::head_commit(repo.path()).unwrap(), original);
            assert_eq!(
                current_branch(repo.path()).unwrap().as_deref(),
                Some("task/source")
            );
        }
        let worktree = tempfile::tempdir().unwrap();
        let linked = worktree.path().join("linked");
        git(
            repo.path(),
            &[
                "worktree",
                "add",
                "-q",
                "-b",
                "linked",
                linked.to_str().unwrap(),
                &base,
            ],
        );
        assert_eq!(current_branch(&linked).unwrap().as_deref(), Some("linked"));
        assert_eq!(git_freshness::head_commit(&linked).unwrap(), base);
        let whitespace_branch = "task/unicode\u{3000}";
        git(
            repo.path(),
            &["branch", whitespace_branch, "refs/heads/task/source"],
        );
        git(
            repo.path(),
            &[
                "symbolic-ref",
                "HEAD",
                &format!("refs/heads/{whitespace_branch}"),
            ],
        );
        assert_eq!(
            checkout_branch(repo.path()).unwrap().as_deref(),
            Some(whitespace_branch)
        );
        assert_eq!(
            current_branch(repo.path()).unwrap().as_deref(),
            Some("task/unicode")
        );
        git(
            repo.path(),
            &["symbolic-ref", "HEAD", "refs/heads/task/source"],
        );
        git(repo.path(), &["tag", "-d", "HEAD"]);
        git(repo.path(), &["checkout", "--detach", &original]);
        assert!(current_branch(repo.path()).unwrap().is_none());
        git(
            repo.path(),
            &["symbolic-ref", "HEAD", "refs/tags/task/source"],
        );
        assert!(current_branch(repo.path()).is_err());
        git(
            repo.path(),
            &["symbolic-ref", "HEAD", "refs/heads/task/source"],
        );
        fs::write(repo.path().join("code"), "new state").unwrap();
        git(repo.path(), &["commit", "-qam", "fresh ref state"]);
        assert_ne!(git_freshness::head_commit(repo.path()).unwrap(), original);
        let tag = git(repo.path(), &["rev-parse", "refs/tags/task/source"]);
        fs::write(
            repo.path().join(".git/refs/heads/tagged"),
            format!("{tag}\n"),
        )
        .unwrap();
        git(repo.path(), &["symbolic-ref", "HEAD", "refs/heads/tagged"]);
        assert_eq!(git_freshness::head_commit(repo.path()).unwrap(), tag);
    }

    #[test]
    fn native_base_refs_peel_annotated_objects_in_both_hash_formats() {
        for format in ["sha1", "sha256"] {
            let repo = tempfile::tempdir().unwrap();
            git(
                repo.path(),
                &["init", "-qb", "main", &format!("--object-format={format}")],
            );
            git(repo.path(), &["config", "user.name", "Author"]);
            git(
                repo.path(),
                &["config", "user.email", "author@example.test"],
            );
            fs::write(repo.path().join("code"), "base").unwrap();
            git(repo.path(), &["add", "."]);
            git(repo.path(), &["commit", "-qm", "base"]);
            let head = git_freshness::head_commit(repo.path()).unwrap();
            git(repo.path(), &["tag", "-am", "annotated", "base-tag"]);
            git(
                repo.path(),
                &[
                    "symbolic-ref",
                    "refs/heads/base-alias",
                    "refs/tags/base-tag",
                ],
            );
            assert_eq!(head.len(), if format == "sha1" { 40 } else { 64 });
            assert_eq!(resolve(repo.path(), "refs/heads/base-alias").unwrap(), head);
        }
    }
}
