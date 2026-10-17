# Human CLI Output

Normal output should let an operator identify the work, understand its state,
and choose the next concrete action. Markdown records are the durable machine
boundary; human output is a focused view over them.

## Dense Output

Output is dense Markdown for every reader (ADR 0022). Agents read redirected
output as plain text, so every token they read should carry a fact:

- One line per item.
- A symbol that reads without a legend goes beside the status word, never in
  place of it: ✓ for done, ⚠ for an item that needs attention, such as
  blocked work.
- No token a heading already states. A section named for a work state, such
  as Ready Work or Done Work, leaves the state off its rows; only a list that
  mixes states leads a row with its state, and not when the status word
  already says it. A page title that names the status needs no `Status:` line.
- Detail views are brief by default, with a flag such as `--full` for the
  whole record.
- Views gain no JSON or YAML mode. `--quiet` and the `--json` flags on
  `review list` and `merge --check` are the interfaces for scripts.

When agents routinely pipe atelier output through `head`, `grep`, `sed`, or
loops to find what they need, that is a missing view: file it as a defect in
the views rather than adding a machine format.

## Output Contract

Every command writes the same shapes. Rendering tests pin them, and the shared
helpers in `crates/atelier-cli/src/human_output.rs` implement them, so a view
fills in fields rather than writing its own format.

### Item Line

One item is one line:

```
[<lead> ]<id>[ <type>] <status> <priority> <title>[ | <cue>]...
```

- `<status>` is the exact configured status, marked ✓ or ⚠ as above. A view
  that shows the work state instead, such as a parent's children, puts the
  state word there.
- `<type>` appears only where types mix: `issue list`, and a record's own
  title line (`issue show`, `work mission`, `work parent`). It is a plain
  word.
- `<lead>` names what a row is in a list that mixes kinds of rows: the work
  state in a list that mixes states (`ready atelier-x todo high ...`), or the
  relationship in a list of links (`advances atelier-x backlog low ...`).
- Each `| <cue>` adds one fact about the item: `| 2 blockers`,
  `| open blocker`, `| blockers: atelier-a, atelier-b`, `| merged into
  master`, `| not merged into master`, `| closed 2026-10-15`, `| 1 active`.
- Nothing is bracketed. Brackets never carry a status, type, category, work
  state, record kind or decision.

A row too wide for the terminal continues on lines indented past the rows, so
the continuation never reads as another item. Redirected output has no width
limit.

The fields come from one fixed vocabulary in one fixed order, so a built-in
view is a choice of rows, fields and grouping over the shared order. That
keeps views expressible as declarations later (atelier-e95f).

Every view that lists items uses this line: the `work` views, `status`
(Current Work, Owner, Mission, Owner Work, Ready Work, unassigned work and the
other-branch rows), `work missions`, `issue list`, and `issue show` (title
line, Mission, Parent, Children, Blocked by, Blocking, Linked Issues, Impact,
Subissues and the objective rollup). So the Current Work row and the Owner
row in `status` read alike. A record that is not an issue uses the same
order without a priority, as `evidence show` titles it: `atelier-waj0
evidence recorded <title>`. The other review and evidence views adopt it in
their own items: `status`'s Reviews and Evidence rows in atelier-d2xe,
`review list` in atelier-h6o4, `review show` in atelier-3u2u, and
`evidence list` in atelier-u0ig.

### Commits and Timestamps

- A commit is shown by its first 8 characters. The full 40 appear only where
  the value must be passed back to a command, such as `--commit` or
  `--resolution-commit`.
- A timestamp is shown in one form, local time `YYYY-MM-DD HH:MM ±hh:mm`, as
  in `2026-10-16 20:31 -04:00`. Records keep RFC 3339; a date alone is
  `YYYY-MM-DD` and is the local day that form shows, as in note bullets and
  the `prune` cutoff.

### Headings

A command's output has at most one page title, underlined with `=`. Every
other heading is a section, underlined with `-`. Underlines match the
heading's display width: one column per character, not per byte, so `é`, `✓`
and `—` count once. A heading that states a fact, such as a page title that
is an item line, means no metadata line repeats it: `issue show --full` has
no `Status:`, `Type:` or `Priority:` line, `evidence show` no `Status:` line,
and `status` no `Current work:` count above its Current Work section.

### Verdict Line

A command that gates (`land`, `doctor`, `check`, `merge`, `merge --check`,
`start`, `evidence record`) ends with one line that says whether it passed:

```
land: passed
land: refused — 2 checks (land_records_uncommitted, land_base_unreviewed)
```

The line is the command name, `passed` or `refused`, and on refusal the count
and codes of the checks that failed. It is the last line of the combined
output, so `| tail -1` reads it, and a refusal still exits non-zero: a piped
command's exit status is the pipe's last command's, so check `$?` before
acting on it. Each command adopts the line in its own item: `land` in
atelier-bvl3 and atelier-9jr8, `merge` and `merge --check` in atelier-6rh7,
`evidence record` in atelier-3ekh, `check` in atelier-4t5l, `start` in
atelier-501y, and `doctor` in atelier-rc8j.

### Mutation Confirmation

A command that changes records prints each value it changed, as it now
stands, one line per field: `<id> <field>: <value>`, or
`<id> <field>: <old> → <new>` where the old value helps, as in
`atelier-43m1 priority: medium → high`. A value that did not change is not
listed. Each command adopts this in its own item (atelier-7ebi for `issue
update`, `note` and `link`; atelier-8bve for `issue create`; atelier-093h
for review records; atelier-3ekh for evidence).

### Quiet Tokens

`--quiet` prints one documented token per command, and nothing else on
stdout. atelier-pr4k makes every command honour this table:

| Command | `--quiet` output |
| --- | --- |
| `start` | the branch name |
| `issue show` | `<id> <status>` |
| `issue transition <id>` (options) | the allowed transition names, one per line |
| `issue transition <id> <name>` | the new status |
| `issue create`, `update`, `note`, `link`, `unlink` | the ID |
| `check`, `land`, `doctor` | `passed`, or nothing and a non-zero exit |
| `init` | the ID prefix (slug) |
| `status` | `key=value` pairs whose values are numbers |
| `work` views | IDs, one per line; `work mission` and `work parent` print their counts as `key=value` |
| `review` and `evidence` record commands | the new record's ID |

## Views

Detail views show the item title, exact configured status, intent, blockers,
proof, and ownership. Queues use the item line above, grouped under sections
named by work state. Rollups can use workflow categories.

`issue show` is brief by default, and `--full` prints the whole record. The
brief view gives one line per fact and leaves out empty facts:

- the title line: the item line with the type;
- `Mission:` for work in a mission's scope, or for a mission `Strategy:` and
  `Work:` with its scope counts by work state;
- `Parent:`, then `Children:` in work order, each with its work state;
- `Blocked by:`, and `Blocked via parent <id>:` for blockers a child inherits;
- `Transitions:`, which names the allowed transitions, then each group of
  blocked transitions with the reason they share, separated by `|`. A
  transition whose only gap is a required argument is allowed and shows its
  flag, as in `supersede --reason`;
- the two lead sections of the record's body shape: Goal and Done when for
  work items, Outcome and Success criteria for missions, and Question and
  Decision for decisions. These say what the item is for and what finishes
  it. A one-line section follows its label, and a longer one is indented
  under it;
- `More with --full:`, which names the record's other sections.

Related items are item lines. A list with one item stays on its label's line,
and a longer list is indented under it. `issue show <id> <id>...`
resolves every ID before printing anything, so one unknown ID prints nothing,
and then prints each brief view, separated by a blank line. With `--full`, it
prints each full record the same way.

Use specific labels for evidence, reviews, hierarchy, blockers, and Git facts.
Never present an attached receipt as passing proof without checking its result
and freshness. Branch summaries must name the branch and remain separate from
current-checkout state.

Next actions come from actual state: inspect a blocker, select ready work,
record proof, resolve a finding, or commit record changes. Avoid generic command
menus in every row. Errors name the failed invariant or path and give a useful
recovery step. Optional verbosity may show check details and command context.

`--quiet` limits output to essential IDs, counts, and result tokens. It does not
hide failures or change behavior. Atelier has no diagnostic logging options;
`--log-level`, `--log-format`, `ATELIER_LOG` and `ATELIER_LOG_FORMAT` were
removed. Warnings go to stderr as plain text. Color and terminal width may
improve scanning without changing meaning or breaking redirected output.

See [inventory and mission views](issue-inventory-and-mission-overview.md) and
[work ordering](work-view-ordering.md) for view semantics.
