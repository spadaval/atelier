#!/usr/bin/env bash
# A demo server tracked by the new binary, with Atelier's Git hooks installed
# and a local bare origin. main carries one unreviewed code commit, not
# pushed: the server reads its port from PORT, and the README says so. The
# pre-push hook refuses it, since a code commit reaches the base only in an
# Atelier merge, and offers ATELIER_SKIP. The config names no forge, so
# `atelier merge` pushes nothing itself.
set -euo pipefail
work=$1
here=$(cd "$(dirname "$0")" && pwd)
source "$here/../next-loop/lib.sh"
git init -q --bare -b main "$work/origin.git"
next_repo "$work" demo
git remote add origin ../origin.git
cat > server.py <<'PY'
import http.server
import socketserver

PORT = 8080

if __name__ == "__main__":
    with socketserver.TCPServer(("", PORT), http.server.SimpleHTTPRequestHandler) as httpd:
        httpd.serve_forever()
PY
cat > test_server.py <<'PY'
import unittest

import server


class ServerTest(unittest.TestCase):
    def test_port(self):
        self.assertIsInstance(server.PORT, int)


if __name__ == "__main__":
    unittest.main()
PY
printf '# Demo\n\nA tiny demo server.\n' > README.md
git add . && git commit -qm "A demo server, tracked in Atelier"
git push -q -u origin main
git rev-parse HEAD > "$work/.setup_head"
cp "$work/.setup_head" "$work/.origin_base"
atelier setup >/dev/null

cat > server.py <<'PY'
import http.server
import os
import socketserver

PORT = int(os.environ.get("PORT", "8080"))

if __name__ == "__main__":
    with socketserver.TCPServer(("", PORT), http.server.SimpleHTTPRequestHandler) as httpd:
        httpd.serve_forever()
PY
printf '# Demo\n\nA tiny demo server.\n\nRun `python3 server.py` to start it on port 8080, or set `PORT` to\nuse another. Stop it with Ctrl-C.\n' > README.md
git commit -qam "Read the server's port from PORT"
git rev-parse HEAD > "$work/.change_commit"
mkdir "$work/.hooks" && cp -p .git/hooks/pre-commit .git/hooks/pre-push .git/hooks/post-checkout "$work/.hooks/"
