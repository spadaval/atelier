---
created_at: "2026-07-16T18:40:10.103340347+00:00"
id: "atelier-xtiz"
evidence_type: "validation"
captured_at: "2026-07-16T18:40:08.643801714+00:00"
command: "bash -lc 'set -eu\nfor id in atelier-4h62 atelier-hdff atelier-oc4x; do atelier issue show \"$id\" > \"/tmp/$id.show\"; done\nprintf \"claim.successors_terminal: \"; for id in atelier-4h62 atelier-hdff atelier-oc4x; do rg -q \"^Status:   done$\" \"/tmp/$id.show\"; done; echo pass\nprintf \"claim.successors_evidence_complete: \"; rg -q \"Evidence Gates: linked validating evidence 19; scoped issues without evidence 0\" /tmp/atelier-4h62.show; rg -q \"Evidence Gates: linked validating evidence 5; scoped issues without evidence 0\" /tmp/atelier-hdff.show; rg -q \"Evidence Gates: linked validating evidence 4; scoped issues without evidence 0\" /tmp/atelier-oc4x.show; echo pass\nprintf \"claim.successors_contained_in_master: \"; for id in atelier-4h62 atelier-hdff atelier-oc4x; do git cat-file -e \"master:.atelier/issues/$id.md\"; done; echo pass\nprintf \"claim.no_unique_implementation_diff: \"; unique=0; while IFS= read -r path; do case \"$path\" in .atelier/issues/atelier-ikuv.md|.atelier/issues/atelier-ikuv.activity/*) ;; *) unique=1; echo \"$path\" ;; esac; done < <(git diff --name-only master...HEAD); test \"$unique\" = 0; echo pass\nprintf \"claim.tracker_checks: \"; atelier check atelier-ikuv >/dev/null; atelier check >/dev/null; echo pass\nprintf \"claim.whitespace: \"; git diff --check master...HEAD; echo pass'"
exit_status: "0"
target:
  kind: "issue"
  id: "atelier-ikuv"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-ikuv"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 1
status: "recorded"
title: "bash -lc 'set -eu\nfor id in atelier-4h62 atelier-hdff atelier-oc4x; do atelier issue show \"$id\" > \"/tmp/$id.show\"; done\nprintf \"claim.successors_terminal: \"; for id in atelier-4h62 atelier-hdff atelier-oc4x; do rg -q \"^Status:   done$\" \"/tmp/$id.show\"; done; echo pass\nprintf \"claim.successors_evidence_complete: \"; rg -q \"Evidence Gates: linked validating evidence 19; scoped issues without evidence 0\" /tmp/atelier-4h62.show; rg -q \"Evidence Gates: linked validating evidence 5; scoped issues without evidence 0\" /tmp/atelier-hdff.show; rg -q \"Evidence Gates: linked validating evidence 4; scoped issues without evidence 0\" /tmp/atelier-oc4x.show; echo pass\nprintf \"claim.successors_contained_in_master: \"; for id in atelier-4h62 atelier-hdff atelier-oc4x; do git cat-file -e \"master:.atelier/issues/$id.md\"; done; echo pass\nprintf \"claim.no_unique_implementation_diff: \"; unique=0; while IFS= read -r path; do case \"$path\" in .atelier/issues/atelier-ikuv.md|.atelier/issues/atelier-ikuv.activity/*) ;; *) unique=1; echo \"$path\" ;; esac; done < <(git diff --name-only master...HEAD); test \"$unique\" = 0; echo pass\nprintf \"claim.tracker_checks: \"; atelier check atelier-ikuv >/dev/null; atelier check >/dev/null; echo pass\nprintf \"claim.whitespace: \"; git diff --check master...HEAD; echo pass'"
updated_at: "2026-07-16T18:40:15.434914299+00:00"
---

## Summary

bash -lc 'set -eu
for id in atelier-4h62 atelier-hdff atelier-oc4x; do atelier issue show "$id" > "/tmp/$id.show"; done
printf "claim.successors_terminal: "; for id in atelier-4h62 atelier-hdff atelier-oc4x; do rg -q "^Status:   done$" "/tmp/$id.show"; done; echo pass
printf "claim.successors_evidence_complete: "; rg -q "Evidence Gates: linked validating evidence 19; scoped issues without evidence 0" /tmp/atelier-4h62.show; rg -q "Evidence Gates: linked validating evidence 5; scoped issues without evidence 0" /tmp/atelier-hdff.show; rg -q "Evidence Gates: linked validating evidence 4; scoped issues without evidence 0" /tmp/atelier-oc4x.show; echo pass
printf "claim.successors_contained_in_master: "; for id in atelier-4h62 atelier-hdff atelier-oc4x; do git cat-file -e "master:.atelier/issues/$id.md"; done; echo pass
printf "claim.no_unique_implementation_diff: "; unique=0; while IFS= read -r path; do case "$path" in .atelier/issues/atelier-ikuv.md|.atelier/issues/atelier-ikuv.activity/*) ;; *) unique=1; echo "$path" ;; esac; done < <(git diff --name-only master...HEAD); test "$unique" = 0; echo pass
printf "claim.tracker_checks: "; atelier check atelier-ikuv >/dev/null; atelier check >/dev/null; echo pass
printf "claim.whitespace: "; git diff --check master...HEAD; echo pass'

## Command

```console
bash -lc 'set -eu
for id in atelier-4h62 atelier-hdff atelier-oc4x; do atelier issue show "$id" > "/tmp/$id.show"; done
printf "claim.successors_terminal: "; for id in atelier-4h62 atelier-hdff atelier-oc4x; do rg -q "^Status:   done$" "/tmp/$id.show"; done; echo pass
printf "claim.successors_evidence_complete: "; rg -q "Evidence Gates: linked validating evidence 19; scoped issues without evidence 0" /tmp/atelier-4h62.show; rg -q "Evidence Gates: linked validating evidence 5; scoped issues without evidence 0" /tmp/atelier-hdff.show; rg -q "Evidence Gates: linked validating evidence 4; scoped issues without evidence 0" /tmp/atelier-oc4x.show; echo pass
printf "claim.successors_contained_in_master: "; for id in atelier-4h62 atelier-hdff atelier-oc4x; do git cat-file -e "master:.atelier/issues/$id.md"; done; echo pass
printf "claim.no_unique_implementation_diff: "; unique=0; while IFS= read -r path; do case "$path" in .atelier/issues/atelier-ikuv.md|.atelier/issues/atelier-ikuv.activity/*) ;; *) unique=1; echo "$path" ;; esac; done < <(git diff --name-only master...HEAD); test "$unique" = 0; echo pass
printf "claim.tracker_checks: "; atelier check atelier-ikuv >/dev/null; atelier check >/dev/null; echo pass
printf "claim.whitespace: "; git diff --check master...HEAD; echo pass'
```

Exit status: 0

## Stdout

Bytes: 208
Truncated: no

```text
claim.successors_terminal: pass
claim.successors_evidence_complete: pass
claim.successors_contained_in_master: pass
claim.no_unique_implementation_diff: pass
claim.tracker_checks: pass
claim.whitespace: pass
```

## Stderr

Bytes: 0
Truncated: no

```text
```

