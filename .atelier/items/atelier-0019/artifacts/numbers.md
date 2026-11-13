# Numbers

Window: 2026-11-11T15:20:09Z (the first line `atelier log` printed; atelier's first run) to 2026-11-11T17:56:43Z (`date -u`, taken once). Every number below was taken at 2026-11-11T17:56:43Z.

## The items

`atelier log | awk -v s=2026-11-11T15:20:09Z -v e=2026-11-11T17:56:43Z '$4 == "merge" && $1 > s && $1 <= e {print $3}'` printed 7 items: atelier-2mlh atelier-ivbq atelier-6w8d atelier-vl5n atelier-8b0a atelier-9dku atelier-x3n5.

## Their log lines

All of `atelier log <id>` for each, as printed at 2026-11-11T17:56:43Z.

### atelier-2mlh

```text
2026-11-11T15:43:38Z 96d02deb7 atelier-2mlh ready - draft -> ready; by sumeet
2026-11-11T15:43:53Z 46b0014e8 atelier-2mlh start thread:delegated-task:command%3Amcp%3A2074a88e-920b-49f9-bd17-caf5903cc60f%3Adelegate-task%3A77050191-cad3-4b48-9372-f665e6d5a28e ready -> building
2026-11-11T15:46:36Z 2db1f306d atelier-2mlh submit thread:delegated-task:command%3Amcp%3A2074a88e-920b-49f9-bd17-caf5903cc60f%3Adelegate-task%3A77050191-cad3-4b48-9372-f665e6d5a28e building -> in_review
2026-11-11T15:48:51Z eb7645a2f atelier-2mlh review thread:delegated-task:command%3Amcp%3A905f6c68-aef7-443e-a5d1-2c968ed4d00d%3Adelegate-task%3A3de7621a-470f-447c-9c82-f9cf0f444a39 1-4u37
2026-11-11T15:48:51Z eb7645a2f atelier-2mlh verdict thread:delegated-task:command%3Amcp%3A905f6c68-aef7-443e-a5d1-2c968ed4d00d%3Adelegate-task%3A3de7621a-470f-447c-9c82-f9cf0f444a39 1-4u37 approve; in_review -> approved
2026-11-11T15:49:09Z 9233f75e4 atelier-2mlh merge thread:delegated-task:command%3Amcp%3A2074a88e-920b-49f9-bd17-caf5903cc60f%3Adelegate-task%3A77050191-cad3-4b48-9372-f665e6d5a28e approved -> done
```

### atelier-ivbq

```text
2026-11-11T15:43:38Z 96d02deb7 atelier-ivbq ready - draft -> ready; by sumeet
2026-11-11T15:43:57Z 4ad06abae atelier-ivbq start thread:delegated-task:command%3Amcp%3A2074a88e-920b-49f9-bd17-caf5903cc60f%3Adelegate-task%3Ae81e5c93-fcde-468f-a99a-334005140e39 ready -> building
2026-11-11T15:56:52Z 122c0f5a7 atelier-ivbq submit thread:delegated-task:command%3Amcp%3A2074a88e-920b-49f9-bd17-caf5903cc60f%3Adelegate-task%3Ae81e5c93-fcde-468f-a99a-334005140e39 building -> in_review
2026-11-11T16:00:25Z 5dec1e884 atelier-ivbq review thread:delegated-task:command%3Amcp%3A570d4615-b9d9-43d7-b838-51d47e80cbc3%3Adelegate-task%3A636d6beb-49d0-4dc4-80bc-5a4fcfc971b6 1-o9m9
2026-11-11T16:00:25Z 5dec1e884 atelier-ivbq verdict thread:delegated-task:command%3Amcp%3A570d4615-b9d9-43d7-b838-51d47e80cbc3%3Adelegate-task%3A636d6beb-49d0-4dc4-80bc-5a4fcfc971b6 1-o9m9 changes; in_review -> changes_requested
2026-11-11T16:03:26Z 6dd7fc218 atelier-ivbq submit thread:delegated-task:command%3Amcp%3A2074a88e-920b-49f9-bd17-caf5903cc60f%3Adelegate-task%3Ae81e5c93-fcde-468f-a99a-334005140e39 changes_requested -> in_review
2026-11-11T16:05:39Z 6318ffd9d atelier-ivbq review thread:delegated-task:command%3Amcp%3A570d4615-b9d9-43d7-b838-51d47e80cbc3%3Adelegate-task%3Aec69df6c-e11e-4188-ab74-e4158a0c920d 2-6zsp
2026-11-11T16:05:39Z 6318ffd9d atelier-ivbq verdict thread:delegated-task:command%3Amcp%3A570d4615-b9d9-43d7-b838-51d47e80cbc3%3Adelegate-task%3Aec69df6c-e11e-4188-ab74-e4158a0c920d 2-6zsp approve; in_review -> approved
2026-11-11T16:06:11Z 730e5bf2c atelier-ivbq merge thread:delegated-task:command%3Amcp%3A2074a88e-920b-49f9-bd17-caf5903cc60f%3Adelegate-task%3Ae81e5c93-fcde-468f-a99a-334005140e39 approved -> done
```

### atelier-6w8d

```text
2026-11-11T15:43:38Z 96d02deb7 atelier-6w8d ready - draft -> ready; by sumeet
2026-11-11T15:43:54Z 1d16479f5 atelier-6w8d start thread:delegated-task:command%3Amcp%3A2074a88e-920b-49f9-bd17-caf5903cc60f%3Adelegate-task%3A9ecd8d63-8fcc-407b-8c2c-a18464f2529d ready -> building
2026-11-11T16:03:37Z e4297c91c atelier-6w8d submit thread:delegated-task:command%3Amcp%3A2074a88e-920b-49f9-bd17-caf5903cc60f%3Adelegate-task%3A9ecd8d63-8fcc-407b-8c2c-a18464f2529d building -> in_review
2026-11-11T16:08:12Z 690ea012c atelier-6w8d review thread:delegated-task:command%3Amcp%3A201bf233-fdc5-4e41-814c-36f3d465a567%3Adelegate-task%3Aa563d0e7-3706-4845-bb8c-b4afd6f2d370 1-rzbt
2026-11-11T16:08:12Z 690ea012c atelier-6w8d verdict thread:delegated-task:command%3Amcp%3A201bf233-fdc5-4e41-814c-36f3d465a567%3Adelegate-task%3Aa563d0e7-3706-4845-bb8c-b4afd6f2d370 1-rzbt changes; in_review -> changes_requested
2026-11-11T16:09:00Z ed7221b48 atelier-6w8d submit thread:delegated-task:command%3Amcp%3A2074a88e-920b-49f9-bd17-caf5903cc60f%3Adelegate-task%3A9ecd8d63-8fcc-407b-8c2c-a18464f2529d changes_requested -> in_review
2026-11-11T16:12:28Z 2922d9142 atelier-6w8d review thread:delegated-task:command%3Amcp%3A201bf233-fdc5-4e41-814c-36f3d465a567%3Adelegate-task%3A2b7e5f06-ed0a-406c-9e31-9645d6316db8 2-tbnq
2026-11-11T16:12:28Z 2922d9142 atelier-6w8d verdict thread:delegated-task:command%3Amcp%3A201bf233-fdc5-4e41-814c-36f3d465a567%3Adelegate-task%3A2b7e5f06-ed0a-406c-9e31-9645d6316db8 2-tbnq approve; in_review -> approved
2026-11-11T16:13:44Z 12abb2f38 atelier-6w8d submit thread:delegated-task:command%3Amcp%3A2074a88e-920b-49f9-bd17-caf5903cc60f%3Adelegate-task%3A9ecd8d63-8fcc-407b-8c2c-a18464f2529d approved -> in_review
2026-11-11T16:18:29Z 0a772ac96 atelier-6w8d review thread:delegated-task:command%3Amcp%3A201bf233-fdc5-4e41-814c-36f3d465a567%3Adelegate-task%3Aca5f8aee-07be-4ea7-bf86-80eed73fa29d 3-kvll
2026-11-11T16:18:29Z 0a772ac96 atelier-6w8d verdict thread:delegated-task:command%3Amcp%3A201bf233-fdc5-4e41-814c-36f3d465a567%3Adelegate-task%3Aca5f8aee-07be-4ea7-bf86-80eed73fa29d 3-kvll approve; in_review -> approved
2026-11-11T16:47:06Z 78f7b3dd3 atelier-6w8d merge thread:delegated-task:command%3Amcp%3A2074a88e-920b-49f9-bd17-caf5903cc60f%3Adelegate-task%3A9ecd8d63-8fcc-407b-8c2c-a18464f2529d approved -> done; skip CI passed on the head (the latest run of ci.yml was failure: https://github.com/spadaval/atelier/actions/runs/43056232363): desk CI job failed three times installing Playwright's apt packages (runner DNS could not resolve the Ubuntu mirrors, run 43056232363); its typecheck, unit tests and build passed in CI, the rust job passed, and pnpm e2e passed 52 of 52 locally; this item changes only desk/README.md under desk/
2026-11-11T18:02:55Z 638aaa87b atelier-6w8d settle thread:delegated-task:command%3Amcp%3A1e4a355d-d905-4146-a4d8-89e6984f49f1%3Adelegate-task%3A1c7e22f2-4ce0-40e1-a447-156f728619a3 3-kvll.1 dropped: Fixed by the gardener's upkeep commit 94bbfa149 on atelier-0019: both citations now link the design record's sections.
```

### atelier-vl5n

```text
2026-11-11T17:10:10Z dae385d23 atelier-vl5n ready - draft -> ready; by sumeet, relayed by the task that started the lead
2026-11-11T17:10:54Z 0ea3b21ce atelier-vl5n start thread:delegated-task:command%3Amcp%3A1e4a355d-d905-4146-a4d8-89e6984f49f1%3Adelegate-task%3Acfed5884-1a86-461b-9b6a-d4a2a1a326ee ready -> building
2026-11-11T17:21:12Z db7a7a43b atelier-vl5n submit thread:delegated-task:command%3Amcp%3A1e4a355d-d905-4146-a4d8-89e6984f49f1%3Adelegate-task%3Acfed5884-1a86-461b-9b6a-d4a2a1a326ee building -> in_review
2026-11-11T17:23:56Z 164ec3feb atelier-vl5n review thread:delegated-task:command%3Amcp%3Af03ae543-67b7-4101-adec-2ed71d3b4324%3Adelegate-task%3A30cfb179-13b5-4776-8e2a-ca3b5e0c6619 1-pv9w
2026-11-11T17:23:56Z 164ec3feb atelier-vl5n verdict thread:delegated-task:command%3Amcp%3Af03ae543-67b7-4101-adec-2ed71d3b4324%3Adelegate-task%3A30cfb179-13b5-4776-8e2a-ca3b5e0c6619 1-pv9w changes; in_review -> changes_requested
2026-11-11T17:25:00Z a577cc3f3 atelier-vl5n submit thread:delegated-task:command%3Amcp%3A1e4a355d-d905-4146-a4d8-89e6984f49f1%3Adelegate-task%3Acfed5884-1a86-461b-9b6a-d4a2a1a326ee changes_requested -> in_review
2026-11-11T17:27:02Z e0e79c24f atelier-vl5n review thread:delegated-task:command%3Amcp%3Af03ae543-67b7-4101-adec-2ed71d3b4324%3Adelegate-task%3Ac88fdfbd-7254-4bc4-a211-911d677dbc66 2-cmz4
2026-11-11T17:27:02Z e0e79c24f atelier-vl5n verdict thread:delegated-task:command%3Amcp%3Af03ae543-67b7-4101-adec-2ed71d3b4324%3Adelegate-task%3Ac88fdfbd-7254-4bc4-a211-911d677dbc66 2-cmz4 changes; in_review -> changes_requested
2026-11-11T17:27:50Z 3a9a1f72a atelier-vl5n submit thread:delegated-task:command%3Amcp%3A1e4a355d-d905-4146-a4d8-89e6984f49f1%3Adelegate-task%3Acfed5884-1a86-461b-9b6a-d4a2a1a326ee changes_requested -> in_review
2026-11-11T17:29:42Z e8ca3ee40 atelier-vl5n review thread:delegated-task:command%3Amcp%3Af03ae543-67b7-4101-adec-2ed71d3b4324%3Adelegate-task%3Ad93dd380-5523-4fdf-a88b-c48845e43703 3-9zna
2026-11-11T17:29:42Z e8ca3ee40 atelier-vl5n verdict thread:delegated-task:command%3Amcp%3Af03ae543-67b7-4101-adec-2ed71d3b4324%3Adelegate-task%3Ad93dd380-5523-4fdf-a88b-c48845e43703 3-9zna approve; in_review -> approved
2026-11-11T17:29:58Z c60bac906 atelier-vl5n merge thread:delegated-task:command%3Amcp%3A1e4a355d-d905-4146-a4d8-89e6984f49f1%3Adelegate-task%3Acfed5884-1a86-461b-9b6a-d4a2a1a326ee approved -> done
```

### atelier-8b0a

```text
2026-11-11T17:10:10Z dae385d23 atelier-8b0a ready - draft -> ready; by sumeet, relayed by the task that started the lead
2026-11-11T17:10:54Z 5da86eb73 atelier-8b0a start thread:delegated-task:command%3Amcp%3A1e4a355d-d905-4146-a4d8-89e6984f49f1%3Adelegate-task%3Ad3b10278-c0d6-48d8-ab9f-2f66c69a7e79 ready -> building
2026-11-11T17:27:20Z 8169e44f4 atelier-8b0a submit thread:delegated-task:command%3Amcp%3A1e4a355d-d905-4146-a4d8-89e6984f49f1%3Adelegate-task%3Ad3b10278-c0d6-48d8-ab9f-2f66c69a7e79 building -> in_review
2026-11-11T17:30:51Z 09aa81329 atelier-8b0a review thread:delegated-task:command%3Amcp%3A0950c503-e754-4673-a60a-b22e87c91dff%3Adelegate-task%3Aaac59384-65ea-4a6d-804a-591d9c9c7ca0 1-kn9e
2026-11-11T17:30:51Z 09aa81329 atelier-8b0a verdict thread:delegated-task:command%3Amcp%3A0950c503-e754-4673-a60a-b22e87c91dff%3Adelegate-task%3Aaac59384-65ea-4a6d-804a-591d9c9c7ca0 1-kn9e changes; in_review -> changes_requested
2026-11-11T17:33:16Z 353a943b2 atelier-8b0a submit thread:delegated-task:command%3Amcp%3A1e4a355d-d905-4146-a4d8-89e6984f49f1%3Adelegate-task%3Ad3b10278-c0d6-48d8-ab9f-2f66c69a7e79 changes_requested -> in_review
2026-11-11T17:35:39Z 6367bae4d atelier-8b0a review thread:delegated-task:command%3Amcp%3A0950c503-e754-4673-a60a-b22e87c91dff%3Adelegate-task%3Ab72ac20f-6e56-40a7-b1f8-6c59ab07c9bf 2-0f0m
2026-11-11T17:35:39Z 6367bae4d atelier-8b0a verdict thread:delegated-task:command%3Amcp%3A0950c503-e754-4673-a60a-b22e87c91dff%3Adelegate-task%3Ab72ac20f-6e56-40a7-b1f8-6c59ab07c9bf 2-0f0m approve; in_review -> approved
2026-11-11T17:35:50Z df9c84a43 atelier-8b0a merge thread:delegated-task:command%3Amcp%3A1e4a355d-d905-4146-a4d8-89e6984f49f1%3Adelegate-task%3Ad3b10278-c0d6-48d8-ab9f-2f66c69a7e79 approved -> done
```

### atelier-9dku

```text
2026-11-11T17:41:08Z e182f58d2 atelier-9dku ready - draft -> ready; by sumeet, relayed by the task that started the lead
2026-11-11T17:41:25Z c221d1b22 atelier-9dku start thread:delegated-task:command%3Amcp%3A1e4a355d-d905-4146-a4d8-89e6984f49f1%3Adelegate-task%3Ab88e5ac4-cc78-4fea-add2-d618a1992e35 ready -> building
2026-11-11T17:45:50Z d774571fa atelier-9dku submit thread:delegated-task:command%3Amcp%3A1e4a355d-d905-4146-a4d8-89e6984f49f1%3Adelegate-task%3Ab88e5ac4-cc78-4fea-add2-d618a1992e35 building -> in_review
2026-11-11T17:48:55Z 45f115827 atelier-9dku review thread:delegated-task:command%3Amcp%3A0733f24c-2d7b-46d7-9f02-108da8a49154%3Adelegate-task%3A8ec90a3c-8a71-4bf0-84ff-5901ead1edcf 1-2x6c
2026-11-11T17:48:55Z 45f115827 atelier-9dku verdict thread:delegated-task:command%3Amcp%3A0733f24c-2d7b-46d7-9f02-108da8a49154%3Adelegate-task%3A8ec90a3c-8a71-4bf0-84ff-5901ead1edcf 1-2x6c approve; in_review -> approved
2026-11-11T17:49:11Z 997032dd2 atelier-9dku merge thread:delegated-task:command%3Amcp%3A1e4a355d-d905-4146-a4d8-89e6984f49f1%3Adelegate-task%3Ab88e5ac4-cc78-4fea-add2-d618a1992e35 approved -> done
```

### atelier-x3n5

```text
2026-11-11T17:10:10Z dae385d23 atelier-x3n5 ready - draft -> ready; by sumeet, relayed by the task that started the lead
2026-11-11T17:37:01Z fea8b2f6c atelier-x3n5 start thread:delegated-task:command%3Amcp%3A1e4a355d-d905-4146-a4d8-89e6984f49f1%3Adelegate-task%3Ac0bf3903-20c0-4459-bc9e-8bb9a1af8d59 ready -> building
2026-11-11T17:47:40Z 23b13d37a atelier-x3n5 submit thread:delegated-task:command%3Amcp%3A1e4a355d-d905-4146-a4d8-89e6984f49f1%3Adelegate-task%3Ac0bf3903-20c0-4459-bc9e-8bb9a1af8d59 building -> in_review
2026-11-11T17:51:16Z e04da0f0c atelier-x3n5 review thread:delegated-task:command%3Amcp%3A952a8bcb-a049-4c76-bd07-27d029a0ca95%3Adelegate-task%3A88b1dd08-9e2e-46da-8e72-fe1ccdf563c0 1-c6ma
2026-11-11T17:51:16Z e04da0f0c atelier-x3n5 verdict thread:delegated-task:command%3Amcp%3A952a8bcb-a049-4c76-bd07-27d029a0ca95%3Adelegate-task%3A88b1dd08-9e2e-46da-8e72-fe1ccdf563c0 1-c6ma changes; in_review -> changes_requested
2026-11-11T17:52:59Z 0e2100e96 atelier-x3n5 submit thread:delegated-task:command%3Amcp%3A1e4a355d-d905-4146-a4d8-89e6984f49f1%3Adelegate-task%3Ac0bf3903-20c0-4459-bc9e-8bb9a1af8d59 changes_requested -> in_review
2026-11-11T17:55:13Z 5180fdb0c atelier-x3n5 review thread:delegated-task:command%3Amcp%3A952a8bcb-a049-4c76-bd07-27d029a0ca95%3Adelegate-task%3Ac53f8e28-2011-4d39-8af4-baa1a90a4164 2-3puu
2026-11-11T17:55:13Z 5180fdb0c atelier-x3n5 verdict thread:delegated-task:command%3Amcp%3A952a8bcb-a049-4c76-bd07-27d029a0ca95%3Adelegate-task%3Ac53f8e28-2011-4d39-8af4-baa1a90a4164 2-3puu approve; in_review -> approved
2026-11-11T17:55:26Z 77026092e atelier-x3n5 merge thread:delegated-task:command%3Amcp%3A1e4a355d-d905-4146-a4d8-89e6984f49f1%3Adelegate-task%3Ac0bf3903-20c0-4459-bc9e-8bb9a1af8d59 approved -> done
2026-11-11T18:02:55Z 638aaa87b atelier-x3n5 settle thread:delegated-task:command%3Amcp%3A1e4a355d-d905-4146-a4d8-89e6984f49f1%3Adelegate-task%3A1c7e22f2-4ce0-40e1-a447-156f728619a3 1-c6ma.3 taken into atelier-qf7u
```

## Per item

Whole minutes, halves rounded up; seconds in brackets. Waiting is time in `ready`.

```text
atelier-2mlh ready=0m(15s) building=3m(163s) in_review=2m(135s) changes_requested=0m(0s) approved=0m(18s) rounds=1 takeovers=0 escalations=0 skips={}
atelier-ivbq ready=0m(19s) building=13m(775s) in_review=6m(346s) changes_requested=3m(181s) approved=1m(32s) rounds=2 takeovers=0 escalations=0 skips={}
atelier-6w8d ready=0m(16s) building=20m(1183s) in_review=13m(768s) changes_requested=1m(48s) approved=30m(1793s) rounds=3 takeovers=0 escalations=0 skips={'CI passed on the head': 1}
atelier-vl5n ready=1m(44s) building=10m(618s) in_review=7m(398s) changes_requested=2m(112s) approved=0m(16s) rounds=3 takeovers=0 escalations=0 skips={}
atelier-8b0a ready=1m(44s) building=16m(986s) in_review=6m(354s) changes_requested=2m(145s) approved=0m(11s) rounds=2 takeovers=0 escalations=0 skips={}
atelier-9dku ready=0m(17s) building=4m(265s) in_review=3m(185s) changes_requested=0m(0s) approved=0m(16s) rounds=1 takeovers=0 escalations=0 skips={}
atelier-x3n5 ready=27m(1611s) building=11m(639s) in_review=6m(350s) changes_requested=2m(103s) approved=0m(13s) rounds=2 takeovers=0 escalations=0 skips={}
---
ready median 0 worst atelier-x3n5 27
building median 11 worst atelier-6w8d 20
in_review median 6 worst atelier-6w8d 13
changes_requested median 2 worst atelier-ivbq 3
approved median 0 worst atelier-6w8d 30
rounds median 2 worst ('atelier-6w8d', 3)
takeovers 0 escalations 0
skips {'CI passed on the head': 1}
```

Ties: rounds 3 for atelier-6w8d and atelier-vl5n; atelier-6w8d merged first, so it is named.

Across the 7 items: medians waiting 0, building 11, in review 6, changes requested 2, approved 0 minutes; rounds median 2. Take-overs 0, escalations 0. Skips by rule: `CI passed on the head` 1 (atelier-6w8d).

## Open work

Counted at 2026-11-11T17:56:43Z with the commands garden.md gives, on the base at 87ee3ccd0.

- Open items by status: 106 draft, 22 ready, 1 building (this run).
- Blocked (`atelier list --blocked`): 5 draft, 3 ready.
- Unsettled deferred findings: 2, `atelier-6w8d/3-kvll.1` and `atelier-x3n5/1-c6ma.3`. Both are settled on this branch (see the Report).

## Carried measures

A first run carries the November baseline, `docs/architecture/quality/scorecard-baseline-2026-11.md`, dated 2026-11-10 (atelier's column). No snapshot was taken and nothing in `scripts/scorecard/` was run.

| Measure | atelier, measured 2026-11-10 |
| --- | --- |
| Defects after merge | 15 of 277 merged items (5.4 per 100), a floor |
| Interventions per mission | 0 to 7, 12 missions |
| Process share | 28.0% of shell commands, 21.6% of token cost |
| Confusion per 100 `atelier` calls | 25.2 |
| Tokens per merged item, median (mean) | 3.06 (7.61) USD |
| Gate precision | 4 of 54 |
| Review yield | 8 of 20 |
| Calls before first edit, median | 22 |
| Rework | 10.8% of 57,671 lines |
| Where caught: review, CI, after merge | about 191, 6, 15 |
| Non-test lines of Rust | 48,989 |

Skips are counted from the log this run (above); the baseline had no such mechanism.

## Explaining the worst

From the records, without transcripts:

- atelier-x3n5, 27 minutes waiting: it was blocked by atelier-8b0a, which merged at 17:35:50; x3n5 started at 17:37:01. The wait is the blocker, not a stall.
- atelier-6w8d, 30 minutes approved: CI run 43056232363 on its head ran from 16:13:52 to 16:46:16 and failed in the desk job, whose reason the skip gives (Playwright's apt packages, runner DNS); the merge at 16:47:06 skipped `CI passed on the head`. Its 13 minutes in review and round 3 include a re-review after the base was merged in (its third `submit` is `approved -> in_review` at 16:13:44).
- atelier-6w8d, 20 minutes building: the largest change of the window, a docs rewrite that deleted 31 ADRs and moved two specifications.
- atelier-ivbq, 3 minutes in changes requested: two fix-now findings in round 1.

## Fix-now findings in the window

10 distinct fix-now findings in round 1 reviews (later rounds repeat them to mark them fixed), and 2 defer findings.

| Item | Finding | File it names |
| --- | --- | --- |
| atelier-ivbq | 1-o9m9.1 waiting questions hidden from finished reviewers | `crates/atelier-cli/src/output.rs` |
| atelier-ivbq | 1-o9m9.2 wrapped text starting with `#` read as a heading | `crates/atelier-records/src/body.rs` |
| atelier-6w8d | 1-rzbt.1 decisions lost with deleted ADRs | `docs/design.md` |
| atelier-6w8d | 1-rzbt.2 doc tells reader to start an item not made ready | `CONTRIBUTING.md` |
| atelier-vl5n | 1-pv9w.1 path check misses root documents | `skills/agent-factory/procedures/readiness.md` |
| atelier-vl5n | 1-pv9w.2 Claude's loading rule untested | `skills/agent-factory/procedures/readiness.md` |
| atelier-8b0a | 1-kn9e.1 history aborts on an old encoding error | `crates/atelier-app/src/log.rs` |
| atelier-8b0a | 1-kn9e.2 invalid version's predecessor not per occurrence | `crates/atelier-app/src/log.rs` |
| atelier-x3n5 | 1-c6ma.1 fenced examples read as findings | `scripts/scorecard/draw_samples.py` |
| atelier-x3n5 | 1-c6ma.2 mission membership read from the body | `scripts/scorecard/draw_samples.py` |

Most named: `readiness.md`, `log.rs` and `draw_samples.py`, 2 each, each from one item. Three of the ten (ivbq .2, x3n5 .1 and .2) are record Markdown read by a second reader with its own rules: `body.rs` treats any leading `#` as a heading where `document.rs` does not, and `draw_samples.py` reads review findings and mission membership by its own text matching.

## CI

`gh run list --workflow ci.yml`, runs created in the window: on `master`, 20 of 20 succeeded; on branches, 1 failure (43056232363, atelier-6w8d, desk job, runner DNS) and 1 cancelled (43054523288, atelier-6w8d, replaced by a newer push). The last base run that ran the Rust tests, 43072803941 on 77026092e, took 64 seconds.

## Navigation sample

The three items longest in `building`, read with T3's thread-read tool up to the first file change; a sample of three. Tool calls before the first edit, counted as T3 lists them (shell commands, file reads and the skill load):

- atelier-6w8d: 31. Read the item, the implementer role, the mission, both specifications then in `docs/strategies/`, `docs/index.md` and the other index pages, `README.md`, `CONTRIBUTING.md`, `CHANGELOG.md`, `scripts/cutover/README.md`, `desk/README.md`, all 31 ADRs, `docs/architecture/quality/*.md`, `Cargo.toml`, `crates/atelier-cli/src/main.rs`, `evals/agent-factory/README.md`, scenario `setup.sh` files and `evals/agent-factory/harness/run.py`.
- atelier-8b0a: 26. Read the item, the implementer role, `docs/design.md` (The log, Decisions), `docs/product.md`, mission atelier-295u's three files, `crates/atelier-records/src/{at_commit,lib,store,body}.rs`, `crates/atelier-core/src/*.rs`, `crates/atelier-app/src/{lib,show,skip,git,repo}.rs`, `crates/atelier-cli/src/{main,views,git,output,merge}.rs`, `crates/atelier-cli/tests/{common/mod,scenarios,merge}.rs` and the crates' `Cargo.toml`.
- atelier-ivbq: 34. Read the item, an eval result, four old items' Reports, `crates/atelier-app/src/{repo,say,view,decide,show,acts}.rs`, `crates/atelier-cli/src/{main,output,commands}.rs`, `crates/atelier-records/src/{store,body}.rs`, both specifications then in `docs/strategies/` (Commands, Threads and roles, Review), `crates/atelier-cli/tests/{scenarios,common/mod}.rs`.

Read by all three: the product specification and the design record (now `docs/product.md`, 513 lines, and `docs/design.md`, 1,384 lines) and `crates/atelier-cli/src/main.rs`. Read by both code items: `crates/atelier-cli/tests/scenarios.rs` (1,376 lines) with `tests/common/mod.rs`, `crates/atelier-cli/src/output.rs` (1,471 lines), `crates/atelier-app/src/repo.rs`, `crates/atelier-records/src/{store,body}.rs`. Median 31 against the baseline's 22; the baseline's method counts differently and three threads are no median, so it is not compared on the page.

## atelier-j86z's entries

Read as evidence; checked against 0.5 at 87ee3ccd0.

- Moot: R1 (`issue create --parent`; no `issue` command), R2 (a read from a checkout behind the base: `atelier show` now reads the base's record, checked from a checkout 52 commits behind), R3 (`review disposition --follow-up`; findings settle with `new --from` or `drop`), R4 (the draft-PR line is gone from `SKILL.md`), R7 (no `git archive` extraction left in `crates/`), R10 (`merge-ci-passed.sh` and `docs/strategies/` are gone).
- Still apply: R5 (`atelier new "--parent is wrong"` is refused as an unexpected argument in the scratch clone), R6 (stale docs: this run fixed four lines naming deleted paths), R8 (`atelier check` does not look at whitespace), R9 (`config.toml` still runs `git diff --check` and `cargo fmt -- --check`, which CI also runs).
