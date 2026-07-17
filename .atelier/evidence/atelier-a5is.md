---
created_at: "2026-07-16T23:51:06.998331680+00:00"
id: "atelier-a5is"
evidence_type: "validation"
captured_at: "2026-07-16T23:50:53.921876384+00:00"
command: "bash -lc 'set -euo pipefail\nbin=/root/atelier-worktrees/atelier-p2wk/target/debug/atelier\nplanner=\"$($bin man planner)\"\nreviewer=\"$($bin man reviewer)\"\nmanager=\"$($bin man manager)\"\nmission=\"$($bin work mission atelier-p4z2 --ready)\"\ngrep -Fq \"approval of a graph you authored or materially edited\" <<<\"$planner\"\ngrep -Fq \"atelier issue plan-review <mission-id> request\" <<<\"$planner\"\ngrep -Fq \"atelier issue plan-review <mission-id> approve\" <<<\"$reviewer\"\ngrep -Fq \"authoring or materially editing the graph being approved\" <<<\"$reviewer\"\ngrep -Fq \"Inspect dependency-safe work\" <<<\"$manager\"\ngrep -Fq \"mission-plan approval; planners author and reviewers independently approve\" <<<\"$manager\"\ngrep -Fq \"ready 0\" <<<\"$mission\"\ngrep -Fq \"List mission blockers\" <<<\"$mission\"\nfixture=.agents/skills/agent-factory/fixtures/mission-review-handoffs.md\ngrep -Fq \"Repository: /repo/atelier\" \"$fixture\"\ngrep -Fq \"Exact graph revision: mission-plan-graph-v1:sha256:\" \"$fixture\"\ngrep -Fq \"Complete graph scope: mission; roots\" \"$fixture\"\ngrep -Fq \"Evidence destination: issue/atelier-fixture-complete\" \"$fixture\"\ngrep -Fq \"reviewer fixture-independent-reviewer\" \"$fixture\"\ngrep -Fq \"Result: not fully operable for Agent Factory mission planning\" \"$fixture\"\nfor id in atelier-k19f atelier-0162 atelier-pt92 atelier-vooj atelier-iw6l; do test -f \".atelier/evidence/$id.md\"; done\nprintf \"%s\\n\" \"planner=request/no-approval reviewer=independent-exact-review manager=repository-readiness current-p4z2=ready-0-blocker-routed reused=atelier-k19f,atelier-0162,atelier-pt92,atelier-vooj,atelier-iw6l\"'"
exit_status: "0"
target:
  kind: "issue"
  id: "atelier-qi40"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-qi40"
    role: "validates"
  - kind: "issue"
    id: "atelier-t876"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 1
status: "recorded"
title: "bash -lc 'set -euo pipefail\nbin=/root/atelier-worktrees/atelier-p2wk/target/debug/atelier\nplanner=\"$($bin man planner)\"\nreviewer=\"$($bin man reviewer)\"\nmanager=\"$($bin man manager)\"\nmission=\"$($bin work mission atelier-p4z2 --ready)\"\ngrep -Fq \"approval of a graph you authored or materially edited\" <<<\"$planner\"\ngrep -Fq \"atelier issue plan-review <mission-id> request\" <<<\"$planner\"\ngrep -Fq \"atelier issue plan-review <mission-id> approve\" <<<\"$reviewer\"\ngrep -Fq \"authoring or materially editing the graph being approved\" <<<\"$reviewer\"\ngrep -Fq \"Inspect dependency-safe work\" <<<\"$manager\"\ngrep -Fq \"mission-plan approval; planners author and reviewers independently approve\" <<<\"$manager\"\ngrep -Fq \"ready 0\" <<<\"$mission\"\ngrep -Fq \"List mission blockers\" <<<\"$mission\"\nfixture=.agents/skills/agent-factory/fixtures/mission-review-handoffs.md\ngrep -Fq \"Repository: /repo/atelier\" \"$fixture\"\ngrep -Fq \"Exact graph revision: mission-plan-graph-v1:sha256:\" \"$fixture\"\ngrep -Fq \"Complete graph scope: mission; roots\" \"$fixture\"\ngrep -Fq \"Evidence destination: issue/atelier-fixture-complete\" \"$fixture\"\ngrep -Fq \"reviewer fixture-independent-reviewer\" \"$fixture\"\ngrep -Fq \"Result: not fully operable for Agent Factory mission planning\" \"$fixture\"\nfor id in atelier-k19f atelier-0162 atelier-pt92 atelier-vooj atelier-iw6l; do test -f \".atelier/evidence/$id.md\"; done\nprintf \"%s\\n\" \"planner=request/no-approval reviewer=independent-exact-review manager=repository-readiness current-p4z2=ready-0-blocker-routed reused=atelier-k19f,atelier-0162,atelier-pt92,atelier-vooj,atelier-iw6l\"'"
updated_at: "2026-07-17T00:14:28.812236459+00:00"
---

## Summary

bash -lc 'set -euo pipefail
bin=/root/atelier-worktrees/atelier-p2wk/target/debug/atelier
planner="$($bin man planner)"
reviewer="$($bin man reviewer)"
manager="$($bin man manager)"
mission="$($bin work mission atelier-p4z2 --ready)"
grep -Fq "approval of a graph you authored or materially edited" <<<"$planner"
grep -Fq "atelier issue plan-review <mission-id> request" <<<"$planner"
grep -Fq "atelier issue plan-review <mission-id> approve" <<<"$reviewer"
grep -Fq "authoring or materially editing the graph being approved" <<<"$reviewer"
grep -Fq "Inspect dependency-safe work" <<<"$manager"
grep -Fq "mission-plan approval; planners author and reviewers independently approve" <<<"$manager"
grep -Fq "ready 0" <<<"$mission"
grep -Fq "List mission blockers" <<<"$mission"
fixture=.agents/skills/agent-factory/fixtures/mission-review-handoffs.md
grep -Fq "Repository: /repo/atelier" "$fixture"
grep -Fq "Exact graph revision: mission-plan-graph-v1:sha256:" "$fixture"
grep -Fq "Complete graph scope: mission; roots" "$fixture"
grep -Fq "Evidence destination: issue/atelier-fixture-complete" "$fixture"
grep -Fq "reviewer fixture-independent-reviewer" "$fixture"
grep -Fq "Result: not fully operable for Agent Factory mission planning" "$fixture"
for id in atelier-k19f atelier-0162 atelier-pt92 atelier-vooj atelier-iw6l; do test -f ".atelier/evidence/$id.md"; done
printf "%s\n" "planner=request/no-approval reviewer=independent-exact-review manager=repository-readiness current-p4z2=ready-0-blocker-routed reused=atelier-k19f,atelier-0162,atelier-pt92,atelier-vooj,atelier-iw6l"'

## Command

```console
bash -lc 'set -euo pipefail
bin=/root/atelier-worktrees/atelier-p2wk/target/debug/atelier
planner="$($bin man planner)"
reviewer="$($bin man reviewer)"
manager="$($bin man manager)"
mission="$($bin work mission atelier-p4z2 --ready)"
grep -Fq "approval of a graph you authored or materially edited" <<<"$planner"
grep -Fq "atelier issue plan-review <mission-id> request" <<<"$planner"
grep -Fq "atelier issue plan-review <mission-id> approve" <<<"$reviewer"
grep -Fq "authoring or materially editing the graph being approved" <<<"$reviewer"
grep -Fq "Inspect dependency-safe work" <<<"$manager"
grep -Fq "mission-plan approval; planners author and reviewers independently approve" <<<"$manager"
grep -Fq "ready 0" <<<"$mission"
grep -Fq "List mission blockers" <<<"$mission"
fixture=.agents/skills/agent-factory/fixtures/mission-review-handoffs.md
grep -Fq "Repository: /repo/atelier" "$fixture"
grep -Fq "Exact graph revision: mission-plan-graph-v1:sha256:" "$fixture"
grep -Fq "Complete graph scope: mission; roots" "$fixture"
grep -Fq "Evidence destination: issue/atelier-fixture-complete" "$fixture"
grep -Fq "reviewer fixture-independent-reviewer" "$fixture"
grep -Fq "Result: not fully operable for Agent Factory mission planning" "$fixture"
for id in atelier-k19f atelier-0162 atelier-pt92 atelier-vooj atelier-iw6l; do test -f ".atelier/evidence/$id.md"; done
printf "%s\n" "planner=request/no-approval reviewer=independent-exact-review manager=repository-readiness current-p4z2=ready-0-blocker-routed reused=atelier-k19f,atelier-0162,atelier-pt92,atelier-vooj,atelier-iw6l"'
```

Exit status: 0

## Stdout

Bytes: 199
Truncated: no

```text
planner=request/no-approval reviewer=independent-exact-review manager=repository-readiness current-p4z2=ready-0-blocker-routed reused=atelier-k19f,atelier-0162,atelier-pt92,atelier-vooj,atelier-iw6l
```

## Stderr

Bytes: 0
Truncated: no

```text
```
