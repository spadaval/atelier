---
created_at: "2026-07-06T20:52:15.388637880+00:00"
id: "atelier-z6q1"
evidence_type: "review"
captured_at: "2026-07-06T20:52:15.388624196+00:00"
agent_identity: "independent-publish-reviewer"
target:
  kind: "issue"
  id: "atelier-durs"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-durs"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 1
status: "recorded"
title: "REQUEST CHANGES — independent final PR51 re-review at 52f06d97. Resolved and independently confirmed: exact c0mp contract ancestry/content and shared-doc split; CLI Zen whole-document Historical/Non-Normative classification; prior six product/spec/architecture/ADR groups; maintenance derivation; 88 recursively indexed docs including 10 quality docs; 11 visible/10 hidden/46 removed inventory; compatibility wrapper behavior; branch help; focused nextest 9/9; fmt, master-range diff, tracker health, and c0mp implementation deferred/not-applicable classification. One guard blocker remains. The 106-case self-test is false-complete: using the integrated script’s actual active_content and scan_content functions, historical and rejected-section re-entry is rejected correctly, but four live fixtures are incorrectly accepted: a raw Markdown bullet \"- atelier start\", punctuated \"atelier start, then continue\", mixed \"The legacy API is gone; run `atelier start` now\", and mixed \"The retired command differed; use `atelier issue close demo` for current work.\" Root cause is the narrow atelier_prefix/command_boundary plus line-wide prohibited_allowance_pattern in scripts/check_active_command_guidance.sh. Thus authoritative validation atelier-e3x2 does not establish no-permissive-gap coverage; its transcript only runs the incomplete generated self-test. Expand token boundaries to ordinary Markdown/prose, constrain allowances to explicit excluded sections or the matched command’s negative context, and add these adversarial fixtures. Initial projection freshness required explicit check --fix after new tracker records; canonical records were unchanged and subsequent tracker/evidence/provider reads pass. Superseded diagnostics and evidence atelier-neip, atelier-fc6x, atelier-68lu, atelier-3ezp, atelier-k19d, and atelier-e3x2 were inspected."
updated_at: "2026-07-06T20:52:21.541030665+00:00"
---

REQUEST CHANGES — independent final PR51 re-review at 52f06d97. Resolved and independently confirmed: exact c0mp contract ancestry/content and shared-doc split; CLI Zen whole-document Historical/Non-Normative classification; prior six product/spec/architecture/ADR groups; maintenance derivation; 88 recursively indexed docs including 10 quality docs; 11 visible/10 hidden/46 removed inventory; compatibility wrapper behavior; branch help; focused nextest 9/9; fmt, master-range diff, tracker health, and c0mp implementation deferred/not-applicable classification. One guard blocker remains. The 106-case self-test is false-complete: using the integrated script’s actual active_content and scan_content functions, historical and rejected-section re-entry is rejected correctly, but four live fixtures are incorrectly accepted: a raw Markdown bullet "- atelier start", punctuated "atelier start, then continue", mixed "The legacy API is gone; run `atelier start` now", and mixed "The retired command differed; use `atelier issue close demo` for current work." Root cause is the narrow atelier_prefix/command_boundary plus line-wide prohibited_allowance_pattern in scripts/check_active_command_guidance.sh. Thus authoritative validation atelier-e3x2 does not establish no-permissive-gap coverage; its transcript only runs the incomplete generated self-test. Expand token boundaries to ordinary Markdown/prose, constrain allowances to explicit excluded sections or the matched command’s negative context, and add these adversarial fixtures. Initial projection freshness required explicit check --fix after new tracker records; canonical records were unchanged and subsequent tracker/evidence/provider reads pass. Superseded diagnostics and evidence atelier-neip, atelier-fc6x, atelier-68lu, atelier-3ezp, atelier-k19d, and atelier-e3x2 were inspected.
