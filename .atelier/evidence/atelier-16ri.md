---
created_at: "2026-11-10T15:25:06.140366703+00:00"
id: "atelier-16ri"
evidence_type: "validation"
commit: "90ef631949114062def96d142f563b18a680812e"
result: "pass"
captured_at: "2026-11-10T15:25:06.140318694+00:00"
target:
  kind: "issue"
  id: "atelier-mal9"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-mal9"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "The new CI job's run steps, read from ci.yml and run each with bash -e in next/ of a scratch clone (/home/supadava/.cache/atelier-fcb7/run_next_job.py): on 2decbc46 rm -rf ../crates, cargo fmt -- --check, cargo build --locked --all-targets and cargo nextest run --locked all passed (37 tests). On throwaway commit 1bc9f821, which adds atelier-workflow = { path = \"../../../crates/atelier-workflow\" } to atelier-records and uses it (it builds with crates/ present), the job failed at its first cargo step: failed to read .../crates/atelier-workflow/Cargo.toml: No such file or directory; cargo build --locked alone failed the same way (exit 101)."
updated_at: "2026-11-10T15:25:06.281312281+00:00"
---

The new CI job's run steps, read from ci.yml and run each with bash -e in next/ of a scratch clone (/home/supadava/.cache/atelier-fcb7/run_next_job.py): on 2decbc46 rm -rf ../crates, cargo fmt -- --check, cargo build --locked --all-targets and cargo nextest run --locked all passed (37 tests). On throwaway commit 1bc9f821, which adds atelier-workflow = { path = "../../../crates/atelier-workflow" } to atelier-records and uses it (it builds with crates/ present), the job failed at its first cargo step: failed to read .../crates/atelier-workflow/Cargo.toml: No such file or directory; cargo build --locked alone failed the same way (exit 101).
