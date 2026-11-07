---
schema: atelier.review
schema_version: 2
id: atelier-irdr
title: 'Design item 1 is right in substance, but two details in it are imprecise; the conclusion still holds. (a) It says isolation ''would leave text and eol attributes intact''. With --attr-source=<empty tree>, check-attr reports no attributes at all, so text and eol are dropped too. What stays the same is the merge result for those files (it differs only under merge.renormalize). (b) It says isolation would ''turn every driver-resolved file into a conflict''. That holds for --attr-source. Dropping only the driver config does not touch merge=union, which is built in. For a custom merge=ours file it falls back to a text merge, which can be clean but different: on git 2.53 the merge-tree result was 225e8b0f against the driver''s 9b3f6a2c with no conflict. Either way the base merge stops reproducing and needs renewed review, so the stated consequence is true. Also, ''the real merge used it too'' assumes whoever merged had the same local merge.<name>.driver config, and that config is not versioned. Reword these details the next time the record is touched.'
target: atelier-7hh4
identity: Independent reviewer (Claude subagent)
commit: 76d7af940461b7572283b461b9ec56f28cafd73a
created_at: 2026-10-16T20:47:24.475273148Z
event:
  event: finding
  recommendation: defer
  disposition:
    decision: no_action
    identity: Claude Opus 5.5 (orchestrator)
    commit: a79ca2fb0f6d627a1472bb0652ac1647ff08ba0a
    recorded_at: 2026-10-16T20:48:02.786474595Z
    reason: 'Accepted: the imprecisions do not change the conclusion that driver merges are documented, not isolated; the reviewer''s details stand on this finding for anyone revisiting it.'
    migrated_from: atelier-cwzq
---

Design item 1 is right in substance, but two details in it are imprecise; the conclusion still holds. (a) It says isolation 'would leave text and eol attributes intact'. With --attr-source=<empty tree>, check-attr reports no attributes at all, so text and eol are dropped too. What stays the same is the merge result for those files (it differs only under merge.renormalize). (b) It says isolation would 'turn every driver-resolved file into a conflict'. That holds for --attr-source. Dropping only the driver config does not touch merge=union, which is built in. For a custom merge=ours file it falls back to a text merge, which can be clean but different: on git 2.53 the merge-tree result was 225e8b0f against the driver's 9b3f6a2c with no conflict. Either way the base merge stops reproducing and needs renewed review, so the stated consequence is true. Also, 'the real merge used it too' assumes whoever merged had the same local merge.<name>.driver config, and that config is not versioned. Reword these details the next time the record is touched.
