# Design of atelier-to83

What was decided in conversation with Sumeet on 2026-11-12 and 2026-11-13,
and why. Item atelier-ogpg moves this into `docs/design.md` and
`docs/product.md`; until then this file is the record. An idea in this
conversation had been raised once before, on 2026-11-10, and was lost
because it never left the thread.

## Shape

- **Two kinds of data were in the same files.** Content (what to build, the
  plan, findings, the Report, decisions) is written, reviewed and tied to
  code at a commit, and belongs in Git. Coordination state (who holds what,
  what is in flight, what is open everywhere) is global and has no version
  per branch. Committing the second kind to the base is what made
  `.atelier/` behave like a database.
- **Most coordination state is not stored at all.** An item is building when
  its branch exists, in review when a review file is on it, approved when a
  verdict is, and done when it is merged. A claim is the creation of a named
  branch on the remote, which is atomic. The whole picture is computed from
  the base and the named branches.
- **Each place has one meaning.** The base is the agreed backlog and the
  history of finished work, written only by merges. A branch is work in
  motion, with its own records, held by one thread. The Desk's database
  holds observations with no home in Git, such as cost and thread activity,
  and can be rebuilt; it is never a second source of truth for records.
- **A branch merges on what it contains.** Code: an independent review. A
  new mission or a changed plan: two approving plan reviews. Records alone:
  `atelier check`, and a branch counts as records alone only when every path
  it changes is under `.atelier/`. A mission's plan is its Why, Outcome,
  Success criteria, Scope and Authority and the files beside `mission.md`
  other than `notes.md` and the reviews. Until the plan has merged, the mission's items are
  part of it too: changing, adding, removing or reassigning one makes the
  approvals stale. Its lead, notes and status, the Result section of
  `mission.md`, and an item added to an open mission, are records. The
  comparison is by section of `mission.md`, not by file. Conflicts are merge conflicts, and `check` on the merged
  tree catches two branches that are each valid and contradict each other.
- **The trunk, not an integration branch.** Each item merges to the base on
  its own, and each merge leaves the base working. A mission half done on
  the base is an accurate picture. A change that cannot land in pieces is
  hidden in the code, as the 0.5 rewrite was in `next/`, by that mission's
  plan.

## Decisions

| Decision | Why |
| --- | --- |
| Only a merge writes the base | A protected branch refuses anything else, and most repositories protect theirs. Requiring the opposite would be backwards. |
| Branch names: `<item-id>`, `<mission-id>`, `backlog/<slug>` and `review/<mission-id>/<round>-<suffix>`, with one optional prefix in `config.toml` | The names are the index where there is no forge, the claim, and the line between published work and scratch. One prefix covers a host's own naming rule; a free pattern could not be read back into an ID. |
| Commands make and choose the branch | An agent never invents a name. A command that would switch the base's own checkout refuses or makes a worktree, since `start` doing so has cost us before. |
| A mission's stored status is `open`, `done` or `dropped` | An open mission read from a branch that has not merged is shown as proposed; on the base it is agreed and may be led and closed. An old record's `ready` reads as `open`, and an old `draft` on the base reads as proposed until the rollout moves it to a branch or its plan is reviewed. |
| A mission records its planners | `new --mission` binds the calling thread, as `start` binds an implementer, and a later thread takes the plan over the same way. It is how `merge` knows a plan review is independent. |
| Two plan reviewers publish on their own refs | Reviewers of one round must not see each other's files, and two pushes to one branch cannot both be first. `verdict` writes only the branch `review/<mission-id>/<round>-<suffix>`, where the suffix is the review file's, and pushes it; it never writes the mission's branch. The planner's next `submit` or `merge` brings every such branch of the round onto the mission's branch itself. The reader finds these branches too, so `show` and the Desk can count a proposed plan's verdicts; `review` never reads another reviewer's branch of the same round. |
| A started item's branch is pushed at `start` where a remote is configured | That push is the claim across clones. The loser is refused and told the holder, and `start` removes the local branch it had made, so nothing in that clone shows the item as held there; a later `start` there is a take-over. With no remote the local ref is the claim, as now. |
| Stopping a mission is dropping it, and `drop` finishes the job | A drop is done only when it is published. On a proposed plan it is committed on the mission's branch, pushed, and the pull request closed. On an open mission `drop` commits on a `backlog/` branch and itself submits and merges it by the record-only path, stopping with status 3 while CI runs, as `merge` does. From that merge `start`, a take-over and an item's `merge` refuse, naming the stop. A planner may run it, and so may the Desk as the caller its config names. |
| A started item is dropped on its own branch | A started item's record changes only on its branch, so that is where a drop is effective at once: `drop` commits there and pushes, and from then the item's `submit` and `merge` are refused. The same `drop` then lands the base's copy itself: it commits it on a `backlog/` branch and submits and merges that by the record-only path, with status 3 while CI runs, so the base never keeps the item as ready after its branch is gone. A drop of an item already merged is refused. |
| A mission has no draft status | A plan on its branch is a draft by where it is. An idea for a mission is a draft item on the base, so the base stays the one list of what we might do and branches stay short-lived. |
| A mission has no go-ahead | It protected direction, and direction is given when the mission starts: in a conversation with Sumeet, or a proposal he agreed. The Why says which. The plan reviews do the checking. He keeps a veto at any time, made real by the Desk telling him when a plan is proposed and merged. |
| The two plan reviews are enforced by `merge` | They are now the only gate, and until now the rule lived only in the skill. |
| Items keep `draft` and `ready` | They record intent: agreed to exist, or agreed to start now. Readying a mission never readied its items, so the lead can hold later work until earlier work has shown what it shows. |
| `lead` stays a field on the mission, as attribution | The Desk wants to show who leads. It is written through a record-only branch once per mission. It is not a lock: the guard on `decide` reads the item's implementers, and a second lead loses at merge. |
| A lead's notes are ordinary record changes | Two or three per mission, through the record-only path. |
| Record-only changes take about a minute | A tracker in Git cannot be instant, and does not need to be. Each repository's CI must give records a fast path, which becomes a line of the readiness checklist. |
| With a forge, a merge finishes as the forge's merge of the pull request | So it works on a protected base and with human reviewers, where `merge` stops with status 3 until they approve. With no forge, `merge` moves the base locally as now. |
| `ready.at` and `created_at` are read from Git | The commit holds the time, and `atelier log` already reads times there. |

## Considered and turned down

| Idea | Why not |
| --- | --- |
| A mission integration branch: items merge into the mission's branch, which merges once at close | It fixes mid-mission additions and gives one human gate per mission, but it is the long-lived feature branch: late conflicts, late delivery and a large final merge. Raised and dropped several times, last as atelier-w0su. The trunk answer above is the one the 0.5 rewrite already used. |
| Moving records to a database, with the Desk as the service | It is consistent with itself, not with the code, which is still on branches. Every checkout, sandbox, eval clone and host would need to reach it. A pull request could no longer carry its record. Mission documents would stay in Git anyway, leaving both. |
| A branch per running mission as the lead's claim | It solved exclusion nobody needs. Nothing that reads `lead` needs it to be a lock. |
| Approval read from GitHub's reviews | GitHub cannot tell Sumeet from an agent on one account, and a second account is against its terms. Dropped on 2026-11-10 for this reason, with the whole idea of plans as pull requests, which did not depend on it. |
| Small record changes direct on the base, plans through pull requests | The base would still need direct pushes, so it fails the first decision. |

## Sizing

- Complexity: high. It changes where seven commands write, how `merge`
  finishes, one hook's rule, the record format of a mission, and the Desk's
  reads, in a binary five repositories share.
- Value: high. It removes the reason the tool cannot be used on a protected
  repository, the cherry-picking and draft pushes of planning, a wait on
  Sumeet for every mission, and the Desk's blind spot.
- Risk: high. A mistake in `merge` or the pre-push rule stops work in five
  repositories at once.

## Derisking

- The design is written and reviewed first (atelier-ogpg), and every other
  item waits on it.
- `merge` through the forge (atelier-t5am) is the riskiest piece. It is
  proved against a local remote whose hook refuses direct pushes before any
  repository's settings change, and with no forge configured.
- The record format keeps reading the old form, and the rollout
  (atelier-ql64) converts one repository, atelier, before the other four.
- Whether to protect the base of any real repository is Sumeet's call and no
  item depends on it.

## Risks and unknowns

- How `done` is recorded when the forge makes the merge commit. Today
  `merge` writes it into the merge commit it builds. Candidates: a last
  commit on the branch, or reading a merged item as done.
- Whether a record-only pull request is quick in the four product
  repositories, whose CI has no fast path for records today.
- Whether reading every named remote branch on each `list` is fast enough
  without a cache outside the Desk.
- A record-writing command run in the base's own checkout: refuse, or make a
  worktree.
