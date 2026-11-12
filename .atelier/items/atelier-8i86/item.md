---
schema: atelier.item
schema_version: 3
id: atelier-8i86
title: 'Groom whole backlog: 21 proposals'
status: ready
mission: null
labels:
- groom
- needs-human
- decision
priority: medium
blocked_by: []
implementers: []
ready:
  by: conversion
  at: 2026-11-09T00:01:27.608258423Z
trivial: null
caused_by: null
created_at: 2026-11-09T00:01:27.608258423Z
---

## Goal

Which of these 21 proposals from the first groom of this repository's whole open backlog go ahead? Answer each entry in a note on this item (`atelier issue note <this id> --kind human "G1: yes"`): yes, yes with a change, or no with a reason. G9 has a deadline: atelier-vt8q's records become prunable after 2026-11-11.

## Done when

- [ ] The question is answered and the answer is written in this item's Report.

## Context

First run of `procedures/groom.md` (atelier-sb0l), scope: the whole open backlog, 125 items. Drafted by its trial at 87433db5, then rechecked against master at 6772ac6f: every named item is still open, and G14 now applies because atelier-nz6a merged. Each stale and duplicate claim was checked by hand against the records and code (atelier-sb0l's Report lists them). Entries are numbered as in the trial; G9 is listed first for its deadline.

Record fixes applied with this digest, evident from the records: `related` links lkuy to om6l, cj54 to sxcj, k55o to w4ef and kset to sfd9 (each pair's text names the other); the `agent-factory` label on 2yef, rqzu, c6g2 and pg2b; and the Done when bullets of 34 items boxed as `- [ ]`, wording unchanged, as `atelier check` asks. 38 items still have prose Done when text that needs rewording, which is planning, not a record fix.

Not checked, and left out of the entries: whether atelier-1nlr reproduces (G4 orders it first for that reason), and the astromech CI pin covering binaries installed elsewhere (G6).

## Options

### Deadline

```text
G9 atelier-3km4: Sumeet's taste read goes stale on 2026-11-11.
   Evidence: 3km4 asks Sumeet to read atelier-vt8q's HTML debrief and record it on
   atelier-r6bt (done). k7vp, les9, wg6p and okfy say vt8q's records become prunable after
   2026-11-11, after which that debrief cannot be regenerated as it is. jdpf says whether
   the debrief earns its place is Sumeet's question alongside 3km4.
   Change: Sumeet reads vt8q's debrief before 2026-11-11; or rewrite 3km4 to read the debrief
   jdpf reviews (atelier-mmpv's) and `atelier issue link atelier-3km4 atelier-jdpf
   --role blocked_by`.
   Size: complexity low; value medium (decides whether ten debrief items are worth doing);
   risk low.
```

### Sequence

```text
G1 atelier-4pzj: split out the start-and-ready half as its own mission.
   Evidence: 4pzj holds 33 open items, and its Background says "With about 33 items the
   mission is large". Its only criterion, ready-means-startable, is about `start` agreeing
   with the ready views, and eight items carry it: hugc, hueh, mfyd, oxnw, xw8r, l9ty, tf9n,
   b17k. atelier-hugc (P1) reproduces today: on master in a scratch clone,
   `atelier start atelier-taxm` (a backlog item) created branch atelier-taxm, printed
   "Status: backlog (no start transition from this status)" and exited 0.
   Change: a new mission "Make start accept exactly what the views offer as ready", taking
   ready-means-startable from 4pzj; for each of the eight,
   `atelier issue unlink <id> atelier-4pzj --role advances` and
   `atelier issue link <id> <new mission> --role advances`. 4pzj keeps the view items and no
   criterion. The new mission's body is planning: written as plan does, reviewed before ready.
   Size: complexity medium (eight items over start.rs and the work-row renderer); value high
   (hugc lets agents branch parked work, a current hazard); risk low (record moves only).

G2 atelier-4pzj: move its nine P3 polish items to the unassigned backlog.
   Evidence: 4pzj's Background: "leaving the P3 polish unassigned is the first cut to
   consider". The P3 items outside G1: 1a5k, 8g9d, d2xe, d4b6, efnf, lehs, o1bv, pvo0, x0vs.
   Spot checks show they still hold, so this is a cut, not a decline: 1a5k
   (`atelier work parent atelier-rqzu` lists rqzu as its own row), x0vs
   (`atelier issue show atelier-rc8j --full` prints each decision note twice), o1bv
   (`COLUMNS=30 atelier work ready | cat` wraps rows), lehs (Impact still printed).
   Change: `atelier issue unlink <id> atelier-4pzj --role advances` for each. After G1 and G2,
   4pzj holds 16 items.
   Size: complexity low each; value low (display polish with workarounds); risk low.

G3 atelier-b17k: merge into atelier-tf9n.
   Evidence: b17k asks to remove `apply_ready_execution_gates`
   (crates/atelier-cli/src/commands/work.rs) or justify it. tf9n's fourth Done when line
   exists because that function pushes the literal "readiness gate" into blocker lists, still
   true (`grep -n '"readiness gate"' crates/atelier-cli/src/commands/work.rs`). Done apart,
   they change the same function twice.
   Change: add to tf9n's Done when "- [ ] apply_ready_execution_gates is removed, or a test
   records the case it catches that classification does not";
   `atelier issue transition atelier-b17k supersede --reason "Merged into atelier-tf9n"`.
   Size: complexity low; value low (a dead gate and one scan); risk low.

G4 atelier-fad5, atelier-nl9a, atelier-1nlr: form a mission "No unreviewed code reaches master".
   Evidence: all three guard one invariant, which nl9a's Context names: "the one invariant
   atelier-mmpv keeps, that no unreviewed code reaches the base". fad5 (P1) stops piped gates
   (incident 1620b534); nl9a catches a slipped push in CI and names fad5 as the prevention;
   1nlr reports `land` passing a plain `git merge --no-ff` of unreviewed code (not yet
   reproduced). All three are unassigned, at P1, P2 and P2.
   Change: a new mission with that outcome, the three linked `--role advances`, 1nlr first
   because its reproduction decides whether land needs a fix. The body is planning
   (written as plan does, reviewed).
   Size: complexity medium (merge --push reuses land --push; land --since and a CI job);
   value high (a hazard with a recorded incident); risk medium (gate changes reach every
   repository through the shared binary).

G5 atelier-jjvc: plan it; its pick-up condition is met.
   Evidence: jjvc says to plan it "when neither this repository's nor astromech's committed
   .claude/settings.json contains prime --hook claude". Both now run
   `atelier status --hook claude`: `git -C ~/projects/astromech show
   origin/main:.claude/settings.json` line 7, and this repository's .claude/settings.json has
   no `prime --hook`. The 2026-10-17 note that held it back read astromech-atelier at
   dd28efe, an older checkout.
   Change: `atelier issue transition atelier-jjvc plan`.
   Size: complexity low (remove an alias); value low; risk medium (an adopter with an old hook
   loses session context; its Done when makes check and land name such hooks).

G6 atelier-hfme: plan it; its pick-up condition is met.
   Evidence: hfme waits for every Atelier CI pin to pass atelier-nn67. astromech's pin is
   4910be45 (origin/main .github/workflows/atelier.yml:26; "Merge atelier-stnv",
   2026-11-08), and `git merge-base --is-ancestor 135c1f9e 4910be45` succeeds, where
   135c1f9e is "Merge atelier-nn67 into master". Its blocker nn67 is done.
   Change: `atelier issue transition atelier-hfme plan`.
   Size: complexity low (remove two shims); value low (cleanup); risk medium (the pin covers
   astromech's CI, not an older binary installed elsewhere).

G7 atelier-fpxc: plan it, after atelier-pu2i; its pick-up condition is met.
   Evidence: Sumeet's note of 2026-10-17: plan it "when transcript counts still show issue
   list piped through grep after atelier-7u4o and atelier-ihns land". Both closed on
   2026-10-17. Since then, 34 of 85 distinct `atelier issue list` command lines in this
   host's Claude Code transcripts (~/.claude/projects/**/*.jsonl) pipe through grep. Several
   are `grep -v` on done rows, which pu2i (P1, in 4pzj) removes, hence after pu2i.
   Change: `atelier issue transition atelier-fpxc plan`;
   `atelier issue link atelier-fpxc atelier-pu2i --role blocked_by`.
   Size: complexity low; value medium (a path in current use); risk low.

G8 atelier-jdpf and the ten debrief items: block the ten on jdpf, which is ready now.
   Evidence: jdpf names j79t, k7vp, wg6p, 1hbk, g21n, les9, lozn, okfy, rdsa and ztuq and
   plans them against what its review finds; okfy, rdsa and ztuq say they "stand alone until
   loose end atelier-jdpf reviews a debrief". jdpf's blocker atelier-mmpv is done and
   `atelier work ready` lists jdpf. None of the ten has a blocked_by link, so nothing stops
   one being started first.
   Change: `atelier issue link <id> atelier-jdpf --role blocked_by` for each of the ten.
   Size (jdpf): complexity low; value medium (decides ten items); risk low.


G10 atelier-c6g2: fold its two skill edits into atelier-avx0 and atelier-fb1i, and supersede it.
   Evidence: c6g2 exists because skill edits had to be committed in spadaval/.agents
   (ADR 0023). The skill now lives in skills/agent-factory here (atelier-dkul done,
   ADR 0029), so each edit can land with its command, which is mission atelier-g6rp's point.
   Its second line names "the Administration section of references/atelier.md", which no
   longer exists (`grep -rn Administration skills/agent-factory` finds nothing).
   Change: add to avx0's Done when "- [ ] skills/agent-factory/procedures/orchestrate.md
   tells orchestrators to read `atelier work mission <id> --since <commit>` on wake-up";
   add to fb1i's "- [ ] skills/agent-factory/references/atelier.md says how check reports
   graph errors when a record fails to parse";
   `atelier issue transition atelier-c6g2 supersede --reason "Folded into atelier-avx0 and
   atelier-fb1i now that the skill lives here"`.
   Size: complexity low; value low; risk low.
```

### Not doing

```text
G11 atelier-wiri: supersede; atelier-dkul did it.
   Evidence: 2fd7620f "Point docs at the moved Agent Factory skill in skills/agent-factory"
   (atelier-dkul) repointed all four links: docs/product/mission-authoring.md:4 and
   docs/product/work-model.md:6 now link references/atelier.md#records, and
   mission-authoring.md:15 and docs/adr/0024 line 52 link plan.md#ready-work.
   `grep -rn "tracker-records\|ready-missions" docs/product docs/adr` finds nothing.
   Change: `atelier issue transition atelier-wiri supersede --reason "Done by atelier-dkul
   in 2fd7620f"`.
   Size: complexity low; value none (done); risk low.

G12 atelier-4gsb: decline; the skill asks for severity again.
   Evidence: 4gsb rests on Agent Factory having removed severity. It is back:
   skills/agent-factory/procedures/review.md:99 "R1 path:line: severity, problem, impact,
   and suggested fix", and :106 defines it ("sized low, medium or high as Risk is"). The
   brief (crates/atelier-cli/src/commands/review_request.rs:676, :683) asks for severity, so
   the two agree today; doing 4gsb would reopen the conflict.
   Change: `atelier issue transition atelier-4gsb decline --reason "Agent Factory's review.md
   asks for severity again (lines 99 and 106), so the brief agrees with it"`.
   Size: complexity low; value none now; risk low.

G13 atelier-8ghm: supersede; the test it fixes was deleted.
   Evidence: `a_retained_claim_whose_pass_later_commits_staled_keeps_its_failure_cleared`
   is gone from crates/ (`grep -rn` finds nothing). `git log -S` names bf4fddad "Cut the
   records and gates that review does not need (atelier-md0v)", which removed it along with
   retained claims.
   Change: `atelier issue transition atelier-8ghm supersede --reason "atelier-md0v removed the
   test and the rule it covered (bf4fddad)"`.
   Size: complexity low; value none; risk low.

G14 atelier-pg2b: supersede; atelier-nz6a did it.
   Evidence: atelier-nz6a merged (done). skills/agent-factory/procedures/install.md:31 now
   says "If `atelier` is not found, give the install lines in SKILL.md", and SKILL.md:9-10
   gives them. pg2b also says to commit in spadaval/.agents, which no longer holds the skill.
   Change: `atelier issue transition atelier-pg2b supersede --reason "Done by atelier-nz6a"`.
   Size: complexity low; value none (done); risk low.

G15 atelier-4urj: decline; the evidence gate it would unify is gone.
   Evidence: 4urj asks gates and views to share one supersession rule across
   `evaluate_issue_gate`, `newest_failure` and `item_proof`. `evaluate_issue_gate` no longer
   exists (`grep -rn evaluate_issue_gate crates` finds nothing); receipts gate nothing since
   atelier-md0v. The two left (crates/atelier-app/src/evidence_freshness.rs:280 and :391)
   feed views and the "failing evidence" readiness fact, and atelier-efnf already asks
   whether that fact stays.
   Change: `atelier issue transition atelier-4urj decline --reason "Receipts gate nothing
   since atelier-md0v; atelier-efnf decides whether the failing-evidence fact stays"`.
   Size: complexity medium (a shared helper over two read paths); value low (views only);
   risk low.

G16 atelier-w0su: decline; the skill no longer wants integration branches.
   Evidence: w0su's alternative Done when is "a decision record explains why the skill should
   stop recommending integration branches". It has stopped:
   skills/agent-factory/procedures/orchestrate.md:53-55 says "with Atelier do not stack one
   owner's branch on another's or plan an integration or rewrite branch", and
   procedures/migrate.md:27 says "Atelier cannot merge such a branch yet, so with Atelier
   prefer" another route. No open item depends on it. Sumeet deferred it on 2026-10-16 as
   "planned now, built later" for parallel missions, so this is his call.
   Change: `atelier issue transition atelier-w0su decline --reason "..."`, or keep it with a
   note that the skill routes around it.
   Size: complexity high (new branch state, a mission-merge gate, a trust model); value low
   (no current path needs it); risk medium (merge gates).
   Answer, Sumeet 2026-11-11: yes, drop it.

G17 atelier-e95f: decline, not worth its cost. Low confidence.
   Evidence: e95f asks for an ADR and a declarative view format, with the built-in views
   rewritten in it. It names its own test of need: wanting a custom view "means the built-in
   view is incomplete: file that defect, per ADR 0022 Decision 4", and the backlog already
   holds those defects (pu2i, 5qep, fpxc, om6l, l9ty). It cites no transcript or repository
   asking for a custom view. Its stated blocker atelier-i2qq is done, so only priority holds
   it. It came from a conversation with Sumeet.
   Change: `atelier issue transition atelier-e95f decline --reason "..."`.
   Size: complexity high (a public config format, built-in views rebuilt on it); value low
   (no demand cited); risk medium.
   Answer, Sumeet 2026-11-11: yes, drop it.
```

### Shape

```text
G18 atelier-2yef: rewrite its title and Done when.
   Evidence: its title is the head of the finding text ("skills/agent-factory/references/
   atelier.md:15-16 vs :66-67, and procedures/orchestrate.md:89 (eb14a31). Declining a
   sta…") and its Done when is "Done when was not specified.". The gap still holds:
   references/atelier.md:15-16 says a declined item has "no branch, review or merge", and
   :69-70 says "a started item's own record changes only on its work branch".
   Change: `atelier issue update atelier-2yef --title "Say where a declined started item's
   record goes and what happens to its branch"`, and Done when "- [ ] references/atelier.md
   says a declined started item's record is committed on the base, its branch is
   archive-tagged and deleted, and follow-ups filed only on that branch lapse with it."
   Size: complexity low (one sentence); value medium (stop signals made this path common);
   risk low.

G19 atelier-rqzu: give it a Done when, and file a bug on `atelier review defer`'s output.
   Evidence: rqzu, 2yef and y3d6 were all made by `atelier review defer atelier-l58v` and all
   carry "Done when was not specified."; the children's titles are cut-off finding text.
   `atelier check` warns on rqzu and 2yef. Every future `review defer` does the same, and
   atelier-k55o (P1) would make `issue create` refuse such records.
   Change: rqzu's Done when "- [ ] Every child is done or declined."; file a bug "Make review
   defer write a titled follow-up with a Done when a reviewer can check" (title from the
   finding's first clause, Done when from its suggested fix), blocked by the decision in G20.
   Size: complexity low; value medium (each review defer files shapeless items); risk low.

G20 atelier-k55o and atelier-w4ef: file the decision both wait on.
   Evidence: both carry the same Unknown, "Whether backlog items may be filed with
   placeholders", and neither can finish without the answer; no item tracks it. k55o
   reproduces today: in a scratch clone, `atelier issue create "x"` succeeds and writes
   "Done when was not specified.". This repository already holds such records (G19).
   Change: `atelier issue create "May backlog items be filed with a placeholder Goal or Done
   when?" --issue-type decision -l needs-human`, then
   `atelier issue link atelier-k55o <decision> --role blocked_by` and the same for w4ef.
   Size (the decision): complexity low; value high (unblocks a P1 and decides review defer's
   output); risk low.

G21 atelier-sl01: rewrite it to the half that remains.
   Evidence: the doc half is done: docs/product/human-cli-output.md no longer says a refused
   transition "prints no transition report" (`grep -n "transition report"` finds nothing).
   The error half is partly done: `atelier issue transition atelier-taxm supersede`
   (scratch clone) now ends "missing required field close_reason; rerun with
   `--reason "..."`, or write it in the record's `## Report` section", a recovery step, but
   still opens "Transition 'supersede' is blocked" where views say "allowed, needs --reason".
   Change: Done when becomes "- [ ] A transition run without its required --reason says it
   needs --reason, not that it is blocked; a test covers supersede." Drop the doc line.
   Size: complexity low; value low; risk low.
```

Entries by kind: sequence 10 (G1-G10), not doing 7 (G11-G17), shape 4 (G18-G21).
