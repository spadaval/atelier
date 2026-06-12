---
created_at: "2026-06-12T22:18:07.920851975+00:00"
id: "atelier-3ud4"
data: "{\"captured_at\":\"2026-06-12T22:18:00.438377236+00:00\",\"command\":\"bash -lc 'set -euo pipefail\\n! rg \\\"^data:\\\" .atelier/missions -g \\\"*.md\\\"\\nrg \\\"## Intent|## Constraints|## Risks|## Validation\\\" .atelier/missions/atelier-tcmr.md\\ngit diff -- .atelier/missions/atelier-tcmr.md | sed -n \\\"1,140p\\\"\\ncargo test record_store::tests::legacy_mission_data_record_loads_into_typed_sections_and_relationships -- --nocapture\\ncargo test record_store::tests::mission_render_normalizes_legacy_evidence_attachments -- --nocapture\\ntarget/debug/atelier rebuild\\ntarget/debug/atelier export --check\\ntarget/debug/atelier lint\\ntarget/debug/atelier doctor\\ntarget/debug/atelier lint atelier-ys5p'\",\"exit_code\":0,\"exit_status\":\"0\",\"kind\":\"validation\",\"output\":{\"limit_bytes_per_stream\":4096,\"stderr\":{\"bytes\":25862,\"summary\":\"   Compiling atelier-tracker v0.2.0 (/root/atelier)\\nwarning: function `list_all_issue_activities` is never used\\n   --> src/activity.rs:125:8\\n    |\\n125 | pub fn list_all_issue_activities(state_dir: &Path) -> Result<Vec<IssueActivity>> {\\n    |        ^^^^^^^^^^^^^^^^^^^^^^^^^\\n    |\\n    = note: `#[warn(dead_code)]` (part of `#[warn(unused)]`) on by default\\n\\nwarning: struct `CreateInput` is never constructed\\n    --> src/commands/agent_factory.rs:1252:12\\n     |\\n1252 | pub struct CreateInput<'a> {\\n     |            ^^^^^^^^^^^\\n\\nwarning: function `create` is never used\\n    --> src/commands/agent_factory.rs:1380:8\\n     |\\n1380 | pub fn create(db: &Database, input: CreateInput<'_>) -> Result<()> {\\n     |        ^^^^^^\\n\\nwarning: function `update` is never used\\n    --> src/commands/agent_factory.rs:1430:8\\n     |\\n1430 | pub fn update(db: &Database, input: UpdateInput<'_>) -> Result<()> {\\n     |        ^^^^^^\\n\\nwarning: function `close` is never used\\n    --> src/commands/agent_factory.rs:1757:8\\n     |\\n1757 | pub fn close(db: &Database, issue_ref: &str, reason: Option<&str>) -> Result<()> {\\n     |        ^^^^^\\n\\nwarning: function `ensure_canonical_issue_sections_valid` is never used\\n    --> src/commands/agent_factory.rs:1846:4\\n     |\\n1846 | fn ensure_canonical_issue_sections_valid(issue_id: &str) -> Result<()> {\\n     |    ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\\n\\nwarning: function `reopen` is never used\\n    --> src/commands/agent_factory.rs:1854:8\\n     |\\n1854 | pub fn reopen(db: &Database, issue_ref: &str) -> Result<()> {\\n     |        ^^^^^^\\n\\nwarning: function `dep_add` is never used\\n    --> src/commands/agent_factory.rs:1982:8\\n     |\\n1982 | pub fn dep_add(db: &Database, blocked_ref: &str, blocker_ref: &str) -> Result<()> {\\n     |        ^^^^^^^\\n\\nwarning: function `dep_remove` is never used\\n    --> src/commands/agent_factory.rs:2001:8\\n     |\\n2001 | pub fn dep_remove(db: &Database, blocked_ref: &str, blocker_ref: &str) -> Result<()> {\\n     |        ^^^^^^^^^^\\n\\nwarning: function `run` is never used\\n  --> src/commands/comment.rs:21:8\\n   |\\n21 | pub fn run(db: &Database, issue_id: &str, content: &str, kind: &str) -> Result<()> {\\n   |        ^^^\\n\\nwarning: struct `CreateOpts` is never constructed\\n  --> src/commands/create.rs:79:12\\n   |\\n79 | pub struct CreateOpts<'a> {\\n   |            ^^^^^^^^^^\\n\\nwarning: function `run` is never used\\n  --> src/commands/create.rs:86:8\\n   |\\n86 | pub fn run(\\n   |        ^^^\\n\\nwarning: function `run_subissue` is never used\\n   --> src/commands/create.rs:175:8\\n    |\\n175 | pub fn run_subissue(\\n    |        ^^^^^^^^^^^^\\n\\nwarning: function `run` is never used\\n --> src/commands/delete.rs:9:8\\n  |\\n9 | pub fn run(db: &Database, id: &str, force: bool) -> Result<()> {\\n  |        ^^^\\n\\nwarning: function `add` is never used\\n --> src/commands/label.rs:7:8\\n  |\\n7 | pub fn add(db: &Database, issue_id: &str, label: &str) -> Result<()> {\\n  |        ^^^\\n\\nwarning: function `remove` is never used\\n  --> src/commands/label.rs:50:8\\n   |\\n50 | pub fn remove(db: &Database, issue_id: &str, label: &str) -> Result<()> {\\n   |        ^^^^^^\\n\\nwarning: function `refresh_open_database_after_canonical_write` is never used\\n  --> src/commands/projection.rs:26:8\\n   |\\n26 | pub fn refresh_open_database_after_canonical_write(\\n   |        ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\\n\\nwarning: function `add_typed` is never used\\n --> src/commands/relate.rs:7:8\\n  |\\n7 | pub fn add_typed(\\n  |        ^^^^^^^^^\\n\\nwarning: function `remove_typed` is never used\\n  --> src/commands/relate.rs:66:8\\n   |\\n66 | pub fn remove_typed(\\n   |        ^^^^^^^^^^^^\\n\\nwarning: function `close_all` is never used\\n  --> src/commands/status.rs:92:8\\n   |\\n92 | pub fn close_all(\\n   |        ^^^^^^^^^\\n\\nwarning: method `remove_dependency` is never used\\n  --> src/db/dependencies.rs:53:12\\n   |\\n 7 | impl Database {\\n   | ------------- method in this implementation\\n...\\n53 |     pub fn remove_dependency(\\n   |            ^^^^^^^^^^^^^^^^^\\n\\nwarning: multiple methods are never used\\n   --> src/db/issues.rs:54:12\\n    |\\n 10 | impl Database {\\n    | ------------- methods in this implementation\\n...\\n 54 |     pub fn\",\"truncated\":true},\"stdout\":{\"bytes\":6906,\"summary\":\"## Intent\\n## Constraints\\n## Risks\\n## Validation\\ndiff --git a/.atelier/missions/atelier-tcmr.md b/.atelier/missions/atelier-tcmr.md\\nindex c5f5bd1..de4a601 100644\\n--- a/.atelier/missions/atelier-tcmr.md\\n+++ b/.atelier/missions/atelier-tcmr.md\\n@@ -1,39 +1,64 @@\\n ---\\n created_at: \\\"2026-06-12T04:58:38.294509848+00:00\\\"\\n id: \\\"atelier-tcmr\\\"\\n-data: \\\"{\\\\\\\"constraints\\\\\\\":[\\\\\\\"Create new repair issues instead of reopening misleading closed mission work unless the old issue was closed accidentally and has no replacement.\\\\\\\",\\\\\\\"Use sectioned issue Markdown with Description, Outcome, Evidence, and optional Notes for all new repair work.\\\\\\\",\\\\\\\"Every repair item must name observable behavior and evidence before it can close.\\\\\\\"],\\\\\\\"evidence\\\\\\\":[],\\\\\\\"milestones\\\\\\\":[],\\\\\\\"plans\\\\\\\":[],\\\\\\\"risks\\\\\\\":[\\\\\\\"Reopening old closed issues can obscure audit history and make it harder to see what failed in the previous mission.\\\\\\\",\\\\\\\"Large rework can sprawl unless grouped under one mission with explicit blockers and validation.\\\\\\\"],\\\\\\\"validation\\\\\\\":[\\\\\\\"Mission links all repair epics and tasks needed to make the CLI surface, issue section parser, validators, docs, Agent Factory skill, projection freshness, and closeout checks match product intent.\\\\\\\",\\\\\\\"Agent Factory guidance explains how to write good mission, epic, issue, validation, Outcome, Evidence, and Notes text without prescribing implementation scripts.\\\\\\\",\\\\\\\"Mission closeout requires a contract audit mapping every mission and epic Outcome line to linked work and attached evidence.\\\\\\\",\\\\\\\"Mission closeout requires an independent adversarial validation pass by a validation agent that did not implement the slices being validated.\\\\\\\",\\\\\\\"Mission closeout requires positive and negative command transcripts for each major repaired surface, including old-command absence or replacement behavior.\\\\\\\",\\\\\\\"Mission closeout requires focused tests, stale/ignored-test inventory, docs/help/Agent Factory guidance parity checks, export/lint/doctor checks, and attached evidence records for each major repair area.\\\\\\\"],\\\\\\\"work\\\\\\\":[]}\\\"\\n+labels:\\n+- \\\"mission\\\"\\n relationships:\\n   blocks: []\\n   children: []\\n-  attachments:\\n+  attachments: []\\n+  relates:\\n   - kind: \\\"issue\\\"\\n     id: \\\"atelier-40ou\\\"\\n-    role: \\\"advances\\\"\\n+    type: \\\"advances\\\"\\n   - kind: \\\"issue\\\"\\n     id: \\\"atelier-efpk\\\"\\n-    role: \\\"advances\\\"\\n+    type: \\\"advances\\\"\\n   - kind: \\\"issue\\\"\\n     id: \\\"atelier-gjaz\\\"\\n-    role: \\\"advances\\\"\\n+    type: \\\"advances\\\"\\n   - kind: \\\"issue\\\"\\n     id: \\\"atelier-nzy1\\\"\\n-    role: \\\"advances\\\"\\n+    type: \\\"advances\\\"\\n   - kind: \\\"issue\\\"\\n     id: \\\"atelier-v9id\\\"\\n-    role: \\\"validates\\\"\\n+    type: \\\"validates\\\"\\n   - kind: \\\"issue\\\"\\n     id: \\\"atelier-wpyb\\\"\\n-    role: \\\"advances\\\"\\n+    type: \\\"advances\\\"\\n   - kind: \\\"issue\\\"\\n     id: \\\"atelier-ymfl\\\"\\n-    role: \\\"advances\\\"\\n+    type: \\\"advances\\\"\\n   - kind: \\\"issue\\\"\\n     id: \\\"atelier-zue4\\\"\\n-    role: \\\"advances\\\"\\n-  relates: []\\n+    type: \\\"advances\\\"\\n schema: \\\"atelier.mission\\\"\\n schema_version: 1\\n status: \\\"ready\\\"\\n title: \\\"Repair CLI workflow rework and validation gaps\\\"\\n updated_at: \\\"2026-06-12T19:19:18.924846572+00:00\\\"\\n ---\\n+\\n+## Intent\\n+\\n+Repair CLI workflow rework and validation gaps\\n+\\n+## Constraints\\n+\\n+- Create new repair issues instead of reopening misleading closed mission work unless the old issue was closed accidentally and has no replacement.\\n+- Use sectioned issue Markdown with Description, Outcome, Evidence, and optional Notes for all new repair work.\\n+- Every repair item must name observable behavior and evidence before it can close.\\n+\\n+## Risks\\n+\\n+- Reopening old closed issues can obscure audit history and make it harder to see what failed in the previous mission.\\n+- Large rework can sprawl unless grouped under one mission with explicit blockers and validation.\\n+\\n+## Validation\\n+\\n+- Mission links all repair epics and tasks needed to make the CLI surface, issue section parser, validators, docs, Agent Factory skill, projection freshness, and closeout checks match product intent.\\n+- Agent Factory guidance explains how to write good mission, epic, issue, validation, Outcome, Evidence, and Notes text without prescribing implementation scripts.\\n+- Mission\",\"truncated\":true}},\"path\":null,\"producer\":null,\"result\":\"pass\",\"spawn_error\":null,\"success\":true,\"target\":{\"id\":\"atelier-ys5p\",\"kind\":\"issue\",\"role\":\"validates\"},\"uri\":null}"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-ys5p"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 1
status: "pass"
title: "Mission records migrated from escaped data JSON to typed sections"
updated_at: "2026-06-12T22:18:09.250521939+00:00"
---

Mission records migrated from escaped data JSON to typed sections

Command: bash -lc 'set -euo pipefail
! rg "^data:" .atelier/missions -g "*.md"
rg "## Intent|## Constraints|## Risks|## Validation" .atelier/missions/atelier-tcmr.md
git diff -- .atelier/missions/atelier-tcmr.md | sed -n "1,140p"
cargo test record_store::tests::legacy_mission_data_record_loads_into_typed_sections_and_relationships -- --nocapture
cargo test record_store::tests::mission_render_normalizes_legacy_evidence_attachments -- --nocapture
target/debug/atelier rebuild
target/debug/atelier export --check
target/debug/atelier lint
target/debug/atelier doctor
target/debug/atelier lint atelier-ys5p'
Exit status: 0

Stdout summary (truncated):
## Intent
## Constraints
## Risks
## Validation
diff --git a/.atelier/missions/atelier-tcmr.md b/.atelier/missions/atelier-tcmr.md
index c5f5bd1..de4a601 100644
--- a/.atelier/missions/atelier-tcmr.md
+++ b/.atelier/missions/atelier-tcmr.md
@@ -1,39 +1,64 @@
 ---
 created_at: "2026-06-12T04:58:38.294509848+00:00"
 id: "atelier-tcmr"
-data: "{\"constraints\":[\"Create new repair issues instead of reopening misleading closed mission work unless the old issue was closed accidentally and has no replacement.\",\"Use sectioned issue Markdown with Description, Outcome, Evidence, and optional Notes for all new repair work.\",\"Every repair item must name observable behavior and evidence before it can close.\"],\"evidence\":[],\"milestones\":[],\"plans\":[],\"risks\":[\"Reopening old closed issues can obscure audit history and make it harder to see what failed in the previous mission.\",\"Large rework can sprawl unless grouped under one mission with explicit blockers and validation.\"],\"validation\":[\"Mission links all repair epics and tasks needed to make the CLI surface, issue section parser, validators, docs, Agent Factory skill, projection freshness, and closeout checks match product intent.\",\"Agent Factory guidance explains how to write good mission, epic, issue, validation, Outcome, Evidence, and Notes text without prescribing implementation scripts.\",\"Mission closeout requires a contract audit mapping every mission and epic Outcome line to linked work and attached evidence.\",\"Mission closeout requires an independent adversarial validation pass by a validation agent that did not implement the slices being validated.\",\"Mission closeout requires positive and negative command transcripts for each major repaired surface, including old-command absence or replacement behavior.\",\"Mission closeout requires focused tests, stale/ignored-test inventory, docs/help/Agent Factory guidance parity checks, export/lint/doctor checks, and attached evidence records for each major repair area.\"],\"work\":[]}"
+labels:
+- "mission"
 relationships:
   blocks: []
   children: []
-  attachments:
+  attachments: []
+  relates:
   - kind: "issue"
     id: "atelier-40ou"
-    role: "advances"
+    type: "advances"
   - kind: "issue"
     id: "atelier-efpk"
-    role: "advances"
+    type: "advances"
   - kind: "issue"
     id: "atelier-gjaz"
-    role: "advances"
+    type: "advances"
   - kind: "issue"
     id: "atelier-nzy1"
-    role: "advances"
+    type: "advances"
   - kind: "issue"
     id: "atelier-v9id"
-    role: "validates"
+    type: "validates"
   - kind: "issue"
     id: "atelier-wpyb"
-    role: "advances"
+    type: "advances"
   - kind: "issue"
     id: "atelier-ymfl"
-    role: "advances"
+    type: "advances"
   - kind: "issue"
     id: "atelier-zue4"
-    role: "advances"
-  relates: []
+    type: "advances"
 schema: "atelier.mission"
 schema_version: 1
 status: "ready"
 title: "Repair CLI workflow rework and validation gaps"
 updated_at: "2026-06-12T19:19:18.924846572+00:00"
 ---
+
+## Intent
+
+Repair CLI workflow rework and validation gaps
+
+## Constraints
+
+- Create new repair issues instead of reopening misleading closed mission work unless the old issue was closed accidentally and has no replacement.
+- Use sectioned issue Markdown with Description, Outcome, Evidence, and optional Notes for all new repair work.
+- Every repair item must name observable behavior and evidence before it can close.
+
+## Risks
+
+- Reopening old closed issues can obscure audit history and make it harder to see what failed in the previous mission.
+- Large rework can sprawl unless grouped under one mission with explicit blockers and validation.
+
+## Validation
+
+- Mission links all repair epics and tasks needed to make the CLI surface, issue section parser, validators, docs, Agent Factory skill, projection freshness, and closeout checks match product intent.
+- Agent Factory guidance explains how to write good mission, epic, issue, validation, Outcome, Evidence, and Notes text without prescribing implementation scripts.
+- Mission

Stderr summary (truncated):
   Compiling atelier-tracker v0.2.0 (/root/atelier)
warning: function `list_all_issue_activities` is never used
   --> src/activity.rs:125:8
    |
125 | pub fn list_all_issue_activities(state_dir: &Path) -> Result<Vec<IssueActivity>> {
    |        ^^^^^^^^^^^^^^^^^^^^^^^^^
    |
    = note: `#[warn(dead_code)]` (part of `#[warn(unused)]`) on by default

warning: struct `CreateInput` is never constructed
    --> src/commands/agent_factory.rs:1252:12
     |
1252 | pub struct CreateInput<'a> {
     |            ^^^^^^^^^^^

warning: function `create` is never used
    --> src/commands/agent_factory.rs:1380:8
     |
1380 | pub fn create(db: &Database, input: CreateInput<'_>) -> Result<()> {
     |        ^^^^^^

warning: function `update` is never used
    --> src/commands/agent_factory.rs:1430:8
     |
1430 | pub fn update(db: &Database, input: UpdateInput<'_>) -> Result<()> {
     |        ^^^^^^

warning: function `close` is never used
    --> src/commands/agent_factory.rs:1757:8
     |
1757 | pub fn close(db: &Database, issue_ref: &str, reason: Option<&str>) -> Result<()> {
     |        ^^^^^

warning: function `ensure_canonical_issue_sections_valid` is never used
    --> src/commands/agent_factory.rs:1846:4
     |
1846 | fn ensure_canonical_issue_sections_valid(issue_id: &str) -> Result<()> {
     |    ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: function `reopen` is never used
    --> src/commands/agent_factory.rs:1854:8
     |
1854 | pub fn reopen(db: &Database, issue_ref: &str) -> Result<()> {
     |        ^^^^^^

warning: function `dep_add` is never used
    --> src/commands/agent_factory.rs:1982:8
     |
1982 | pub fn dep_add(db: &Database, blocked_ref: &str, blocker_ref: &str) -> Result<()> {
     |        ^^^^^^^

warning: function `dep_remove` is never used
    --> src/commands/agent_factory.rs:2001:8
     |
2001 | pub fn dep_remove(db: &Database, blocked_ref: &str, blocker_ref: &str) -> Result<()> {
     |        ^^^^^^^^^^

warning: function `run` is never used
  --> src/commands/comment.rs:21:8
   |
21 | pub fn run(db: &Database, issue_id: &str, content: &str, kind: &str) -> Result<()> {
   |        ^^^

warning: struct `CreateOpts` is never constructed
  --> src/commands/create.rs:79:12
   |
79 | pub struct CreateOpts<'a> {
   |            ^^^^^^^^^^

warning: function `run` is never used
  --> src/commands/create.rs:86:8
   |
86 | pub fn run(
   |        ^^^

warning: function `run_subissue` is never used
   --> src/commands/create.rs:175:8
    |
175 | pub fn run_subissue(
    |        ^^^^^^^^^^^^

warning: function `run` is never used
 --> src/commands/delete.rs:9:8
  |
9 | pub fn run(db: &Database, id: &str, force: bool) -> Result<()> {
  |        ^^^

warning: function `add` is never used
 --> src/commands/label.rs:7:8
  |
7 | pub fn add(db: &Database, issue_id: &str, label: &str) -> Result<()> {
  |        ^^^

warning: function `remove` is never used
  --> src/commands/label.rs:50:8
   |
50 | pub fn remove(db: &Database, issue_id: &str, label: &str) -> Result<()> {
   |        ^^^^^^

warning: function `refresh_open_database_after_canonical_write` is never used
  --> src/commands/projection.rs:26:8
   |
26 | pub fn refresh_open_database_after_canonical_write(
   |        ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: function `add_typed` is never used
 --> src/commands/relate.rs:7:8
  |
7 | pub fn add_typed(
  |        ^^^^^^^^^

warning: function `remove_typed` is never used
  --> src/commands/relate.rs:66:8
   |
66 | pub fn remove_typed(
   |        ^^^^^^^^^^^^

warning: function `close_all` is never used
  --> src/commands/status.rs:92:8
   |
92 | pub fn close_all(
   |        ^^^^^^^^^

warning: method `remove_dependency` is never used
  --> src/db/dependencies.rs:53:12
   |
 7 | impl Database {
   | ------------- method in this implementation
...
53 |     pub fn remove_dependency(
   |            ^^^^^^^^^^^^^^^^^

warning: multiple methods are never used
   --> src/db/issues.rs:54:12
    |
 10 | impl Database {
    | ------------- methods in this implementation
...
 54 |     pub fn

