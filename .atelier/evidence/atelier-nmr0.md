---
created_at: "2026-06-21T19:20:33.458986511+00:00"
id: "atelier-nmr0"
evidence_type: "validation"
captured_at: "2026-06-21T19:20:30.727191995+00:00"
command: "bash -lc 'set -euo pipefail\nBIN=/root/atelier-9n3r/target/debug/atelier\nTMP=$(mktemp -d /tmp/atelier-lkz6-proof.XXXXXX)\ncleanup() { rm -rf \"$TMP\"; }\ntrap cleanup EXIT\ncd \"$TMP\"\ngit init -q\ngit branch -M main\ngit config user.email atelier-validation@example.com\ngit config user.name \"Atelier Validation\"\n\"$BIN\" init >/tmp/lkz6-init.out\nif [ ! -f .atelier/workflow.yaml ]; then\n  cp /root/atelier-9n3r/.atelier/workflow.yaml .atelier/workflow.yaml\n  perl -0pi -e \"s/base_branch: master/base_branch: main/\" .atelier/workflow.yaml\nfi\ngit add -A && git commit -qm init\nNO_EVIDENCE_ID=$(\"$BIN\" issue create -q --issue-type task \"No evidence start workflow\")\n\"$BIN\" issue transition \"$NO_EVIDENCE_ID\" start >/tmp/lkz6-no-evidence-start.out\nrg -Fq \"Applied transition start\" /tmp/lkz6-no-evidence-start.out\nGATED_ID=$(\"$BIN\" issue create -q --issue-type task \"Evidence gated start workflow\")\nperl -0pi -e \"s/(      start:\\\\n        from: \\\\[todo, blocked\\\\]\\\\n        to: in_progress\\\\n)/\\$1        validators: [evidence.attached]\\\\n/\" .atelier/workflow.yaml\ngit add -A && git commit -qm \"gate start with evidence\"\n\"$BIN\" issue transition \"$GATED_ID\" --options >/tmp/lkz6-gated-options-before.out\nrg -Fq \"start [blocked]\" /tmp/lkz6-gated-options-before.out\nrg -Fq \"fail  evidence.attached\" /tmp/lkz6-gated-options-before.out\nif ! rg -Fq \"expected at least 1 validating evidence record(s); found 0\" /tmp/lkz6-gated-options-before.out && ! rg -Fq \"no validating evidence link found\" /tmp/lkz6-gated-options-before.out; then\n  echo \"missing evidence.attached reason\" >&2\n  exit 1\nfi\nrg -Fq \"Hint: record proof with\" /tmp/lkz6-gated-options-before.out\nrg -Fq \"atelier evidence record --target issue/<id> --kind validation\" /tmp/lkz6-gated-options-before.out\nif \"$BIN\" issue transition \"$GATED_ID\" start >/tmp/lkz6-gated-start.out 2>/tmp/lkz6-gated-start.err; then\n  echo \"gated start unexpectedly succeeded\" >&2\n  exit 1\nfi\nrg -Fq \"Hint: record proof with\" /tmp/lkz6-gated-start.out\nrg -Fq \"atelier evidence record --target issue/<id> --kind validation\" /tmp/lkz6-gated-start.out\nrg -Fq \"evidence.attached\" /tmp/lkz6-gated-start.err\n\"$BIN\" evidence record --target \"issue/$GATED_ID\" --kind validation \"validation proof for gated start\" >/tmp/lkz6-evidence-record.out\nEVIDENCE_ID=$(rg -o \"atelier-[a-z0-9]+\" /tmp/lkz6-evidence-record.out | head -1)\ntest -n \"$EVIDENCE_ID\"\ngit add -A && git commit -qm \"attach validation evidence\"\n\"$BIN\" issue transition \"$GATED_ID\" --options >/tmp/lkz6-gated-options-after.out\nrg -Fq \"pass  evidence.attached\" /tmp/lkz6-gated-options-after.out\n\"$BIN\" evidence show \"$EVIDENCE_ID\" >/tmp/lkz6-evidence-show.out\n\"$BIN\" evidence list >/tmp/lkz6-evidence-list.out\nrg -Fq \"$GATED_ID\" /tmp/lkz6-evidence-show.out\nrg -Fq \"$EVIDENCE_ID\" /tmp/lkz6-evidence-list.out\n\"$BIN\" status >/tmp/lkz6-status.out\n\"$BIN\" issue status \"$GATED_ID\" >/tmp/lkz6-issue-status.out\n\"$BIN\" issue show \"$GATED_ID\" >/tmp/lkz6-issue-show.out\nfor forbidden in \"Evidence Status\" \"Attached Proof: missing\" \"evidence gaps\" \"proof_gaps\"; do\n  if rg -Fq \"$forbidden\" /tmp/lkz6-status.out /tmp/lkz6-issue-status.out /tmp/lkz6-issue-show.out; then\n    echo \"unexpected hardcoded evidence text: $forbidden\" >&2\n    exit 1\n  fi\ndone\ncd /root/atelier-9n3r\ntarget/debug/atelier lint\ngit diff --check\necho \"lkz6 clean validation passed: default start allowed without evidence; configured evidence.attached blocks with reason and hint; attached validation evidence passes; evidence show/list work; status surfaces avoid hardcoded proof-gap text; atelier lint and git diff --check pass.\"'"
exit_status: "0"
target:
  kind: "issue"
  id: "atelier-lkz6"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-9n3r"
    role: "validates"
  - kind: "issue"
    id: "atelier-lkz6"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 1
status: "recorded"
title: "Clean end-to-end validation confirms evidence is optional by default, evidence.attached blocks only configured transitions with a concise reason and help hint, attached validation evidence makes the validator pass, evidence show/list still work, and repository lint/diff checks pass."
updated_at: "2026-06-21T19:20:48.985925034+00:00"
---

## Summary

Clean end-to-end validation confirms evidence is optional by default, evidence.attached blocks only configured transitions with a concise reason and help hint, attached validation evidence makes the validator pass, evidence show/list still work, and repository lint/diff checks pass.

## Command

```console
bash -lc 'set -euo pipefail
BIN=/root/atelier-9n3r/target/debug/atelier
TMP=$(mktemp -d /tmp/atelier-lkz6-proof.XXXXXX)
cleanup() { rm -rf "$TMP"; }
trap cleanup EXIT
cd "$TMP"
git init -q
git branch -M main
git config user.email atelier-validation@example.com
git config user.name "Atelier Validation"
"$BIN" init >/tmp/lkz6-init.out
if [ ! -f .atelier/workflow.yaml ]; then
  cp /root/atelier-9n3r/.atelier/workflow.yaml .atelier/workflow.yaml
  perl -0pi -e "s/base_branch: master/base_branch: main/" .atelier/workflow.yaml
fi
git add -A && git commit -qm init
NO_EVIDENCE_ID=$("$BIN" issue create -q --issue-type task "No evidence start workflow")
"$BIN" issue transition "$NO_EVIDENCE_ID" start >/tmp/lkz6-no-evidence-start.out
rg -Fq "Applied transition start" /tmp/lkz6-no-evidence-start.out
GATED_ID=$("$BIN" issue create -q --issue-type task "Evidence gated start workflow")
perl -0pi -e "s/(      start:\\n        from: \\[todo, blocked\\]\\n        to: in_progress\\n)/\$1        validators: [evidence.attached]\\n/" .atelier/workflow.yaml
git add -A && git commit -qm "gate start with evidence"
"$BIN" issue transition "$GATED_ID" --options >/tmp/lkz6-gated-options-before.out
rg -Fq "start [blocked]" /tmp/lkz6-gated-options-before.out
rg -Fq "fail  evidence.attached" /tmp/lkz6-gated-options-before.out
if ! rg -Fq "expected at least 1 validating evidence record(s); found 0" /tmp/lkz6-gated-options-before.out && ! rg -Fq "no validating evidence link found" /tmp/lkz6-gated-options-before.out; then
  echo "missing evidence.attached reason" >&2
  exit 1
fi
rg -Fq "Hint: record proof with" /tmp/lkz6-gated-options-before.out
rg -Fq "atelier evidence record --target issue/<id> --kind validation" /tmp/lkz6-gated-options-before.out
if "$BIN" issue transition "$GATED_ID" start >/tmp/lkz6-gated-start.out 2>/tmp/lkz6-gated-start.err; then
  echo "gated start unexpectedly succeeded" >&2
  exit 1
fi
rg -Fq "Hint: record proof with" /tmp/lkz6-gated-start.out
rg -Fq "atelier evidence record --target issue/<id> --kind validation" /tmp/lkz6-gated-start.out
rg -Fq "evidence.attached" /tmp/lkz6-gated-start.err
"$BIN" evidence record --target "issue/$GATED_ID" --kind validation "validation proof for gated start" >/tmp/lkz6-evidence-record.out
EVIDENCE_ID=$(rg -o "atelier-[a-z0-9]+" /tmp/lkz6-evidence-record.out | head -1)
test -n "$EVIDENCE_ID"
git add -A && git commit -qm "attach validation evidence"
"$BIN" issue transition "$GATED_ID" --options >/tmp/lkz6-gated-options-after.out
rg -Fq "pass  evidence.attached" /tmp/lkz6-gated-options-after.out
"$BIN" evidence show "$EVIDENCE_ID" >/tmp/lkz6-evidence-show.out
"$BIN" evidence list >/tmp/lkz6-evidence-list.out
rg -Fq "$GATED_ID" /tmp/lkz6-evidence-show.out
rg -Fq "$EVIDENCE_ID" /tmp/lkz6-evidence-list.out
"$BIN" status >/tmp/lkz6-status.out
"$BIN" issue status "$GATED_ID" >/tmp/lkz6-issue-status.out
"$BIN" issue show "$GATED_ID" >/tmp/lkz6-issue-show.out
for forbidden in "Evidence Status" "Attached Proof: missing" "evidence gaps" "proof_gaps"; do
  if rg -Fq "$forbidden" /tmp/lkz6-status.out /tmp/lkz6-issue-status.out /tmp/lkz6-issue-show.out; then
    echo "unexpected hardcoded evidence text: $forbidden" >&2
    exit 1
  fi
done
cd /root/atelier-9n3r
target/debug/atelier lint
git diff --check
echo "lkz6 clean validation passed: default start allowed without evidence; configured evidence.attached blocks with reason and hint; attached validation evidence passes; evidence show/list work; status surfaces avoid hardcoded proof-gap text; atelier lint and git diff --check pass."'
```

Exit status: 0

## Stdout

Bytes: 291
Truncated: no

```text
Lint passed.
lkz6 clean validation passed: default start allowed without evidence; configured evidence.attached blocks with reason and hint; attached validation evidence passes; evidence show/list work; status surfaces avoid hardcoded proof-gap text; atelier lint and git diff --check pass.
```

## Stderr

Bytes: 0
Truncated: no

```text
```

