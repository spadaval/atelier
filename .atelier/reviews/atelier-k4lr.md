---
schema: atelier.review
schema_version: 1
id: atelier-k4lr
title: 'Re-reviewed git diff b8814149 da839997 (code in 1a9d6b9c). ep6p fixed: review finding --recommendation and review disposition --decision parse to FindingDecision through named_values (PossibleValuesParser), evidence attach <TARGET_KIND> uses [issue, evidence], and evidence record/attach --role use [validates]. Help lists every one, and bad values exit 2; checked by running the real commands and by the new fixed_values test. The runtime role check it removes had no caller outside the clap-guarded CLI; bundles do not use it. Manual record now rejects a bad role before any write, as before. Losing the old --kind hint is acceptable: clap prints ''tip: a similar value exists: validates'' for --role validation, which corrects the mistake the transcripts recorded, and the error still names the field, the value and the accepted values, as human-cli-output.md Errors requires. xsiv fixed: exhaustive-match tests tie EvidenceResult::ALL and FindingDecision::ALL to their enums and round-trip them through FromStr and serde. Build has 0 warnings; 123 affected tests pass. Full suite, fmt and diff-check rest on atelier-xv48, atelier-e85j and atelier-tjn3. All findings are dispositioned.'
target: atelier-7w1g
identity: Independent reviewer (Claude subagent)
commit: da8399979894e3a204930628210d2e9eed2c4979
created_at: 2026-10-17T07:46:14.698653533Z
event:
  event: approval
---

Re-reviewed git diff b8814149 da839997 (code in 1a9d6b9c). ep6p fixed: review finding --recommendation and review disposition --decision parse to FindingDecision through named_values (PossibleValuesParser), evidence attach <TARGET_KIND> uses [issue, evidence], and evidence record/attach --role use [validates]. Help lists every one, and bad values exit 2; checked by running the real commands and by the new fixed_values test. The runtime role check it removes had no caller outside the clap-guarded CLI; bundles do not use it. Manual record now rejects a bad role before any write, as before. Losing the old --kind hint is acceptable: clap prints 'tip: a similar value exists: validates' for --role validation, which corrects the mistake the transcripts recorded, and the error still names the field, the value and the accepted values, as human-cli-output.md Errors requires. xsiv fixed: exhaustive-match tests tie EvidenceResult::ALL and FindingDecision::ALL to their enums and round-trip them through FromStr and serde. Build has 0 warnings; 123 affected tests pass. Full suite, fmt and diff-check rest on atelier-xv48, atelier-e85j and atelier-tjn3. All findings are dispositioned.
