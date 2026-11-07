---
schema: atelier.review
schema_version: 2
id: atelier-09wh
title: 'The Design''s reason for item 1 is wrong. It says isolating merge-tree from the repository''s drivers and attributes ''would also drop the eol and text attributes the real merge used, so ordinary merges would stop matching''. The close reason repeats this as ''would break ordinary attribute-driven merges''. I tested this on git 2.53 in a scratch repo with ''* text=auto'' and ''*.txt eol=crlf'', where both sides edited a CRLF file and an LF file. The real merge''s tree, plain ''git merge-tree --write-tree'', and ''git --attr-source=<empty tree> merge-tree'' all gave the same tree (fe1dc8b8), and so did the same runs with merge.renormalize=true. In merging, text and eol only matter through renormalize. Isolation changes only merges driven by merge= attributes: union, binary, or a custom driver. With a configured ''CHANGELOG merge=ours'' driver and a ''union.log merge=union'' file, isolation turned both into conflicts. That fails closed (the merge is no longer clean, so it needs renewed review); it does not wrongly pass a merge. Another option keeps all attributes and only drops custom drivers: with no merge.<name>.driver config, Git falls back to its text merge (7fb8bd7f vs 3f679b92 in the same test). Documenting is allowed by done-when, and the reviews-and-merge.md text is accurate. But the decision record states a false premise. Fix: correct the item 1 rationale and the close reason. Isolation only affects merge=union, binary or custom-driver files and renormalized repos, and it fails closed. Then either keep the documented caveat for a stated reason, or isolate from custom drivers.'
target: atelier-7hh4
identity: Independent reviewer (Claude subagent)
commit: 473e265ee751093ca481e6c14b34f442821f381b
created_at: 2026-10-16T20:36:46.184163647Z
event:
  event: finding
  recommendation: fix_now
  disposition:
    decision: fix_now
    identity: Independent reviewer (Claude subagent)
    commit: 76d7af940461b7572283b461b9ec56f28cafd73a
    recorded_at: 2026-10-16T20:47:24.446655765Z
    resolution_commit: 76d7af940461b7572283b461b9ec56f28cafd73a
    reason: 'Commit 76d7af94 removes the false premise that isolation would drop the eol and text behaviour ordinary merges rely on, from both Design item 1 and the Close Reason. The new reason holds up. A configured driver is the repository''s chosen merge. Isolating fails closed: on git 2.53, with --attr-source set to the empty tree, the merge=ours and merge=union files conflict, and dropping the driver config gives a different text merge, so a driver-resolved base merge would no longer reproduce and would need renewed review. Keeping the documented caveat for that reason is a decision 09wh allowed.'
    migrated_from: atelier-qp34
---

The Design's reason for item 1 is wrong. It says isolating merge-tree from the repository's drivers and attributes 'would also drop the eol and text attributes the real merge used, so ordinary merges would stop matching'. The close reason repeats this as 'would break ordinary attribute-driven merges'. I tested this on git 2.53 in a scratch repo with '* text=auto' and '*.txt eol=crlf', where both sides edited a CRLF file and an LF file. The real merge's tree, plain 'git merge-tree --write-tree', and 'git --attr-source=<empty tree> merge-tree' all gave the same tree (fe1dc8b8), and so did the same runs with merge.renormalize=true. In merging, text and eol only matter through renormalize. Isolation changes only merges driven by merge= attributes: union, binary, or a custom driver. With a configured 'CHANGELOG merge=ours' driver and a 'union.log merge=union' file, isolation turned both into conflicts. That fails closed (the merge is no longer clean, so it needs renewed review); it does not wrongly pass a merge. Another option keeps all attributes and only drops custom drivers: with no merge.<name>.driver config, Git falls back to its text merge (7fb8bd7f vs 3f679b92 in the same test). Documenting is allowed by done-when, and the reviews-and-merge.md text is accurate. But the decision record states a false premise. Fix: correct the item 1 rationale and the close reason. Isolation only affects merge=union, binary or custom-driver files and renormalized repos, and it fails closed. Then either keep the documented caveat for a stated reason, or isolate from custom drivers.
