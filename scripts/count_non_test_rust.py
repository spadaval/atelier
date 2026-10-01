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


def count(source):
    code = mask(source)
    excluded = set()
    items = 0
    for attribute in CFG_TEST.finditer(code):
        token = re.search(r'[;{]', code[attribute.end():])
        if token is None:
            raise ValueError('cfg(test) item has no body or declaration terminator')
        end = attribute.end() + token.end()
        if token.group() == '{':
            depth = 1
            while depth:
                if end >= len(code):
                    raise ValueError('unbalanced cfg(test) item')
                depth += (code[end] == '{') - (code[end] == '}')
                end += 1
        start_line = source.count('\n', 0, attribute.start())
        end_line = source.count('\n', 0, end)
        excluded.update(range(start_line, end_line + 1))
        items += 1
    physical = len(source.splitlines())
    return physical - len(excluded), items, len(excluded)


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('revision', nargs='?', default='HEAD')
    args = parser.parse_args()
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
