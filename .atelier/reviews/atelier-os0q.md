---
schema: atelier.review
schema_version: 1
id: atelier-os0q
title: 'Reviewed 1d145e0d..HEAD (b338068c, 1f8465d9). link_mentions now labels a mentioned review or receipt with the first six words of its summary in curly quotes, escaped via escape(); the summary in the popover goes through inline(), and the ID shows only in the popover''s .m line, so atelier-ncba is resolved. Empty review summaries are refused by the CLI; receipt summaries are always ''result: title''; one-word and up-to-six-word summaries render without an ellipsis. The only code change is that block, plus the fixture check now counting every label as page text and a new test, record_mentions_are_named_by_what_they_say. Rebuilt and generated atelier-r6bt --html from the main checkout: with popovers, style and script removed, the page text has no atelier-xxxx ID, and the 4 record labels are summary openings. debrief tests 14/14 pass, cargo fmt --check is clean, and git diff --check reports only the trailing blank line that every receipt has. Three DEFER findings: atelier-3jh7, atelier-g0oy, atelier-2boy.'
target: atelier-nexd
identity: Independent reviewer 3 (Claude subagent)
commit: 1f8465d99fbeaa96241d5b0f1031bf6259a43aa9
created_at: 2026-10-17T22:12:57.312526775Z
event:
  event: approval
---

Reviewed 1d145e0d..HEAD (b338068c, 1f8465d9). link_mentions now labels a mentioned review or receipt with the first six words of its summary in curly quotes, escaped via escape(); the summary in the popover goes through inline(), and the ID shows only in the popover's .m line, so atelier-ncba is resolved. Empty review summaries are refused by the CLI; receipt summaries are always 'result: title'; one-word and up-to-six-word summaries render without an ellipsis. The only code change is that block, plus the fixture check now counting every label as page text and a new test, record_mentions_are_named_by_what_they_say. Rebuilt and generated atelier-r6bt --html from the main checkout: with popovers, style and script removed, the page text has no atelier-xxxx ID, and the 4 record labels are summary openings. debrief tests 14/14 pass, cargo fmt --check is clean, and git diff --check reports only the trailing blank line that every receipt has. Three DEFER findings: atelier-3jh7, atelier-g0oy, atelier-2boy.
