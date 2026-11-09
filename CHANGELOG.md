# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/).

## [0.4.0] - 2026-11-09

Reviewed work now reaches a published base in one command, a site shows each
mission's brief and debrief, and the work lifecycle has fewer records and
steps. Repositories written by 0.3.x keep working unmigrated; the `migrate`
commands below are optional cleanup.

### Added
- `atelier serve`: a read-only site with every mission's brief, debrief and
  strategy as pages and JSON, for every Atelier repository under the given
  roots, read from each one's base branch. The release binary embeds the web
  app.
- `merge --push --wait`: with a `forge` block in the workflow policy, one
  command pushes the work branch, reuses or opens its pull request, waits for
  CI, merges, pushes the base and confirms the pull request merged. A new
  `ci.passed` merge check refuses code that has no passing CI run on its
  latest commit, using `gh` (ADR 0031).
- A change with no issue can be reviewed and merged: `review request` and
  `merge` with no ID, and `review approve --branch`.
- Loose ends (`--issue-type loose_end`) track work that can only happen after
  a merge or release, outside any mission. `atelier migrate after-merge` turns
  open After merge lines into loose ends.
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

### Removed
- `brief --html` and `debrief --html`. Use `atelier serve`; when
  `~/.config/atelier/site-url` names the site, brief and debrief footers link
  to the mission's page.
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
([ADR 0019](docs/adr/0019-verified-repo-native-tracker.md)). It replaces the
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
