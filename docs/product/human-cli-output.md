# Human CLI Output

Normal output should let an operator identify the work, understand its state,
and choose the next concrete action. Markdown records are the durable machine
boundary; human output is a focused view over them.

## Dense Output

Output is dense Markdown for every reader (ADR 0022). Agents read redirected
output as plain text, so every token they read should carry a fact:

- One line per item: `<id> [<status>] <priority> - <title>`, with a blocker
  count when there are blockers.
- Symbols that read without a legend go before the status word, never in
  place of it: ✓ for a done status, ⚠ for a blocked status or work state, as
  in `[✓ done]` or `[⚠ todo]`.
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

## Views

Detail views show the item title, exact configured status, intent, blockers,
proof, and ownership. Queues use the item line above, grouped under sections
named by work state. Rollups can use workflow categories.

`issue show` is brief by default, and `--full` prints the whole record. The
brief view gives one line per fact and leaves out empty facts:

- the title line: ID, type, status, and title;
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

Related items are `<id> <status> - <title>`. A done status is marked ✓, and a
blocked status or work state is marked ⚠. A list with one item stays on its
label's line, and a longer list is indented under it. `issue show <id> <id>...`
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
hide failures or change behavior. Diagnostic logging uses `--log-level` and
`--log-format`; logging is not a second work-state store. Color and terminal
width may improve scanning without changing meaning or breaking redirected
output.

See [inventory and mission views](issue-inventory-and-mission-overview.md) and
[work ordering](work-view-ordering.md) for view semantics.
