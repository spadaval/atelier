#!/usr/bin/env bash
# next-loop's repository one round of review later: the harness's first
# thread (T3CODE_THREAD_ID) started slug-k3fa and submitted a change that
# trims hyphens only at the end, and another thread's round-1 review asks,
# in one fix-now finding, for the leading ones too. The real binary drives
# every step, so the records are what it writes. NEXT_ATELIER names it, and
# next-loop's setup leaves the grader main's first commit in .setup_head.
set -euo pipefail
work=$1
: "${T3CODE_THREAD_ID:?set T3CODE_THREAD_ID to the thread the agent runs as}"
bash "$(dirname "$0")/../next-loop/setup.sh" "$work"
impl=$T3CODE_THREAD_ID
unset ATELIER_SKIP CLAUDE_CODE_SESSION_ID CODEX_SESSION_ID
export PATH="$work/bin:$PATH"
cd "$work/repo"
as() { T3CODE_THREAD_ID=$1 atelier "${@:2}" >/dev/null; }

as "$impl" start slug-k3fa
cat > slug.py <<'PY'
"""Turn titles into URL slugs."""
import re


def slugify(title: str) -> str:
    """Lowercase `title`, replace each run of other characters with one hyphen, and trim hyphens."""
    return re.sub(r"[^a-z0-9]+", "-", title.lower()).rstrip("-")
PY
cat > test_slug.py <<'PY'
import unittest

from slug import slugify


class SlugifyTest(unittest.TestCase):
    def test_lowercases_and_hyphenates(self):
        self.assertEqual(slugify("Hello World"), "hello-world")

    def test_collapses_runs_of_punctuation(self):
        self.assertEqual(slugify("Hello,  World!"), "hello-world")


if __name__ == "__main__":
    unittest.main()
PY
git commit -qam "Collapse runs of punctuation in slugs, and trim hyphens"
item=.atelier/items/slug-k3fa/item.md
sed -i 's/^- \[ \]/- [x]/' "$item"
cat >> "$item" <<'MD'

`slugify` replaces each run of characters other than letters and digits with
one hyphen, and trims hyphens. `test_slug.py` tests a run of punctuation, and
`python3 -m unittest` passes.
MD
as "$impl" submit

# Round 1, by a thread that is neither the agent nor any thread it starts.
as thread-5e1f0c2a review slug-k3fa
review=$(ls .atelier/items/slug-k3fa/reviews/1-*.md)
python3 - "$review" <<'PY'
import sys
path = sys.argv[1]
text = open(path).read().replace(
    "<!-- What you checked and how, and what you could not check. -->",
    "I read the diff, ran `python3 -m unittest` (two tests, both pass) and tried "
    "both Done when examples in Python.")
text += """
## Finding 1: slugify keeps leading hyphens, so the second Done when example fails
Recommendation: fix now
`slug.py` line 7 trims hyphens only at the end (`rstrip`), so
`slugify("  --Rust & Go--  ")` returns `-rust-go`, not `rust-go`. `test_slug.py`
has no test for that example, though the second Done when line asks for one and
is ticked.
"""
open(path, "w").write(text)
PY
as thread-5e1f0c2a verdict changes --model gpt-6.1-sol
