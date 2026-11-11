#!/usr/bin/env bash
# next-loop's repository with a blog's URL builder beside slugify. Thread
# thread-1a2b3c4d implemented slug-k3fa, and round 1's reviewer, another
# thread, raised one fix-now finding that is right: a title of punctuation
# alone gives an empty slug, and urls.py then builds a broken link. Its fix is
# in urls.py, outside the item's Scope, so the implementer escalated it with
# `escalate --finding`, and committed the question. The agent is started with
# the task line `escalate` printed, in the implementer's checkout on the
# item's branch. The review file is renamed to a fixed suffix so prompt.md
# can quote the task line as the binary prints it.
set -euo pipefail
work=$1
here=$(cd "$(dirname "$0")" && pwd)
bash "$here/../next-loop/setup.sh" "$work"
source "$here/../next-loop/lib.sh"
export PATH="$work/bin:$PATH"
cd "$work/repo"
id=slug-k3fa
dir=.atelier/items/$id
impl=thread-1a2b3c4d rev=thread-5e1f0c2a

cat > urls.py <<'PY'
"""URLs of blog posts."""
from slug import slugify


def post_url(title: str) -> str:
    """The path of the post with this title."""
    return f"/posts/{slugify(title)}/"
PY
cat > test_urls.py <<'PY'
import unittest

from urls import post_url


class PostUrlTest(unittest.TestCase):
    def test_path(self):
        self.assertEqual(post_url("Hello World"), "/posts/hello-world/")


if __name__ == "__main__":
    unittest.main()
PY
git add urls.py test_urls.py && git commit -qm "Build post URLs from slugs"
git rev-parse HEAD > "$work/.base_head"

as $impl start $id
python3 - <<'PY'
from pathlib import Path
Path("slug.py").write_text(Path("slug.py").read_text().replace(
    'return re.sub(r"[^a-z0-9]", "-", title.lower())',
    'return re.sub(r"[^a-z0-9]+", "-", title.lower()).strip("-")'))
tests = Path("test_slug.py")
tests.write_text(tests.read_text().replace("\n\nif __name__", '''
    def test_collapses_runs_of_punctuation(self):
        self.assertEqual(slugify("Hello,  World!"), "hello-world")

    def test_trims_hyphens_at_both_ends(self):
        self.assertEqual(slugify("  --Rust & Go--  "), "rust-go")


if __name__'''))
PY
git commit -qam "Collapse runs of punctuation in slugs, and trim hyphens"
sed -i 's/^- \[ \]/- [x]/' $dir/item.md
cat >> $dir/item.md <<'MD'

`slugify` replaces each run of characters other than letters and digits with
one hyphen and trims hyphens at both ends. `test_slug.py` tests both Done when
examples, and `python3 -m unittest` passes.
MD
as $impl submit

as $rev review $id
mv $dir/reviews/1-*.md $dir/reviews/1-d3c1.md
python3 - $dir/reviews/1-d3c1.md <<'PY'
import sys
path = sys.argv[1]
text = open(path).read().replace(
    "<!-- What you checked and how, and what you could not check. -->",
    "I read the diff, ran `python3 -m unittest` (four tests pass) and tried both Done "
    "when examples and some titles of punctuation alone.")
text += """
## Finding 1: a title of punctuation alone now gives an empty slug and a broken post URL
Recommendation: fix now
`slugify("!!!")` now returns `""` (it returned `---` before), so `urls.post_url("!!!")`
returns `/posts//`, a link that 404s. Any post titled with punctuation alone, such as
"?!", gets it. `urls.py` should fall back to something non-empty.
"""
open(path, "w").write(text)
PY
as $rev verdict changes --model gpt-6.1-sol

as $impl escalate --finding 1-d3c1.1 "Finding 1-d3c1.1 is right that post_url builds /posts// for a title of punctuation alone, but its fix is in urls.py, outside this item's Scope (slug.py and test_slug.py), and no post has such a title today; the reviewer wants it fixed now, and I propose deferring it to an item of its own."
git commit -qam "Ask whether to defer 1-d3c1.1"

# Grader inputs, outside the repository.
git rev-parse HEAD > "$work/.branch_head"
cp $dir/item.md "$work/.item_before"
