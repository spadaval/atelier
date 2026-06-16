---
created_at: "2026-06-14T22:11:58.093353854+00:00"
id: "atelier-ogi4"
evidence_type: "validation"
captured_at: "2026-06-14T22:11:46.071710392+00:00"
command: "bash -lc '\nset -eu\nclassify_branch() {\n  branch=\"$1\"\n  issue=${branch#codex/}\n  wt=\"/root/atelier/.atelier-worktrees/${issue}\"\n  if [ -d \"$wt/.git\" ] || [ -f \"$wt/.git\" ]; then\n    dirty=$(git -C \"$wt\" status --porcelain | wc -l | tr -d \" \")\n    if git merge-base --is-ancestor \"$branch\" master; then contained_master=yes; else contained_master=no; fi\n    if [ \"$dirty\" != 0 ]; then printf preserve; elif [ \"$contained_master\" = yes ]; then printf delete; else printf \"fold into epic branch\"; fi\n  else\n    printf investigate\n  fi\n}\n\necho \"# git status --short --branch\"\ngit status --short --branch\n\necho \"# git worktree list --porcelain\"\ngit worktree list --porcelain\n\necho \"# .atelier-worktrees directories\"\nfind /root/atelier/.atelier-worktrees -mindepth 1 -maxdepth 1 -type d -print | sort\n\necho \"# du -sh /root/atelier/.atelier-worktrees/*\"\ndu -sh /root/atelier/.atelier-worktrees/*\n\necho \"# branch containment and classification table\"\nprintf \"| branch | worktree | head | dirty_count | unique_vs_master | unique_vs_origin_master | contained_in_master | contained_in_origin_master | disk | classification |\\n\"\nprintf \"| --- | --- | --- | ---: | ---: | ---: | --- | --- | ---: | --- |\\n\"\nfor branch in $(git for-each-ref --format=\"%(refname:short)\" \"refs/heads/codex/atelier-*\" | sort); do\n  issue=${branch#codex/}\n  wt=\"/root/atelier/.atelier-worktrees/${issue}\"\n  head=$(git rev-parse --short \"$branch\")\n  if [ -d \"$wt/.git\" ] || [ -f \"$wt/.git\" ]; then\n    dirty=$(git -C \"$wt\" status --porcelain | wc -l | tr -d \" \")\n    disk=$(du -sh \"$wt\" | awk \"{print \\$1}\")\n    wt_state=present\n  else\n    dirty=NA\n    disk=NA\n    wt_state=missing\n  fi\n  unique_master=$(git rev-list --count \"master..$branch\")\n  unique_origin=$(git rev-list --count \"origin/master..$branch\")\n  if git merge-base --is-ancestor \"$branch\" master; then contained_master=yes; else contained_master=no; fi\n  if git merge-base --is-ancestor \"$branch\" origin/master; then contained_origin=yes; else contained_origin=no; fi\n  if [ \"$wt_state\" = missing ]; then class=investigate; elif [ \"$dirty\" != 0 ]; then class=preserve; elif [ \"$contained_master\" = yes ]; then class=delete; else class=\"fold into epic branch\"; fi\n  printf \"| %s | %s | %s | %s | %s | %s | %s | %s | %s | %s |\\n\" \"$branch\" \"$wt_state\" \"$head\" \"$dirty\" \"$unique_master\" \"$unique_origin\" \"$contained_master\" \"$contained_origin\" \"$disk\" \"$class\"\ndone\n\necho \"# classification totals\"\nfor class in delete preserve \"fold into epic branch\" investigate; do\n  count=0\n  for branch in $(git for-each-ref --format=\"%(refname:short)\" \"refs/heads/codex/atelier-*\" | sort); do\n    c=$(classify_branch \"$branch\")\n    [ \"$c\" = \"$class\" ] && count=$((count+1))\n  done\n  printf \"%s: %s\\n\" \"$class\" \"$count\"\ndone\nprintf \"stale registrations: \"\ngit worktree list --porcelain | awk \"BEGIN{c=0} /^prunable/ {c++} END{print c}\"\n\necho \"# duplicate codex/atelier-* heads\"\ngit for-each-ref --format=\"%(objectname:short) %(refname:short)\" \"refs/heads/codex/atelier-*\" | sort | awk \"{heads[\\$1]=heads[\\$1] \\\" \\\" \\$2; count[\\$1]++} END {for (sha in count) if (count[sha] > 1) print sha heads[sha]}\" | sort\n\necho \"# stale/prunable Git worktree registrations\"\ngit worktree list --porcelain | awk '\"'\"'/^worktree / {path=$2; prunable=0; reason=\"\"} /^prunable/ {prunable=1; reason=$0} /^$/ {if (prunable) print path \" | \" reason; prunable=0; reason=\"\"} END {if (prunable) print path \" | \" reason}'\"'\"'\n\necho \"# branches without matching worktree directory\"\nfor branch in $(git for-each-ref --format=\"%(refname:short)\" \"refs/heads/codex/atelier-*\" | sort); do\n  issue=${branch#codex/}\n  wt=\"/root/atelier/.atelier-worktrees/${issue}\"\n  [ -d \"$wt\" ] || printf \"%s missing %s\\n\" \"$branch\" \"$wt\"\ndone\n\necho \"# worktree directories without matching codex branch\"\nfor wt in /root/atelier/.atelier-worktrees/*; do\n  [ -d \"$wt\" ] || continue\n  issue=${wt##*/}\n  git show-ref --verify --quiet \"refs/heads/codex/${issue}\" || printf \"%s has no codex/%s branch\\n\" \"$wt\" \"$issue\"\ndone\n'"
exit_status: "0"
target:
  kind: "issue"
  id: "atelier-zbd4"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-zbd4"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 1
status: "recorded"
title: "Classification transcript for current issue worktrees and codex atelier branches"
updated_at: "2026-06-14T22:11:59.850828708+00:00"
---

## Summary

Classification transcript for current issue worktrees and codex atelier branches

## Command

```console
bash -lc '
set -eu
classify_branch() {
  branch="$1"
  issue=${branch#codex/}
  wt="/root/atelier/.atelier-worktrees/${issue}"
  if [ -d "$wt/.git" ] || [ -f "$wt/.git" ]; then
    dirty=$(git -C "$wt" status --porcelain | wc -l | tr -d " ")
    if git merge-base --is-ancestor "$branch" master; then contained_master=yes; else contained_master=no; fi
    if [ "$dirty" != 0 ]; then printf preserve; elif [ "$contained_master" = yes ]; then printf delete; else printf "fold into epic branch"; fi
  else
    printf investigate
  fi
}

echo "# git status --short --branch"
git status --short --branch

echo "# git worktree list --porcelain"
git worktree list --porcelain

echo "# .atelier-worktrees directories"
find /root/atelier/.atelier-worktrees -mindepth 1 -maxdepth 1 -type d -print | sort

echo "# du -sh /root/atelier/.atelier-worktrees/*"
du -sh /root/atelier/.atelier-worktrees/*

echo "# branch containment and classification table"
printf "| branch | worktree | head | dirty_count | unique_vs_master | unique_vs_origin_master | contained_in_master | contained_in_origin_master | disk | classification |\n"
printf "| --- | --- | --- | ---: | ---: | ---: | --- | --- | ---: | --- |\n"
for branch in $(git for-each-ref --format="%(refname:short)" "refs/heads/codex/atelier-*" | sort); do
  issue=${branch#codex/}
  wt="/root/atelier/.atelier-worktrees/${issue}"
  head=$(git rev-parse --short "$branch")
  if [ -d "$wt/.git" ] || [ -f "$wt/.git" ]; then
    dirty=$(git -C "$wt" status --porcelain | wc -l | tr -d " ")
    disk=$(du -sh "$wt" | awk "{print \$1}")
    wt_state=present
  else
    dirty=NA
    disk=NA
    wt_state=missing
  fi
  unique_master=$(git rev-list --count "master..$branch")
  unique_origin=$(git rev-list --count "origin/master..$branch")
  if git merge-base --is-ancestor "$branch" master; then contained_master=yes; else contained_master=no; fi
  if git merge-base --is-ancestor "$branch" origin/master; then contained_origin=yes; else contained_origin=no; fi
  if [ "$wt_state" = missing ]; then class=investigate; elif [ "$dirty" != 0 ]; then class=preserve; elif [ "$contained_master" = yes ]; then class=delete; else class="fold into epic branch"; fi
  printf "| %s | %s | %s | %s | %s | %s | %s | %s | %s | %s |\n" "$branch" "$wt_state" "$head" "$dirty" "$unique_master" "$unique_origin" "$contained_master" "$contained_origin" "$disk" "$class"
done

echo "# classification totals"
for class in delete preserve "fold into epic branch" investigate; do
  count=0
  for branch in $(git for-each-ref --format="%(refname:short)" "refs/heads/codex/atelier-*" | sort); do
    c=$(classify_branch "$branch")
    [ "$c" = "$class" ] && count=$((count+1))
  done
  printf "%s: %s\n" "$class" "$count"
done
printf "stale registrations: "
git worktree list --porcelain | awk "BEGIN{c=0} /^prunable/ {c++} END{print c}"

echo "# duplicate codex/atelier-* heads"
git for-each-ref --format="%(objectname:short) %(refname:short)" "refs/heads/codex/atelier-*" | sort | awk "{heads[\$1]=heads[\$1] \" \" \$2; count[\$1]++} END {for (sha in count) if (count[sha] > 1) print sha heads[sha]}" | sort

echo "# stale/prunable Git worktree registrations"
git worktree list --porcelain | awk '"'"'/^worktree / {path=$2; prunable=0; reason=""} /^prunable/ {prunable=1; reason=$0} /^$/ {if (prunable) print path " | " reason; prunable=0; reason=""} END {if (prunable) print path " | " reason}'"'"'

echo "# branches without matching worktree directory"
for branch in $(git for-each-ref --format="%(refname:short)" "refs/heads/codex/atelier-*" | sort); do
  issue=${branch#codex/}
  wt="/root/atelier/.atelier-worktrees/${issue}"
  [ -d "$wt" ] || printf "%s missing %s\n" "$branch" "$wt"
done

echo "# worktree directories without matching codex branch"
for wt in /root/atelier/.atelier-worktrees/*; do
  [ -d "$wt" ] || continue
  issue=${wt##*/}
  git show-ref --verify --quiet "refs/heads/codex/${issue}" || printf "%s has no codex/%s branch\n" "$wt" "$issue"
done
'
```

Exit status: 0

## Stdout

Bytes: 20530
Truncated: yes

```text
# git status --short --branch
## master...origin/master [ahead 514]
 M .atelier/issues/atelier-kybc.md
 M .atelier/issues/atelier-xzco.md
 M .atelier/issues/atelier-zbd4.md
 M CONTEXT.md
 M SPEC.md
?? .atelier/issues/atelier-kybc.activity/
?? .atelier/issues/atelier-xzco.activity/
?? .atelier/issues/atelier-zbd4.activity/
# git worktree list --porcelain
worktree /root/atelier
HEAD 30ed464eea69fc5293b1d0175bbbc15119976af7
branch refs/heads/master

worktree /root/atelier/.atelier-worktrees/atelier-10qm
HEAD 8594a8cddf8f0eb4335535c3dce5584dc82baa8f
branch refs/heads/codex/atelier-10qm

worktree /root/atelier/.atelier-worktrees/atelier-3iom
HEAD fff34e59005cc8b1a041910c6bdd7a18d6492866
branch refs/heads/codex/atelier-3iom

worktree /root/atelier/.atelier-worktrees/atelier-3z35
HEAD 9b2a5a74428a475f035f1419e1ce0f4704c9ca4c
branch refs/heads/codex/atelier-3z35

worktree /root/atelier/.atelier-worktrees/atelier-4eot
HEAD 97fd3730e39c7debbd5eb9448810c09e7b756b5c
branch refs/heads/codex/atelier-4eot

worktree /root/atelier/.atelier-worktrees/atelier-4u5h
HEAD cf488fb29fc83de8625b6585f8ade5773c940d72
branch refs/heads/codex/atelier-4u5h

worktree /root/atelier/.atelier-worktrees/atelier-613f
HEAD aad3551ca5148a439217e96d440c3fab3c004f38
branch refs/heads/codex/atelier-613f

worktree /root/atelier/.atelier-worktrees/atelier-6w0u
HEAD be3f4a0a6306a0d7a9362acec0172abe232e047e
branch refs/heads/codex/atelier-6w0u

worktree /root/atelier/.atelier-worktrees/atelier-7r55
HEAD c05f8e26dd4a0641c847ffcbe1d48d3757afe4c7
branch refs/heads/codex/atelier-7r55

worktree /root/atelier/.atelier-worktrees/atelier-7yen
HEAD 3277a4600548786b493981015408fdeb187b2350
branch refs/heads/codex/atelier-7yen

worktree /root/atelier/.atelier-worktrees/atelier-9soq
HEAD e8ad446dd494fc5cf59c395b9d14c0410a725a4e
branch refs/heads/codex/atelier-9soq

worktree /root/atelier/.atelier-worktrees/atelier-b2vi
HEAD 05c15330ca5812d942de0a729d65296e81ddef53
branch refs/heads/codex/atelier-b2vi

worktree /root/atelier/.atelier-worktrees/atelier-c4uz
HEAD 11c2d6e8e38fe990a72b56855297d738b26bbe53
branch refs/heads/codex/atelier-c4uz

worktree /root/atelier/.atelier-worktrees/atelier-c64h
HEAD ca0301fd577ecd20f4d7bfd5d6f84520854bd8ac
branch refs/heads/codex/atelier-c64h

worktree /root/atelier/.atelier-worktrees/atelier-ca32
HEAD 0badd6fac554d9e6a58d2fe4d7e5972c94f9feab
branch refs/heads/codex/atelier-ca32

worktree /root/atelier/.atelier-worktrees/atelier-cbru
HEAD 05c15330ca5812d942de0a729d65296e81ddef53
branch refs/heads/codex/atelier-cbru

worktree /root/atelier/.atelier-worktrees/atelier-exz1
HEAD 76dbc4b2c9c078970d7095c2e9f23cbd306e2d38
branch refs/heads/codex/atelier-exz1

worktree /root/atelier/.atelier-worktrees/atelier-fx9r
HEAD 748a1037b5026ccb98dd5fc1cd71ae44b4592038
branch refs/heads/codex/atelier-fx9r

worktree /root/atelier/.atelier-worktrees/atelier-ggls
HEAD 1eb1d748977f47e0ba162991421b3858eeaf1cfc
branch refs/heads/codex/atelier-ggls

worktree /root/atelier/.atelier-worktrees/atelier-gzel
HEAD 586929821ac8650102ee7de8e094926d0f9b59a9
branch refs/heads/codex/atelier-gzel

worktree /root/atelier/.atelier-worktrees/atelier-h184
HEAD be4171d11c92775255bd198bcfe03e7b07de0356
branch refs/heads/codex/atelier-h184

worktree /root/atelier/.atelier-worktrees/atelier-i9ob
HEAD a19109a945a3881ceac8865ba26e4af4ac286b38
branch refs/heads/codex/atelier-i9ob

worktree /root/atelier/.atelier-worktrees/atelier-j01c
HEAD b28959eca78d61796840307a92e7fb24505804dd
branch refs/heads/codex/atelier-j01c

worktree /root/atelier/.atelier-worktrees/atelier-ja3o
HEAD ce1642e0bdff6e26af99503a81e3d2c48c98c027
branch refs/heads/codex/atelier-ja3o

worktree /root/atelier/.atelier-worktrees/atelier-jarw
HEAD 97c485168a86b8933702ce17ed4b5fa36c8cec8c
branch refs/heads/codex/atelier-jarw

worktree /root/atelier/.atelier-worktrees/atelier-kpm8
HEAD 2be83a134417018c6ad6c9d4a3c9c167e6accc04
branch refs/heads/codex/atelier-kpm8

worktree /root/atelier/.atelier-worktrees/atelier-l0yk
HEAD c05f8e26dd4a0641c847ffcbe1d48d3757afe4c7
branch refs/heads/codex/atelier-l0yk

worktree /root/atelier/.atelier
```

## Stderr

Bytes: 0
Truncated: no

```text
```
