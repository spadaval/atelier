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
(Current Work, Owner, Mission, Owner Work, Ready Work and unassigned work;
its `Branches` rows use the branch row below), `work missions`, `issue list`, and `issue show` (title
line, Mission, Parent, Children, Blocked by, Blocking, Linked Issues, Impact,
Subissues and the Mission Rollup or Parent Rollup). So the Current Work row
and the Owner row in `status` read alike. A record that is not an issue uses
the same order without a priority, as `evidence show` titles it:
`atelier-waj0 evidence recorded <title>`. The other evidence views adopt it in
their own items: `status`'s Evidence rows in atelier-d2xe and
`evidence list` in atelier-u0ig.

### Review Rows

A review event is one row, oldest first, in `review list` and in `status`'s
Reviews (newest first there):

```
atelier-1isf  disposition fix_now → atelier-xs5u  @d1c3b1bf  Reviewer R  2026-10-16  Done when: setup claude warns…
```

The ID, the kind (a finding adds its recommendation, a disposition its
decision and finding), `on <issue>` unless the list is for that issue, the
pinned commit, the identity, the local date and the summary's first line, cut
at 100 characters. A base exemption reads `trivial_exemption  of <base>`.
`review list <id>` lists the events of the issue's branch owner and its
scope; for a child without its own branch it first says
`<child> is reviewed with its branch owner <owner>.` After the rows come the
owner's approval state and its findings:

```
Approval: ⚠ stale — atelier-x16l @e2562bd6 by Independent reviewer 3; changed since by 1 commit:
  1a2b3c4d Tweak the sample: proof-sample.txt
Findings: 21 disposed, 1 open
  ⚠ atelier-4cid defer, no disposition  The Stop hook runs at the end of every turn…
  ✓ atelier-ul0a no_action → no_action atelier-ul3r → no_action atelier-6o75  Name could be better
  ✓ atelier-1ia7 defer → fix_now atelier-g6fd, fixed at a485ef02  docs/adr/0022…
```

On the owner's work branch, while it holds commits the base does not, the
approval is `✓ current`, `⚠ stale` or `⚠ missing` as `merge`'s
`review.current` judges it at the branch tip, and a stale approval lists the
commits that made it stale with the paths a new review has to read. Merged
work is judged on the base as `land` judges history: `✓ accepted on <base>`
or `⚠ not accepted on <base>`. The views read whichever records the checkout
has. Why an approval is not current is not shown yet (atelier-cisa). A
finding is open until it has a disposition; its chain lists every
disposition, the latest last.

`review show <id>` is brief: a header line,
`<id> <kind> on <issue> @<pin> by <identity> | <timestamp>`, ending
`| ✓ current` (or the base's word) for an approval or exemption; then a
finding's `Dispositions:`, or a disposition's `Finding:`, `Fixed at:`,
`Follow-up:` and `Supersedes:` lines; then the text once.

### Branches

`status` lists the other local work branches in a `Branches` section (a
`-----` section before Next), one line per owned branch:
`<branch>  <id> <status>[, <merge>][, <readiness>][, <children>]  <title>`.
The branch, two spaces, the owner's ID and status word marked ✓ or ⚠ as in
any row, then for a done owner `, merged into <base>` or `, not merged` (the
wording of the `↳` in-flight mark), then two spaces and the title. Facts
about a branch go in the status part, before the free-text title. There is no
priority, commit or active count.

A done owner that is not merged is waiting to land, and its row adds its
readiness against the current base (atelier-tciw):

```
task/atelier-cmhv  atelier-cmhv ✓ done, not merged, ✓ approved, behind 3, no proof at head, merges cleanly  Title
task/atelier-sfys  atelier-sfys ✓ done, not merged, ⚠ approval stale, behind 3, proof at head, conflicts: Cargo.lock, src/lib.rs  Title
```

- The approval mark: `✓ approved` when `merge`'s `review.current` would pass
  at the branch head, `⚠ approval stale` when an independent approval or
  trivial exemption exists but later commits changed what it covers (see
  [approval freshness](reviews-and-merge.md#approval-freshness)), and
  `⚠ not approved` otherwise. The stale cause is not shown yet
  (atelier-cisa).
- `behind <n>`: base commits the branch does not have.
- `proof at head` when every done item in the owner's scope, except
  superseded ones, has fresh passing proof at the branch head; otherwise
  `no proof at head`.
- `merges cleanly`, or `conflicts: <path>, <path>` naming the files
  `git merge-tree` cannot merge into the current base. It comes last, so its
  commas end the part.

When readiness cannot be read, the row ends its status part with
`, readiness unknown: <reason>`. An owner that is not done and has children
adds their activity, `, 1 of 3 children done, 1 active`, so a parent-owned
branch does not read as idle.

Branches with no started work or no tracker share one trailing line,
`Unowned: <branch>, <branch>`. Each pair of waiting branches that change the
same files since they left the base gets a line,
`Shared files: <branch> and <branch> both touch <path>, <path>`, naming up to
five paths and then `and <n> more`, so an orchestrator can land them back to
back. A branch whose records cannot be read is
`<branch>: unavailable — <reason>`.

The view changes no Git state: it reads the merge with
`git merge-tree --write-tree`, which moves no ref, index or checkout.

### Decision Rows

`work decisions` lists decision records as item lines, with `| ADR <path>`,
`| supersedes <ids>` and `| superseded by <ids>` cues. A decision note is not
an item, so it has its own row, in the same field order:

```
<item-id> <date> <choice> | because <reason>[ | rejected <alternative>; ...]
⚠ <item-id> decision note <n> (<date>) <defect>
```

The second form lists a note `check` refuses, numbered as `check` numbers
it. Both rows wrap as item lines do, continuing on lines indented past the
rows.

`issue show` and `status` list the current decisions bearing on an item:
the decision notes on the item, its parent and its mission, and the decided,
unsuperseded decision records in its mission's scope. The brief form, in
`issue show` and `status`, is a count with the one-way decisions beneath it;
`issue show --full` adds a Decisions section with all of them, one-way notes
first, then decision records, then reversible notes:

```
Decisions: 4 current, 1 one-way:
  atelier-x 2026-10-17 Keep the format | because Readers rely on it
```

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
A parent is not in its own scope, so `work parent`'s `Proof gaps` counts its
scope items without passing proof and leaves the parent out.
Text that says whether work is finished calls work that is not closed `open`
and closed work `closed`, never `nonterminal` or `terminal`: `Unassigned: 6
open issues`, `closed ancestor <id> retires open descendant <id>`. Workflow
configuration terms keep their names, such as the `work.terminal` validator
and "terminal status" in policy errors.

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
and codes of the checks that failed. A command that finished its work but
leaves a gate it does not own unmet says so in the line instead of `passed`:
`merge --sync` ends `merge: synced — approval needs fresh review` (exit
status zero) when the approval it reported is stale. `land --push` appends what happened to
the push: `land: passed, pushed to origin/<branch>` or `land: refused — 1
check (land_push_failed), nothing pushed`. It is the last line of the combined
output, so `| tail -1` reads it, and a refusal still exits non-zero: a piped
command's exit status is the pipe's last command's, so check `$?` before
acting on it. Each command adopts the line in its own item: `land` prints it since
atelier-ecah, counting every failed gate check since atelier-9jr8, `merge --sync` since
atelier-s2ei (`merge: passed`, `merge: synced — approval needs fresh review`, or `merge:
refused — 1 check (sync_conflict)`),
`merge` and `merge --check` since atelier-6rh7 (`merge: passed`, or `merge:
refused — 2 checks (review.current, findings.disposed)`),
`evidence record` in atelier-3ekh, `check` in atelier-4t5l, `start` in
atelier-501y, and `doctor` in atelier-rc8j. A refusal by a check that stops
the command before its gates, such as `land_records_uncommitted` or
`merge_diverged`, prints its error and counts that one check; atelier-0uaj
evaluates the gates anyway.

### Gate Lines

`land`, `merge` and `merge --check` print their gates through one renderer
(`crates/atelier-cli/src/gate_report.rs`), so a check reads the same in
each. Failed checks come first, one line per record, each naming the record,
the check, the cause and the command that fixes it; then the passes,
collapsed; then the verdict:

```
⚠ atelier-43m1 review.current: no current independent approval or trivial exemption; authors: sumeet <sumeet@example.com> (also claims.reviewed). Next: atelier review request atelier-43m1
⚠ atelier-43m1 findings.disposed: 21 fix_now findings resolved; they need a fresh approval (see review.current). Next: atelier review request atelier-43m1
⚠ atelier-jb2t blockers.none_open: done but blocked by atelier-ihns todo high Show the between-missions state (also atelier-jb2t blockers.transitive_none_open). Next: start it with `atelier start atelier-ihns`, or drop the link with `atelier issue unlink atelier-jb2t atelier-ihns --role blocked_by` if it is wrong
✓ 3 checks pass (blockers.transitive_none_open, required_field.close_reason, work.terminal)
✓ 2 done claims hold
merge: refused — 5 checks (review.current, findings.disposed, claims.reviewed, atelier-jb2t blockers.none_open, atelier-jb2t blockers.transitive_none_open)
```

- The owner's failures lead, `review.current` first since other lines point
  to it, then other records' in the order the gates ran. A check on the owner
  is named bare in the verdict; a check on another record is
  `<record> <check>`.
- Each finding, open item and blocker is its own line. Resolved fix_now
  findings that only wait on a fresh approval share one line; when a trivial
  exemption carries `review.current`, the line says it does not cover a
  fix_now resolution.
- A failure with the same cause as another is folded into that line as
  `(also <check>, ...)` and still counted in the verdict: a blocker check that
  repeats the owner's `blockers.none_open`, `claims.reviewed` for a claim in
  the owner's scope when its review or findings fail, and `merge.merged_tree`
  when a `shell.check` failed on it.
- The fix uses the real IDs, paths and commands it knows, and each moves the
  check toward passing: for an open item in the scope, the workflow's
  transitions that close it from its status, in order (`--reason "…"` where
  the policy requires one); under a closed ancestor, which keeps it out of
  active statuses, transitions that avoid them, such as `supersede`, and the
  ancestor's reopen transition when its workflow has one; for an open
  blocker, the transition that closes it in one step, else `atelier start
  <blocker>` when its workflow lets it start, else the listing of its
  transitions, said as such, and the `atelier issue unlink` that drops the
  link if it is wrong, from the record that declares it and with its role
  (`--role blocked_by`, or `--role blocks` from the blocker); the failing receipt's own
  command to record proof again; the shell check to rerun, after `atelier
  merge <owner> --sync` when it failed on the merged tree under
  `merge_strategy: merge_commit`, which holds the base's changes the work
  checkout lacks; and `atelier review request <owner>` for what needs an
  independent reviewer. A fix of more than one command is a sentence with
  each command in backticks. The refusal tests run every printed fix
  verbatim in a copy of the fixture and require it to exit 0 and to change
  the records or checkout unless it only reads. A value only a person can supply, such as the
  reviewer identity, the resolution commit or the reason text, stays a
  placeholder (`<identity>`, `<commit>`, `"…"`).
- The passes are one line for the owner's checks, naming the approval or
  exemption `review.current` passed with, and a count of the other done
  claims that hold. The authors `review.current` lists name each person once.

A passing `land` starts with `✓ records committed`, then the passes; a done
owner ends with `Next: atelier merge <owner>`, and an active owner with
uncommitted changes names the first five paths and the count, and points to
`atelier status`. On the base it says `No unpushed base commits need
review.`, or how many record-only commits it found. `merge --check` passes
with its passes and `Next: atelier merge <id>`. A successful `merge` prints
no gate lines, only what it did and the next step:
`✓ Merged task/atelier-nqgr into master at c24af21a; branch deleted. Next:
atelier land, then git push.` It notes that Git hooks did not run only when
the repository has hooks of its own.

### Mutation Confirmation

A command that changes records prints each value it changed, as it now
stands, one line per field: `<id> <field>: <value>`, or
`<id> <field>: <old> → <new>` where the old value helps, as in
`atelier-43m1 priority: medium → high`. A value that did not change is not
listed. Each command adopts this in its own item (atelier-7ebi for `issue
update`, `note` and `link`; atelier-8bve for `issue create`; atelier-3ekh
for evidence).

A command that adds a review event, which changes no existing value,
confirms it in one line naming its kind, ID, issue and pinned commit, and
what a disposition settled or the step a finding needs:

```
Approval atelier-hleo on atelier-nqgr @c24af21a
Finding atelier-45rc fix_now on atelier-nqgr @fed2d571. Dispose: atelier review disposition atelier-45rc --identity "..." --decision fix_now --resolution-commit ... "..."
Disposition atelier-6o75 no_action on atelier-nqgr @c24af21a: finding atelier-rm9e "Name could be better", supersedes atelier-ul3r
```

An approval, or a defer or no_action disposition, whose identity is one of
the substantive Git authors `review.current` counts is still recorded, and a
second line warns `⚠ identity matches a Git author; this will not satisfy
review.current` (or `findings.disposed`); under `--quiet` the warning goes to
stderr.

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

A view offers only the steps the record's state allows, and never the
command just run or a filter for rows already on screen:

- a done record offers no edit and no transition options, since its body
  cannot be revised and its status has no transitions out;
- after a transition to a done status, the steps are the handoff, `atelier
  land <owner>` and `atelier merge <owner> --check`, where the owner is the
  parent or else the issue itself; an owner with no recorded branch, such as
  a mission or decision, offers `atelier land` alone;
- `work mission` and `work parent` offer `atelier start <id>` for the first
  item of Ready Work, in work order, that no other branch has started, and
  their transitions only while the objective has any;
- `issue create` offers `atelier start <new id>` when the item's workflow has
  a `start` transition from the status it was created in; otherwise, as for
  backlog work or a mission or decision, which have no branch, it offers the
  transition options.

A list that leaves rows out ends with a line that counts them and names the
command that shows them all: ``... 4 more done work items omitted; see
`atelier work mission atelier-kgcr --done` ``.

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
command in the help tree with `-q`, checks its token, and checks that a
refusal of it leaves stdout empty:

| Command | `--quiet` output |
| --- | --- |
| `init` | the project slug, new or already configured |
| `setup claude` | the settings file path |
| `status` (and its alias `prime`) | `work=<n> current_missions=<n> ready=<n> human=<n>`, where `work` counts active work |
| `work` | the view names, one per line |
| `work ready`, `blocked`, `human`, `active`, `all`, `missions` | IDs, one per line |
| `work decisions` | the IDs of the items holding current decisions, one per line |
| `brief` | `criteria=<n> items=<n> blocked=<n> unknowns=<n> human_decisions=<n> approvals=<n>`; with `--html`, the page's path |
| `debrief` | `criteria=<n> proven=<n> failing=<n> unproven=<n> items=<n> decisions=<n> deviations=<n> follow_ups=<n>`; with `--html`, the page's path |
| `work mission`, `work parent` | their counts as `key=value` numbers, `active=<n> ready=<n> human=<n> context=<n> blocked=<n> done=<n> backlog=<n>`, with `proof_gaps=<n>` for a parent; with a filter such as `--ready`, its IDs |
| `issue create`, `update`, `note`, `link`, `unlink` | the ID; for `link` and `unlink`, the first ID given |
| `issue show` | `<id> <status>`, one line per ID |
| `issue list` | IDs, one per line |
| `issue transition <id>` | the allowed transition names, one per line, counting one that needs only a flag such as `--reason` |
| `issue transition <id> <name>` | the new status |
| `start` | the branch name |
| `evidence record`, `attach` | the record's ID |
| `evidence show` | `<id> <result>` |
| `evidence list`, `review list` | IDs, one per line; for `review list <id>`, those of the rows it lists |
| `review record`, `approve`, `finding`, `trivial`, `disposition` | the new event's ID |
| `review request` | the range to review, `<from>..<head>` in full commits |
| `review show` | `<id> <event>`, the event as the record stores it, such as `approval` |
| `check`, `land`, `doctor`, `merge`, `merge --check`, `bundle preview` | `passed`, or nothing and a non-zero exit |
| `bundle apply` | the IDs of the records it created or updated, one per line |
| `migrate epics` | the IDs of the epic records it retypes, or would retype in a dry run |
| `prune` | `mode=<dry-run or apply> eligible=<n> protected=<n> removed=<n> failures=<n>` |

Values are numbers except `prune`'s `mode`. A failure still exits non-zero
and says why on stderr: `check` prints its findings there, and `merge` its
failed gates. A refused `start` or `issue transition` prints no transition
report; its error, in every mode, names each blocker and then gives each
failed validator's hint as its `Next:` step. Notes that would otherwise follow
the token, such as `start`'s needs-human note, also go to stderr. `--quiet`
with `--json` is a usage error, since they are two script formats.

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
  atelier-1w8c` for a mission given to `work parent`. A kind or issue type,
  including one the workflow adds, takes "an" before a vowel letter and "a"
  otherwise: `is an incident, not a mission`.
- An invalid value for a command argument, whether it is outside the set
  the argument accepts or is not in the argument's form, is one message in
  one form: the field, the value and every value that command accepts
  there, and nothing it rejects: `Invalid status 'open'. Valid values:
  backlog, done, todo, all`. The values are the ones that command accepts,
  so `issue create --status` lists the statuses new work of the issue's
  type starts in, and `issue transition` lists the transitions out of the
  issue's status, or `none`. A malformed value names the accepted forms:
  `Invalid target 'bad'. Valid values: issue/<id>, evidence/<id>`. One
  sentence of guidance may follow, such as `Did you mean 'pass'?` for a
  near miss, and a code that shipped before the rule, such as
  `issue_create_status_invalid`, still leads the message.
- An invalid value is a usage error and exits 2, wherever it is checked.
  Values the argument parser knows, such as `evidence list --result`, and
  values the repository's workflow or configuration defines, such as
  statuses, issue types, transitions and custom link roles, are worded and
  exit alike, so a caller never needs to know which checked it. An unknown
  record reference is not an invalid value: it exits 1.
- Flags that exclude each other are declared to the argument parser, so a
  conflict is a usage error with exit status 2, never a runtime error.
- An error carries a code if and only if one of Atelier's checks refuses
  the command. The checks are:
  - the gates of `land`, `merge` (and `merge --check`) and `start`, such as
    `land_records_uncommitted`, `merge_diverged`, `merge_mission`, and the
    record fences they run, `canonical_snapshot_uncommitted`,
    `tracked_snapshot_uncommitted` and `review_append_only`;
  - `check`, `doctor` and `prune` findings, such as `check_findings`;
  - workflow policy and configuration validation: the `workflow_*` and
    `project_config_*` codes.

  No other error carries a code: not a usage error, an unknown reference, an
  invalid value, a failure of Git or the filesystem, a record that fails to
  parse while loading, or a transition that is not available. Codes that
  shipped before this rule outside these checks keep their code so that
  nothing matching them breaks: `bundle_recovery_required`,
  `bundle_recovery_artifact_unsafe`, `canonical_bulk_concurrent_drift`,
  `evidence_capture_association_changed`, `evidence_capture_target_changed`,
  `issue_body_done` and `issue_create_status_invalid`. No new code is added
  outside the list.
- A code is snake_case and starts with the command or subsystem that owns
  the check. It leads the message as `<code>: `, except that workflow policy
  errors print it as the `error: <code>` line of their field block. It never
  changes once it ships: gates, tests and agents match on it, and the
  verdict line lists it.
- An error carries at most one code, counting every line of it, including
  `Caused by:` lines. A command that reports a lower-level check under its
  own check replaces the inner code with its own. `merge` reports a record
  that differs from HEAD as `canonical_snapshot_uncommitted`, and `land`
  reports the same failure as `land_records_uncommitted`, not as both. A
  command that only wraps a coded error, as `land` and `merge` wrap an
  invalid candidate workflow policy, adds a recovery step but no code.
- Every error the list covers states its failure once, then gives one
  recovery step: `<code>: <failure>. Next: <step>`. Errors written before
  this rule switch over in the items that own their gates: `land` and
  `merge` gate failures print as [gate lines](#gate-lines) since atelier-9jr8
  and atelier-6rh7, and their other checks switch over in atelier-0uaj;
  atelier-oxnw for `start`, and atelier-kset for `check` findings.
- Output uses the terms in `CONTEXT.md`: records, check, finding, mission,
  parent. The internal words "lint" and "canonical" appear only inside a
  code, such as `canonical_snapshot_uncommitted`, or a configuration key,
  such as `prune.canonical_retention_days`.

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
  `Scope:` with its scope counts by work state, as [Counts](#counts) writes
  them;
- `Parent:`, then `Children:` in work order, each with its work state;
- `Blocked by:`, and `Blocked via parent <id>:` for blockers a child inherits;
- `Transitions:`, which names the allowed transitions, then each group of
  blocked transitions with the reason they share, separated by `|`. A
  transition whose only gap is a required argument is allowed and shows its
  flag, as in `supersede --reason`. `--full`'s Transition Readiness and
  `issue transition <id>` also show it as allowed with its flag: the first
  as `supersede: allowed - to superseded, needs --reason`, the second as
  `supersede: allowed` with a `Requirements: needs --reason` line. A done
  record shows `Closed <date>: <first
  line of its Close Reason>` instead, as `--full`'s Transition Readiness
  does;
- the two lead sections of the record's body shape: Goal and Done when for
  work items, Outcome and Success criteria for missions, and Question and
  Decision for decisions. These say what the item is for and what finishes
  it. A one-line section follows its label, and a longer one is indented
  under it;
- `More with --full:`, which names the record's other sections, leaving out
  a one-line Close Reason that the `Closed` line already shows.

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
