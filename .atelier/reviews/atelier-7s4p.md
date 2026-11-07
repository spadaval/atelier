---
schema: atelier.review
schema_version: 2
id: atelier-7s4p
title: finding on atelier-uqxh @abb26dc4
target: atelier-uqxh
identity: independent reviewer subagent (claude-opus-5-5), atelier-uqxh
commit: abb26dc45f07a7735d6aa9a76b00b8bf4d7816b8
created_at: 2026-11-07T16:25:15.150747984Z
event:
  event: finding
  recommendation: fix_now
---

medium: spadaval/.agents 658dfe52dd027be49b5f69a8d06e8ecdd93f2aef skills/agent-factory/references/atelier.md:154-160, with procedures/implement.md:35-38. The loose end now goes straight to the base 'even while the issue it waits on is in flight', and :159-160 says the in-flight issue's record is not touched. But the usual mid-flight case, an implementer reporting a Done when line that can only be shown after merge, also rewrites or drops that line in the in-flight item's own record (implement.md: 'that is a plan change ... it becomes a loose end'), and the skill no longer says where that edit goes. Before this change both went on the work branch. Now 'Filing it is planning like any other: commit it straight to the base' and Planning on the Base ('filing, grooming ... may go on the base directly') invite putting the rewrite on the base in the same planning commit; only the merge section's parenthetical at :104-105 ('leave those alone') warns against it. On a fixture with this branch's binary, filing a loose end on the base while an item was in flight left merge --check passing, but rewriting that item's Done when on the base made merge --check refuse with merge_conflict on .atelier/issues/<id>.md, asking for a reviewed resolution (with no textual overlap it would be merge_base_records instead). The final cold read listed this as open. Suggested fix, one sentence in Loose Ends: leave the in-flight issue's own record alone on the base; rewrite or drop the Done when line the loose end replaces on its work branch, where the review brief shows the edit. astromech's AGENTS.md defers this detail to the skill, so it needs no change for the fix.
