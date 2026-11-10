"""Transcript reader: each model request's tokens, list price and shell commands, from the snapshot's transcripts.

Claude writes one transcript per session (`claude-projects/<dir>/<session>.jsonl`) and one per
subagent its Agent tool started (`<session>/subagents/agent-<agent id>.jsonl`). Codex writes one per session (`codex-sessions/YYYY/MM/DD/rollout-...-<session>.jsonl`).
Records time-stamped after the cutoff are dropped.
"""
import json
import re
from dataclasses import dataclass, field

from snapshot import check_not_live

# List prices in USD per million tokens: input, cache read, cache write, output. The source is the
# 2026-11 assessment's table (/var/tmp/atelier-assessment/transcripts/cost.py); a cache write it
# gives no price for is charged as input. A model missing here is counted but left unpriced.
PRICES = {
    "claude-haiku-5-5": (1.0, 0.1, 2.0, 5.0),
    "claude-opus-5-5": (4.0, 0.2, 8.0, 20.0),
    "claude-sonnet-5-5": (3.0, 0.3, 6.0, 15.0),
    "gpt-6-luna": (0.1, 0.01, 0.1, 0.5),
    "gpt-6.1-sol": (2.0, 0.1, 2.0, 10.0),
    "grok-4.6-build": (2.0, 0.5, 2.0, 6.0),
}
TOKEN_KINDS = ("input", "cache_read", "cache_write", "output")
CODEX_TOOL = re.compile(r"\btools\.(\w+)\(")
CODEX_CMD = re.compile(r"""\bcmd\s*:\s*("(?:\\.|[^"\\])*"|'(?:\\.|[^'\\])*'|`(?:\\.|[^`\\])*`)""")


@dataclass
class Request:
    """One model response: its tokens by kind, and its tool calls, each its shell command or None."""
    model: str
    tokens: dict
    calls: list = field(default_factory=list)

    @property
    def cost(self):
        """USD at list price, or None for a model with no price."""
        price = PRICES.get(self.model)
        if price is None:
            return None
        return sum(self.tokens[kind] * rate for kind, rate in zip(TOKEN_KINDS, price)) / 1e6


def index(snapshot):
    """Transcript files by (kind, native id), the keys a thread's sessions use."""
    root = check_not_live(snapshot)
    found = {}
    for path in sorted((root / "claude-projects").glob("*/*.jsonl")):
        found.setdefault(("claudeAgent", path.stem), []).append(path)
    for path in sorted((root / "claude-projects").glob("*/*/subagents/agent-*.jsonl")):
        found.setdefault(("claudeSubagent", path.stem.removeprefix("agent-")), []).append(path)
    for path in sorted((root / "codex-sessions").glob("*/*/*/*.jsonl")):
        found.setdefault(("codex", path.stem[-36:]), []).append(path)
    return found


def requests(path, cutoff):
    """The transcript's requests up to the cutoff, in order."""
    return _codex(path, cutoff) if "codex-sessions" in path.parts else _claude(path, cutoff)


def _claude(path, cutoff):
    by_id = {}  # a response is written once per content block; the last copy has its final usage
    with open(path) as handle:
        for line in handle:
            if '"usage"' not in line:
                continue
            record = json.loads(line)
            message = record.get("message")
            if record.get("type") != "assistant" or not isinstance(message, dict) or not message.get("usage"):
                continue
            entry = by_id.setdefault(message.get("id"), {"time": record.get("timestamp"), "blocks": {}})
            entry["message"] = message
            for block in message.get("content") or []:
                if isinstance(block, dict) and block.get("type") == "tool_use":
                    entry["blocks"][block.get("id")] = block
    for entry in by_id.values():
        message = entry["message"]
        if message.get("model") == "<synthetic>" or not cutoff(entry["time"]):
            continue
        usage = message["usage"]
        tokens = dict(zip(TOKEN_KINDS, (usage.get("input_tokens") or 0, usage.get("cache_read_input_tokens") or 0,
                                        usage.get("cache_creation_input_tokens") or 0, usage.get("output_tokens") or 0)))
        calls = [(b.get("input") or {}).get("command") or "" if b.get("name") == "Bash" else None
                 for b in entry["blocks"].values()]
        yield Request(message.get("model"), tokens, calls)


def _codex(path, cutoff):
    model, previous, calls = None, None, []
    with open(path) as handle:
        for line in handle:
            record = json.loads(line)
            payload = record.get("payload") or {}
            if record.get("type") == "turn_context":
                model = payload.get("model")
            elif record.get("type") == "response_item" and payload.get("type") in ("custom_tool_call", "function_call"):
                text = payload.get("input") or payload.get("arguments") or ""
                starts = [(m.start(), m.group(1)) for m in CODEX_TOOL.finditer(text)]
                if not starts:  # a direct call rather than a script calling tools
                    calls.append(_codex_cmd(text) if payload.get("name") in ("exec_command", "shell") else None)
                for n, (start, name) in enumerate(starts):
                    segment = text[start : starts[n + 1][0] if n + 1 < len(starts) else len(text)]
                    calls.append(_codex_cmd(segment) if name == "exec_command" else None)
            elif payload.get("type") == "token_count" and (payload.get("info") or {}).get("total_token_usage"):
                if not cutoff(record.get("timestamp")):
                    break
                total = payload["info"]["total_token_usage"]
                now = {"input": total.get("input_tokens", 0) - total.get("cached_input_tokens", 0),
                       "cache_read": total.get("cached_input_tokens", 0),
                       "cache_write": total.get("cache_write_input_tokens", 0),
                       "output": total.get("output_tokens", 0)}
                tokens = {k: now[k] - (previous or {}).get(k, 0) for k in TOKEN_KINDS}
                previous = now
                if any(tokens.values()):
                    yield Request(model, tokens, calls)
                    calls = []


def _codex_cmd(text):
    """The shell command a Codex tool call runs: its `cmd` string, decoded, or the call's text."""
    match = CODEX_CMD.search(text)
    if not match:
        return text
    literal = match.group(1)
    if literal.startswith('"'):
        try:
            return json.loads(literal)
        except ValueError:
            pass
    return re.sub(r"\\(.)", lambda m: {"n": "\n", "t": "\t"}.get(m.group(1), m.group(1)), literal[1:-1])
