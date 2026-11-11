# Rubric for the classified scorecard measures

Three scorecard measures need a judgment, not a count: the owner's
interventions per mission, gate precision and review yield
([product specification](../../../product.md#scorecard)).
This rubric is how that judgment is made, before the redesign and again after
it, so a rater who has only this file and an unlabelled sample file can label
every entry. Results are in [classified.md](classified.md).

## What you need

Each sample file lists entries. Each entry says where the thing to judge is.
Read it there, read-only. The snapshot and the four product repositories hold
private material: read locally, quote nothing from them in what you write, and
send nothing from them to any service.

- **T3's database**, a frozen copy at
  `~/.local/share/atelier-baseline/2026-11/t3/statev2.sqlite`. Open it only as
  `file:<path>?mode=ro&immutable=1`, as the snippets below do, so nothing is
  written beside it.
- **Repository records**, under `~/projects/<repository>`, read with
  `git -C ~/projects/<repository> show <commit>:<path>`. Never check out,
  fetch or write in these repositories. Read at these commits, the frozen base:

| Repository | Commit |
| --- | --- |
| atelier | `2ff78f214301f5d8d096f088ced4b97636f04700` |
| astromech | `ae4d18bac93f4165d9c2efd520e1747644ce0f74` |
| demiurge | `9fcc3bd97c49b4023d9d75f80a474325cf5be0de` |
| life-on-rails | `ac6136093f1ef02edad9ce67ca1f6988a3120813` |
| cosplay-studio | `c288d44f024eb98697dd8b5e46017ce1c887d4ba` |

A label file is JSON: `{"measure": ..., "rater": "<model>", "labels": [{"n":
<entry number>, "label": "<label>"}]}`, with exactly the labels named below.

## Interventions

**Entry:** `repository`, `missions` (the closed missions the message may
belong to), `thread`, `message`, `timestamp`. The population is every message
the owner typed in a mission's threads between the mission's start (the first
commit at which its record left draft) and its close. A mission's threads are
those whose branch, title or first message names the mission or an item that
advances it, plus every thread above them in T3's thread lineage, where he
usually talks to the orchestrator. Messages T3 marks as agent-written, such as
an orchestrator's brief to a child, are not in it.

To read the mission, `git -C ~/projects/<repository> show
<commit>:.atelier/issues/<mission>.md`. To read the message in context:

```sh
python3 - <<'EOF'
import json, os, sqlite3
THREAD, MESSAGE = "<thread>", "<message>"
db = sqlite3.connect("file:" + os.path.expanduser(
    "~/.local/share/atelier-baseline/2026-11/t3/statev2.sqlite") + "?mode=ro&immutable=1", uri=True)
rows = db.execute("select message_id, role, created_at, payload_json from orchestration_v2_projection_messages "
                  "where thread_id = ? order by created_at", (THREAD,)).fetchall()
at = [r[0] for r in rows].index(MESSAGE)
for mid, role, created, payload in rows[max(0, at - 3):at + 2]:
    p = json.loads(payload)
    print("=====", ">>" if mid == MESSAGE else "", role, p.get("createdBy"), created)
    print(p.get("text", "")[-2500:])
EOF
```

**Labels.** Answer in this order and stop at the first yes.

1. `other`: the message is not about this mission's work: it plans or asks
   for other work (including planning a mission that has not started), is a
   tangent, is a slash command such as `/compact`, or is only thanks. When
   `missions` names more than one, ask this for each; if it is about one of
   them, add `"mission": "<id>"` to the label.
2. `correcting_direction`: it changes or challenges what the mission builds
   or how: scope, design, approach, priorities, the rules the work follows, the
   quality of its output, or a decision the agent made; or it reports that the
   work or its output is wrong. A question that pushes back ("is this even
   worth fixing?") counts.
3. `unsticking`: it restarts or repairs work that had stopped or failed:
   "continue", restart failed agents, give up on a wait that will not return,
   finish a mission no thread is running, or supply an input the agent had
   stopped to ask for, other than an approval.
4. `status_question`: it asks where things stand, what happened, what is
   next, or what something in the work means, without asking for a change.
5. `approval`: it only approves or accepts what the agent proposed ("yes, do
   it", "looks good, make the changes", "push it"), even if the agent stopped
   to ask.

Only `status_question`, `unsticking` and `correcting_direction` are
interventions. These examples are invented, not taken from the sample:

- `other`: during a release mission, the owner asks the agent to sketch a
  feature for a different project; `/compact`.
- `correcting_direction`: "This rule is too strict, accept both forms"; "the
  page should lead with what needs my review, not the timeline".
- `unsticking`: "The server restarted, carry on from where you were"; "that
  reviewer has hung for an hour, start a new one".
- `status_question`: "Has the second item merged yet?"; "what does the
  stale approval here mean?"
- `approval`: "Sounds good, go ahead"; "yes, file both".

## Gate precision

**Entry:** `repository`, `thread`, `item` (a T3 turn item, one shell
command), `ordinal`, `timestamp`. The population is every shell command in
the five repositories' T3 threads in the window that ran `atelier` and printed
atelier's refusal (`<command>: refused` or a line starting `Error: `), leaving
out scratch or fixture repositories, `--check`, `check`, `doctor` and help
runs, and test runs. The entries are in seeded order; label them in order.

To read the command, its output and what the agent did next:

```sh
python3 - <<'EOF'
import json, os, sqlite3
THREAD, ORDINAL = "<thread>", <ordinal>
db = sqlite3.connect("file:" + os.path.expanduser(
    "~/.local/share/atelier-baseline/2026-11/t3/statev2.sqlite") + "?mode=ro&immutable=1", uri=True)
for kind, payload in db.execute(
        "select type, payload_json from orchestration_v2_projection_turn_items where thread_id = ? "
        "and ordinal >= ? and type in ('command_execution', 'assistant_message', 'file_change') "
        "order by ordinal limit 8", (THREAD, ORDINAL)):
    p = json.loads(payload)
    print("=====", kind)
    print((p.get("input") or p.get("text") or "")[:1500])
    if kind == "command_execution":
        print("--- output:", (p.get("output") or "")[-2500:])
EOF
```

The refusal can sit anywhere in the output; read the lines around it, and
earlier items (a smaller ordinal) when the reason for the command is unclear.

**Labels.** The question is whether the refusal fired on a real problem, not
whether the agent then obeyed it.

- `stopped_mistake`: going ahead would have done something wrong by the
  work's own standard or the repository's written policy: merged conflicting
  or untested code, put a non-trivial change on the base without the review
  the policy requires, recorded a commit, item, branch or follow-up that is
  wrong, or lost work. Invented examples: `merge` refused because the branch
  still conflicted with the base in a source file; `land` refused a
  non-trivial source change committed straight on the base.
- `friction`: the refused action was fine, or the problem was only in the
  tool or its bookkeeping: an ID of another record kind, a record that exists
  on another branch, a missing flag or a short hash, a branch checked out
  elsewhere, an approval made stale by a record-only change, a base that moved
  with documents only, a version of the binary that cannot read newer
  records, a gate bug, or a state the agent already knew and was not trying to
  change (such as a reviewer running `review request` only to read the
  brief). Invented examples: `evidence record` without its required
  `--result`; `start` refused because the item is already in progress.
- `excluded`: not a refusal of real work, and left out of the count: the
  refusal text was printed by something else (a document the command printed,
  another program), or the command deliberately tested atelier's errors or ran
  in a scratch copy. Invented examples: `cat` of a help page that shows a
  sample refusal; a command run only to see what error a bad flag prints.

Gate precision is `stopped_mistake` out of the first 100 entries not labelled
`excluded`.

## Review yield

**Entry:** `repository`, `commit` (the frozen base), `record` (the review
record's path), `finding` (its ID), `target` (the item reviewed, or
`branch:<name>`), `reviewed_commit`. The population is every finding recorded
in the window whose reviewer recommended `fix_now`.

Read the finding with `git -C ~/projects/<repository> show
<commit>:<record>`, the item it is on with `git -C ... show
<commit>:.atelier/issues/<target>.md`, and the code or document it names with
`git -C ... show <reviewed_commit>:<path>`. Judge the claim as the finding
states it; do not try to prove or disprove it.

**Labels.** Take the first that fits the finding's main claim. Where the
defect lives decides between the planning label and the others: a fault in a
plan, Done when, Report, decision record or mission strategy is
`process_nit`, even when the plan describes code that would be wrong if
built.

1. `user_bug`: shipped or about-to-ship behaviour a user of the product would
   hit: a wrong result, crash, lost data, security hole, or broken command. For
   atelier, the user is whoever runs `atelier`, so a gate that decides wrongly
   counts. Examples: atelier-zvuy (a filter hides real missing references),
   atelier-kldl (valid Git remote names with a slash are rejected).
2. `tooling_bug`: a bug in development, CI, deployment, eval or experiment
   tooling, not in the product's behaviour. Examples: atelier-11g2 (a CI step
   fails where it promised a fallback), atelier-8ljp (an eval grader breaks
   under the new lifecycle).
3. `test_gap`: a test is missing, cannot fail, or does not test what it
   claims. Examples: atelier-ajgl, atelier-khd3 (assertions that cannot
   fail).
4. `wrong_statement`: a document, help text, comment, release note or
   output message says something false or out of date. Examples:
   atelier-bpfx (a product doc describes a row the command no longer prints),
   atelier-yzhp (an ADR misstates what the harness loads).
5. `wording_nit`: wording, style or placement, with nothing false. Examples:
   atelier-cdsb (a doc comment's wording), atelier-xzmj (a function inserted
   between another's doc comment and its body).
6. `process_nit`: tracker records and planning: Done when lines that cannot
   hold, a stale or contradictory Report, missing links or blockers, missing
   evidence, or a plan or strategy that is wrong or incomplete. Examples:
   atelier-8fki (a Done when line that cannot hold before its own merge),
   atelier-ejwl (a Report's open items contradict its ticked lines).
7. `overbuilt`: code or text that is more than the need: dead code,
   duplication, speculative generality, an unused dependency feature.
   Examples: atelier-1vuy (a public function nothing calls), atelier-hb7h (a
   dependency feature left on for removed code).

Review yield is `user_bug` plus `tooling_bug` out of the sample, the
definition behind the earlier figure of 39%.

## Redrawing the samples

`python3 scripts/scorecard/draw_samples.py --out <directory>` rebuilds the
three unlabelled sample files from the frozen snapshot and records, with the
seed stated in each file, and prints each population's size. Run it under
`nice -n 19 ionice -c3`; it takes about 30 seconds.
