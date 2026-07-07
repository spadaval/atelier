---
created_at: "2026-07-06T18:08:57.744110736+00:00"
id: "atelier-9xvc"
evidence_type: "test"
captured_at: "2026-07-06T18:08:57.536007792+00:00"
command: "target/debug/atelier review show --issue atelier-ye11 --comments"
exit_status: "1"
target:
  kind: "issue"
  id: "atelier-ye11"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-ye11"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 1
status: "recorded"
title: "target/debug/atelier review show --issue atelier-ye11 --comments"
updated_at: "2026-07-06T18:09:03.910266507+00:00"
---

## Summary

target/debug/atelier review show --issue atelier-ye11 --comments

## Command

```console
target/debug/atelier review show --issue atelier-ye11 --comments
```

Exit status: 1

## Stdout

Bytes: 233
Truncated: no

```text
Review
======
Authority: configured provider review
State:     open
Issue:     atelier-ye11
URL:       http://iiot-grand-central.cisco.com:3000/supadava/atelier/pulls/47
Number:    47
Merged:    false
Review Comments
===============
```

## Stderr

Bytes: 244
Truncated: no

```text
Error: forgejo_api_error: GET /api/v1/repos/supadava/atelier/pulls/47/reviews/comments failed with status 404: {"message":"GetReviewByID","url":"http://iiot-grand-central.cisco.com:3000/api/swagger","errors":["review does not exist [id: 0]"]}
```
