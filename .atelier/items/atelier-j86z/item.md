---
schema: atelier.item
schema_version: 3
id: atelier-j86z
title: 'Garden atelier: 10 recommendations'
status: ready
mission: null
labels:
- garden
- needs-human
- decision
priority: medium
blocked_by: []
implementers: []
ready:
  by: conversion
  at: 2026-11-09T01:49:32.703024421Z
trivial: null
caused_by: null
created_at: 2026-11-09T01:49:32.703024421Z
---

## Goal

Which of these ten recommendations should go ahead? For each, answer in a note: do it, do it differently (say how), or no.

## Done when

- [ ] The question is answered and the answer is written in this item's Report.

## Context

Gardener run 1 for atelier. Window: 2026-11-01 to 2026-11-08. There was no earlier digest, so it covers the last 7 days; repeat counts look back 30 days, to 2026-10-09.

Read:
- 20 root Claude Code sessions for this repository (`~/.claude/projects/-home-supadava-projects-atelier`, `-home-supadava--t3-worktrees-atelier-t3-f937cc1a`) and their 173 subagent transcripts.
- Other repositories' sessions where they hit the same refusals.
- The 161 review findings recorded in the window.
- Failed CI runs: 2, both one macOS test on atelier-45gr, fixed in that item.
- Code and docs: links, documented flags and subcommands, and `#[allow(dead_code)]`.

The Codex store held sessions from 2026-11-08 only, and the one that touched atelier was a reviewer of another gardener trial, so it was left out. The Grok store holds only prompt histories for this repository.

Routine upkeep: one commit removing `atelier check --fix`, which does not exist, from `docs/architecture/quality/agent-factory-atelier-validation.md` (atelier-w4ef's second Done when line). It goes through review and CI on its own branch.

This digest was filed by the orchestrator of atelier-3sk9 from the gardener trials run before that item merged. It holds trial 2's entries, except one the orchestrator applied directly: the stale auto-memory note about `atelier start` (trial 2's R8) was rewritten on 2026-11-08. R8 and R9 come from trial 1, whose other entries trial 2 also found.

## Options

### R1 Say in `issue create --help` that `--parent` cannot name a mission

**Trip.** An agent files work for a mission with `issue create --parent <mission>`, is refused with `workflow_issue_hierarchy_invalid`, and reruns the create without `--parent` and then `issue link --role advances`. The refusal says exactly what to do, so each case costs one retry. In one case it also broke a chained `git switch -c task/$id`, because the id came back empty. This happened 5 times in the window across 4 sessions in 2 repositories, and at least 6 more times from 2026-10-17 on.

**Evidence.**
- `~/.claude/projects/-home-supadava-projects-atelier/ccf37ad9-3e02-42b0-a24b-e1adfa820ccd.jsonl` 2026-11-04T21:42:32Z: "issue atelier-33lr cannot be child of mission atelier-vt8q".
- `.../-home-supadava-projects-atelier/a0cc9335-b19d-4bdd-b733-789aebbb39f8.jsonl` 2026-11-04T23:19:13Z: atelier-mpko under atelier-vt8q, then "fatal: 'task/' is not a valid branch name".
- `.../-home-supadava-projects-astromech/91720d3f-1d27-46b6-8b05-dbc6876889b5.jsonl` 2026-11-07T02:59:31Z: astromech-bif8 under astromech-7q0z.
- `.../-home-supadava-projects-atelier/fa0af964-025c-43a0-a61c-a21cf8023b71/subagents/agent-a7b1fc688d3db2cd7.jsonl` 2026-11-07T18:59:38Z and `agent-a99584e4ad1733801.jsonl` 2026-11-07T18:59:35Z: two planning subagents, each refused for plan-a-jr8n and plan-b-8n2l.
- Before the window: `.../-home-supadava-projects-worldcrafter/62a25c9b-17c5-4516-a7d4-92a3c5dfd5d3.jsonl` 2026-10-17T21:32:28Z.

The help text says only "Parent issue ID" (`crates/atelier-cli/src/main.rs:559`). `skills/agent-factory/procedures/plan.md` never says how work joins a mission. Only `groom.md:75` mentions `advances`.

**Fix (sign).** Change the help to "Parent issue ID (a work item, not a mission; join a mission with `issue link <id> <mission> --role advances`)". Add one line to `plan.md` where it files a mission's items.

**Size.** About 3 lines. Low complexity, low risk, small value per case, but it is the commonest refusal in the window.

**Alternatives.** (Architecture) Add `issue create --mission <id>`, which writes the `advances` link at create time. That is about 40 lines plus a test and removes the retry entirely. (Architecture, riskier) Let `--parent <mission>` write an `advances` link instead of refusing. That hides the parent/mission distinction the policy enforces.

### R2 Tell agents when the checkout they read is far behind the base

**Trip.** The main checkout `/home/supadava/projects/atelier`, where most sessions start, sits on a detached HEAD well behind `master`. It is 179 commits behind as of this run: HEAD 9faf7731, master 3b61dc2c. Its reflog shows it repeatedly moved to a detached `origin/master` on 2026-11-07. Agents there run `atelier issue show` on items filed since then and get "not found on this branch; it exists only on branch master. Next: merge that branch in first". Then they go and find another checkout. `atelier status` there says only "Branch : detached HEAD @ 9faf7731" and reports ready work as of that old commit. This happened 4 times in the window, in 4 sessions.

**Evidence.**
- `.../-home-supadava-projects-atelier/4244a440-bd80-43ba-a507-4763d17afa2c.jsonl` 2026-11-08T05:48:13Z (`issue show atelier-stnv` with the checkout at b81f650d). The same event is in its fork, 7377a49f.
- `.../-home-supadava--t3-scratch-2026-11-08-poteto-s-p-stack-is-6d304d3f/6de5d99b-5271-478a-a789-192018704f4f.jsonl` 2026-11-08T05:52:43Z (atelier-l58v).
- `.../-home-supadava--t3-scratch-2026-11-08-poteto-s-p-stack-is-6d304d3f/e2b31d9f-7f54-480b-94a5-3389ede4e9e7.jsonl` 2026-11-08T07:13:55Z (atelier-stnv; the agent then went to `atelier-wt-l58v`).
- `.../-home-supadava-projects-atelier/dd28600f-932e-4329-93cd-6a5f24ff2422.jsonl` 2026-11-07T15:59:44Z (`evidence show atelier-ulrl`).
- `git -C /home/supadava/projects/atelier rev-list --count HEAD..refs/heads/master` printed 179 on 2026-11-08.

**Fix (sign).** When HEAD is detached at an ancestor of the base, have `atelier status` print a line such as "Records are 179 commits behind master; read master's with `git checkout --detach master`". Have the not-found error (`crates/atelier-app/src/record_index.rs:1371`) give that Next instead of "merge that branch in first", which is wrong advice for a read. A host change also helps: keep `~/projects/atelier` detached at `master` after each merge, as `~/projects/atelier-skill` already is.

**Size.** About 30 lines plus 2 tests. Low complexity, medium value, low risk.

**Alternatives.** Host only: move the T3 project's working directory to the worktree that holds `master` (`~/.t3/worktrees/atelier/t3-f937cc1a`). This needs no code, but it fixes only this host.

### R3 Let `review disposition --follow-up` name an item that exists on the base

**Trip.** The orchestrator files a follow-up on `master`, as planning on the base says, and then records the defer on the work branch. The disposition is refused: "issue atelier-wiri not found on this branch; it exists only on branch master". The agent runs `atelier merge --sync` and retries. In the first case the sync made the approval stale ("Approval needs fresh review: atelier-fo3v predates the point where this branch left the base"), so the item needed another review round. This happened twice in the window.

**Evidence.**
- `.../-home-supadava-projects-atelier/fa0af964-025c-43a0-a61c-a21cf8023b71.jsonl` 2026-11-07T19:04:32Z (atelier-93db, follow-up atelier-wiri). At 19:04:39Z the sync printed "Approval needs fresh review".
- `.../-home-supadava-projects-atelier/dc07e8bf-3cb1-4863-9674-ebd7c070621c.jsonl` 2026-11-08T22:38:33Z (atelier-fr9l, follow-up atelier-1nlr). The sync kept the approval.

**Fix (architecture).** Accept a follow-up that exists on the branch's base, since the merge brings it in anyway. Keep refusing an id that exists nowhere.

**Size.** About 20 to 40 lines plus a test, in the disposition write path. Medium complexity, medium value, low risk.

**Alternatives.** (Sign) Make the error's Next say `atelier merge <id> --sync` and that a record-only base keeps the approval. About 3 lines, but the stale-approval case still costs a review round.

### R4 Remove "its PR is not marked ready" from the skill

**Trip.** On 2026-11-08 the human said there is no case where a draft PR makes sense here, and `c4aed6be` removed "(a draft until it is approved)" from `AGENTS.md`. `skills/agent-factory/SKILL.md:48` still says an unreviewed change's "PR is not marked ready". After that commit an agent still opened a draft and flipped it ready later. Before it, one orchestrator opened two drafts and ran `gh pr ready` four times.

**Evidence.**
- Human direction: `.../-home-supadava-projects-atelier/7377a49f-040f-459e-bbbd-135ff4cd9db3.jsonl` 2026-11-08T15:11:15Z: "I can now think of absolutely zero cases where a draft PR makes sense."
- After the fix: `.../-home-supadava-projects-atelier/9fa7eab8-905e-4778-b03f-68a3f4cff74f.jsonl` 2026-11-08T18:12:55Z `gh pr create --draft ... --head atelier-xe9g`, then 18:14:47Z `gh pr ready 53`.
- Before it: `4244a440-bd80-43ba-a507-4763d17afa2c.jsonl` 2026-11-08T06:59:09Z and 07:08:00Z (`--draft`), then 07:16:01Z, 07:27:26Z, 07:40:12Z and 07:58:22Z (`gh pr ready`).

**Fix (removal).** Delete ", its PR is not marked ready," from `SKILL.md:48`.

**Size.** 1 line. Trivial complexity, small value, no risk. It is a recommendation rather than upkeep only because it removes a rule. The human's 15:12:20Z reply, "Keep the skill separate for now", may have meant to leave the skill alone; please confirm.

**Alternatives.** None worth having.

### R5 Accept free text that starts with `--`

**Trip.** An agent writes a note, reason or summary whose text begins with a flag name, such as "--binary is ignored with --link ...". clap rejects it as an unknown argument. When it is the second command in a chain, the first command has already written a record. The agent rewords and retries. This happened 3 times in the window, in 3 different agents.

**Evidence.**
- `.../-home-supadava-projects-atelier/7377a49f-040f-459e-bbbd-135ff4cd9db3.jsonl` 2026-11-08T06:14:38Z: `review disposition` summary "--binary is ignored...". The agent said: "The first summary started with `--binary`, so it was parsed as a flag." The same event is in fork 4244a440.
- `.../-home-supadava-projects-atelier/7377a49f-040f-459e-bbbd-135ff4cd9db3/subagents/agent-a7b08cf189c28f541.jsonl` 2026-11-08T06:09:51Z: `issue note` text "--unlink removes only symlinks...".
- `.../-home-supadava--t3-worktrees-atelier-t3-f937cc1a/d447132c-155f-4757-87fa-55fb267bddd7/subagents/agent-a7039074243459453.jsonl` 2026-11-05T01:23:32Z: a `--reason` value read as a flag.

**Fix (architecture).** Set `allow_hyphen_values = true` on the free-text arguments: note text, disposition summary, `--reason` and `--body`.

**Size.** About 6 attributes plus 1 test. Low complexity, small value, low risk. The one risk is that a real flag typed in the wrong place becomes text; limiting this to trailing free-text positionals and value-taking options avoids it.

**Alternatives.** Do nothing. Each case costs one retry.

### R6 Make "update docs that describe the old behaviour" part of the implementer's proof

**Trip.** Review keeps catching docs, CLI help and the skill still describing behaviour a change replaced. In the window, 12 of 99 `fix_now` findings were this, across 10 items. `procedures/implement.md:30-31` already says to update stale competing versions, so the rule exists and is not followed. Each finding was fixed, but each cost a review round.

**Evidence.** These `.atelier/reviews/` findings: atelier-a35g (4nsl, `markdown-first-record-store.md:24`), atelier-2x99 (3jdm, `validation.md:157`), atelier-e38y (l457, `workflow-configuration.md:36`), atelier-iup5 (ml3r, help and docs), atelier-kyd9 (jnrh, validation doc and `main.rs:249`), atelier-s90h (md0v, `land --push` help), atelier-u24r (8bh4, `work-model.md:149`), atelier-xb4j (nn67, `workflow-configuration.md:139`), atelier-syf9 (mgl0, `mission-reviews.md`), atelier-qt4c (ml3r, `CONTEXT.md:17`), atelier-ggic (ml3r, ADR 0028) and atelier-wx64 (4yqd, `PRODUCT_INTENT.md:12`).

**Fix (sign).** In `implement.md`'s proof step, have the implementer `git grep` the changed command, flag or term across `docs/`, `skills/`, `CONTEXT.md` and CLI help, and list the hits left alone in the Report. This turns a rule into a step with output a reviewer can check.

**Size.** About 3 lines. Low complexity, medium value, low risk.

**Alternatives.** (Check) Have the review brief list doc files that mention identifiers the diff touches. That is about 80 lines, and it would add noise. (No) Accept it as review working.

### R7 Stop unpacking every branch's tracker into `$TMPDIR`, or at least show why it failed

**Trip.** `atelier status`, `land` and the Stop hook read each branch's records by extracting `git archive` of `.atelier/` into a `tempfile::tempdir()` (`crates/atelier-cli/src/commands/git.rs:116-139`). Here that is 11 MB and 2,282 files per branch. When `/tmp` hits its per-user quota, the extraction fails. `status` then shows "state unknown on <branch>: branch_snapshot: cannot read tracker at <sha>: tar: .atelier/e...". `short_reason` cuts the error at 100 characters (`git.rs:1356-1358`), so "Disk quota exceeded" never shows. The Stop hook printed the full error to the agent. This happened 3 times in 2 repositories in the window. Separately, 15 tool calls failed on the quota on 2026-11-06 and 07, including one subagent that ran `true` four times in a row. Those transcripts do not show what filled `/tmp`.

**Evidence.**
- `.../-home-supadava-projects-atelier/8624a966-e191-448f-b962-4441dacc94cf.jsonl` 2026-11-06T22:14:41Z: "Stop hook feedback: Error: branch_snapshot: ... Cannot write: Disk quota exceeded".
- `.../8624a966-e191-448f-b962-4441dacc94cf/subagents/agent-a937d9b36dbfa2b96.jsonl` 2026-11-06T23:47:58Z: "state unknown on task/atelier-rkxm: branch_snapshot: ... tar: .atelier/e...".
- `.../-home-supadava-projects-astromech/ceea6cb2-75fa-4326-9331-f950d391a690.jsonl` 2026-11-07T00:01:52Z: "Wrote only 4096 of 4196 bytes ... Disk quota exceeded".
- The retry loop: `.../8624a966-e191-448f-b962-4441dacc94cf/subagents/agent-afcc4b86f601b56d9.jsonl` 2026-11-07T02:20:24Z to 02:22:06Z.

**Fix (architecture).** Read a branch's records from Git objects (`git ls-tree` plus `git cat-file --batch`) instead of extracting them. That writes nothing to disk and is faster for many branches.

**Size.** About 60 to 100 lines. Medium complexity, medium value, medium risk, because `RecordIndex::load_structural` reads from a directory today.

**Alternatives.** (Sign) Make `short_reason` keep the last line of tar's stderr, or name EDQUOT/ENOSPC: "the temporary directory is full; free space or set TMPDIR". About 10 lines, low risk. Do this one first if the architecture change is not wanted now.

### R8 Warn in `atelier check` about trailing whitespace in records

**Trip.** Agents hand-edit records (ticking Done when, writing a Report), often with a Python one-off, and leave trailing whitespace or a blank line at the end. Atelier's own writes strip it, but neither `atelier check` nor `land` looks, so CI's `git diff --check` is the first to notice, on master. This happened twice in the window. (Found by gardener trial 1 on 2026-11-08, not by this run.)

**Evidence.**
- Master CI run 42331331319 ("Record approval atelier-hfbr", 2026-11-07) failed `git diff --check` on `.atelier/issues/atelier-akkp.md:32`, a line from a Python edit in `~/.claude/projects/-home-supadava-projects-atelier/8624a966-e191-448f-b962-4441dacc94cf.jsonl` at 2026-11-07T02:51:21Z.
- `.../-home-supadava-projects-atelier/dc07e8bf-3cb1-4863-9674-ebd7c070621c/subagents/agent-a13e4beb9f63fc69e.jsonl` 2026-11-08T23:04:40Z: "new blank line at EOF" on atelier-nz6a's record, which cost a fix commit.

**Fix (check).** `atelier check`, which `land` and the Stop hook run, warns about trailing whitespace and a trailing blank line in a record, with the file, line and fix.

**Size.** About 30 to 50 lines plus a test. Low complexity, medium value (master stays green), low risk as a warning.

**Alternatives.** `land` runs `git diff --check` over unpushed base commits, which covers every file but adds a gate. Or leave it: CI catches it at the cost of one red run.

### R9 Drop the two merge checks CI already runs

**Removal, no trip.** `.atelier/workflow.yaml`'s `merge_checks` run `git diff --check HEAD^ HEAD -- . ':!.atelier'` and `cargo fmt -- --check` on the merged tree. For a code branch, `scripts/merge-ci-passed.sh` already passes only when CI succeeded on code matching the branch head and the base gained no code the branch lacks. CI runs `cargo fmt -- --check` and a wider `git diff --check` (`.github/workflows/ci.yml`). For a record-only branch neither check can fail. Neither refused a merge in the last 30 days. (Proposed by gardener trial 1 on 2026-11-08, checked by hand against the policy, the script and ci.yml.)

**Fix (removal).** Delete those two entries from `merge_checks`. Keep `cargo metadata --locked`, which CI does not check (it builds without `--locked`).

**Size.** About 4 lines of policy, which counts as code, so it needs a PR and CI. Low complexity, low value (two fewer local checks per merge), low risk.

**Alternatives.** Keep them as a cheap local backstop.

### R10 Stop strategy-only commits on master from voiding every branch's CI pass

**Trip, 4 times on 2026-11-08 in one session.** `scripts/merge-ci-passed.sh` counts `docs/strategies/` as code, so when another thread commits a mission strategy to master, every open branch's CI pass stops counting. `atelier merge` refuses with "Run `atelier merge --sync`, push the branch, and wait for CI to pass". Each time the sync was clean and kept the approval, but it cost a fresh CI run of about 12 minutes on macOS, during which the next planning commit could land.

**Evidence.** Session `~/.claude/projects/-home-supadava-projects-atelier/8b4e8565-ace5-4b02-a3b4-e1c8c8c9a37c.jsonl` on 2026-11-08: `atelier merge atelier-3sk9` was refused three times. The causes were `.atelier/missions/atelier-bnek/` and `fleet-dashboard.md` (28ae3d61, afdcfda0), then `fleet-dashboard.md` again (c0a1ead2), then a `plan.md` skill change (5d3f6f88, a real code change). `atelier merge` on `docs/no-check-fix` was refused once, by `docs/strategies/forge-delivery.md` (8095332a).

**Fix (architecture).** Have merge-ci-passed.sh treat base commits that touch only strategy files missions link the way it treats record-only commits. A strategy is still reviewed as planning and still makes an approval stale, but a branch's CI cannot fail on it, since no build or test reads it. About 5 lines in the script's `code` pathspec, plus a test case.

**Size.** About 5 to 15 lines. Low complexity, medium value (fewer reruns while missions are being planned), low risk, though widening "not code" needs care.

**Alternatives.** Leave it: the refusal says exactly what to do, at the cost of a CI run. Or batch planning commits so they land less often.
