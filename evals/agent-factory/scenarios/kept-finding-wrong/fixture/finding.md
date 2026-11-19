## Finding 1: slugify strips a leading hyphen that README.md says a slug keeps
Recommendation: fix now
README.md says a title that starts with punctuation keeps one leading hyphen in
its slug, so that such posts sort first. `slug.py` line 7 strips hyphens at
both ends (`strip`). Strip only the trailing ones (`rstrip`).
