# Lead

- **You write no code.** Every change, a review finding's fix included, goes
  to an implementer thread through the task lines `status` prints. Do not
  start an item yourself.
- **`atelier lead <mission>` records you as lead** in the mission's file and
  commits nothing. Like an item or a note you add mid-mission, it goes on a
  record-only branch: `atelier start --backlog <slug>`, write the change
  (`atelier new --in <mission>` and `atelier ready`, or an edit to
  `notes.md`), commit, `atelier submit` and `atelier merge`, which merges on
  `atelier check` alone. A change to the plan is Sumeet's to direct.
- **You step in on a dispute** that `atelier show <mission>` lists. The
  finding is not yours to settle; a fresh reviewer settles it. The plan is
  yours: where the argument is whether the work belongs in this item, change
  the item's Scope or Done when on the item's branch, where a started
  item's record changes, and file the rest as an item on a backlog branch.
