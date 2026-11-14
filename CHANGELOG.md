# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/).

## [0.6.0] - 2026-11-14

Every change now reaches the base through a branch and a merge, so Atelier
works where the base is protected, and work in motion shows from any clone.
0.6.0 reads a 0.5 repository's records as they are: no migration, and the
hooks `atelier setup` installed keep working, since they call the binary.

### Added
- The branch model. Every change lives on a branch that `atelier start`
  makes: `<item-id>` for an item, `<mission-id>` for a new mission's plan
  (`start --mission "<title>"`), and `backlog/<slug>` for record changes tied
  to neither (`start --backlog <slug>`). `start --new "<title>"` creates an
  item already started, for work asked for directly. An optional
  `branch_prefix` in `config.toml` prefixes every branch name.
- A mission's plan is written on its branch and merges once two plan reviews
  approve it. There is no go-ahead step after.
- `merge` finishes through the forge's merge of the pull request where a
  forge is configured, so it works on a protected base. A backlog branch,
  which changes only records, merges once `check` passes on the merged tree,
  with no review.
- `list` and `show` read missions and items from every named branch the clone
  has, local and fetched, and say where each was read from, so a plan on a
  branch or an item started on another host appears.
- `atelier log`: the history of acts, read from Git, one line each, of
  everything, of one item, or of one mission with its plan reviews and items.
- `list --disputed` lists each finding an implementer has rebutted.

### Changed
- Only `start`, `submit` and `merge` act on Git. `new`, `set`, `ready`,
  `drop`, `close`, `lead`, `review` and `verdict` write files and commit
  nothing; commit and push their changes with Git.
- With a forge configured, the pre-push hook refuses every push that moves
  the base, since the forge makes its merges. Without one, it accepts only
  Atelier's merge commits on the base.
- `status` tells a thread more of what it holds and what to run next, in six
  places first use showed it fell short.

### Removed
- `escalate` and `decide`. A question goes to the thread that started the
  work, as advice; a disputed finding is rebutted in the Report's Responses
  and a fresh reviewer keeps or withdraws it.
- `new --mission`; use `start --mission`.

## [0.5.0] - 2026-11-11

The tracker is rewritten from scratch around one loop: start, submit,
review, verdict, merge. `atelier status` tells each agent thread what it
holds and the next command. Missions and items are directories under
`.atelier/missions/` and `.atelier/items/`, each review is one file beside
its item, and `.atelier/config.toml` replaces the workflow file. Git hooks
installed by `atelier setup` guard the base, and `ATELIER_SKIP="<reason>"`
passes any rule on purpose. `atelier serve`, evidence, parents, `land` and
the `work` views and the `review` subcommands are gone; the
[design record](docs/design.md) lists what was removed and why.

0.5.0 cannot read a 0.4 repository's records. Every repository on a host
moves at once, with one cutover per host: a 0.4 repository goes across by
[`scripts/cutover/README.md`](scripts/cutover/README.md).

## [0.4.0] - 2026-11-09

Reviewed work now reaches a published base in one command, a site shows each
mission's brief and debrief, and the work lifecycle has fewer records and
steps. Repositories written by 0.3.x keep working unmigrated, with one
exception: `[decision one-way]` notes must be rewritten (see Removed). The
`migrate` commands below are optional cleanup.

### Added
- `atelier serve`: a read-only site with every mission's brief, debrief and
  strategy as pages and JSON, for every Atelier repository under the given
  roots, read from each one's base branch. The release binary embeds the web
  app.
- The site also shows a board for each repository (in-progress work with its
  review state, ready work, what needs a human, and work done in the last 14
  days) and a page for each item.
- `merge --push --wait`: with a `forge` block in the workflow policy, one
  command pushes the work branch, reuses or opens its pull request, waits for
  CI, merges, pushes the base and confirms the pull request merged. A new
  `ci.passed` merge check, using `gh`, refuses a branch until CI has passed
  on its latest code; record-only commits after that run, and branches that
  change only tracker records, need no new run (ADR 0031).
- A change with no issue can be reviewed and merged: `review request` and
  `merge` with no ID, and `review approve --branch`.
- Loose ends (`--issue-type loose_end`) track work that can only happen after
  a merge or release, outside any mission. Once the workflow policy declares
  `loose_end`, `atelier migrate after-merge --apply` turns open After merge
  lines into loose ends; without `--apply` it only reports them.
- Review can say no: a won't-fix decision, a `declined` status, and deferring
  an item's findings in one command.
- `setup --link` links the Agent Factory skill to a checkout of this
  repository, which now holds the skill and its evals.
- `doctor` and `status` warn when the installed binary or skill is out of
  date, without blocking.
- `check` catches plan records with no Done when lines and broken links.
- `merge --check` and the review brief show a Done when line edited after the
  item started.
- Commands that take a commit accept a unique short ID.

### Changed
- Work has one order and no close step: start, implement, tick every Done
  when line and write the `## Report`, `review request` (which refuses until
  both hold), approve, then `merge`, whose merge commit marks the work done.
  `## Close Reason` is read as `## Report`.
- `start` on the base creates a branch named with the bare issue ID (no type
  prefix such as `task/`), and on any other unowned branch it takes that
  branch. `merge` targets the policy's `base_branch`; the recorded
  `branch_base` is still written but no longer read.
- A record write that `atelier check` would refuse is refused before any
  file changes (`record_invalid`). Review records can be corrected until they
  reach the base.
- A review finding carries its disposition as a field. Separate disposition
  events still read correctly; `atelier migrate dispositions --apply` folds
  them in.
- An approval of a later commit resolves a `fix_now` finding with no
  disposition, and an approval is judged by the code it saw, not by record
  commits around it.
- Evidence receipts record only what nothing else records, and no gate reads
  them. A pass after a failure clears it. `merge --sync` no longer reruns or
  commits receipts.
- `land` on a work branch reports what is open and exits successfully; on
  the base a refusal still stands.
- `init` refuses outside Git, picks the base branch (origin's HEAD, else
  `main` or `master`) instead of always writing `main`, and prints the commit
  that installs Atelier.
- The starter policy drops the unused `blocked` and `review` statuses.
  Policies written from the earlier starter run unchanged.
- Untracked files no longer hold up `merge` or `land`, unless merging would
  overwrite one.
- `debrief` walks history once: 7.4 s to under 1.5 s on a 45-item mission.
  Briefs show a mission's sizing and derisking first, and debriefs show each
  done child's report.
- Mission success criteria are optional.
- macOS is no longer supported. CI tests only on Linux, and releases include
  Linux binaries only (x86_64 and aarch64, glibc and musl).

### Removed
- `brief --html` and `debrief --html`. Use `atelier serve`; when
  `~/.config/atelier/site-url` names the site, brief and debrief footers link
  to the mission's page.
- `issue note --one-way` and the `[decision one-way]` note tag. Existing
  `[decision one-way]` notes now fail `atelier check`; rewrite each as a plain
  `[decision]` note, and record a costly choice as a decision record instead.
- `start --base` no longer chooses where the branch is created from. It is
  still accepted, prints a deprecation note and is ignored, so starting from
  a detached HEAD with `--base` now refuses when it would need to create a
  new branch.
- `land`'s warning about unpushed base commits.

### Fixed
- `doctor` no longer fails on review records left in the pre-Markdown YAML
  format by an earlier migration.
- Ticking an after-merge check on the base no longer makes `land` refuse
  reviewed items.
- `land` no longer refuses a done validation item whose planning approval
  predates unrelated code.

## [0.3.1] - 2026-11-04

### Fixed
- The shell installer works on Linux hosts with glibc older than 2.35 (such
  as RHEL 9). Releases now include static musl archives for x86_64 and
  aarch64 Linux, which the installer picks when the system glibc is too old
  for the glibc builds.

## [0.3.0] - 2026-10-28

The first published release. Atelier is a verified, repo-native work tracker:
missions, issues, evidence and reviews are Markdown records under `.atelier/`,
committed with the code they describe, and checked at the point of action
(ADR 0019, since replaced by the design record). It replaces the
earlier SQLite-backed tracker; there is no database or cache.

### Added
- Records: `init`, `issue` (create, show, update, transition, note, link),
  `bundle` for multi-record graphs, and `check` to validate records and the
  repository's workflow policy (`.atelier/workflow.yaml`).
- Orientation: `status` and the `work` views (ready, human, blocked, active,
  missions, mission, parent, decisions).
- Proof and review: commit-pinned `evidence` receipts, and `review`
  approvals, findings, dispositions and trivial exemptions.
- Integration: `start` prepares a work branch; `merge` integrates reviewed
  work only after the base branch's policy gates pass; `land` checks the
  handoff; `doctor` rechecks every done claim.
- Mission reviews: `brief` and `debrief`, as Markdown or a static HTML page.
- Claude Code hooks via `setup claude`, running `status` at session start and
  `land` at stop.
- Prebuilt Linux (x86_64, aarch64) and macOS (arm64, x86_64) binaries with a
  shell installer, published as GitHub Releases.
