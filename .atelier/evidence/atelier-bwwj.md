---
created_at: "2026-07-17T00:05:36.655108283+00:00"
id: "atelier-bwwj"
evidence_type: "validation"
captured_at: "2026-07-17T00:05:23.649545175+00:00"
command: "bash -lc '\nset -euo pipefail\nskill=.agents/skills/agent-factory/SKILL.md\nmr=.agents/skills/agent-factory/procedures/mission-review.md\nplan=.agents/skills/agent-factory/procedures/plan.md\norch=.agents/skills/agent-factory/procedures/orchestrate.md\ninstall=.agents/skills/agent-factory/procedures/install.md\nready=.agents/skills/agent-factory/procedures/readiness.md\nshape=.agents/skills/agent-factory/references/repository-shape.md\nfixture=.agents/skills/agent-factory/fixtures/mission-review-handoffs.md\ndogfood=.agents/skills/agent-factory/fixtures/mission-review-dogfood.md\nmrline=$(grep -nF \"3. If the work starts from an exact mission draft\" \"$skill\" | cut -d: -f1)\norchline=$(grep -nF \"4. If work spans a mission, epic, or multiple tracker items\" \"$skill\" | cut -d: -f1)\ntest \"$mrline\" -lt \"$orchline\"\ngrep -Fq \"This rule takes precedence over generic mission\" \"$skill\"\ngrep -Fq \"This is neither code\" \"$skill\"\ngrep -Fq \"Be independent and read-only\" \"$mr\"\ngrep -Fq \"Exact graph revision:\" \"$mr\"\ngrep -Fq \"Complete graph scope:\" \"$mr\"\ngrep -Fq \"initial author identity and every material editor\" \"$mr\"\ngrep -Fq \"edit makes that result stale\" \"$mr\"\ngrep -Fq \"authorize the planner to approve that same graph\" \"$plan\"\ngrep -Fq \"current repository-reported approval and ready\" \"$plan\"\ngrep -Fq \"current manager/status/mission surfaces\" \"$orch\"\ngrep -Fq \"current independent approval and is reported ready\" \"$orch\"\ngrep -Fq \"cannot expose\" \"$install\"\ngrep -Fq \"not rate a repository fully operable for mission planning\" \"$ready\"\ngrep -Fq \"authored draft state\" \"$shape\"\ngrep -Fq \"Expected route: mission-review.\" \"$fixture\"\ngrep -Fq \"Forbidden route: orchestrate until Atelier reports current independent approval\" \"$fixture\"\ngrep -Fq \"Live lifecycle behavior remains deferred\" \"$fixture\"\ngrep -Fq \"Result: not fully operable for Agent Factory mission planning.\" \"$fixture\"\ngrep -Fq \"These packets are input fixtures\" \"$dogfood\"\ngrep -Fq \"The assignee must not approve this packet because its actor is the author\" \"$dogfood\"\nbin=/root/atelier-worktrees/atelier-p2wk/target/debug/atelier\nplanner=\"$($bin man planner)\"\nreviewer=\"$($bin man reviewer)\"\nmanager=\"$($bin man manager)\"\nmission=\"$($bin work mission atelier-p4z2 --ready)\"\ngrep -Fq \"approval of a graph you authored or materially edited\" <<<\"$planner\"\ngrep -Fq \"plan-review <mission-id> request\" <<<\"$planner\"\ngrep -Fq \"plan-review <mission-id> approve\" <<<\"$reviewer\"\ngrep -Fq \"authoring or materially editing the graph being approved\" <<<\"$reviewer\"\ngrep -Fq \"Inspect dependency-safe work\" <<<\"$manager\"\ngrep -Fq \"planners author and reviewers independently approve\" <<<\"$manager\"\ngrep -Fq \"ready 0\" <<<\"$mission\"\ngrep -Fq \"List mission blockers\" <<<\"$mission\"\nfor id in atelier-av92 atelier-havm atelier-a5is atelier-62ap atelier-pt92 atelier-vooj atelier-iw6l atelier-k19f atelier-0162 atelier-83th atelier-aoum; do test -f \".atelier/evidence/$id.md\"; done\nprintf \"%s\\n\" \\\n\"PASS C1 distinct mission-review routing; exact-draft precedence at lines $mrline<$orchline\" \\\n\"PASS C2 read-only exact-revision complete-graph review with author/editor independence and freshness\" \\\n\"PASS C3 planner requests only; orchestrator trusts only Atelier current approval plus ready state\" \\\n\"PASS C4 unsupported lifecycle is explicitly a readiness gap; proof classification is fixture-only\" \\\n\"PASS C5 live planner/reviewer/manager surfaces enforce role separation; atelier-p4z2 ready=0 and blocker-routed\" \\\n\"PASS C6 prerequisite evidence present: static, focused tests, role transcripts, readiness, full suite, diff check\"\n'"
exit_status: "0"
target:
  kind: "issue"
  id: "atelier-h3wg"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-h3wg"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 1
status: "recorded"
title: "bash -lc '\nset -euo pipefail\nskill=.agents/skills/agent-factory/SKILL.md\nmr=.agents/skills/agent-factory/procedures/mission-review.md\nplan=.agents/skills/agent-factory/procedures/plan.md\norch=.agents/skills/agent-factory/procedures/orchestrate.md\ninstall=.agents/skills/agent-factory/procedures/install.md\nready=.agents/skills/agent-factory/procedures/readiness.md\nshape=.agents/skills/agent-factory/references/repository-shape.md\nfixture=.agents/skills/agent-factory/fixtures/mission-review-handoffs.md\ndogfood=.agents/skills/agent-factory/fixtures/mission-review-dogfood.md\nmrline=$(grep -nF \"3. If the work starts from an exact mission draft\" \"$skill\" | cut -d: -f1)\norchline=$(grep -nF \"4. If work spans a mission, epic, or multiple tracker items\" \"$skill\" | cut -d: -f1)\ntest \"$mrline\" -lt \"$orchline\"\ngrep -Fq \"This rule takes precedence over generic mission\" \"$skill\"\ngrep -Fq \"This is neither code\" \"$skill\"\ngrep -Fq \"Be independent and read-only\" \"$mr\"\ngrep -Fq \"Exact graph revision:\" \"$mr\"\ngrep -Fq \"Complete graph scope:\" \"$mr\"\ngrep -Fq \"initial author identity and every material editor\" \"$mr\"\ngrep -Fq \"edit makes that result stale\" \"$mr\"\ngrep -Fq \"authorize the planner to approve that same graph\" \"$plan\"\ngrep -Fq \"current repository-reported approval and ready\" \"$plan\"\ngrep -Fq \"current manager/status/mission surfaces\" \"$orch\"\ngrep -Fq \"current independent approval and is reported ready\" \"$orch\"\ngrep -Fq \"cannot expose\" \"$install\"\ngrep -Fq \"not rate a repository fully operable for mission planning\" \"$ready\"\ngrep -Fq \"authored draft state\" \"$shape\"\ngrep -Fq \"Expected route: mission-review.\" \"$fixture\"\ngrep -Fq \"Forbidden route: orchestrate until Atelier reports current independent approval\" \"$fixture\"\ngrep -Fq \"Live lifecycle behavior remains deferred\" \"$fixture\"\ngrep -Fq \"Result: not fully operable for Agent Factory mission planning.\" \"$fixture\"\ngrep -Fq \"These packets are input fixtures\" \"$dogfood\"\ngrep -Fq \"The assignee must not approve this packet because its actor is the author\" \"$dogfood\"\nbin=/root/atelier-worktrees/atelier-p2wk/target/debug/atelier\nplanner=\"$($bin man planner)\"\nreviewer=\"$($bin man reviewer)\"\nmanager=\"$($bin man manager)\"\nmission=\"$($bin work mission atelier-p4z2 --ready)\"\ngrep -Fq \"approval of a graph you authored or materially edited\" <<<\"$planner\"\ngrep -Fq \"plan-review <mission-id> request\" <<<\"$planner\"\ngrep -Fq \"plan-review <mission-id> approve\" <<<\"$reviewer\"\ngrep -Fq \"authoring or materially editing the graph being approved\" <<<\"$reviewer\"\ngrep -Fq \"Inspect dependency-safe work\" <<<\"$manager\"\ngrep -Fq \"planners author and reviewers independently approve\" <<<\"$manager\"\ngrep -Fq \"ready 0\" <<<\"$mission\"\ngrep -Fq \"List mission blockers\" <<<\"$mission\"\nfor id in atelier-av92 atelier-havm atelier-a5is atelier-62ap atelier-pt92 atelier-vooj atelier-iw6l atelier-k19f atelier-0162 atelier-83th atelier-aoum; do test -f \".atelier/evidence/$id.md\"; done\nprintf \"%s\\n\" \\\n\"PASS C1 distinct mission-review routing; exact-draft precedence at lines $mrline<$orchline\" \\\n\"PASS C2 read-only exact-revision complete-graph review with author/editor independence and freshness\" \\\n\"PASS C3 planner requests only; orchestrator trusts only Atelier current approval plus ready state\" \\\n\"PASS C4 unsupported lifecycle is explicitly a readiness gap; proof classification is fixture-only\" \\\n\"PASS C5 live planner/reviewer/manager surfaces enforce role separation; atelier-p4z2 ready=0 and blocker-routed\" \\\n\"PASS C6 prerequisite evidence present: static, focused tests, role transcripts, readiness, full suite, diff check\"\n'"
updated_at: "2026-07-17T00:05:36.662585614+00:00"
---

## Summary

bash -lc '
set -euo pipefail
skill=.agents/skills/agent-factory/SKILL.md
mr=.agents/skills/agent-factory/procedures/mission-review.md
plan=.agents/skills/agent-factory/procedures/plan.md
orch=.agents/skills/agent-factory/procedures/orchestrate.md
install=.agents/skills/agent-factory/procedures/install.md
ready=.agents/skills/agent-factory/procedures/readiness.md
shape=.agents/skills/agent-factory/references/repository-shape.md
fixture=.agents/skills/agent-factory/fixtures/mission-review-handoffs.md
dogfood=.agents/skills/agent-factory/fixtures/mission-review-dogfood.md
mrline=$(grep -nF "3. If the work starts from an exact mission draft" "$skill" | cut -d: -f1)
orchline=$(grep -nF "4. If work spans a mission, epic, or multiple tracker items" "$skill" | cut -d: -f1)
test "$mrline" -lt "$orchline"
grep -Fq "This rule takes precedence over generic mission" "$skill"
grep -Fq "This is neither code" "$skill"
grep -Fq "Be independent and read-only" "$mr"
grep -Fq "Exact graph revision:" "$mr"
grep -Fq "Complete graph scope:" "$mr"
grep -Fq "initial author identity and every material editor" "$mr"
grep -Fq "edit makes that result stale" "$mr"
grep -Fq "authorize the planner to approve that same graph" "$plan"
grep -Fq "current repository-reported approval and ready" "$plan"
grep -Fq "current manager/status/mission surfaces" "$orch"
grep -Fq "current independent approval and is reported ready" "$orch"
grep -Fq "cannot expose" "$install"
grep -Fq "not rate a repository fully operable for mission planning" "$ready"
grep -Fq "authored draft state" "$shape"
grep -Fq "Expected route: mission-review." "$fixture"
grep -Fq "Forbidden route: orchestrate until Atelier reports current independent approval" "$fixture"
grep -Fq "Live lifecycle behavior remains deferred" "$fixture"
grep -Fq "Result: not fully operable for Agent Factory mission planning." "$fixture"
grep -Fq "These packets are input fixtures" "$dogfood"
grep -Fq "The assignee must not approve this packet because its actor is the author" "$dogfood"
bin=/root/atelier-worktrees/atelier-p2wk/target/debug/atelier
planner="$($bin man planner)"
reviewer="$($bin man reviewer)"
manager="$($bin man manager)"
mission="$($bin work mission atelier-p4z2 --ready)"
grep -Fq "approval of a graph you authored or materially edited" <<<"$planner"
grep -Fq "plan-review <mission-id> request" <<<"$planner"
grep -Fq "plan-review <mission-id> approve" <<<"$reviewer"
grep -Fq "authoring or materially editing the graph being approved" <<<"$reviewer"
grep -Fq "Inspect dependency-safe work" <<<"$manager"
grep -Fq "planners author and reviewers independently approve" <<<"$manager"
grep -Fq "ready 0" <<<"$mission"
grep -Fq "List mission blockers" <<<"$mission"
for id in atelier-av92 atelier-havm atelier-a5is atelier-62ap atelier-pt92 atelier-vooj atelier-iw6l atelier-k19f atelier-0162 atelier-83th atelier-aoum; do test -f ".atelier/evidence/$id.md"; done
printf "%s\n" \
"PASS C1 distinct mission-review routing; exact-draft precedence at lines $mrline<$orchline" \
"PASS C2 read-only exact-revision complete-graph review with author/editor independence and freshness" \
"PASS C3 planner requests only; orchestrator trusts only Atelier current approval plus ready state" \
"PASS C4 unsupported lifecycle is explicitly a readiness gap; proof classification is fixture-only" \
"PASS C5 live planner/reviewer/manager surfaces enforce role separation; atelier-p4z2 ready=0 and blocker-routed" \
"PASS C6 prerequisite evidence present: static, focused tests, role transcripts, readiness, full suite, diff check"
'

## Command

```console
bash -lc '
set -euo pipefail
skill=.agents/skills/agent-factory/SKILL.md
mr=.agents/skills/agent-factory/procedures/mission-review.md
plan=.agents/skills/agent-factory/procedures/plan.md
orch=.agents/skills/agent-factory/procedures/orchestrate.md
install=.agents/skills/agent-factory/procedures/install.md
ready=.agents/skills/agent-factory/procedures/readiness.md
shape=.agents/skills/agent-factory/references/repository-shape.md
fixture=.agents/skills/agent-factory/fixtures/mission-review-handoffs.md
dogfood=.agents/skills/agent-factory/fixtures/mission-review-dogfood.md
mrline=$(grep -nF "3. If the work starts from an exact mission draft" "$skill" | cut -d: -f1)
orchline=$(grep -nF "4. If work spans a mission, epic, or multiple tracker items" "$skill" | cut -d: -f1)
test "$mrline" -lt "$orchline"
grep -Fq "This rule takes precedence over generic mission" "$skill"
grep -Fq "This is neither code" "$skill"
grep -Fq "Be independent and read-only" "$mr"
grep -Fq "Exact graph revision:" "$mr"
grep -Fq "Complete graph scope:" "$mr"
grep -Fq "initial author identity and every material editor" "$mr"
grep -Fq "edit makes that result stale" "$mr"
grep -Fq "authorize the planner to approve that same graph" "$plan"
grep -Fq "current repository-reported approval and ready" "$plan"
grep -Fq "current manager/status/mission surfaces" "$orch"
grep -Fq "current independent approval and is reported ready" "$orch"
grep -Fq "cannot expose" "$install"
grep -Fq "not rate a repository fully operable for mission planning" "$ready"
grep -Fq "authored draft state" "$shape"
grep -Fq "Expected route: mission-review." "$fixture"
grep -Fq "Forbidden route: orchestrate until Atelier reports current independent approval" "$fixture"
grep -Fq "Live lifecycle behavior remains deferred" "$fixture"
grep -Fq "Result: not fully operable for Agent Factory mission planning." "$fixture"
grep -Fq "These packets are input fixtures" "$dogfood"
grep -Fq "The assignee must not approve this packet because its actor is the author" "$dogfood"
bin=/root/atelier-worktrees/atelier-p2wk/target/debug/atelier
planner="$($bin man planner)"
reviewer="$($bin man reviewer)"
manager="$($bin man manager)"
mission="$($bin work mission atelier-p4z2 --ready)"
grep -Fq "approval of a graph you authored or materially edited" <<<"$planner"
grep -Fq "plan-review <mission-id> request" <<<"$planner"
grep -Fq "plan-review <mission-id> approve" <<<"$reviewer"
grep -Fq "authoring or materially editing the graph being approved" <<<"$reviewer"
grep -Fq "Inspect dependency-safe work" <<<"$manager"
grep -Fq "planners author and reviewers independently approve" <<<"$manager"
grep -Fq "ready 0" <<<"$mission"
grep -Fq "List mission blockers" <<<"$mission"
for id in atelier-av92 atelier-havm atelier-a5is atelier-62ap atelier-pt92 atelier-vooj atelier-iw6l atelier-k19f atelier-0162 atelier-83th atelier-aoum; do test -f ".atelier/evidence/$id.md"; done
printf "%s\n" \
"PASS C1 distinct mission-review routing; exact-draft precedence at lines $mrline<$orchline" \
"PASS C2 read-only exact-revision complete-graph review with author/editor independence and freshness" \
"PASS C3 planner requests only; orchestrator trusts only Atelier current approval plus ready state" \
"PASS C4 unsupported lifecycle is explicitly a readiness gap; proof classification is fixture-only" \
"PASS C5 live planner/reviewer/manager surfaces enforce role separation; atelier-p4z2 ready=0 and blocker-routed" \
"PASS C6 prerequisite evidence present: static, focused tests, role transcripts, readiness, full suite, diff check"
'
```

Exit status: 0

## Stdout

Bytes: 602
Truncated: no

```text
PASS C1 distinct mission-review routing; exact-draft precedence at lines 72<77
PASS C2 read-only exact-revision complete-graph review with author/editor independence and freshness
PASS C3 planner requests only; orchestrator trusts only Atelier current approval plus ready state
PASS C4 unsupported lifecycle is explicitly a readiness gap; proof classification is fixture-only
PASS C5 live planner/reviewer/manager surfaces enforce role separation; atelier-p4z2 ready=0 and blocker-routed
PASS C6 prerequisite evidence present: static, focused tests, role transcripts, readiness, full suite, diff check
```

## Stderr

Bytes: 0
Truncated: no

```text
```
