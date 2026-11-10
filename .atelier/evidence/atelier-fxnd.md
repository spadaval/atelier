---
created_at: "2026-11-10T23:29:54.424166260+00:00"
id: "atelier-fxnd"
evidence_type: "validation"
commit: "8b877b2e54e8cf53a94770eb42b92d190eb476bb"
result: "pass"
captured_at: "2026-11-10T23:29:54.424123650+00:00"
target:
  kind: "issue"
  id: "atelier-1sbw"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-1sbw"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "After round 3's fixes, on scratch clones of atelier, astromech, demiurge, life-on-rails and cosplay-studio converted fresh from each one's old commit: the conversion exits 0 (0.07 to 0.29 s), --verify exits 0, a second run from the same old commit gives the same bytes, and the new binary built after merging master at 3d58a88b runs check (Every record is valid on each), status and list with exit 0 (0.03 to 0.40 s). atelier set with an unchanged value on each of the 539 records leaves git diff empty. The fixed script's conversions of the five equal the earlier ones byte for byte. 40 deliberate breakages of the script each fail the test for their rule."
updated_at: "2026-11-10T23:29:54.574718655+00:00"
---

After round 3's fixes, on scratch clones of atelier, astromech, demiurge, life-on-rails and cosplay-studio converted fresh from each one's old commit: the conversion exits 0 (0.07 to 0.29 s), --verify exits 0, a second run from the same old commit gives the same bytes, and the new binary built after merging master at 3d58a88b runs check (Every record is valid on each), status and list with exit 0 (0.03 to 0.40 s). atelier set with an unchanged value on each of the 539 records leaves git diff empty. The fixed script's conversions of the five equal the earlier ones byte for byte. 40 deliberate breakages of the script each fail the test for their rule.
