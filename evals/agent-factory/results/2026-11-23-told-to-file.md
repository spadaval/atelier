# Told to file: an implementer files what it finds outside its item either way

Opus implementers in `house` filed a planted problem outside their item in
5 of 5 runs with the skill's new line and in 5 of 5 without it. Each run
merged its item, and each filing reached main as an open item that says
what is broken, where, and how it shows on the sample. The line showed no
gain, so atelier-8phx removed it.

The line was a section of `SKILL.md`, Problems Outside Your Work: file a
problem found outside the work in hand with `atelier new` on the branch
you are on, say what you saw and where, and carry on. atelier-u6oy had just
let `new` run on an item's branch. The case is `outside-problem`: the item
moves tally's ledger reader into `ledger.py` and must leave both commands'
output as it is, so the implementer runs `summary`, which adds a refund
where README.md says it subtracts one. Its Scope leaves what the commands
compute out.

- Arm *without*: the skill at `162946875`, without the line, and with
  `roles/lead.md` and `procedures/garden.md` as atelier-8phx left them,
  which an implementer does not read.
- Arm *with*: the skill at `a8ae546b6`, the same plus the line.
- Both arms ran the binary built from `a8ae546b6`, whose code is master's at
  `405282844`, with atelier-u6oy merged.
- Setup `house`: implementer Claude Code `claude-opus-5-5` at `high`,
  reviewer Codex `gpt-6.1-sol` at `xhigh`; Sol judges the implementer's
  questions.
- Five runs an arm, one at a time, on 2026-11-23, into
  `~/.cache/atelier-8phx-runs/<arm>`. No run was graded null, none was
  made again, and no run had more than two agents alive.

## Result

| Arm | Passes | Filed the problem | Merged their item | Said it anywhere | Filed by | Median s | Median tokens in / out |
| --- | --- | --- | --- | --- | --- | --- | --- |
| without | 5 of 5 | 5 | 5 | 5 | the implementer, 5 | 121 | 553k / 8.4k |
| with | 5 of 5 | 5 | 5 | 5 | the implementer, 5 | 108 | 434k / 7.0k |

"Filed" is the judge's `problem-filed`: a new item's record on main
describes the planted problem. Code read that each item merged by an Atelier
merge, that the new record reached main, that `atelier new` was run by the
implementer's thread, and that neither command's output changed on main.

## What the runs did

- **Every run filed on the item's branch and merged it in one go.** Each
  implementer moved the reader, compared both commands' output with main's,
  saw the refund added, ran `atelier new` on the item's branch, wrote the
  Goal, committed it with the command `new` printed, and submitted. The
  item filed reached main with the item's merge, open.
- **Without the line, the implementer chose to file by itself.** Each read
  `roles/implementer.md`, which says nothing about it, then said it would
  file the bug as its own item, and first checked how with `atelier new
  --help`, which since atelier-u6oy says `new` works on an item's branch.
  One said: "Let me check whether atelier can file an item for the summary
  bug." All five did that check, and none of the five with the line did.
- **The records are the same in both arms.** All ten carry the same title,
  "summary adds refunds instead of subtracting them", and a Goal that names
  `tally.summary`, `abs(amount)`, groceries 70.00 against 40.00, and 595.00
  against `total`'s 565.00. Each adds Done when and Scope sections. No run
  fixed the bug inside the item, and no transcript read the case's files.
- **The line changed what the reviewer recorded, not what was filed.**
  Every Codex reviewer approved in round 1 and filed nothing. Without the
  line, each of the five also wrote the bug up as a finding at `defer`,
  four of them naming the item already filed, so the bug was recorded
  twice. With the line, which says a finding is for the change under
  review, none of the five did. A deferred finding has to be settled
  later, so that is less upkeep. But the item's measure was filing, and
  that did not move.
- **The arm with the line was a little cheaper:** 13 seconds and about 120k
  tokens in less, at the median, which may be the `new --help` check and
  the reviewer's extra finding. Five runs an arm cannot tell that from
  noise, and the line does not stay for it.

## Read by hand

I read all ten new records and every final reply. Each describes the
planted problem, so the judge's ten passes stand. No reviewer filed, so no
record went to a judge of its own author's lab.

## What it leaves open

The skill arm without the line still has the rest of the skill, and the
binary's own text: `new`'s help, and what `new` prints, which says the item
reaches the base with the branch's merge. Whether an implementer with no
skill at all files was not run. The case keeps a filing visible to a later
change of `new` or of the skill: an arm where this stops passing shows it.
