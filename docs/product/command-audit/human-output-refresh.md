# Human Output Refresh Audit

This audit records formatting complaints found by sampling the current command
surface before the UX refresh. It is about default human output, not quiet mode
or machine contracts.

For evidence from actual Codex agents using the commands, see
[actual agent complaints](agent-complaints.md). That pass adds trust-state,
guidance, and command-language failures that are adjacent to formatting but not
solved by layout alone.

Sampled commands:

- `target/debug/atelier status`
- `target/debug/atelier issue list`
- `target/debug/atelier issue list --ready`
- `target/debug/atelier issue show atelier-kpa1`
- `target/debug/atelier issue blocked`
- `target/debug/atelier issue blocked atelier-t3h3`
- `target/debug/atelier issue table --kind mission`
- `target/debug/atelier issue status atelier-24xn --verbose`
- `target/debug/atelier issue transition atelier-kpa1 --options`
- `target/debug/atelier history --issue atelier-kpa1 --limit 10`
- `target/debug/atelier evidence list`
- `target/debug/atelier man worker`
- `target/debug/atelier man manager`

## Global Complaints

| Complaint | Evidence | Target behavior |
| --- | --- | --- |
| Inline help is repeated per row. | `issue list` and `issue blocked` repeat `details: atelier issue blocked <id>` on many rows. | Move repeated drill-down commands to one footer. Rows may show a short marker such as `blocked by 4`, but command syntax belongs in `Next Commands` or `Drill Down`. |
| Summary lines overuse `key=value`. | Queue summaries print `Category: todo=10`, `Status: ready=2, todo=4`, and similar count blobs. | Human summaries should read as labels and counts, not telemetry. Prefer `10 total · 6 blocked · status: 2 ready, 4 todo` or a compact vertical summary when several dimensions matter. Keep `key=value` for quiet output, logs, diagnostics, or machine-adjacent snippets. |
| Some row labels are opaque. | `context; parent blocked` does not explain what action is blocked or why the parent is rendered as a context group. | Replace implementation labels with domain language: `parent blocked`, `shown for context`, or `blocked through parent`, and put the reason near the grouped parent. |
| Blocker rows show IDs without enough human meaning. | Long blocker lists emphasize issue IDs and repeated commands more than blocker titles or blocker counts. | Inline blockers should answer "what is blocking this?" with title, type, status, and count. IDs remain visible but secondary. Large blocker sets should be summarized with one footer command for full detail. |
| Dirty checkout state becomes an unreadable line. | `issue show` and `issue transition --options` print every dirty path in one long `State:` or `dirty checkout:` line, and `transition --options` repeats that list. | Show a short classified summary first, such as `dirty: 30 tracked, 10 untracked`. Print at most a bounded sample and one command (`git status --short --branch`) for full detail. Do not repeat the same dirty-path list in multiple sections of one output. |
| Activity output leaks raw event fields. | `issue show` recent activity prints YAML-like fields such as `evidence_id:`, `result:`, `branch:`, and `review_artifact_provider:`. | Detail views should show concise event sentences. Raw fields belong behind a verbose flag or in focused history output when needed. |
| History rows are dense pipe-delimited transcripts. | `history --issue` prints timestamp, event kind, actor, target, title, and summary on one long line. | Use a scan-friendly activity list with date grouping or wrapped two-line rows. Keep actor/target metadata visible but de-emphasized. |
| `evidence list` has no useful default information budget. | The current repository prints 713 records by default, many with multi-line command transcripts. | Default evidence list must be bounded, grouped, and filter-oriented. Show newest or most relevant records first, summarize omitted counts, and route broad exploration to filters. |
| Tables mix fixed columns with long free text. | `issue table --kind mission` is readable in this checkout, but long titles or more columns will break narrow terminals. | Keep short fields in columns and make title the last wrapping field. Prefer grouped vertical rows when terminal width is unknown or columns exceed the budget. |
| Section headings are visually heavy in queue output. | Parent groups and standalone groups use full underline separators even when the section is only a small context wrapper. | Detail views may keep strong headings. Queue views should use quieter group headers and reserve heavy headings for the command title or major sections. |
| Next commands are too literal and too numerous. | Detail views print many command lines without ranking by the current problem. | Footer actions should be intent-labeled, ranked, and deduplicated. Put the command after a short action label, and include only commands likely to be useful from the current state. |
| Color is unused. | All sampled output is monochrome, so status, danger, secondary metadata, and headings compete equally. | Add color only behind shared terminal styling: automatic in interactive terminals, disabled when not interactive or when `NO_COLOR` is set. Color must reinforce text, not replace it. |

## Color Contract

Color should be shared formatter behavior, not a per-command flourish.

- Enable color only for interactive terminals by default.
- Disable color for pipes, redirected output, tests that do not explicitly opt
  in, and when `NO_COLOR` is set.
- Use color to reinforce status categories: ready/success, blocked/danger,
  active/in-progress, stale/warning, and secondary metadata.
- De-emphasize headings, separators, paths, IDs after the first mention, and
  repeated context.
- Do not use color as the only signal; colorless output must remain complete.
- Add a future `--color=auto|always|never` only as a deliberate CLI-surface
  artifact, not as a hidden dependency for this refresh.

## Layout Contract

The UX refresh should converge on a shared formatter boundary.

- One command title, then the information needed for the current job.
- Summaries use human words and counts; reserve `key=value` for quiet or
  diagnostic surfaces.
- Lists are bounded by default and state omitted counts.
- Drill-down commands appear once in a footer, grouped by intent.
- Repeated context is collapsed or referenced once.
- Long path lists, command transcripts, and raw activity fields require an
  explicit focused command, filter, or verbose mode.
- Queue views optimize for actionability: ready, blocked, active, backlog,
  done; then priority and parent context.
- Detail views optimize for diagnosis: identity, state, blockers, evidence,
  recent human-readable activity, then next actions.

## Command-Specific Debt

| Surface | Complaint | Target behavior |
| --- | --- | --- |
| `status` | Output is mostly useful but still uses dense labels and a long generic evidence warning. | Keep it compact, color status/health in interactive terminals, and keep next actions ranked by the current checkout state. |
| `issue list` and `search` | Repeated blocker drill-downs, `key=value` summaries, heavy context headings, and opaque labels. | Use grouped rows with readable summaries, one blocker drill-down footer, clear parent/context labels, and bounded child rows. |
| `issue show` | Dirty checkout state and recent activity are too raw and verbose. | Summarize dirty state, wrap or bound path samples, render recent activity as sentences, and move raw fields to history or verbose output. |
| `issue status <objective-id>` | Objective status repeats blocker drill-down commands and uses pipe-heavy row metadata such as parent/proof notes. | Keep objective health first, group ready/blocked work cleanly, move repeated blocker commands to a footer, and de-emphasize validator/proof metadata unless verbose output asks for it. |
| `issue transition --options` | Repeats dirty state and mixes validation, blockers, branch context, commands, and descriptions with equal visual weight. | Start with the allowed/blocked decision, then show blockers and required inputs, then planned actions and one command per transition. Collapse repeated branch context. |
| `issue blocked` | Repeats detail commands and does not prioritize blocker meaning. | Show blocker counts and human titles first; put `atelier issue blocked <id>` once in a footer. |
| `issue table` | Columns are useful but fragile for long titles and narrow terminals. | Keep short columns, wrap the title field, and use color/dim styling for secondary counts. |
| `history` | Pipe-delimited rows are hard to scan. | Group or wrap events, de-emphasize repeated scope/target data, and keep filters visible. |
| `evidence list` | Default output is unbounded and swamps the terminal. | Add a default limit, grouping/filter hints, omitted count, and command transcript elision. |
| `man` | Role guides are readable but still use raw command lists and stale objective wording in places. | Keep guides terse, use objective terminology, and color only headings/roles/health when interactive. |
| `review`, `forgejo`, `branch`, `doctor`, `lint`, `prune`, `maintenance` | Less frequently sampled in this pass, but they should still use the shared footer, color, and bounded-list rules. | Audit implementation output before each command is refreshed; do not invent a separate style per command. |
