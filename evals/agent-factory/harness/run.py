#!/usr/bin/env python3
"""Run Agent Factory scenarios against Claude Code or Codex and grade the result.

Each scenario directory holds:

- scenario.json: {"description": ..., "timeout_sec": ..., "max_budget_usd": ...}
- prompt.md: the task. "{{skill}}" is replaced by an instruction to use the
  skill in the skill arm and removed in the none arm.
- setup.sh: called with a fresh work directory; must create work/repo (the
  agent's working directory) and may create work/bin (prepended to PATH).
- grade.py: called with the work directory and the transcript path; prints
  one JSON object {"pass": bool, "checks": {...}, "notes": [...]}.

A scenario with "threads": true in scenario.json runs each agent as a thread,
as T3 Code does: it gets its own T3CODE_THREAD_ID, which setup.sh is given
too, so a fixture can leave work that thread already holds. Under Claude
Code, work/bin/delegate (harness/delegate.py) starts another agent of the
same kind and model as a new thread in the caller's directory, waits for it
and prints its final reply; the system prompt describes it in THREADS_NOTE,
and nothing else. Under Codex a thread starts a subagent with its own
spawn_agent, which the tool counts as a thread of its own. Each such
thread's transcript goes to threads/ in the run directory: delegate writes
a Claude Code thread's, and the harness writes a Codex subagent's from its
session record, in the form `codex exec --json` prints.

--agent codex runs `codex exec` instead of Claude Code, with HOME and
CODEX_HOME in the work directory, so the host's ~/.codex instructions and
~/.agents/skills are not loaded; the host's Codex sign-in is linked in, never
copied, and the link is removed when the run ends. Its scratch config.toml
(codex_config) turns off plugins and apps, which would otherwise download the
account's own skills and connectors.

Arms: "none" runs with no skills, "skill" stages --skill-dir (default
skills/agent-factory) from a git ref of this repository, without maintainers/,
which describes the expected behaviour, under the name agent-factory: into
the workspace's .claude/skills for Claude Code, and into the scratch
CODEX_HOME's skills/ for Codex, where Codex 0.167 loads a user's skills. Runs
use no MCP servers and an allowlisted environment; Claude Code runs use
--setting-sources project. Each run's init event, and for Codex its session
record ($CODEX_HOME/sessions), is checked: a run that loaded MCP servers or
skills from anywhere else, had the skill in the wrong arm, was offered a way
to start an agent the harness has not bounded, exited non-zero, timed out or
produced no result is recorded with "pass": null and an "invalid" reason
instead of a grade.

The agent limit. At most two eval agents are alive at once, and an agent a
run starts counts. A scenario without threads is one agent: Claude Code's
agent-starting tools (AGENT_TOOLS) are disallowed on its command line, and
Codex's spawn_agent is removed in its scratch config. Such scenarios run two
at a time, and --jobs above 2 is refused. A scenario with threads runs alone
(--jobs above 1 is refused when one is in the matrix) and has at most two
agents alive: its first agent and one child. Under Claude Code the only way to
start one is `delegate`, which holds a lock while its child runs, so a second
`delegate` waits for the first child to end, and which a child may not call.
When the first agent ends, any child still running is killed. Under Codex the
only way is spawn_agent, capped by agents.max_threads = 1, which counts every
live subagent of the session, grandchildren included, so a child can start
none while it lives; `delegate` is not installed. Each run records when each
of its agents started and ended (a Codex subagent from its own session
record), and the matrix prints the most agents alive at once.

Claude Code loads instruction files (CLAUDE.md, AGENTS.md, .claude/CLAUDE.md,
.claude/rules/ and the like) from every directory above its working directory,
whatever --setting-sources says, so a run under $HOME sees ~/.claude/CLAUDE.md.
Each run's work directory is therefore made under --work-root, which must lie
outside $HOME (default /var/tmp/af-evals); transcripts and results still go to
--out. As a backstop, every run passes --settings with claudeMdExcludes naming
each instruction file path in the directories above its working directory.
Before a matrix, a canary run in the same setup on a cheap model lists the
instruction files it was given, and the harness aborts unless the canary
repository's own AGENTS.md is the only one, and its own record shows no
agent-starting tool and no foreign skill.

The agent still runs as the user with permissions skipped and full access to
$HOME and the network; fixtures use local remotes, but nothing confines it.
"""

import argparse
import concurrent.futures
import datetime
import json
import os
import re
import secrets
import shlex
import shutil
import signal
import subprocess
import sys
import time
from pathlib import Path

REPO = Path(__file__).resolve().parents[3]
SCENARIOS = Path(__file__).resolve().parents[1] / "scenarios"
DEFAULT_OUT = Path.home() / ".cache" / "af-evals" / "runs"
DEFAULT_WORK_ROOT = Path("/var/tmp/af-evals")
DEFAULT_SKILL_DIR = "skills/agent-factory"
SKILL_LINE = "Use the agent-factory skill for this task.\n\n"
# Passed through to the agent; everything else in the parent environment
# (including a parent Claude Code or T3 session's sockets and tokens) is dropped.
ENV_KEEP = ("HOME", "USER", "LOGNAME", "LANG", "LC_ALL", "SHELL", "PATH")
NO_MCP = ["--strict-mcp-config", "--mcp-config", '{"mcpServers":{}}']
# What Claude Code 2.1.318 loads as instructions from each directory above its
# working directory (.claude/rules is a directory of rule files). AGENTS.md
# loads where no CLAUDE.md does, so hiding a CLAUDE.md can expose one.
INSTRUCTION_NAMES = ("CLAUDE.md", "CLAUDE.local.md", "AGENTS.md",
                     ".claude/CLAUDE.md", ".claude/AGENTS.md", ".claude/rules/**")
# Claude Code 2.1.322's tools that start or schedule another agent run. They
# are disallowed in every run, so `delegate` is the only way to start one.
AGENT_TOOLS = ("Agent", "Task", "Workflow", "RemoteTrigger", "CronCreate",
               "ScheduleWakeup")
# Codex skill roots, under CODEX_HOME, a run may load from: the staged skill
# and Codex's bundled system skills. Any other (an account's plugin) is foreign.
CODEX_SKILL_ROOTS = ("skills", "skills/.system")
CANARY_MODEL = {"claude": "haiku", "codex": "gpt-6-luna"}
THREADS_NOTE = (
    "Agents here run as threads, each with its own thread ID in "
    "T3CODE_THREAD_ID. To start another agent as a new thread, run "
    "`delegate \"<task>\"` in a shell: it starts a fresh agent of your kind in "
    "the current directory with that task as its only prompt, waits for it to "
    "finish, and prints its final reply."
)
# Codex starts threads with its own spawn_agent, so it is not told of delegate.
CODEX_THREADS_NOTE = ("Agents here run as threads, each with its own thread ID "
                      "in T3CODE_THREAD_ID.")
CODEX_AUTH = Path.home() / ".codex" / "auth.json"
CANARY_PROMPT = (
    "This is an automated isolation check. Do not use any tools. List every "
    "instruction or memory file (such as CLAUDE.md, CLAUDE.local.md, AGENTS.md "
    "or a rules file) whose contents were loaded into your context, one per "
    "line, as its absolute path followed by ' | ' and the first Markdown "
    "heading in it. If no such file was loaded, reply NONE."
)


def inside(path: Path, root: Path) -> bool:
    path, root = path.resolve(), root.resolve()
    return path == root or root in path.parents


def instruction_excludes(cwd: Path) -> list:
    """claudeMdExcludes naming every instruction file path above cwd."""
    dirs = []
    for d in [*cwd.parents, *cwd.resolve().parents]:
        if d not in dirs:
            dirs.append(d)
    return [str(d / name) for d in dirs for name in INSTRUCTION_NAMES]


def claude_command(prompt: str, model: str, effort: str, budget, cwd: Path) -> list:
    settings = {"claudeMdExcludes": instruction_excludes(cwd)}
    return [
        "claude", "-p", prompt,
        "--model", model, "--effort", effort,
        "--setting-sources", "project",
        "--settings", json.dumps(settings),
        "--output-format", "stream-json", "--verbose",
        "--dangerously-skip-permissions",
        "--no-session-persistence",
        "--max-budget-usd", str(budget),
        "--disallowedTools", ",".join(AGENT_TOOLS),
        *NO_MCP,
    ]


def codex_command(prompt: str, model: str, system=None) -> list:
    """codex exec at Codex's default effort, in the directory it is run in."""
    extra = ["-c", f"developer_instructions={json.dumps(system)}"] if system else []
    return ["codex", "exec", "--json", "--dangerously-bypass-approvals-and-sandbox",
            "--skip-git-repo-check", "-m", model, *extra, prompt]


def agent_command(agent: str, prompt: str, model: str, effort: str, budget,
                  cwd: Path, system=None) -> list:
    if agent == "codex":
        return codex_command(prompt, model, system)
    cmd = claude_command(prompt, model, effort, budget, cwd)
    return cmd + (["--append-system-prompt", system] if system else [])


def codex_config(threads: bool) -> str:
    """The scratch config.toml. Codex 0.167 runs gpt-6 models on multi-agent
    v2 whatever features.multi_agent says, and there disable_native_tools
    removes spawn_agent and the other collaboration tools. agents.max_threads
    caps live subagents across the session, so with 1 a child can start no
    grandchild; max_depth is v1's, set for a model on v1."""
    return (
        "[features]\nplugins = false\nremote_plugin = false\napps = false\n\n"
        "[features.multi_agent_v2]\n"
        f"disable_native_tools = {'false' if threads else 'true'}\n\n"
        "[agents]\nmax_threads = 1\nmax_depth = 1\n")


def codex_home(work: Path, threads: bool = False) -> Path:
    """A scratch CODEX_HOME holding its config and a link to the host's sign-in."""
    home = work / "home" / ".codex"
    home.mkdir(parents=True, exist_ok=True)
    (home / "config.toml").write_text(codex_config(threads))
    link = home / "auth.json"
    if not link.is_symlink():
        link.symlink_to(CODEX_AUTH)
    return home


def unlink_auth(work: Path) -> None:
    """Remove the sign-in from the scratch home, whether link or file."""
    (work / "home" / ".codex" / "auth.json").unlink(missing_ok=True)


def agent_env(work: Path, agent: str = "claude", threads: bool = False) -> dict:
    env = {k: os.environ[k] for k in ENV_KEEP if k in os.environ}
    if agent == "codex":
        env["CODEX_HOME"] = str(codex_home(work, threads))
        env["HOME"] = str(work / "home")
    env["TERM"] = "dumb"
    env["ENABLE_CLAUDEAI_MCP_SERVERS"] = "false"
    # atelier's update notice names the host's own skill checkout and its age,
    # which is not the skill a run stages.
    env["ATELIER_NO_UPDATE_CHECK"] = "1"
    if (work / "bin").is_dir():
        env["PATH"] = f"{work / 'bin'}:{env['PATH']}"
    env["TMPDIR"] = str(work / "tmp")
    (work / "tmp").mkdir(exist_ok=True)
    return env


def run_claude(cmd: list, cwd: Path, env: dict, transcript: Path, timeout):
    """Run the agent into transcript; return its exit code or "timeout"."""
    with transcript.open("w") as out:
        # Own process group, so servers the agent starts die with the run.
        proc = subprocess.Popen(cmd, cwd=cwd, env=env, stdin=subprocess.DEVNULL,
                                stdout=out, stderr=subprocess.STDOUT,
                                start_new_session=True)
        try:
            exit_code = proc.wait(timeout=timeout)
        except subprocess.TimeoutExpired:
            exit_code = "timeout"
        try:
            os.killpg(proc.pid, signal.SIGKILL)
        except ProcessLookupError:
            pass
        proc.wait()
    return exit_code


def stage_skill(ref: str, skills: Path, skill_dir: str = DEFAULT_SKILL_DIR) -> str:
    """Install skill_dir at ref as skills/agent-factory, without maintainers/."""
    skill_dir = skill_dir.strip("/")
    sha = subprocess.run(
        ["git", "-C", str(REPO), "rev-parse", "--short", ref],
        check=True, capture_output=True, text=True,
    ).stdout.strip()
    dest = skills / "agent-factory"
    dest.mkdir(parents=True, exist_ok=True)
    archive = subprocess.run(
        ["git", "-C", str(REPO), "archive", ref, skill_dir],
        check=True, capture_output=True,
    ).stdout
    subprocess.run(
        ["tar", "-x", f"--strip-components={skill_dir.count('/') + 1}", "-C", str(dest),
         f"--exclude={skill_dir}/maintainers"],
        input=archive, check=True,
    )
    return sha


def stage_for(agent: str, ref: str, work: Path, skill_dir: str) -> str:
    """Stage the skill where `agent` loads a user's skills from."""
    if agent == "codex":
        return stage_skill(ref, work / "home" / ".codex" / "skills", skill_dir)
    repo_dir = work / "repo"
    sha = stage_skill(ref, repo_dir / ".claude" / "skills", skill_dir)
    exclude = repo_dir / ".git" / "info" / "exclude"
    if exclude.parent.exists():
        with exclude.open("a") as f:
            f.write("\n.claude/\n")
    return sha


def read_transcript(transcript: Path) -> dict:
    """The init event's isolation facts and the final result, if any. A
    Codex run's result is its last reply once its turn completed; what it
    loaded is in its session record (codex_session), not in this stream."""
    info = {"init": None, "result": None}
    reply = None
    for line in transcript.read_text().splitlines():
        try:
            event = json.loads(line)
        except json.JSONDecodeError:
            continue
        if event.get("type") == "thread.started" and info["init"] is None:
            info["init"] = {"thread_id": event.get("thread_id"), "mcp_servers": [],
                            "agent_factory": False}
        item = event.get("item") or {}
        if event.get("type") == "item.completed" and item.get("type") == "agent_message":
            reply = item.get("text")
        if event.get("type") == "turn.completed":
            info["result"] = {"subtype": "success", "result": reply,
                              "usage": event.get("usage")}
        if event.get("type") in ("turn.failed", "error"):
            info["result"] = {"subtype": "error", "is_error": True, "result": reply,
                              "error": event}
        if event.get("type") == "system" and event.get("subtype") == "init" \
                and info["init"] is None:
            info["init"] = {
                "cwd": event.get("cwd"),
                "model": event.get("model"),
                "version": event.get("claude_code_version"),
                "mcp_servers": [m.get("name") for m in event.get("mcp_servers") or []],
                "agent_factory": any("agent-factory" in str(x)
                                     for x in event.get("skills") or []),
                "agent_tools": [t for t in event.get("tools") or [] if t in AGENT_TOOLS],
            }
        if event.get("type") == "result":
            info["result"] = event
    return info


def rollouts(home: Path) -> list:
    """Each session record under a CODEX_HOME, as (meta, events, path)."""
    found = []
    for path in sorted((home / "sessions").glob("**/rollout-*.jsonl")):
        events = []
        for line in path.read_text().splitlines():
            try:
                events.append(json.loads(line))
            except json.JSONDecodeError:
                continue
        meta = next((e.get("payload") or {} for e in events
                     if e.get("type") == "session_meta"), {})
        found.append((meta, events, path))
    return found


def codex_given(items: list, home: Path) -> dict:
    """What Codex gave a session, from the items of its model input (a
    session record's payloads, or `codex debug prompt-input`): each skill
    and the root it loaded from, and whether spawn_agent was offered."""
    facts = {"skills": [], "foreign_skills": [], "agent_factory": False,
             "spawn_agent": False}
    allowed = {str(home / r) for r in CODEX_SKILL_ROOTS}
    for item in items:
        if item.get("type") == "additional_tools":
            facts["spawn_agent"] |= '"spawn_agent"' in json.dumps(item.get("tools"))
        if item.get("type") != "message":
            continue
        for c in item.get("content") or []:
            text = (c.get("text") or "") if isinstance(c, dict) else ""
            if "<skills_instructions>" not in text:
                continue
            roots = dict(re.findall(r"^- `(r\d+)` = `([^`]+)`", text, re.M))
            for name, file in re.findall(r"^- ([\w:.-]+): .*\(file: ([^)]+)\)\s*$",
                                         text, re.M):
                alias = file.split("/", 1)[0]
                root = roots.get(alias, os.path.dirname(os.path.dirname(file)))
                facts["skills"].append(name)
                if name == "agent-factory" and root == str(home / "skills"):
                    facts["agent_factory"] = True
                if root not in allowed:
                    facts["foreign_skills"].append(f"{name} ({root})")
    facts["skills"] = sorted(set(facts["skills"]))
    facts["foreign_skills"] = sorted(set(facts["foreign_skills"]))
    return facts


def codex_session(home: Path) -> dict:
    """What the root session's own record says it was given (codex_given)."""
    for meta, events, _ in rollouts(home):
        if not meta.get("parent_thread_id"):
            return codex_given([e.get("payload") or {} for e in events], home)
    return codex_given([], home)


def codex_thread_events(events: list) -> list:
    """A subagent's session record as the events `codex exec --json` prints
    for its own commands and replies, which the graders read."""
    out = []
    for e in events:
        payload = e.get("payload") or {}
        if e.get("type") != "event_msg" or payload.get("type") != "item_completed":
            continue
        item = payload.get("item") or {}
        kind = item.get("type")
        if kind == "CommandExecution":
            argv = item.get("command") or []
            command = argv if isinstance(argv, str) else (
                f"{argv[0]} {argv[1]} {shlex.quote(argv[2])}"
                if len(argv) == 3 and argv[1] in ("-lc", "-c") else shlex.join(argv))
            out.append({"type": "item.completed", "item": {
                "type": "command_execution", "command": command,
                "exit_code": item.get("exit_code"),
                "aggregated_output": item.get("aggregated_output") or ""}})
        elif kind == "AgentMessage":
            text = "".join(c.get("text", "") for c in item.get("content") or []
                           if isinstance(c, dict))
            out.append({"type": "item.completed",
                        "item": {"type": "agent_message", "text": text}})
        elif kind not in (None, "Reasoning", "UserMessage", "SubAgentActivity"):
            out.append({"type": "item.completed", "item": {"type": kind.lower()}})
    return out + [{"type": "turn.completed"}]


def epoch(stamp: str) -> float:
    return datetime.datetime.fromisoformat(stamp.replace("Z", "+00:00")).timestamp()


def codex_threads(home: Path, run_dir: Path) -> list:
    """Write each subagent's transcript to threads/ and its raw record to
    sessions/; return each one's (thread, start, end) from its own record."""
    agents = []
    for meta, events, path in rollouts(home):
        if not meta.get("parent_thread_id"):
            continue
        (run_dir / "threads").mkdir(exist_ok=True)
        (run_dir / "sessions").mkdir(exist_ok=True)
        thread = f"codex-{meta.get('id')}"
        (run_dir / "threads" / f"{thread}.jsonl").write_text(
            "".join(json.dumps(e) + "\n" for e in codex_thread_events(events)))
        shutil.copy(path, run_dir / "sessions" / path.name)
        stamps = [epoch(e["timestamp"]) for e in events if e.get("timestamp")]
        if stamps:
            agents.append({"thread": thread, "start": min(stamps), "end": max(stamps)})
    return agents


def delegated_threads(work: Path) -> list:
    """Each thread delegate started, from its log; any still running is
    killed, and ends now."""
    log, started, ended = work / ".agents.jsonl", {}, {}
    for line in (log.read_text().splitlines() if log.exists() else []):
        entry = json.loads(line)
        (started if entry["event"] == "start" else ended)[entry["thread"]] = entry
    agents = []
    for thread, entry in started.items():
        if thread not in ended:
            try:
                os.killpg(entry["pgid"], signal.SIGKILL)
            except ProcessLookupError:
                pass
        agents.append({"thread": thread, "start": entry["time"],
                       "end": ended.get(thread, {}).get("time", time.time())})
    return agents


def peak(agents: list) -> int:
    """The most agents alive at once among (start, end) intervals."""
    edges = sorted([(a["start"], 1) for a in agents] + [(a["end"], -1) for a in agents],
                   key=lambda e: (e[0], e[1]))
    alive = top = 0
    for _, step in edges:
        alive += step
        top = max(top, alive)
    return top


def invalid_reason(arm: str, exit_code, info: dict, threads: bool = False):
    if exit_code != 0:
        return f"the agent exited {exit_code}"
    if info["init"] is None:
        return "no init event"
    if info["result"] is None or info["result"].get("is_error") \
            or info["result"].get("subtype") != "success":
        return "no successful result event"
    if info["init"]["mcp_servers"]:
        return f"MCP servers loaded: {info['init']['mcp_servers']}"
    if info["init"].get("agent_tools"):
        return f"agent-starting tools offered: {info['init']['agent_tools']}"
    if info["init"].get("foreign_skills"):
        return f"skills loaded from outside the scratch home: {info['init']['foreign_skills']}"
    if info["init"].get("spawn_agent") and not threads:
        return "spawn_agent was offered in a scenario without threads"
    if info["init"]["agent_factory"] != (arm == "skill"):
        return f"agent-factory present={info['init']['agent_factory']} in arm {arm}"
    return None


def canary_verdict(reply: str, repo: Path, token: str):
    """Whether the reply quotes the repository's own heading, and every path
    it names outside the repository. Any such path counts, whether or not it
    exists, so a mangled or made-up one costs a rerun rather than a leak."""
    text = re.sub(r"[*`]", " ", reply)
    own_seen = f"canary {token}" in text.lower()
    names = re.findall(r"(?<![\w/.~-])~?/[^\s|'\"<>()\[\],;]+", text)
    paths = {os.path.expanduser(n.rstrip(".:")) for n in names}
    return own_seen, sorted(p for p in paths if not inside(Path(p), repo))


def check_isolation(work_root: Path, effort: str, out_root: Path,
                    agent: str = "claude") -> None:
    """Exit unless a run in this setup sees only its repository's own
    instruction file, an AGENTS.md whose heading carries a random token
    that appears nowhere else the run can see, and its record shows no
    agent-starting tool and no foreign skill."""
    stamp = datetime.datetime.now().strftime("%Y%m%d-%H%M%S")
    name = f"{stamp}-canary-{secrets.token_hex(4)}"
    token = secrets.token_hex(4)
    work = work_root / name
    repo = work / "repo"
    repo.mkdir(parents=True)
    subprocess.run(["git", "init", "-q", str(repo)], check=True)
    (repo / "AGENTS.md").write_text(
        f"# Isolation canary {token}\n\nThis repository's own guide.\n")
    transcript = out_root / f"{name}.jsonl"
    model = CANARY_MODEL[agent]
    exit_code = run_claude(
        agent_command(agent, CANARY_PROMPT, model, effort, 0.5, repo),
        repo, agent_env(work, agent), transcript, timeout=300)
    unlink_auth(work)
    info = read_transcript(transcript)
    if agent == "codex" and info["init"] is not None:
        info["init"].update(codex_session(work / "home" / ".codex"))
    reply = (info["result"] or {}).get("result") or ""
    own_seen, foreign = canary_verdict(reply, repo, token)
    unbounded = invalid_reason("none", 0, info) if exit_code == 0 else None
    ok = exit_code == 0 and own_seen and not foreign and not unbounded
    print(json.dumps({"canary": "pass" if ok else "fail", "model": model,
                      "exit": exit_code, "foreign": foreign, "own_seen": own_seen,
                      "invalid": unbounded, "transcript": str(transcript)}), flush=True)
    if ok:
        shutil.rmtree(work, ignore_errors=True)
        return
    if exit_code != 0:
        why = f"the canary run exited {exit_code}"
    elif unbounded:
        why = unbounded
    elif foreign:
        why = ("the run reports instruction files other than the scenario's "
               "own: " + ", ".join(foreign))
    else:
        why = ("the canary did not report its repository's own AGENTS.md, so "
               "the run's instructions are unconfirmed")
    sys.exit(f"Isolation canary failed: {why}.\nIts reply:\n{reply.strip()}\n"
             f"Transcript: {transcript}\nWork directory kept: {work}\n"
             "No scenario was run.")


def threads(work: Path, run_dir: Path, env: dict, agent: str, model: str,
            effort: str, meta: dict, thread: str) -> tuple:
    """Set up threads for a run; return its env, with its own thread ID
    `thread`, and its system note. Under Claude Code install `delegate` and
    what it needs; Codex starts threads with spawn_agent."""
    if agent == "codex":
        return {**env, "T3CODE_THREAD_ID": thread}, CODEX_THREADS_NOTE
    shutil.copy(Path(__file__).with_name("delegate.py"), work / "bin" / "delegate")
    (work / "bin" / "delegate").chmod(0o755)
    budget = meta.get("thread_budget_usd", meta.get("max_budget_usd", 5))
    (work / ".threads.json").write_text(json.dumps({
        "agent": agent, "env": env, "out": str(run_dir / "threads"),
        "log": str(work / ".agents.jsonl"), "lock": str(work / ".threads.lock"),
        "timeout": meta.get("thread_timeout_sec", 1200),
        "argv": agent_command(agent, "{task}", model, effort, budget, work / "repo",
                              THREADS_NOTE),
    }))
    return {**env, "T3CODE_THREAD_ID": thread}, THREADS_NOTE


def run_one(scenario: str, arm: str, ref: str, model: str, effort: str,
            index: int, out_root: Path, work_root: Path, agent: str = "claude",
            skill_dir: str = DEFAULT_SKILL_DIR) -> dict:
    sdir = SCENARIOS / scenario
    meta = json.loads((sdir / "scenario.json").read_text())
    has_threads = bool(meta.get("threads"))
    stamp = datetime.datetime.now().strftime("%Y%m%d-%H%M%S")
    name = f"{stamp}-{scenario}-{arm}-{agent}-{model}-{index}-{secrets.token_hex(2)}"
    run_dir = out_root / name
    work = work_root / name
    run_dir.mkdir(parents=True)
    work.mkdir(parents=True)
    (run_dir / "work").symlink_to(work)

    thread = f"thread-{secrets.token_hex(4)}"
    setup_env = {**os.environ, "T3CODE_THREAD_ID": thread} if has_threads else None
    setup = subprocess.run(["bash", str(sdir / "setup.sh"), str(work)],
                           env=setup_env, capture_output=True, text=True)
    if setup.returncode != 0:
        return {"scenario": scenario, "arm": arm, "model": model, "index": index,
                "pass": None, "invalid": "setup failed: " + setup.stderr[-500:],
                "run_dir": str(run_dir), "work_dir": str(work)}
    repo_dir = work / "repo"
    env, system = agent_env(work, agent, has_threads), None
    skill_sha = stage_for(agent, ref, work, skill_dir) if arm == "skill" else None

    prompt = (sdir / "prompt.md").read_text()
    prompt = prompt.replace("{{skill}}", SKILL_LINE if arm == "skill" else "")
    transcript = run_dir / "transcript.jsonl"
    if has_threads:
        env, system = threads(work, run_dir, env, agent, model, effort, meta, thread)
    cmd = agent_command(agent, prompt, model, effort, meta.get("max_budget_usd", 5),
                        repo_dir, system)
    started = time.time()
    try:
        exit_code = run_claude(cmd, repo_dir, env, transcript,
                               timeout=meta.get("timeout_sec", 1200))
    finally:
        unlink_auth(work)
        agents = [{"thread": thread if has_threads else "main", "start": started,
                   "end": time.time()}] + delegated_threads(work)
    seconds = agents[0]["end"] - started

    info = read_transcript(transcript)
    if agent == "codex":
        home = work / "home" / ".codex"
        agents += codex_threads(home, run_dir)
        if info["init"] is not None:
            info["init"].update(codex_session(home))
    (run_dir / "final.md").write_text((info["result"] or {}).get("result") or "")
    grade = subprocess.run(
        [sys.executable, "-I", str(sdir / "grade.py"), str(work), str(transcript)],
        capture_output=True, text=True,
    )
    try:
        verdict = json.loads(grade.stdout)
    except json.JSONDecodeError:
        verdict = {"pass": None, "error": grade.stdout + grade.stderr}
    invalid = invalid_reason(arm, exit_code, info, has_threads)
    if invalid:
        verdict = {**verdict, "pass": None, "invalid": invalid}

    result = {
        "scenario": scenario, "arm": arm, "skill_ref": skill_sha,
        "skill_dir": skill_dir if arm == "skill" else None,
        "agent": agent, "model": model, "effort": effort, "index": index,
        "exit": exit_code, "seconds": round(seconds), "run_dir": str(run_dir),
        "work_dir": str(work), "init": info["init"],
        "agents": agents, "peak_agents": peak(agents),
        **verdict,
    }
    (run_dir / "result.json").write_text(json.dumps(result, indent=1))
    return result


def check_jobs(jobs: int, scenarios: list) -> str:
    """Why --jobs breaks the agent limit for these scenarios, or None."""
    if jobs > 2:
        return "--jobs above 2 would run more than two agents at once"
    threaded = [s for s in scenarios
                if json.loads((SCENARIOS / s / "scenario.json").read_text()).get("threads")]
    if threaded and jobs > 1:
        return (f"{', '.join(threaded)} start(s) a second agent, so a matrix with "
                "threads runs one at a time: pass --jobs 1")
    return None


def main() -> None:
    p = argparse.ArgumentParser(description=__doc__.splitlines()[0])
    p.add_argument("scenarios", nargs="+")
    p.add_argument("--agent", choices=("claude", "codex"), default="claude")
    p.add_argument("--arms", default="none,skill")
    p.add_argument("--ref", default="HEAD", help="git ref of the skill to stage")
    p.add_argument("--skill-dir", default=DEFAULT_SKILL_DIR,
                   help="the skill's directory in this repository, staged as agent-factory")
    p.add_argument("--model", default="opus")
    p.add_argument("--effort", default="high")
    p.add_argument("--runs", type=int, default=1)
    p.add_argument("--jobs", type=int, default=2)
    p.add_argument("--out", type=Path, default=DEFAULT_OUT,
                   help="where transcripts and results go")
    p.add_argument("--work-root", type=Path, default=DEFAULT_WORK_ROOT,
                   help="where work directories are made; must be outside $HOME")
    p.add_argument("--keep", action="store_true",
                   help="keep work directories (default: keep only on failure)")
    a = p.parse_args()
    if inside(a.work_root, Path.home()):
        p.error(f"--work-root {a.work_root} is inside $HOME, where the user's "
                "own CLAUDE.md would load into every run")
    too_many = check_jobs(a.jobs, a.scenarios)
    if too_many:
        p.error(too_many)

    jobs = [(s, arm, i) for s in a.scenarios for arm in a.arms.split(",")
            for i in range(a.runs)]
    results_path = a.out / "results.jsonl"
    a.out.mkdir(parents=True, exist_ok=True)
    a.work_root.mkdir(parents=True, exist_ok=True)
    check_isolation(a.work_root, a.effort, a.out, a.agent)
    alive = []
    with concurrent.futures.ThreadPoolExecutor(a.jobs) as pool:
        futures = [pool.submit(run_one, s, arm, a.ref, a.model, a.effort, i,
                               a.out, a.work_root, a.agent, a.skill_dir)
                   for s, arm, i in jobs]
        for fut, (s, arm, i) in zip(futures, jobs):
            try:
                r = fut.result()
            except Exception as exc:  # record it; keep the other runs' results
                r = {"scenario": s, "arm": arm, "index": i, "pass": None,
                     "invalid": f"harness error: {exc!r}", "run_dir": ""}
            alive += r.get("agents") or []
            with results_path.open("a") as f:
                f.write(json.dumps(r) + "\n")
            if r.get("pass") is True and not a.keep and r.get("work_dir"):
                shutil.rmtree(r["work_dir"], ignore_errors=True)
                (Path(r["run_dir"]) / "work").unlink(missing_ok=True)
            print(json.dumps({k: r.get(k) for k in
                              ("scenario", "arm", "index", "pass", "invalid",
                               "exit", "seconds", "peak_agents", "run_dir")}), flush=True)
    print(json.dumps({"matrix_peak_agents": peak(alive), "runs": len(jobs)}), flush=True)


if __name__ == "__main__":
    main()
