---
schema: atelier.review
schema_version: 1
id: atelier-2tvh
title: 'crates/atelier-cli/src/commands/work.rs:782 (note_line) and 836 (malformed): `work decisions` introduces two new row shapes (''<id> <date> <choice> | because ... | rejected ...'' and ''⚠ <id> <date> <defect>'') that docs/product/human-cli-output.md does not describe (only the quiet token was added), and they are built with format! rather than the shared wrapping helper, so a long row wraps at the terminal edge with no continuation indent, against the contract that a too-wide row continues on indented lines. Malformed rows also omit the note number check uses, so two defective notes on the same day in one item print identical lines (reproduced). Fix: document the decision-note row in human-cli-output.md, render rows through the shared wrap helper, and include the note number (''Decision note 2'') in malformed rows to match check.'
target: atelier-fip8
identity: Independent reviewer (Claude subagent)
commit: cc8bd55a9665bf7504da8bdb128b240bab442e8a
created_at: 2026-10-17T17:43:39.719742716Z
event:
  event: finding
  recommendation: fix_now
---

crates/atelier-cli/src/commands/work.rs:782 (note_line) and 836 (malformed): `work decisions` introduces two new row shapes ('<id> <date> <choice> | because ... | rejected ...' and '⚠ <id> <date> <defect>') that docs/product/human-cli-output.md does not describe (only the quiet token was added), and they are built with format! rather than the shared wrapping helper, so a long row wraps at the terminal edge with no continuation indent, against the contract that a too-wide row continues on indented lines. Malformed rows also omit the note number check uses, so two defective notes on the same day in one item print identical lines (reproduced). Fix: document the decision-note row in human-cli-output.md, render rows through the shared wrap helper, and include the note number ('Decision note 2') in malformed rows to match check.
