---
status: draft
last_changed: "2026-11-07: created from Sumeet's direction, after an analysis of every Atelier session on this host"
---

# Why

Agents and Sumeet keep tripping over Atelier's rules. An analysis of every Claude session on this host that used Atelier (about 110 sessions and 1,100 subagent transcripts, 2026-10-15 to 2026-11-07, five repositories) found:

- **Review earns its keep.** Independent review found real bugs in nearly every round in every repository, and re-reviewing a fix is cheap and still finds things. Issue and mission records work as shared memory across compactions and restarts.
- **One design causes most of the rest.** An approval is pinned to the bytes of a history that also holds the tracker's own records, so any record edit can make it stale. Atelier grew a list of allowed edits, ordering rules around that list, and new exceptions each time a rule hit an edge. Approvals went stale from bookkeeping dozens of times, still on 2026-11-06 in four repositories, and the re-reviews almost always found nothing changed. Agents left links out, put notes in Close Reason and narrowed Done when lists to avoid it.
- **There is no single order of steps.** The skill says close before review, the policy had a `review` status, and `merge --check` says to close after approval. An agent that read only the skill, astromech's AGENTS.md and the CLI help could not say what the order is.
- **Bad records are accepted, then block everything.** A record that breaks a graph rule is written without complaint, then every later command refuses, and append-only rules stop its author fixing it. On 2026-11-05 this cost astromech a 5-hour stall and a force-push of main.
- **One ceremony fits nothing.** Releases, docs and mockups take the same path as code. Small repositories hit about ten refusals before their first merge, and cosplay-studio bypassed `atelier merge` twice.
- **Bookkeeping outweighs code.** 70–82% of commits in every repository touch only records. The cold read counted about 9 concepts that matter for correct code and about 40 that are bookkeeping.

The lighter-workflow mission (atelier-n4mj) trimmed records at the edges. This mission replaces the layer that produces them.

# Outcome

An agent takes one issue from start to merge in one obvious order, and nothing but a change to code undoes a review:

- An approval goes stale only when code changes after it. Tracker records never make anything stale.
- The order is: start, implement, tick every Done when line, review, fix and re-review, approve, merge. Merge marks the issue done. An issue is done when its change is merged, and a mission is closed when all its work is merged.
- Work that changes no code and can only happen after a merge or a release, such as pushing a tag, a play-check or an eval run, is a **loose end**: its own issue type, linked to what it waits on, and never part of a mission.
- Atelier refuses an invalid record when it is written, and an author can correct or delete its own records until they reach the base branch.
- What remains is what review needs: findings, a decision for each deferred or dismissed finding, an approval, and a merge gate that requires an independent approval of the current code plus the repository's merge checks.
- A fresh agent can learn the whole lifecycle from the skill, the repository's AGENTS.md and the CLI help.

Partial outcome worth shipping: approvals that only code makes stale, the single order, and loose ends.

What does not change: no non-trivial change reaches the base branch without an independent review of its code, and a fix made after review is reviewed again before merge. That re-review may cover only the fix's diff.

# Success Criteria

- lifecycle: A scenario test in a fixture repository with the `atelier init` starter policy drives one issue through: start; edit its notes and links and tick its Done when; a review with one `fix_now` and one `defer` finding; a fix commit; re-review and approval; a note and a link edited after approval; merge. It finishes with no refusal and no approval beyond the fix's re-review, the issue is done in the merge commit, and the author and reviewer together run at most 8 `atelier` commands. The test reports the count.
- code-stales: Scenario tests show that an approval stays current through every record-only change and a clean merge from the base, and goes stale after any code change on the branch, including a conflict resolution that changes code. Merging without a current approval is refused, as today.
- legible: An independent agent that reads only the agent-factory skill, astromech's AGENTS.md and workflow policy, and the CLI help (as the cold read of 2026-11-07 did) correctly states the lifecycle order, what makes an approval stale, and when to file a loose end, and finds no contradiction among those sources. Shown by a validate run whose report is attached at close.
- every-repo: Each repository on this host that uses Atelier (atelier, astromech, cosplay-studio, demiurge, life-on-rails) runs the new policy with the installed binary: `atelier check` and `atelier doctor` pass, and every open after-merge check has become a loose end.

# Shape

These decisions are Sumeet's (2026-11-06, in the thread that produced this strategy). ADR 0028 records them and supersedes the parts of ADRs 0019, 0025, 0026 and 0027 and of PRODUCT_INTENT.md that they replace.

- **Approvals cover code.** An approval is stale when anything except tracker records (`.atelier/issues`, `reviews`, `evidence`) changed since the approved commit, not counting a clean merge from the base. Workflow policy and config count as code, as they do for the CI merge check. The allowed-edit list and the rules built on it go. A Done when edited after start is still shown to the reviewer (atelier-r9hb).
- **One order, enforced where it happens.** `atelier review request` refuses while any Done when line is unticked. `atelier merge` marks the issue done, so no separate close step can land in the wrong place. The author writes the implementer report before review, so the reviewer reads it.
- **Loose ends replace after-merge checks.** A new `loose_end` issue type has its own workflow, as `decision` does: no branch, review or merge, and it is done on the base with a one-line result. A bad result files a linked bug. It is `blocked_by` the issue or release it waits on and is never a mission's child. The After merge marker, stand-ins, `carried to`, the awaiting-validation views and the mission-close warning go, superseding ADR 0027. The `validation` type keeps its meaning, an independent check of a mission's criteria before it closes, and loses its own branch and review round.
- **Records can be corrected.** Each write is checked against the rules `atelier check` applies. Append-only applies once a record is on the base; before that, its author may edit or delete it.
- **Fewer records and gates.** The approval that follows a `fix_now` finding resolves it, and only `defer` and `no_action` record a decision. Evidence receipts stay as notes for checks nothing else records and gate nothing. `land` and the hooks report, and never refuse work that is still in progress. These go: the check of reviewer identity against Git authors (the identity is still recorded), the recorded base branch and the branch-type rules, the source rule on deferral follow-ups, and full-length-only commit ids.
- **A light path.** A change with no issue, such as a release, docs or a mockup, is reviewed and merged on an approval of its branch head, with no start or close.
- **Small repositories work out of the box.** `atelier init` writes a policy that can merge as it is, and every schema change ships a migration.
- **One meaning per word.** check, land, close, review and done each mean one thing across the CLI, the skill and the docs.
- **No regrowth.** PRODUCT_INTENT.md gains a principle: a new gate or record type must name the kind of defect it would have caught in real threads, and a rule that only patches another rule's edge case is a reason to remove the first rule. Every item in this mission names what it deletes. An item that would add a concept other than the loose end goes back to Sumeet.
- **One binary serves every repository on this host.** Record and policy changes keep reading the old form until every repository has migrated.

When qualities conflict, fewer concepts win over finer guarantees, and refusing before a write wins over refusing after it.

# Scope

In: Atelier's CLI, records, gates, starter and own policy, docs and PRODUCT_INTENT.md; the agent-factory skill (spadaval/.agents, edited in a worktree there); the policy, AGENTS.md and open records of every Atelier repository on this host; triage of the six draft missions (atelier-2mky, atelier-4pzj, atelier-gg6b, atelier-l63c, atelier-ll3p, atelier-uf97) against this model.

Out: independent review and the re-review of fixes; the Markdown record store, the issue graph and the work views, except where a removal touches them; the draft missions' new features; re-measuring agent effort after release (atelier-akkp, outside this mission).

# Authority

The orchestrator may change the items, their order and the mechanics within each Shape decision, such as command names other than "loose end", where the implementer report lives before merge, and migration steps. These come back to Sumeet: the outcome and success criteria; anything that would let unreviewed code merge or drop the re-review of fixes; any new concept; renaming the loose end; and which draft-mission items are superseded.

# Risks and Unknowns

- Merge marking an issue done contradicts today's "merge writes no tracker state" principle, and it changes how every repository's records reach the base. The lifecycle item settles the mechanics before any code, and ADR 0028 is amended to match.
- With record edits no longer making approvals stale, a Done when could change after approval without a new review. atelier-r9hb's view and the ticked-before-review rule are meant to cover this; the lifecycle item confirms it with a test.
- The six draft missions hold over a hundred backlog items, and some contradict this model (for example atelier-jzh9, atelier-bywq and atelier-4ppu). Triage comes first so nobody builds them in the meantime.
- The skill's checkout in spadaval/.agents is live and must not change branch, so edits go through a worktree there.
