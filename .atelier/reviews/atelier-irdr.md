---
schema: atelier.review
schema_version: 1
id: atelier-irdr
title: 'Design item 1 is right in substance, but two details in it are imprecise; the conclusion still holds. (a) It says isolation ''would leave text and eol attributes intact''. With --attr-source=<empty tree>, check-attr reports no attributes at all, so text and eol are dropped too. What stays the same is the merge result for those files (it differs only under merge.renormalize). (b) It says isolation would ''turn every driver-resolved file into a conflict''. That holds for --attr-source. Dropping only the driver config does not touch merge=union, which is built in. For a custom merge=ours file it falls back to a text merge, which can be clean but different: on git 2.53 the merge-tree result was 225e8b0f against the driver''s 9b3f6a2c with no conflict. Either way the base merge stops reproducing and needs renewed review, so the stated consequence is true. Also, ''the real merge used it too'' assumes whoever merged had the same local merge.<name>.driver config, and that config is not versioned. Reword these details the next time the record is touched.'
target: atelier-7hh4
identity: Independent reviewer (Claude subagent)
commit: 76d7af940461b7572283b461b9ec56f28cafd73a
created_at: 2026-10-16T20:47:24.475273148Z
event:
  event: finding
  recommendation: defer
---

Design item 1 is right in substance, but two details in it are imprecise; the conclusion still holds. (a) It says isolation 'would leave text and eol attributes intact'. With --attr-source=<empty tree>, check-attr reports no attributes at all, so text and eol are dropped too. What stays the same is the merge result for those files (it differs only under merge.renormalize). (b) It says isolation would 'turn every driver-resolved file into a conflict'. That holds for --attr-source. Dropping only the driver config does not touch merge=union, which is built in. For a custom merge=ours file it falls back to a text merge, which can be clean but different: on git 2.53 the merge-tree result was 225e8b0f against the driver's 9b3f6a2c with no conflict. Either way the base merge stops reproducing and needs renewed review, so the stated consequence is true. Also, 'the real merge used it too' assumes whoever merged had the same local merge.<name>.driver config, and that config is not versioned. Reword these details the next time the record is touched.
