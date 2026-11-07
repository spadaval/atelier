---
status: completed
last_changed: "2026-11-07: completed; earlier that day, the implementer report stays out of the review brief (jnrh review); earlier that day, active on Sumeet's approval; earlier that day, planning review fixes; created that day from Sumeet's direction, after an analysis of every Atelier session on this host"
---

# Why

Agents and Sumeet keep tripping over Atelier's rules. An analysis of every Claude session on this host that used Atelier (about 110 sessions and 1,100 subagent transcripts, 2026-10-15 to 2026-11-07, five repositories) found that independent review earns its keep, finding real bugs in nearly every round, and that issue and mission records work as shared memory. Nearly everything else that cost agents traces back to one design: an approval is pinned to the bytes of a history that also holds the tracker's own records, so any record edit can make it stale. Atelier grew a list of allowed edits, ordering rules around it, and a new exception each time a rule hit an edge.

The results: approvals made stale by bookkeeping dozens of times, still on 2026-11-06 in four repositories, with re-reviews that almost always found nothing changed; three sources giving three different orders of steps; invalid records accepted and then blocking every command (a 5-hour astromech stall and a force-push of main on 2026-11-05); the same ceremony for releases, docs and mockups as for code; and 70–82% of commits touching only records. An agent reading only the skill, AGENTS.md and the CLI help could not state the order of steps. The lighter-workflow mission (atelier-n4mj) trimmed records at the edges; this mission replaces the layer that produces them.

# Outcome

An agent takes one issue from start to merge in one obvious order, and only a change to code makes an approval stale:

- Edits to tracker records never make an approval stale.
- The order is: start, implement, tick every Done when line, review, fix and re-review, approve, merge. Merge marks the issue done. An issue is done when its change is merged. A mission is closed when all its work is merged and its validation, if it has one, is done.
- Work that changes no code and can only happen after a merge or a release, such as pushing a tag, a play-check or an eval run, is a **loose end**: its own issue type, linked to what it waits on, and never part of a mission.
- Atelier refuses an invalid record when it is written, and an author can correct or delete its own records until they reach the base branch.
- What remains is what review needs: findings, a decision for each deferred or dismissed finding, an approval, and a merge gate that requires an independent approval of the current code plus the repository's merge checks.
- A fresh agent can learn the whole lifecycle from the skill, the repository's AGENTS.md and the CLI help.

Partial outcome worth shipping: approvals that only code makes stale, the single order, and loose ends.

What does not change: no non-trivial change reaches the base branch without an independent review of its code, and a fix made after review is reviewed again before merge. That re-review may cover only the fix's diff.

# Success Criteria

- lifecycle: A scenario test in a fixture repository with the `atelier init` starter policy drives one issue through: start; edit its notes and links and tick its Done when; a review with one `fix_now` and one `defer` finding; a fix commit; re-review and approval; a note and a link edited after approval; merge. It finishes with no refusal and no approval beyond the fix's re-review, and the issue is done in the merge commit. The test reports how many `atelier` commands the author and reviewer ran.
- legible: An independent agent that reads only the agent-factory skill, astromech's AGENTS.md and workflow policy, and the CLI help (as the cold read of 2026-11-07 did) correctly states the lifecycle order, what makes an approval stale, and when to file a loose end, and finds no contradiction among those sources. Shown by a validate run whose report is attached at close.
- every-repo: Each repository on this host that uses Atelier (atelier, astromech, cosplay-studio, demiurge, life-on-rails) runs the new policy with the installed binary: `atelier check` and `atelier doctor` pass, and every open after-merge check has become a loose end.

# Shape

Atelier exists so that agents produce high-quality, correct code that solves real problems, with as little red tape and overhead as possible. Nothing else about it is sacred, and this governs every tradeoff below. When qualities conflict, fewer concepts win over finer guarantees, and refusing before a write wins over refusing after it.

These decisions are Sumeet's (2026-11-06 and 2026-11-07, in the thread that produced this strategy). ADR 0028 records them and supersedes the parts of ADRs 0016, 0019, 0021, 0025, 0026 and 0027 and of PRODUCT_INTENT.md that they replace.

- **Approvals cover code.** An approval is stale when anything except tracker records (`.atelier/issues`, `reviews`, `evidence`) changed since the approved commit, not counting a clean merge from the base. Workflow policy and config count as code. The allowed-edit list and the rules built on it go. A Done when line whose text changed after start is still shown to the reviewer (atelier-r9hb), including after approval, and is not refused; ticking a box alone is not reported.
- **One order, enforced where it happens.** `atelier review request` refuses while any Done when line is unticked: asking for review means the work is implemented and every line is done to the best of the author's knowledge. `atelier merge` marks the issue done, so no separate step can land in the wrong place. The author writes the implementer report before review; the review brief leaves it out, as Agent Factory keeps the author's self-assessment from the reviewer.
- **Loose ends replace after-merge checks.** A new `loose_end` issue type has its own workflow, as `decision` does: no branch, review or merge, and it is done on the base with a one-line result. A bad result files a linked bug. It is `blocked_by` the issue or release it waits on and is never a mission's child. The After merge marker, stand-ins, `carried to`, the awaiting-validation views and the mission-close warning go, superseding ADR 0027. The `validation` type keeps its meaning, an independent assessment of a mission's criteria before it closes, and loses its own branch and review round.
- **Records can be corrected.** Each write is validated against the rules `atelier check` applies. Append-only applies once a record is on the base; before that, its author may edit or delete it.
- **Fewer records and gates.** The approval that follows a `fix_now` finding resolves it, and only `defer` and `no_action` record a decision. Evidence receipts are optional notes for what nothing else records and gate nothing. `land` and the hooks report, and never refuse work that is still in progress. These rules go: a reviewer's identity must differ from the Git authors (the identity is still recorded), the recorded base branch and the branch-type names, and the source rule on deferral follow-ups.
- **A light path.** A change with no issue, such as a release, docs or a mockup, is reviewed and merged on an approval of its branch head, with no start or close.
- **Small repositories work out of the box.** `atelier init` writes a policy that can merge as it is, and every schema change ships a migration.
- **One meaning per word.** check, land, close, review and done each mean one thing across the CLI, the skill and the docs.
- **No regrowth.** PRODUCT_INTENT.md states Atelier's purpose (above) and a principle: a new gate or record type must name the kind of defect it would have caught in real agent sessions, and a rule that only patches another rule's edge case is a reason to remove the first rule. Every item in this mission names what it deletes. An item that would add a concept other than the loose end goes back to Sumeet.
- **One binary serves every repository on this host.** Record and policy changes keep reading the old form until every repository has migrated.

# Scope

In: Atelier's CLI, records, gates, starter and own policy, docs and PRODUCT_INTENT.md; the agent-factory skill (spadaval/.agents, edited in a worktree there); the policy, AGENTS.md and open records of every Atelier repository on this host; triage of the six draft missions (atelier-2mky, atelier-4pzj, atelier-gg6b, atelier-l63c, atelier-ll3p, atelier-uf97) against this model.

Out: independent review and the re-review of fixes; the Markdown record store, the issue graph and the work views, except where a removal touches them; the draft missions' new features; re-measuring agent effort after release (atelier-akkp, outside this mission).

# Authority

The orchestrator may change the items, their order and the mechanics within each Shape decision, such as command names other than "loose end", where the implementer report lives before merge, and migration steps. It may also make small cuts in the same spirit that add no concept, such as accepting unique short commit ids, and say so in the item. ADR 0028 is accepted within this strategy's authority, as ADRs 0025 and 0027 were; Sumeet reads it after merge. These come back to Sumeet: the outcome and success criteria; anything that would let unreviewed code merge or drop the re-review of fixes; any new concept; renaming the loose end; and which draft-mission items are superseded.

# Risks and Unknowns

- Merge marking an issue done contradicts today's "merge writes no tracker state" principle and changes how every repository's records reach the base. The lifecycle item settles the mechanics before any code, and ADR 0028 is amended to match.
- With record edits no longer making approvals stale, a Done when could change after approval without a new review. The lifecycle item shows such a change in `merge --check`, and a test proves that ticking boxes alone is not reported.
- The six draft missions hold over a hundred backlog items, and some contradict this model (for example atelier-jzh9, atelier-bywq and atelier-4ppu). Triage comes first so nobody builds them in the meantime.
- The skill's checkout in spadaval/.agents is live and must not change branch, so edits go through a worktree there.
