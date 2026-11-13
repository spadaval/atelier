# Numbers, atelier, 2026-11-13

Window: 2026-11-11T17:56:43Z (the end of atelier-0019's `Window:` line) to 2026-11-13T08:41:43Z, the time taken once with `date -u`.

Items merged in the window, by `atelier log | awk -v s=2026-11-11T17:56:43Z -v e=2026-11-13T08:41:43Z '$4 == "merge" && $1 > s && $1 <= e {print $3}'`: atelier-f6dv, atelier-s47q, atelier-wb05.

## Seconds in each status

| Item | ready | building | in_review | changes_requested | approved | rounds | take-overs | escalations | skips |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| atelier-f6dv | 14 | 88 | 187 | 22 | 10 | 2 | 0 | 0 | 0 |
| atelier-s47q | 40 | 45 | 100 | 0 | 19 | 1 | 0 | 0 | 0 |
| atelier-wb05 | 33 | 352 | 338 | 37 | 13 | 2 | 0 | 0 | 0 |

Medians in whole minutes, halves up (worst beside): ready 1 (s47q, 40s, 1), building 1 (wb05, 352s, 6), in review 3 (wb05, 338s, 6), changes requested 0 (wb05, 37s, 1), approved 0 (s47q, 19s, 0). Rounds median 2 (worst f6dv, 2, tied with wb05 and merged first).

## Open work at the end

`atelier list`: 22 draft, 3 ready, 3 building (atelier-ogpg, atelier-s0j2, atelier-qb4f), 1 approved (atelier-0019). `atelier list --blocked`: 9 draft, 1 ready, so 10 blocked. `atelier list --deferred`: 2 unsettled (atelier-6w8d/3-kvll.1, atelier-x3n5/1-c6ma.3), both already settled on atelier-0019's unmerged branch.

## CI on master

The last ten CI runs on master passed. No CI run on master failed in the window; the newest failure is 2026-11-11T15:18:25Z, before it.

## The log lines

```
2026-11-11T18:14:51Z 5710e8b5e atelier-f6dv ready - draft -> ready; by sumeet, relayed by the task that started the lead
2026-11-11T18:15:05Z 2826d6220 atelier-f6dv start thread:delegated-task:command%3Amcp%3A1e4a355d-d905-4146-a4d8-89e6984f49f1%3Adelegate-task%3A932c2504-ac02-42e4-95a0-bbe6ecd5863b ready -> building
2026-11-11T18:16:33Z 717330cf0 atelier-f6dv submit thread:delegated-task:command%3Amcp%3A1e4a355d-d905-4146-a4d8-89e6984f49f1%3Adelegate-task%3A932c2504-ac02-42e4-95a0-bbe6ecd5863b building -> in_review
2026-11-11T18:18:28Z 3f4a40e63 atelier-f6dv review thread:delegated-task:command%3Amcp%3A7b31cbcf-39a2-4bb0-b4aa-53c1f77ac8b6%3Adelegate-task%3A15c59a9e-b519-495d-b13c-db22917f04ab 1-3oy0
2026-11-11T18:18:28Z 3f4a40e63 atelier-f6dv verdict thread:delegated-task:command%3Amcp%3A7b31cbcf-39a2-4bb0-b4aa-53c1f77ac8b6%3Adelegate-task%3A15c59a9e-b519-495d-b13c-db22917f04ab 1-3oy0 changes; in_review -> changes_requested
2026-11-11T18:18:50Z 272d823cf atelier-f6dv submit thread:delegated-task:command%3Amcp%3A1e4a355d-d905-4146-a4d8-89e6984f49f1%3Adelegate-task%3A932c2504-ac02-42e4-95a0-bbe6ecd5863b changes_requested -> in_review
2026-11-11T18:20:02Z 504ece619 atelier-f6dv review thread:delegated-task:command%3Amcp%3A7b31cbcf-39a2-4bb0-b4aa-53c1f77ac8b6%3Adelegate-task%3A5db8b0dc-2f1d-4c66-b073-0e0eae48b727 2-s2cm
2026-11-11T18:20:02Z 504ece619 atelier-f6dv verdict thread:delegated-task:command%3Amcp%3A7b31cbcf-39a2-4bb0-b4aa-53c1f77ac8b6%3Adelegate-task%3A5db8b0dc-2f1d-4c66-b073-0e0eae48b727 2-s2cm approve; in_review -> approved
2026-11-11T18:20:12Z 5576e2138 atelier-f6dv merge thread:delegated-task:command%3Amcp%3A1e4a355d-d905-4146-a4d8-89e6984f49f1%3Adelegate-task%3A932c2504-ac02-42e4-95a0-bbe6ecd5863b approved -> done
2026-11-13T08:27:11Z dcc7e6e51 atelier-s47q ready - draft -> ready; by sumeet, in conversation on 2026-11-13
2026-11-13T08:27:51Z 6c621c4e8 atelier-s47q start thread:delegated-task:command%3Amcp%3Abe2146c6-f4c6-4217-8f3a-ee1dc9eec577%3Adelegate-task%3Aede59fb7-cf1e-46fc-bfdc-d4202d8ed2aa ready -> building
2026-11-13T08:28:36Z abbfc8292 atelier-s47q submit thread:delegated-task:command%3Amcp%3Abe2146c6-f4c6-4217-8f3a-ee1dc9eec577%3Adelegate-task%3Aede59fb7-cf1e-46fc-bfdc-d4202d8ed2aa building -> in_review
2026-11-13T08:30:16Z 625b0212f atelier-s47q review thread:delegated-task:command%3Amcp%3Ac6c8bab6-0717-458b-b656-a4e9fa122080%3Adelegate-task%3Ae02619d2-c09e-4beb-b4f3-f8c6c984496b 1-j73d
2026-11-13T08:30:16Z 625b0212f atelier-s47q verdict thread:delegated-task:command%3Amcp%3Ac6c8bab6-0717-458b-b656-a4e9fa122080%3Adelegate-task%3Ae02619d2-c09e-4beb-b4f3-f8c6c984496b 1-j73d approve; in_review -> approved
2026-11-13T08:30:35Z 617a8eb6d atelier-s47q merge thread:delegated-task:command%3Amcp%3Abe2146c6-f4c6-4217-8f3a-ee1dc9eec577%3Adelegate-task%3Aede59fb7-cf1e-46fc-bfdc-d4202d8ed2aa approved -> done
2026-11-13T08:27:11Z dcc7e6e51 atelier-wb05 ready - draft -> ready; by sumeet, in conversation on 2026-11-13
2026-11-13T08:27:44Z 7f4802703 atelier-wb05 start thread:delegated-task:command%3Amcp%3Abe2146c6-f4c6-4217-8f3a-ee1dc9eec577%3Adelegate-task%3A5cec947d-b329-4fc8-a2a7-de5ceac8e5db ready -> building
2026-11-13T08:33:36Z 4f8351922 atelier-wb05 submit thread:delegated-task:command%3Amcp%3Abe2146c6-f4c6-4217-8f3a-ee1dc9eec577%3Adelegate-task%3A5cec947d-b329-4fc8-a2a7-de5ceac8e5db building -> in_review
2026-11-13T08:36:36Z 9e6211810 atelier-wb05 review thread:delegated-task:command%3Amcp%3A64389874-c2e8-4544-b20a-b4c28f7b418f%3Adelegate-task%3A57a9e909-2865-4a7a-b177-d81f8fc8baae 1-net6
2026-11-13T08:36:36Z 9e6211810 atelier-wb05 verdict thread:delegated-task:command%3Amcp%3A64389874-c2e8-4544-b20a-b4c28f7b418f%3Adelegate-task%3A57a9e909-2865-4a7a-b177-d81f8fc8baae 1-net6 changes; in_review -> changes_requested
2026-11-13T08:37:13Z 31354fae3 atelier-wb05 submit thread:delegated-task:command%3Amcp%3Abe2146c6-f4c6-4217-8f3a-ee1dc9eec577%3Adelegate-task%3A5cec947d-b329-4fc8-a2a7-de5ceac8e5db changes_requested -> in_review
2026-11-13T08:39:51Z dd9faf33d atelier-wb05 review thread:delegated-task:command%3Amcp%3A64389874-c2e8-4544-b20a-b4c28f7b418f%3Adelegate-task%3A9410a0fe-2682-4c9b-8a63-9f0102ddefac 2-phww
2026-11-13T08:39:51Z dd9faf33d atelier-wb05 verdict thread:delegated-task:command%3Amcp%3A64389874-c2e8-4544-b20a-b4c28f7b418f%3Adelegate-task%3A9410a0fe-2682-4c9b-8a63-9f0102ddefac 2-phww approve; in_review -> approved
2026-11-13T08:40:04Z 1fffc88ae atelier-wb05 merge thread:delegated-task:command%3Amcp%3Abe2146c6-f4c6-4217-8f3a-ee1dc9eec577%3Adelegate-task%3A5cec947d-b329-4fc8-a2a7-de5ceac8e5db approved -> done
```
