"""What a shell command, its output or an edited path says, for the counted measures.

One owner for the patterns, so the thread reader, the transcript reader and the measures
classify a command the same way. This is a heuristic over the closed list of command forms in
`README.md`, not a shell: a command is split into words and operators as a shell would split it,
and each simple command is read on its own, with the heredoc it is fed, the `$(...)` it runs and
the directory it runs in. Python code is read with Python's own parser.
"""
import ast
import re
import warnings
from dataclasses import dataclass, field

# Operators a shell recognises outside quotes, longest first. Those holding `<` or `>` redirect.
OPERATORS = ("&>>", "&&", "||", "|&", ";;", "&>", ">>", ">&", ">|", "<<", ";", "&", "|", "(", ")", "\n", "<", ">")
# Wrappers that run the command after them, with their options that take an argument.
WRAPPERS = {"command": (), "env": ("-u", "-C", "-S"), "exec": ("-a",), "ionice": ("-c", "-n", "-p"), "nice": ("-n",),
            "stdbuf": ("-i", "-o", "-e"), "time": ("-f", "-o"), "timeout": ("-s", "-k"),
            "xargs": ("-I", "-n", "-P", "-d", "-E", "-L", "-s", "-a")}
KEYWORDS = {"!", "{", "do", "elif", "else", "if", "then", "until", "while"}
SHELLS = {"bash", "sh", "zsh"}
# A heredoc: its delimiter (not one already lifted), the rest of its line, and its body up to the delimiter
# on a line of its own (which may end a quoted `bash -c` script or a `$(...)`).
HEREDOC = re.compile(r"<<-?[ \t]*(['\"]?)(?!__heredoc_)(\w+)\1([^\n]*)\n(.*?)(?:\n[ \t]*\2[ \t]*(?=[\n'\")]|$)|\Z)", re.S)
LIFTED = re.compile(r"__heredoc_(\d+)__")
PYTHON = re.compile(r"python[\d.]*")
RECORDS_PATH = re.compile(r"(?<![\w-])\.atelier(?:/|(?=[\s\"'`;)|&]|$))")
COMMIT_LINE = re.compile(r"^\[[^\]\s]+(?: \(root-commit\))? ([0-9a-f]{7,40})\]", re.M)
PATCH_FILE = re.compile(r"^\*\*\* (?:Add|Update|Delete) File: (\S+)", re.M)
# atelier's own refusal and error formats; a harness's "Error: Exit code N" is not one.
REFUSAL = re.compile(
    r"^(?:[a-z][\w-]*: refused\b|Error: (?!Exit code \d)|error: (?:unexpected argument|unrecognized subcommand"
    r"|the argument|the following required arguments|invalid value))",
    re.M,
)
SCRATCH = re.compile(r"(?:^|/)(?:\.atelier|\.cache|\.claude|\.codex|\.local|target|node_modules|tmp[^/]*|[^/]*scratch[^/]*)(?:/|$)")
# A shell write counts as an edit only to a source, configuration or documentation file, not captured output.
SOURCE_FILE = re.compile(r"\.(?:rs|ts|tsx|js|jsx|mjs|cjs|svelte|kt|kts|py|go|java|swift|md|toml|json|ya?ml|sh|css|html|sql)$")


@dataclass
class Simple:
    """One simple command: its words after wrappers, the files it redirects output to, the heredoc it
    reads, and the commands of each `$(...)` in it, which run first, in a subshell."""
    words: list = field(default_factory=list)
    writes: list = field(default_factory=list)
    stdin: str = ""
    substitutions: list = field(default_factory=list)


def simple_commands(command, _bodies=None):
    """The simple commands a shell would run for this text, in order, entering `bash -c` scripts."""
    bodies = [] if _bodies is None else _bodies  # shared, since a lifted heredoc can sit inside a script or `$(...)`

    def lift_heredoc(match):
        bodies.append(match.group(4))
        return f"<< __heredoc_{len(bodies) - 1}__{match.group(3)}\n"

    found, current, redirect = [], Simple(), None
    for is_operator, value, substitutions in _tokens(HEREDOC.sub(lift_heredoc, command or "")):
        if is_operator and ("<" in value or ">" in value):
            redirect = value
        elif is_operator:
            found.append(current)
            current = Simple()
        else:
            current.substitutions += [simple_commands(inner, bodies) for inner in substitutions]
            if redirect is None:
                current.words.append(value)
            elif ">" in redirect and not redirect.endswith("&"):
                current.writes.append(value)
            elif redirect == "<<" and (lifted := LIFTED.fullmatch(value)):
                current.stdin = bodies[int(lifted.group(1))]
            redirect = None
    found.append(current)
    result = []
    for simple in found:
        simple.words = _unwrap(simple.words)
        script = _shell_script(simple.words)
        if script is not None:
            result += [Simple(substitutions=simple.substitutions)] + simple_commands(script, bodies)
        elif simple.words or simple.writes or simple.substitutions:
            result.append(simple)
    return result


def _tokens(text):
    """Words and operators as a shell reads them: (is operator, text, the `$(...)` contents in a word).

    Quotes and backslashes are removed from words; an operator is one only outside quotes. A `#`
    starting a word begins a comment. A number written against a redirection (`2>`) is dropped.
    """
    tokens, word, substitutions, quote, i = [], None, [], None, 0

    def end_word():
        nonlocal word, substitutions
        if word is not None:
            tokens.append((False, "".join(word), substitutions))
        word, substitutions = None, []

    while i < len(text):
        char = text[i]
        if quote == "'" and char != "'":
            word.append(char)
        elif char == "\\" and quote != "'":
            escaped = text[i + 1 : i + 2]
            if quote == '"' and escaped not in ('$', '`', '"', "\\"):
                escaped = "\\" + escaped
            word = (word or []) + [escaped] if escaped != "\n" else word
            i += 1
        elif text.startswith("$(", i):
            depth, end = 1, i + 2
            while end < len(text) and depth:
                depth += {"(": 1, ")": -1}.get(text[end], 0)
                end += 1
            substitutions.append(text[i + 2 : end - 1])
            word = (word or []) + ["$(...)"]
            i = end
            continue
        elif char in "'\"" and quote in (None, char):
            quote = None if quote else char
            word = word or []
        elif quote == '"':
            word.append(char)
        elif char in " \t\r":
            end_word()
        elif char == "#" and word is None:
            while i < len(text) and text[i] != "\n":
                i += 1
            continue
        elif operator := next((op for op in OPERATORS if text.startswith(op, i)), None):
            if word is not None and "".join(word).isdigit() and ("<" in operator or ">" in operator):
                word = None
            end_word()
            tokens.append((True, operator, []))
            i += len(operator)
            continue
        else:
            word = (word or []) + [char]
        i += 1
    end_word()
    return tokens


def _unwrap(words):
    """Drop leading variable assignments, keywords, and wrappers such as `nice -n 19` or `env -u NAME`."""
    i = 0
    while i < len(words):
        name = words[i].rsplit("/", 1)[-1]
        if re.match(r"^[A-Za-z_]\w*=", words[i]) or words[i] in KEYWORDS:
            i += 1
        elif name == "command" and words[i + 1 : i + 2] in (["-v"], ["-V"]):
            return []  # a lookup, not a run
        elif name in WRAPPERS:
            i += 1
            while i < len(words) and words[i].startswith("-"):
                i += 2 if words[i] in WRAPPERS[name] else 1
            if name == "timeout" and i < len(words) and words[i][:1].isdigit():
                i += 1  # its duration
        else:
            break
    return words[i:]


def _shell_script(words):
    if words and words[0].rsplit("/", 1)[-1] in SHELLS:
        for flag, script in zip(words[1:], words[2:]):
            if re.fullmatch(r"-[a-z]*c[a-z]*", flag):
                return script
    return None


def _flatten(simples):
    for simple in simples:
        for inner in simple.substitutions:
            yield from _flatten(inner)
        yield simple


def atelier_calls(command):
    """The arguments of each call of the installed atelier in a shell command."""
    return [s.words[1:] for s in _flatten(simple_commands(command))
            if s.words and (s.words[0] == "atelier" or s.words[0].endswith("/.cargo/bin/atelier"))]


def is_help(command):
    """An atelier call that asks for help: `--help`, `-h`, `atelier help` or `atelier man`."""
    for args in atelier_calls(command):
        first = next((a for a in args if not a.startswith("-")), None)
        if "--help" in args or "-h" in args or first in ("help", "man"):
            return True
    return False


def is_process(command, output="", record_commits=frozenset()):
    """Runs atelier, names a path under `.atelier/`, or made a commit that changed only `.atelier/`.

    A commit is recognised by the hash `git commit` printed, looked up in `record_commits`.
    """
    if atelier_calls(command) or RECORDS_PATH.search(command or ""):
        return True
    if record_commits and any(s.words[:1] == ["git"] and "commit" in s.words for s in _flatten(simple_commands(command))):
        return any(full.startswith(short) for short in COMMIT_LINE.findall(output or "") for full in record_commits)
    return False


def refused(output):
    """The output carries one of atelier's refusal or error lines."""
    return bool(REFUSAL.search(output or ""))


def is_product_edit(path):
    """An edited file that is neither a tracker record nor a scratch, build or harness file."""
    return bool(path) and not path.startswith("/dev/") and "$" not in path and not SCRATCH.search(path)


def shell_writes(command):
    """Files a shell command writes: redirections, `tee`, `sed -i`, `perl -i`, `cp`/`mv` targets, `apply_patch`
    files and Python writes in code a Python interpreter runs (`-c` or a heredoc).

    A relative path is taken from the directory the last `cd` before the write changed to, if any. A
    `$(...)` runs before its command, from the same directory, and its own `cd` stays inside it.
    """
    targets = []

    def walk(simples, cwd):
        def add(paths):
            targets.extend(p if p.startswith(("/", "~")) or not cwd else f"{cwd.rstrip('/')}/{p}" for p in paths)

        for simple in simples:
            for inner in simple.substitutions:
                walk(inner, cwd)
            if simple.words[:1] == ["cd"] and len(simple.words) > 1:
                cwd = simple.words[1] if simple.words[1].startswith(("/", "~")) or not cwd else f"{cwd}/{simple.words[1]}"
            add(simple.writes)
            name, args = (simple.words[0].rsplit("/", 1)[-1], simple.words[1:]) if simple.words else ("", [])
            positional = [a for a in args if not a.startswith("-")]
            if name in ("sed", "perl") and any(re.match(r"^-\w*i|^--in-place", a) for a in args):
                files, script_option, skip = [], False, False
                for arg in args:
                    if skip:
                        skip = False
                    elif arg.startswith("-"):
                        skip = arg in ("-f", "--expression", "--file") or bool(re.fullmatch(r"-\w*e", arg))
                        script_option = script_option or skip
                    else:
                        files.append(arg)
                add(files if script_option else files[1:])
            elif name == "tee":
                add(positional)
            elif name in ("cp", "mv") and len(positional) >= 2:
                add(positional[-1:])
            elif name == "apply_patch":
                add(PATCH_FILE.findall("\n".join([simple.stdin, *args])))
            elif PYTHON.fullmatch(name):
                add(_python_writes(args[args.index("-c") + 1] if "-c" in args[:-1] else simple.stdin if args[:1] in ([], ["-"]) else ""))

    walk(simple_commands(command), "")
    return targets


def _python_writes(code):
    """Paths Python code writes: `open(p, mode)`, `p.write_text()`, `p.write_bytes()` and `p.open(mode)` with a
    mode starting `w`, `a` or `x`, where `p` is a literal path, `Path(literal)`, or a name bound earlier to one.
    Code Python cannot parse is not read."""
    try:
        with warnings.catch_warnings():  # agents' code often has invalid escapes; that is not ours to report
            warnings.simplefilter("ignore")
            tree = ast.parse(code)
    except (SyntaxError, ValueError):
        return []
    bound, found = {}, []
    for node in sorted((n for n in ast.walk(tree) if isinstance(n, (ast.Assign, ast.Call))), key=lambda n: (n.lineno, n.col_offset)):
        if isinstance(node, ast.Assign):
            for target in node.targets:
                if isinstance(target, ast.Name):
                    bound[target.id] = _literal_path(node.value, bound)
            continue
        func, mode = node.func, None
        if isinstance(func, ast.Name) and func.id == "open" and node.args:
            path, mode = node.args[0], (node.args[1:2] or [kw.value for kw in node.keywords if kw.arg == "mode"] or [None])[0]
        elif isinstance(func, ast.Attribute) and func.attr in ("write_text", "write_bytes"):
            path, mode = func.value, ast.Constant("w")
        elif isinstance(func, ast.Attribute) and func.attr == "open":
            path, mode = func.value, (node.args[:1] or [kw.value for kw in node.keywords if kw.arg == "mode"] or [None])[0]
        if isinstance(mode, ast.Constant) and str(mode.value)[:1] in ("w", "a", "x") and (written := _literal_path(path, bound)):
            found.append(written)
    return found


def _literal_path(node, bound):
    if isinstance(node, ast.Constant) and isinstance(node.value, str):
        return node.value
    if isinstance(node, ast.Name):
        return bound.get(node.id)
    if isinstance(node, ast.Call) and getattr(node.func, "id", getattr(node.func, "attr", None)) == "Path" and node.args:
        return _literal_path(node.args[0], bound)
    return None


def writes_product(command):
    return any(is_product_edit(path) and SOURCE_FILE.search(path) for path in shell_writes(command))
