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

### Counts

A count is a plain phrase, count first, with a real plural: `1 blocker`,
`2 blockers`, never `blocker(s)` or `blockers 2`. A field whose whole value
is one count keeps its label first, as metadata does: `Open blockers: 1`,
`Ready work: 9`.

Counts by work state use the state words in one order: active, ready, needs a
human (`2 need a human`), context, blocked, done, backlog. A scope's counts
follow its size, and `work missions`, `work mission`, `work parent` and
`issue show` all write them alike: `Scope: 3 issues: 1 ready, 2 blocked`.
Text that says whether work is finished calls work that is not closed `open`
and closed work `closed`, never `nonterminal` or `terminal`: `Unassigned: 6
open issues`, `closed ancestor <id> retires open descendant <id>`. Workflow
configuration terms keep their names, such as the `work.terminal` validator
and "terminal status" in policy errors. A status row on another branch drops its
`| N active` cue when nothing is active.

A list of counts leaves out its zeros, and a line that would count nothing is
left out: `status` prints no `Needs a human: 0`, and `work missions` no `Open
blockers: 0`. A zero stays only where it carries another fact, as in
`Current missions: 0; last closed <mission>`.

`count_noun` writes every count phrase. It lives in `atelier-core` so that
messages from every crate can use it, and `human_output.rs` re-exports it
beside `work_state_counts`, `scope_counts` and `noun_counts`, which write the
lists. A source test refuses a plural written as `(s)` anywhere in the
workspace's Rust.

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

### Next Footer

A view that offers next steps ends them in one section, headed `Next`, with
one step per line:

```
Next
----
  Inspect this issue: atelier issue show atelier-onwh
  Show transition options: atelier issue transition atelier-onwh
```

Each line is `  <label>: <command>`: the label says what the step does, and
the command is what to run. A line too wide for the terminal continues
indented under its label. A view with no step to offer prints no `Next`
section. `render_footer` in `human_output.rs` is the one helper that writes
it; no view prints its own footer heading or line. A pointer on one row reads
the same way, as `More info: atelier work mission <id>` does in `work
missions`, and so does a transition's `Command:` line. A validator hint is
one sentence after `Hint:` that names its command in backticks, such as
``Hint: Inspect the blocker with `atelier issue show <id>` ``; it carries no
second prefix.

Every next step, hint and error recovery step names the real ID when the
command knows it: the top ready item in `status`, the first row of `issue
list`, each runnable transition's command in `issue
show --full`'s Transition Readiness, and the issue, mission or owner an error
is about, as in
``link mission work with `atelier issue link atelier-xmtr atelier-1w8c --role
advances` ``. A placeholder such as `<issue-id>` stays only where the command
cannot know the ID, such as which issue should own an unowned branch. A step
with no row to point at is left out, so an empty `issue list` has no `Next`
section. A command inside a sentence is quoted in backticks, never bare or in
single quotes; a command that is the whole value after a label, as in a `Next`
line, a `Command:` line or an error's `Next: <command>`, is bare.

### Quiet Tokens

`--quiet` prints one documented token per command, and nothing else on
stdout. Every command honours this table, and one integration test runs each
command in the help tree with `-q` and checks its token:

| Command | `--quiet` output |
| --- | --- |
| `init` | the project slug, new or already configured |
| `setup claude` | the settings file path |
| `status` (and its alias `prime`) | `work=<n> current_missions=<n> ready=<n> human=<n>`, where `work` counts active work |
| `work` | the view names, one per line |
| `work ready`, `blocked`, `human`, `active`, `all`, `missions` | IDs, one per line |
| `work mission`, `work parent` | their counts as `key=value` numbers, `active=<n> ready=<n> human=<n> context=<n> blocked=<n> done=<n> backlog=<n>`, with `proof_gaps=<n>` for a parent; with a filter such as `--ready`, its IDs |
| `issue create`, `update`, `note`, `link`, `unlink` | the ID; for `link` and `unlink`, the first ID given |
| `issue show` | `<id> <status>`, one line per ID |
| `issue list` | IDs, one per line |
| `issue transition <id>` | the allowed transition names, one per line, counting one that needs only a flag such as `--reason` |
| `issue transition <id> <name>` | the new status |
| `start` | the branch name |
| `evidence record`, `attach` | the record's ID |
| `evidence show` | `<id> <result>` |
| `evidence list`, `review list` | IDs, one per line |
| `review record`, `approve`, `finding`, `trivial`, `disposition` | the new event's ID |
| `review show` | `<id> <event>`, the event as the record stores it, such as `approval` |
| `check`, `land`, `doctor`, `merge`, `merge --check`, `bundle preview` | `passed`, or nothing and a non-zero exit |
| `bundle apply` | the IDs of the records it created or updated, one per line |
| `migrate epics` | the IDs of the epic records it retypes, or would retype in a dry run |
| `prune` | `mode=<dry-run or apply> eligible=<n> protected=<n> removed=<n> failures=<n>` |

Values are numbers except `prune`'s `mode`. A failure still exits non-zero
and says why on stderr: `check` prints its findings there, and `merge` its
failed gates. Notes that would otherwise follow the token, such as `start`'s
needs-human note, also go to stderr. `--quiet` with `--json` is a usage
error, since they are two script formats.

### Errors

Every command takes the same record references and words the same errors
about them. The helpers in `crates/atelier-app/src/record_ref.rs` and
`atelier_core::invalid_value_message` implement them.

- A record reference is a full ID or a short ID, the part after the project
  slug, in any case: `1w8c`, `1W8C` and `atelier-1w8c` all name
  `atelier-1w8c`. Issues, evidence records and reviews resolve alike.
- An unknown reference is one message, `<kind> <ref> not found`, followed by
  up to three IDs it may have meant, or the command that lists the kind, and
  exits 1:

  ```
  issue 1w8 not found. Did you mean atelier-1w8c?
  issue zzzz not found. Next: atelier issue list
  ```

- A reference to a record of another kind names both kinds and the command
  that shows it: ``atelier-waj0 is an evidence record, not an issue. Use
  `atelier evidence show atelier-waj0`.`` A scoped view that refuses an issue
  names the view that does show it, such as `atelier work mission
  atelier-1w8c` for a mission given to `work parent`.
- An invalid value names the field, the value and every value the field
  accepts: `Invalid status 'open'. Valid values: backlog, done, todo, all`.
- Flags that exclude each other are declared to the argument parser, so a
  conflict is a usage error with exit status 2, never a runtime error.

### Help

Every argument and flag in `--help` has a one-line description. An option
with a fixed set of values lists them as `[possible values: ...]`; the status
and category options of `issue list` and `issue create --status` list the
repository workflow's values, read when help is shown. Global options, such as
`--quiet`, follow the command's own options.

Every command's description is its one-line `about`, and help lists are
generated from it, so no list is written by hand. Root help lists every
command that is not hidden, then `Common commands` in lifecycle order: orient,
pick work, start it, record proof, close it, review, land and check the merge.
Bare `atelier work` prints `atelier work --help`, and with `--quiet` the view
names.

## Views

Detail views show the item title, exact configured status, intent, blockers,
proof, and ownership. Queues use the item line above, grouped under sections
named by work state. Rollups can use workflow categories.

`issue show` is brief by default, and `--full` prints the whole record. The
brief view gives one line per fact and leaves out empty facts:

- the title line: the item line with the type;
- `Mission:` for work in a mission's scope, or for a mission `Strategy:` and
  `Work:` with its scope counts by work state, as [Counts](#counts) writes
  them;
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
recovery step. The recovery step is an inline sentence in the error,
`Next: <step>`, not a `Next` footer. Optional verbosity may show check
details and command context.

`--quiet` limits output to essential IDs, counts, and result tokens. It does not
hide failures or change behavior. Atelier has no diagnostic logging options;
`--log-level`, `--log-format`, `ATELIER_LOG` and `ATELIER_LOG_FORMAT` were
removed. Warnings go to stderr as plain text. Color and terminal width may
improve scanning without changing meaning or breaking redirected output.

See [inventory and mission views](issue-inventory-and-mission-overview.md) and
[work ordering](work-view-ordering.md) for view semantics.
