#!/usr/bin/env python3
"""Compute the scorecard's counted measures per repository and write counted.json and counted.md.

  counted.py --snapshot DIR [--projects DIR] [--manifest FILE] [--out DIR]

Reads T3's database and the transcripts from the snapshot, and each repository's records and
history at its manifest commit. Run it under `nice -n 19 ionice -c3`.
"""
import argparse
import json
import statistics
import sys
from contextlib import closing
from dataclasses import dataclass
from pathlib import Path

import commands
import gitreader
import records
import threads
import transcripts
from snapshot import MANIFEST, WINDOW_START, InputError, Window, check_not_live, load_manifest

IMPLEMENTER_ROLES = ("implementation", "unlabelled")


@dataclass
class ThreadStats:
    item: str
    tied_by: str
    role: str
    shell: int = 0
    process: int = 0
    atelier: int = 0
    refused: int = 0
    help: int = 0
    confused: int = 0
    calls_before_edit: int = None  # every tool call before its first product edit
    shell_before_edit: int = None
    transcript: bool = False
    unpriced: int = 0  # requests to a model with no list price
    cost: float = 0.0
    process_cost: float = 0.0


def thread_stats(thread, item_ids, calls, window):
    item, tied_by = threads.tie(thread, item_ids)
    stats = ThreadStats(item, tied_by, thread.role)
    tool_calls = shell_calls = 0
    for call in calls:
        edits = (commands.is_product_edit(call.path) if call.kind == "edit"
                 else call.kind == "shell" and commands.writes_product(call.command))
        if edits and stats.calls_before_edit is None:
            stats.calls_before_edit, stats.shell_before_edit = tool_calls, shell_calls
        tool_calls += 1
        if call.kind != "shell":
            continue
        shell_calls += 1
        stats.shell += 1
        stats.process += commands.is_process(call.command)
        if commands.atelier_calls(call.command) and call.started_at in window:
            refused, help_ = commands.refused(call.output), commands.is_help(call.command)
            stats.atelier += 1
            stats.refused += refused
            stats.help += help_
            stats.confused += refused or help_
    return stats


def add_tokens(stats, thread, files, claimed, cutoff):
    """Price the thread's transcripts; a session two threads name is counted for the first only."""
    for key in thread.sessions:
        if key in claimed or key not in files:
            continue
        claimed.add(key)
        stats.transcript = True
        for path in files[key]:
            for request in transcripts.requests(path, cutoff):
                if request.cost is None:
                    stats.unpriced += 1
                    continue
                stats.cost += request.cost
                if request.calls:
                    process = sum(shell is not None and commands.is_process(shell) for shell in request.calls)
                    stats.process_cost += request.cost * process / len(request.calls)


def measure(db, files, repo, name, commit, window):
    all_items = records.items(repo, commit)
    merged = records.merged(repo, commit, all_items, window)
    found = threads.threads(db, name, window.before_cutoff)
    claimed, stats = set(), []
    for thread in sorted(found, key=lambda t: (t.created_at, t.id)):
        s = thread_stats(thread, all_items.keys(), threads.calls(db, thread.id, window.before_cutoff), window)
        if s.item in merged:
            add_tokens(s, thread, files, claimed, window.before_cutoff)
        stats.append(s)
    tied = [s for s in stats if s.item in merged]
    covered = [s for s in tied if s.transcript]
    coverage = {"threads_with_transcript": len(covered), "threads_counted": len(tied)}
    priced_cost = sum(s.cost for s in covered)

    per_item = {}
    for item in sorted({s.item for s in tied}):
        mine = [s for s in tied if s.item == item]
        if all(s.transcript and not s.unpriced for s in mine):
            per_item[item] = round(sum(s.cost for s in mine), 2)

    implementers = [s for s in tied if s.role in IMPLEMENTER_ROLES and s.calls_before_edit is not None]
    done = sum(1 for i in all_items.values() if i.status == "done" and i.kind != "mission" and i.closed_at and i.closed_at in window)
    return {
        "commit": commit,
        "items": {"done_in_window": done, "merged": len(merged), "merged_with_threads": len({s.item for s in tied})},
        "threads": {
            "in_repository": len(stats),
            "tied_to_merged_item": len(tied),
            "tied_by": {source: sum(1 for s in tied if s.tied_by == source) for source in ("branch", "title", "first_message")},
            "tied_to_other_item": sum(1 for s in stats if s.item and s.item not in merged),
            "untied": sum(1 for s in stats if not s.item),
        },
        "process_share": {
            "shell_commands": sum(s.shell for s in tied),
            "process_commands": sum(s.process for s in tied),
            "share": _ratio(sum(s.process for s in tied), sum(s.shell for s in tied)),
            "token_cost_share": _ratio(sum(s.process_cost for s in covered), priced_cost),
            "token_coverage": coverage,
        },
        "confusion": {
            "atelier_calls": sum(s.atelier for s in stats),
            "refused_or_error": sum(s.refused for s in stats),
            "help_lookups": sum(s.help for s in stats),
            "confused_calls": sum(s.confused for s in stats),
            "per_100_calls": _ratio(100 * sum(s.confused for s in stats), sum(s.atelier for s in stats), 2),
        },
        "tokens_per_merged_item": {
            "items_fully_covered": len(per_item),
            "median_usd": round(statistics.median(per_item.values()), 2) if per_item else None,
            "mean_usd": round(statistics.fmean(per_item.values()), 2) if per_item else None,
            "per_item_usd": per_item,
            "threads_with_unpriced_model": sum(1 for s in covered if s.unpriced),
            "token_coverage": coverage,
        },
        "calls_before_first_edit": {
            "implementer_threads": len(implementers),
            "median_tool_calls": _median([s.calls_before_edit for s in implementers]),
            "median_shell_commands": _median([s.shell_before_edit for s in implementers]),
        },
        "rework": gitreader.rework(repo, commit, window),
        "record_only_commits": gitreader.record_only(repo, commit, window),
        "line_count": gitreader.line_count(repo, commit),
    }


def _ratio(part, whole, digits=4):
    return round(part / whole, digits) if whole else None


def _median(values):
    return round(float(statistics.median(values)), 1) if values else None


def render(result):
    repos = result["repositories"]
    names = list(repos)

    def row(label, value):
        return f"| {label} | " + " | ".join(str(value(repos[n])).replace("None", "n/a") for n in names) + " |"

    def pct(x):
        return "n/a" if x is None else f"{100 * x:.1f}%"

    def cov(c):
        return f"{c['threads_with_transcript']} of {c['threads_counted']} threads"

    lines = [
        "# Scorecard baseline: counted measures",
        "",
        f"Written by `scripts/scorecard/counted.py` from the snapshot described by `manifest.json`. "
        f"Window: {result['window_start']} to the cutoff, {result['cutoff']}. T3 schema: {result['t3_schema']}. "
        "How each number is taken is in `scripts/scorecard/README.md`.",
        "",
        "| Measure | " + " | ".join(names) + " |",
        "| --- |" + " --- |" * len(names),
        row("Merged items (done in window)", lambda r: f"{r['items']['merged']} ({r['items']['done_in_window']})"),
        row("Merged items with a tied thread", lambda r: r["items"]["merged_with_threads"]),
        row("Threads tied to a merged item", lambda r: f"{r['threads']['tied_to_merged_item']} of {r['threads']['in_repository']}"),
        row("Process share of shell commands", lambda r: f"{pct(r['process_share']['share'])} of {r['process_share']['shell_commands']}"),
        row("Process share of token cost", lambda r: f"{pct(r['process_share']['token_cost_share'])} ({cov(r['process_share']['token_coverage'])})"),
        row("Confusion per 100 atelier calls", lambda r: f"{r['confusion']['per_100_calls']} of {r['confusion']['atelier_calls']}"),
        row("Refused or errored atelier calls", lambda r: r["confusion"]["refused_or_error"]),
        row("Help lookups", lambda r: r["confusion"]["help_lookups"]),
        row("Tokens per merged item, median USD", lambda r: f"{r['tokens_per_merged_item']['median_usd']} ({r['tokens_per_merged_item']['items_fully_covered']} items; {cov(r['tokens_per_merged_item']['token_coverage'])})"),
        row("Tokens per merged item, mean USD", lambda r: r["tokens_per_merged_item"]["mean_usd"]),
        row("Calls before first edit, median tool calls", lambda r: f"{r['calls_before_first_edit']['median_tool_calls']} ({r['calls_before_first_edit']['implementer_threads']} threads)"),
        row("Calls before first edit, median shell commands", lambda r: r["calls_before_first_edit"]["median_shell_commands"]),
        row("Rework", lambda r: f"{pct(r['rework']['share'])} of {r['rework']['lines_written']} lines"),
        row("Rework: commits old enough to count", lambda r: f"{r['rework']['eligible_commits']} ({(r['rework']['first_eligible'] or '-')[:8]}..{(r['rework']['last_eligible'] or '-')[:8]})"),
        row("Record-only share of commits", lambda r: f"{pct(r['record_only_commits']['share'])} of {r['record_only_commits']['commits']}"),
        row("Non-test lines", lambda r: f"{r['line_count']['non_test_lines']} {r['line_count']['language']}"),
        "",
        "Each token number has its coverage beside it: threads tied to a merged item that have a surviving transcript, "
        "out of those threads. Tokens are not estimated for threads without one. An item's tokens count only when every "
        "thread tied to it has a transcript and every request in them has a list price.",
        "",
        f"Rework counts lines that reached the base from {result['window_start']} until "
        f"{next(iter(repos.values()))['rework']['written_until']}, so each was watched for {gitreader.REWORK_DAYS} days before the cutoff; "
        "the row above names the first and last first-parent commit old enough to count.",
        "",
        "## What counted as a test",
        "",
        *[f"- {n}: {repos[n]['line_count']['language']} ({', '.join('.' + e for e in repos[n]['line_count'].get('extensions', ['rs']))}); "
          f"tests are {repos[n]['line_count']['tests']}." for n in names],
        "",
        "## List prices",
        "",
        "USD per million tokens, from the 2026-11 assessment's table. A model not listed is counted but not priced.",
        "",
        "| Model | Input | Cache read | Cache write | Output |",
        "| --- | --- | --- | --- | --- |",
        *[f"| {m} | " + " | ".join(str(p) for p in prices) + " |" for m, prices in sorted(result["prices"].items())],
        "",
    ]
    return "\n".join(lines)


def main(argv=None):
    parser = argparse.ArgumentParser(description=__doc__, formatter_class=argparse.RawDescriptionHelpFormatter)
    parser.add_argument("--snapshot", required=True, help="the frozen snapshot directory (never a live store)")
    parser.add_argument("--projects", default=str(Path.home() / "projects"), help="directory holding the five repositories")
    parser.add_argument("--manifest", default=str(MANIFEST), help="the snapshot's manifest.json")
    parser.add_argument("--out", default=str(Path(MANIFEST).parent), help="where to write counted.json and counted.md")
    args = parser.parse_args(argv)
    try:
        snapshot = check_not_live(args.snapshot)
        manifest = load_manifest(args.manifest)
        window = Window(WINDOW_START, manifest["cutoff"])
        db = threads.open_db(snapshot)
        files = transcripts.index(snapshot)
        with closing(db):
            result = {
                "cutoff": manifest["cutoff"],
                "window_start": WINDOW_START,
                "t3_schema": threads.SCHEMA,
                "prices": {model: list(price) for model, price in transcripts.PRICES.items()},
                "repositories": {name: measure(db, files, Path(args.projects) / name, name, repo["commit"], window)
                                 for name, repo in manifest["repositories"].items()},
            }
    except InputError as error:
        sys.exit(f"counted.py: {error}")
    out = Path(args.out)
    out.mkdir(parents=True, exist_ok=True)
    (out / "counted.json").write_text(json.dumps(result, indent=1, sort_keys=True) + "\n")
    (out / "counted.md").write_text(render(result))


if __name__ == "__main__":
    main()
