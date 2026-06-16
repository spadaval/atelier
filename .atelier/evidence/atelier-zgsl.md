---
created_at: "2026-06-15T04:33:10.705813660+00:00"
id: "atelier-zgsl"
evidence_type: "validation"
captured_at: "2026-06-15T04:33:09.356925123+00:00"
command: "bash -lc '\nset -euo pipefail\necho \"# before/after cleanup verification for atelier-lfwg\"\necho \"# du -sh /root/atelier/.atelier-worktrees\"\ndu -sh /root/atelier/.atelier-worktrees 2>/dev/null || true\necho \"# git worktree list --porcelain\"\ngit worktree list --porcelain\necho \"# current removable-worktree classification\"\nprintf \"| branch | path | dirty | unique_vs_master | contained_in_master | classification |\\n\"\nprintf \"| --- | --- | ---: | ---: | --- | --- |\\n\"\nremovable=0\nfor branch in $(git for-each-ref --format=\"%(refname:short)\" \"refs/heads/codex/atelier-*\" | sort); do\n  issue=${branch#codex/}\n  wt=\"/root/atelier/.atelier-worktrees/${issue}\"\n  if [ ! -d \"$wt\" ] && [ ! -f \"$wt/.git\" ]; then\n    continue\n  fi\n  dirty=$(git -C \"$wt\" status --porcelain | wc -l | tr -d \" \")\n  unique=$(git rev-list --count \"master..$branch\")\n  if git merge-base --is-ancestor \"$branch\" master; then contained=yes; else contained=no; fi\n  if [ \"$dirty\" != 0 ]; then class=preserve; elif [ \"$contained\" = yes ]; then class=delete; removable=$((removable+1)); else class=\"fold into epic branch\"; fi\n  printf \"| %s | %s | %s | %s | %s | %s |\\n\" \"$branch\" \"$wt\" \"$dirty\" \"$unique\" \"$contained\" \"$class\"\ndone\necho \"removable_clean_contained_count=$removable\"\necho \"# stale/prunable Git worktree registrations\"\ngit worktree list --porcelain | awk \"/^worktree / {path=\\$2; prunable=0; reason=\\\"\\\"} /^prunable/ {prunable=1; reason=\\$0} /^$/ {if (prunable) print path \\\" | \\\" reason; prunable=0; reason=\\\"\\\"} END {if (prunable) print path \\\" | \\\" reason}\"\ntest \"$removable\" = 0\n'"
exit_status: "0"
target:
  kind: "issue"
  id: "atelier-lfwg"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-lfwg"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 1
status: "recorded"
title: "bash -lc '\nset -euo pipefail\necho \"# before/after cleanup verification for atelier-lfwg\"\necho \"# du -sh /root/atelier/.atelier-worktrees\"\ndu -sh /root/atelier/.atelier-worktrees 2>/dev/null || true\necho \"# git worktree list --porcelain\"\ngit worktree list --porcelain\necho \"# current removable-worktree classification\"\nprintf \"| branch | path | dirty | unique_vs_master | contained_in_master | classification |\\n\"\nprintf \"| --- | --- | ---: | ---: | --- | --- |\\n\"\nremovable=0\nfor branch in $(git for-each-ref --format=\"%(refname:short)\" \"refs/heads/codex/atelier-*\" | sort); do\n  issue=${branch#codex/}\n  wt=\"/root/atelier/.atelier-worktrees/${issue}\"\n  if [ ! -d \"$wt\" ] && [ ! -f \"$wt/.git\" ]; then\n    continue\n  fi\n  dirty=$(git -C \"$wt\" status --porcelain | wc -l | tr -d \" \")\n  unique=$(git rev-list --count \"master..$branch\")\n  if git merge-base --is-ancestor \"$branch\" master; then contained=yes; else contained=no; fi\n  if [ \"$dirty\" != 0 ]; then class=preserve; elif [ \"$contained\" = yes ]; then class=delete; removable=$((removable+1)); else class=\"fold into epic branch\"; fi\n  printf \"| %s | %s | %s | %s | %s | %s |\\n\" \"$branch\" \"$wt\" \"$dirty\" \"$unique\" \"$contained\" \"$class\"\ndone\necho \"removable_clean_contained_count=$removable\"\necho \"# stale/prunable Git worktree registrations\"\ngit worktree list --porcelain | awk \"/^worktree / {path=\\$2; prunable=0; reason=\\\"\\\"} /^prunable/ {prunable=1; reason=\\$0} /^$/ {if (prunable) print path \\\" | \\\" reason; prunable=0; reason=\\\"\\\"} END {if (prunable) print path \\\" | \\\" reason}\"\ntest \"$removable\" = 0\n'"
updated_at: "2026-06-15T04:33:12.567753738+00:00"
---

## Summary

bash -lc '
set -euo pipefail
echo "# before/after cleanup verification for atelier-lfwg"
echo "# du -sh /root/atelier/.atelier-worktrees"
du -sh /root/atelier/.atelier-worktrees 2>/dev/null || true
echo "# git worktree list --porcelain"
git worktree list --porcelain
echo "# current removable-worktree classification"
printf "| branch | path | dirty | unique_vs_master | contained_in_master | classification |\n"
printf "| --- | --- | ---: | ---: | --- | --- |\n"
removable=0
for branch in $(git for-each-ref --format="%(refname:short)" "refs/heads/codex/atelier-*" | sort); do
  issue=${branch#codex/}
  wt="/root/atelier/.atelier-worktrees/${issue}"
  if [ ! -d "$wt" ] && [ ! -f "$wt/.git" ]; then
    continue
  fi
  dirty=$(git -C "$wt" status --porcelain | wc -l | tr -d " ")
  unique=$(git rev-list --count "master..$branch")
  if git merge-base --is-ancestor "$branch" master; then contained=yes; else contained=no; fi
  if [ "$dirty" != 0 ]; then class=preserve; elif [ "$contained" = yes ]; then class=delete; removable=$((removable+1)); else class="fold into epic branch"; fi
  printf "| %s | %s | %s | %s | %s | %s |\n" "$branch" "$wt" "$dirty" "$unique" "$contained" "$class"
done
echo "removable_clean_contained_count=$removable"
echo "# stale/prunable Git worktree registrations"
git worktree list --porcelain | awk "/^worktree / {path=\$2; prunable=0; reason=\"\"} /^prunable/ {prunable=1; reason=\$0} /^$/ {if (prunable) print path \" | \" reason; prunable=0; reason=\"\"} END {if (prunable) print path \" | \" reason}"
test "$removable" = 0
'

## Command

```console
bash -lc '
set -euo pipefail
echo "# before/after cleanup verification for atelier-lfwg"
echo "# du -sh /root/atelier/.atelier-worktrees"
du -sh /root/atelier/.atelier-worktrees 2>/dev/null || true
echo "# git worktree list --porcelain"
git worktree list --porcelain
echo "# current removable-worktree classification"
printf "| branch | path | dirty | unique_vs_master | contained_in_master | classification |\n"
printf "| --- | --- | ---: | ---: | --- | --- |\n"
removable=0
for branch in $(git for-each-ref --format="%(refname:short)" "refs/heads/codex/atelier-*" | sort); do
  issue=${branch#codex/}
  wt="/root/atelier/.atelier-worktrees/${issue}"
  if [ ! -d "$wt" ] && [ ! -f "$wt/.git" ]; then
    continue
  fi
  dirty=$(git -C "$wt" status --porcelain | wc -l | tr -d " ")
  unique=$(git rev-list --count "master..$branch")
  if git merge-base --is-ancestor "$branch" master; then contained=yes; else contained=no; fi
  if [ "$dirty" != 0 ]; then class=preserve; elif [ "$contained" = yes ]; then class=delete; removable=$((removable+1)); else class="fold into epic branch"; fi
  printf "| %s | %s | %s | %s | %s | %s |\n" "$branch" "$wt" "$dirty" "$unique" "$contained" "$class"
done
echo "removable_clean_contained_count=$removable"
echo "# stale/prunable Git worktree registrations"
git worktree list --porcelain | awk "/^worktree / {path=\$2; prunable=0; reason=\"\"} /^prunable/ {prunable=1; reason=\$0} /^$/ {if (prunable) print path \" | \" reason; prunable=0; reason=\"\"} END {if (prunable) print path \" | \" reason}"
test "$removable" = 0
'
```

Exit status: 0

## Stdout

Bytes: 5689
Truncated: yes

```text
# before/after cleanup verification for atelier-lfwg
# du -sh /root/atelier/.atelier-worktrees
23G	/root/atelier/.atelier-worktrees
# git worktree list --porcelain
worktree /root/atelier
HEAD 5a27f5cb6dc682051a65a3ef18c97300599f8d86
branch refs/heads/master

worktree /root/atelier/.atelier-worktrees/atelier-3iom
HEAD fff34e59005cc8b1a041910c6bdd7a18d6492866
branch refs/heads/codex/atelier-3iom

worktree /root/atelier/.atelier-worktrees/atelier-3z35
HEAD 9b2a5a74428a475f035f1419e1ce0f4704c9ca4c
branch refs/heads/codex/atelier-3z35

worktree /root/atelier/.atelier-worktrees/atelier-613f
HEAD aad3551ca5148a439217e96d440c3fab3c004f38
branch refs/heads/codex/atelier-613f

worktree /root/atelier/.atelier-worktrees/atelier-c64h
HEAD ca0301fd577ecd20f4d7bfd5d6f84520854bd8ac
branch refs/heads/codex/atelier-c64h

worktree /root/atelier/.atelier-worktrees/atelier-gzel
HEAD 586929821ac8650102ee7de8e094926d0f9b59a9
branch refs/heads/codex/atelier-gzel

worktree /root/atelier/.atelier-worktrees/atelier-j01c
HEAD b28959eca78d61796840307a92e7fb24505804dd
branch refs/heads/codex/atelier-j01c

worktree /root/atelier/.atelier-worktrees/atelier-liqk
HEAD b28959eca78d61796840307a92e7fb24505804dd
branch refs/heads/codex/atelier-liqk

worktree /root/atelier/.atelier-worktrees/atelier-n0p4
HEAD 95185d033628febed38b451da43b972c664b8105
branch refs/heads/codex/atelier-n0p4

worktree /root/atelier/.atelier-worktrees/atelier-nqp4
HEAD 8070895877d723f3e79683f1988fdda540dad89a
branch refs/heads/codex/atelier-nqp4

worktree /root/atelier/.atelier-worktrees/atelier-nwlx
HEAD 9206ec9842e75c1983c773ee6ccbe1444d45e01c
branch refs/heads/codex/atelier-nwlx

worktree /root/atelier/.atelier-worktrees/atelier-of3h
HEAD 7ed85e9ea25772a1d53081ff7b704d435bfabf2b
branch refs/heads/codex/atelier-of3h

worktree /root/atelier/.atelier-worktrees/atelier-oku1
HEAD ca0301fd577ecd20f4d7bfd5d6f84520854bd8ac
branch refs/heads/codex/atelier-oku1

worktree /root/atelier/.atelier-worktrees/atelier-oqtz
HEAD 35b2696cf5871943d7f8271ea0084f9e84b632cf
branch refs/heads/codex/atelier-oqtz

worktree /root/atelier/.atelier-worktrees/atelier-papa
HEAD 35b2696cf5871943d7f8271ea0084f9e84b632cf
branch refs/heads/codex/atelier-papa

worktree /root/atelier/.atelier-worktrees/atelier-qnxs
HEAD 12065e3c75301c1d9e1889106136762c1d3c5cb3
branch refs/heads/codex/atelier-qnxs

worktree /root/atelier/.atelier-worktrees/atelier-sxh8
HEAD ccbc5948978de40a0a3654d89252c54015ac46c0
branch refs/heads/codex/atelier-sxh8

worktree /root/atelier/.atelier-worktrees/atelier-u08r
HEAD fa9ca5d6bcfdf3dd6f2f0fce61c510ac2f43f3e3
branch refs/heads/codex/atelier-u08r

worktree /root/atelier/.atelier-worktrees/atelier-u6ax
HEAD 69724b1ad5025d5c6fbcf95cbed74f1ab65b9661
branch refs/heads/codex/atelier-u6ax

worktree /root/atelier/.atelier-worktrees/atelier-vau5
HEAD d9a230d20a9ed3f2e95c3bea60bdfb70862458cd
branch refs/heads/codex/atelier-vau5

worktree /root/atelier/.atelier-worktrees/atelier-vj08
HEAD d9a230d20a9ed3f2e95c3bea60bdfb70862458cd
branch refs/heads/codex/atelier-vj08

worktree /root/atelier/.atelier-worktrees/atelier-vu88
HEAD ce1642e0bdff6e26af99503a81e3d2c48c98c027
branch refs/heads/codex/atelier-vu88

worktree /root/atelier/.atelier-worktrees/atelier-y041
HEAD 9287234523b1ec7bc92c80676fb08c4f352b821e
branch refs/heads/codex/atelier-y041

# current removable-worktree classification
| branch | path | dirty | unique_vs_master | contained_in_master | classification |
| --- | --- | ---: | ---: | --- | --- |
| codex/atelier-3iom | /root/atelier/.atelier-worktrees/atelier-3iom | 1 | 0 | yes | preserve |
| codex/atelier-3z35 | /root/atelier/.atelier-worktrees/atelier-3z35 | 16 | 0 | yes | preserve |
| codex/atelier-613f | /root/atelier/.atelier-worktrees/atelier-613f | 9 | 0 | yes | preserve |
| codex/atelier-c64h | /root/atelier/.atelier-worktrees/atelier-c64h | 14 | 0 | yes | preserve |
| codex/atelier-gzel | /root/atelier/.atelier-worktrees/atelier-gzel | 1 | 0 | yes | preserve |
| codex/atelier-j01c | /root/atelier/.atelier-worktrees/atelier-j01c | 10 | 0 | yes | preserve |
| codex/atelier-liqk | /root/ate
```

## Stderr

Bytes: 0
Truncated: no

```text
```
