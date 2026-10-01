#!/usr/bin/env python3
"""Count committed non-test Rust physical lines, including blanks/comments.

Uses git objects, never the working tree. Excludes tests/ files, the external
cfg(test) app test_records helper, and full cfg(test) item spans. It masks Rust
comments and literals before balancing item braces; unrelated blank/comment
lines remain included. This is a source-budget count, not an executable-SLOC
metric. Run from the repository: scripts/count_non_test_rust.py REVISION
"""
import argparse
import collections
import re
import subprocess

LEX = re.compile(
    r'//[^\n]*|/\*|(?:b|c)?r(?P<h>\#*)".*?"(?P=h)|'
    r'"(?:\\.|[^"\\])*"|(?:b)?\'(?:\\u\{[0-9a-fA-F_]+\}|\\x[0-9a-fA-F]{2}|\\.|[^\'\\\n])\'',
    re.S,
)
CFG_TEST = re.compile(r'^[ \t]*#\s*\[\s*cfg\s*\(\s*test\s*\)\s*\]', re.M)


def git(*args):
    return subprocess.check_output(['git', *args], text=True)


def mask(source):
    chars = list(source)
    cursor = 0
    while match := LEX.search(source, cursor):
        start, end = match.span()
        if match.group() == '/*':
            depth = 1
            while depth:
                token = re.search(r'/\*|\*/', source[end:])
                if token is None:
                    raise ValueError('unterminated block comment')
                end += token.end()
                depth += 1 if token.group() == '/*' else -1
        for pos in range(start, end):
            if chars[pos] != '\n':
                chars[pos] = ' '
        cursor = end
    return ''.join(chars)


def item_end(code, start):
    """Find a body's closing brace or a declaration's outer semicolon."""
    kind = re.search(r'\b(fn|const|static|type|use|mod|struct|enum|impl|trait|union)\b', code[start:])
    value_item = kind is not None and kind.group() in ('const', 'static', 'type', 'use')
    if kind is not None and re.match(r'const\s+(?:unsafe\s+)?(?:extern\s+)?fn\b', code[start + kind.start():]):
        value_item = False
    stack = []
    value_expression = False
    pairs = {')': '(', ']': '[', '}': '{', '>': '<'}
    for pos in range(start, len(code)):
        char = code[pos]
        if char in '([':
            stack.append(char)
        elif char == '<' and all(group == '<' for group in stack) and (
            not value_expression or code[:pos].rstrip().endswith('::')
        ):
            stack.append(char)
        elif char in pairs and stack and stack[-1] == pairs[char]:
            stack.pop()
        elif char == '=' and not stack:
            value_expression = True
        elif char == '{':
            if stack or value_item:
                stack.append(char)
                continue
            depth = 1
            end = pos + 1
            while depth and end < len(code):
                depth += (code[end] == '{') - (code[end] == '}')
                end += 1
            if depth:
                raise ValueError('unbalanced cfg(test) item body')
            return end
        elif char == ';' and not stack:
            return pos + 1
    raise ValueError('cfg(test) item has no complete body or declaration terminator')


def count(source):
    code = mask(source)
    excluded = set()
    items = 0
    for attribute in CFG_TEST.finditer(code):
        end = item_end(code, attribute.end())
        start_line = source.count('\n', 0, attribute.start())
        end_line = source.count('\n', 0, end)
        excluded.update(range(start_line, end_line + 1))
        items += 1
    physical = len(source.splitlines())
    return physical - len(excluded), items, len(excluded)


def self_check():
    cases = [
        ('array return', '#[cfg(test)]\nfn helper() -> [u8; 3] {\n    [1, 2, 3]\n}\n', 0),
        ('const array', '#[cfg(test)]\nconst ITEMS: [u8; 3] = [\n    1,\n    2,\n    3,\n];\n', 0),
        ('nested signature', '#[cfg(test)]\nfn helper(f: fn() -> [u8; 3]) -> ([u8; 3], u8) {\n    (f(), 1)\n}\n', 0),
        ('generic const', '#[cfg(test)]\nfn helper() -> Buffer<{ 3 }> {\n    Buffer::new()\n}\n', 0),
        ('const block', '#[cfg(test)]\nconst ITEM: usize = {\n    3\n};\n', 0),
        ('array shift', '#[cfg(test)]\nfn helper() -> [u8; 1 << 2] {\n    [0; 4]\n}\n', 0),
        ('array comparison', '#[cfg(test)]\nfn helper() -> [u8; (1 < 2) as usize] {\n    [0; 1]\n}\n', 0),
        ('const extern function', '#[cfg(test)]\nconst unsafe extern "C" fn helper() -> u8 {\n    1\n}\nconst PRODUCTION: u8 = 2;\n', 1),
        ('production after helper', '#[cfg(test)]\nfn helper() -> [u8; 3] {\n    [1, 2, 3]\n}\n// Production comment\n\npub fn production() -> u8 {\n    4\n}\n', 5),
    ]
    for name, source, expected in cases:
        actual, _, _ = count(source)
        assert actual == expected, f'{name}: expected {expected}, counted {actual}'
    print(f'Self-check: {len(cases)} Rust item cases passed')


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('revision', nargs='?', default='HEAD')
    parser.add_argument('--self-check', action='store_true', help='verify Rust item boundaries')
    args = parser.parse_args()
    if args.self_check:
        self_check()
        return
    revision = git('rev-parse', args.revision + '^{commit}').strip()
    totals = collections.Counter()
    excluded_files = []
    item_count = 0
    excluded_lines = 0
    files = git('ls-tree', '-r', '--name-only', revision).splitlines()
    for path in files:
        if not path.endswith('.rs'):
            continue
        if '/tests/' in path or path.endswith('/src/test_records.rs'):
            excluded_files.append(path)
            continue
        lines, items, removed = count(git('show', revision + ':' + path))
        totals[path.split('/')[1] if path.startswith('crates/') else '(root)'] += lines
        item_count += items
        excluded_lines += removed
    print('Command: scripts/count_non_test_rust.py ' + args.revision)
    print('Commit: ' + revision)
    print('Method: physical Rust lines including blanks/comments; exclude tests/ files,')
    print('        cfg(test) test_records helper, and complete cfg(test) item spans.')
    for crate, lines in sorted(totals.items()):
        print(f'{crate}: {lines}')
    print(f'TOTAL: {sum(totals.values())}')
    print(f'Excluded: {len(excluded_files)} test files; {item_count} cfg(test) items ({excluded_lines} lines)')


if __name__ == '__main__':
    main()
