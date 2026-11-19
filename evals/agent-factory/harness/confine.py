#!/usr/bin/env python3
"""Confine an agent run so it cannot reach what the case keeps from it.

A replay case (harness/replay.py) reviews a change from this repository's
history whose defect a later commit fixed. The repository is public, this
host holds other checkouts of it, and the case files name the fix, so a
confined run:

- sees, of the filesystem, only the system (/usr, /etc, /sys), its own work
  directory, the agent's own program, the toolchain (~/.cargo/bin and
  ~/.rustup, read-only), a cargo cache of crates.io sources, and its sign-in;
  every other path under /home, /var, /tmp, /run, /srv, /opt and /mnt is
  absent, other runs' work directories included;
- has no network but a proxy that opens tunnels only to its agent's model
  API (HOSTS), so no command can fetch from GitHub or anywhere else;
- runs without the agent's own web search and fetch tools, which reach the
  web from the model's side: run.py leaves them out (WEB_TOOLS).

It runs under bubblewrap (`bwrap`). The agent's command runs inside the
sandbox behind `confine.py forward`, which listens on 127.0.0.1:PORT in the
sandbox's own network namespace and passes each connection to the proxy's
Unix socket outside it, where Proxy allows a CONNECT to an allowed host on
port 443 and refuses everything else. Each request is logged to the run's
.proxy.jsonl.

Claude Code signs in from ~/.claude/.credentials.json. That file is bound at
its own path, read-write, so a token refresh lands where the host reads it;
every other entry of ~/.claude (session transcripts, history, settings) is
hidden. A Codex run's scratch CODEX_HOME links its auth.json to
~/.codex/auth.json, bound the same way.

harness/test_confine.py runs commands under the same sandbox and shows that
reading another checkout, reading a case file and fetching this repository
fail, and that reading the run's own repository and reaching an allowed host
work.
"""
import json
import os
import selectors
import shutil
import socket
import socketserver
import subprocess
import sys
import tempfile
import threading
import time
from pathlib import Path

PORT = 3128
# The hosts each agent's own process needs: the model API and its sign-in's
# token refresh. Nothing else is reachable from a confined run.
HOSTS = {
    "claude": ("api.anthropic.com", "console.anthropic.com", "platform.claude.com"),
    "codex": ("chatgpt.com", "auth.openai.com", "api.openai.com"),
}
# Claude Code's tools that reach the web from the model's side, which no
# proxy sees; a confined Claude Code run is not offered them.
WEB_TOOLS = ("WebSearch", "WebFetch")
HOME = Path.home()
CLAUDE_CREDENTIALS = HOME / ".claude" / ".credentials.json"
CODEX_AUTH = HOME / ".codex" / "auth.json"
TOOLCHAIN = (HOME / ".cargo" / "bin", HOME / ".rustup")
SYSTEM = ("/usr", "/etc", "/sys")
MERGED = ("/bin", "/sbin", "/lib", "/lib32", "/lib64", "/libx32")


def available():
    """Why this host cannot confine a run, or None."""
    if not shutil.which("bwrap"):
        return "bwrap (bubblewrap) is not installed"
    probe = subprocess.run(["bwrap", "--ro-bind", "/", "/", "--unshare-net", "--dev", "/dev",
                            "--proc", "/proc", "true"], capture_output=True, text=True)
    if probe.returncode != 0:
        return f"bwrap cannot make a sandbox here: {probe.stderr.strip()[-300:]}"
    return None


def pump(a: socket.socket, b: socket.socket) -> None:
    """Copy bytes both ways until either side closes."""
    sel = selectors.DefaultSelector()
    sel.register(a, selectors.EVENT_READ, b)
    sel.register(b, selectors.EVENT_READ, a)
    try:
        while True:
            for key, _ in sel.select():
                data = key.fileobj.recv(65536)
                if not data:
                    return
                key.data.sendall(data)
    except OSError:
        return
    finally:
        sel.close()
        for s in (a, b):
            try:
                s.close()
            except OSError:
                pass


def read_head(conn: socket.socket) -> bytes:
    """An HTTP request's head, up to its blank line, or what came before EOF."""
    head = b""
    while b"\r\n\r\n" not in head and len(head) < 16384:
        chunk = conn.recv(4096)
        if not chunk:
            break
        head += chunk
    return head


class Proxy:
    """A CONNECT proxy on a Unix socket that tunnels only to `hosts` on 443."""

    def __init__(self, path: Path, hosts, log: Path):
        self.path, self.hosts, self.log = Path(path), set(hosts), Path(log)
        self.lock = threading.Lock()
        proxy = self

        class Handler(socketserver.BaseRequestHandler):
            def handle(self):
                proxy.serve(self.request)

        self.server = socketserver.ThreadingUnixStreamServer(str(self.path), Handler)
        self.server.daemon_threads = True
        self.thread = threading.Thread(target=self.server.serve_forever, daemon=True)

    def __enter__(self):
        self.thread.start()
        return self

    def __exit__(self, *exc):
        self.server.shutdown()
        self.server.server_close()
        self.path.unlink(missing_ok=True)

    def record(self, target: str, allowed: bool) -> None:
        with self.lock, self.log.open("a") as f:
            f.write(json.dumps({"time": time.time(), "target": target, "allowed": allowed}) + "\n")

    def serve(self, conn: socket.socket) -> None:
        line = read_head(conn).split(b"\r\n", 1)[0].decode("latin-1")
        parts = line.split()
        target = parts[1] if len(parts) == 3 else line[:200]
        host, _, port = target.rpartition(":")
        # A host is allowed on 443; a test names a local one with its port.
        allowed = len(parts) == 3 and parts[0] == "CONNECT" and (
            (port == "443" and host in self.hosts) or target in self.hosts)
        self.record(target, allowed)
        if not allowed:
            conn.sendall(b"HTTP/1.1 403 Forbidden\r\nContent-Length: 0\r\n\r\n")
            conn.close()
            return
        try:
            upstream = socket.create_connection((host, int(port)), timeout=30)
            upstream.settimeout(None)
        except OSError:
            conn.sendall(b"HTTP/1.1 502 Bad Gateway\r\nContent-Length: 0\r\n\r\n")
            conn.close()
            return
        conn.sendall(b"HTTP/1.1 200 Connection established\r\n\r\n")
        pump(conn, upstream)


def forward(sock: str, port: int, cmd: list) -> int:
    """Inside the sandbox: listen on 127.0.0.1:port, pass each connection to
    the proxy's socket, and run `cmd`; return its exit code."""
    listener = socket.create_server(("127.0.0.1", port))

    def accept():
        while True:
            conn, _ = listener.accept()
            upstream = socket.socket(socket.AF_UNIX)
            try:
                upstream.connect(sock)
            except OSError:
                conn.close()
                continue
            threading.Thread(target=pump, args=(conn, upstream), daemon=True).start()

    threading.Thread(target=accept, daemon=True).start()
    return subprocess.call(cmd)


def program_binds(cmd: list, env: dict) -> list:
    """The agent's program, resolved on the run's PATH, as the directory to
    bind read-only: its own directory (Claude Code's versions/), or for a
    Codex release, releases/<version>/bin/codex, the release, which holds its
    resources beside bin/. Nothing wider: a program in ~/.local/bin binds
    that directory, not ~/.local."""
    found = shutil.which(cmd[0], path=env.get("PATH"))
    if not found:
        return []
    real = Path(found).resolve()
    release = real.parent.name == "bin" and real.parent.parent.parent.name == "releases"
    return [real.parent.parent if release else real.parent]


def claude_masks() -> list:
    """bwrap arguments that bind ~/.claude and hide each entry in it but the
    sign-in, so a refreshed token is written where the host reads it."""
    home = CLAUDE_CREDENTIALS.parent
    if not home.is_dir():
        return []
    args = ["--bind", str(home), str(home)]
    for entry in sorted(home.iterdir()):
        if entry == CLAUDE_CREDENTIALS:
            continue
        if entry.is_dir() and not entry.is_symlink():
            args += ["--tmpfs", str(entry)]
        else:
            args += ["--ro-bind", "/dev/null", str(entry)]
    return args


def bwrap_args(work: Path, agent: str, binds: list, sock: Path, cargo_home: Path = None) -> list:
    """The bwrap command line, up to the command it runs."""
    args = ["bwrap", "--die-with-parent", "--unshare-all", "--hostname", "replay",
            "--proc", "/proc", "--dev", "/dev", "--tmpfs", "/tmp", "--tmpfs", "/run",
            "--tmpfs", "/var", "--tmpfs", "/home"]
    for path in SYSTEM:
        args += ["--ro-bind", path, path]
    for path in MERGED:
        p = Path(path)
        if p.is_symlink():
            args += ["--symlink", os.readlink(p), path]
        elif p.is_dir():
            args += ["--ro-bind", path, path]
    args += ["--dir", str(HOME)]
    for path in [*TOOLCHAIN, *binds]:
        if Path(path).exists():
            args += ["--ro-bind", str(path), str(path)]
    if agent == "claude":
        args += claude_masks()
    elif CODEX_AUTH.exists():
        args += ["--bind", str(CODEX_AUTH), str(CODEX_AUTH)]
    if cargo_home:
        args += ["--bind", str(cargo_home), str(cargo_home)]
    args += ["--bind", str(work), str(work), "--bind", str(sock), "/run/proxy.sock",
             "--chdir", str(work / "repo")]
    return args


def environment(env: dict, work: Path, cargo_home: Path = None) -> dict:
    """The run's environment inside the sandbox: the proxy for every
    process, the toolchain on PATH, and cargo offline on its cache."""
    proxy = f"http://127.0.0.1:{PORT}"
    path = [p for p in env.get("PATH", "").split(":")
            if p.startswith(str(work)) or p.startswith("/usr") or p in ("/bin", "/sbin")]
    out = {**env, "PATH": ":".join([*[p for p in path if p.startswith(str(work))],
                                    str(TOOLCHAIN[0]), "/usr/local/bin", "/usr/bin", "/bin"]),
           "HTTPS_PROXY": proxy, "HTTP_PROXY": proxy, "https_proxy": proxy,
           "http_proxy": proxy, "ALL_PROXY": proxy, "NO_PROXY": "127.0.0.1,localhost",
           "no_proxy": "127.0.0.1,localhost", "RUSTUP_HOME": str(TOOLCHAIN[1]),
           "CARGO_BUILD_JOBS": "4", "CLAUDE_CODE_DISABLE_NONESSENTIAL_TRAFFIC": "1"}
    if cargo_home:
        out.update({"CARGO_HOME": str(cargo_home), "CARGO_NET_OFFLINE": "true"})
    return out


class Sandbox:
    """A confined run's proxy and the command and environment that run an
    agent's command inside its sandbox. Use as a context manager around the
    run, so the proxy lives exactly as long as the run."""

    def __init__(self, work: Path, agent: str, cargo_home: Path = None, hosts=None):
        self.work, self.agent, self.cargo_home = Path(work), agent, cargo_home
        self.hosts = HOSTS[agent] if hosts is None else hosts
        # A Unix socket's path is limited to 108 bytes, so it lives in a
        # short directory of its own and is bound into the sandbox.
        self.dir = Path(tempfile.mkdtemp(prefix="afp-"))
        self.sock = self.dir / "s"
        self.proxy = Proxy(self.sock, self.hosts, self.work / ".proxy.jsonl")

    def __enter__(self):
        self.proxy.__enter__()
        return self

    def __exit__(self, *exc):
        self.proxy.__exit__(*exc)
        shutil.rmtree(self.dir, ignore_errors=True)

    def wrap(self, cmd: list, env: dict) -> tuple:
        """(argv, env) that run `cmd` in the sandbox, behind the forwarder,
        which is copied into the work directory so the sandbox can run it."""
        forwarder = self.work / "libexec" / "confine.py"
        forwarder.parent.mkdir(exist_ok=True)
        shutil.copy(__file__, forwarder)
        inner = environment(env, self.work, self.cargo_home)
        program = shutil.which(cmd[0], path=env.get("PATH")) or cmd[0]
        if not Path(program).resolve().is_relative_to(self.work):
            program = str(Path(program).resolve())
        argv = [*bwrap_args(self.work, self.agent, program_binds(cmd, env), self.sock,
                            self.cargo_home),
                "/usr/bin/python3", "-I", str(forwarder), "forward", "/run/proxy.sock",
                str(PORT), "--", program, *cmd[1:]]
        return argv, inner


def refused(work: Path) -> list:
    """Each target the run's proxy refused, from its log."""
    log = Path(work) / ".proxy.jsonl"
    rows = [json.loads(line) for line in (log.read_text().splitlines() if log.exists() else [])
            if line.strip()]
    return sorted({r["target"] for r in rows if not r["allowed"]})


def main() -> int:
    if len(sys.argv) > 4 and sys.argv[1] == "forward" and sys.argv[4] == "--":
        return forward(sys.argv[2], int(sys.argv[3]), sys.argv[5:])
    sys.exit("usage: confine.py forward <socket> <port> -- <command>...")


if __name__ == "__main__":
    sys.exit(main())
