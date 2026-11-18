// Copies what the extension borrows from a T3 Code checkout into src/t3/, which nothing else
// writes: T3's Tailwind variants and theme mapping (theme.css), its whole stylesheet for the
// component check (index.css, test-components.mjs), its `cn` (cn.ts), the components named below
// (ui/), its licence and the commit copied from (SOURCE). Run it again after a T3 Code update, then
// pnpm build and node run-tests.mjs (README.md, "Drawing with T3 Code's components").
//
//   node sync-t3.mjs [<t3code checkout>]     # default: $T3, else ~/projects/t3code
//
// It stops, naming the file and what it expected, where T3's source no longer has the form a
// rewrite below depends on; nothing is written then.
import fs from 'node:fs/promises';
import os from 'node:os';
import path from 'node:path';
import { execFileSync } from 'node:child_process';
import { fileURLToPath } from 'node:url';

const root = path.dirname(fileURLToPath(import.meta.url));
const checkout = path.resolve(process.argv[2] ?? process.env.T3 ?? path.join(os.homedir(), 'projects/t3code'));
const web = path.join(checkout, 'apps/web/src');
const out = path.join(root, 'src/t3');
// The components the extension draws. A new one is a new name here; its imports must map below.
const components = ['badge', 'kbd', 'tooltip', 'collapsible-section-header', 'button', 'switch', 'select', 'preview-card'];

function fail(message) {
  console.error(`sync-t3: ${message}. Nothing was written.`);
  process.exit(1);
}
const read = file => fs.readFile(path.join(checkout, file), 'utf8').catch(error => fail(`cannot read ${path.join(checkout, file)}: ${error.message}`));
const git = (...args) => execFileSync('git', ['-C', checkout, ...args], { encoding: 'utf8' }).trim();

const sources = ['apps/web/src/index.css', 'apps/web/src/lib/utils.ts', 'LICENSE', ...components.map(name => `apps/web/src/components/ui/${name}.tsx`)];
let commit;
try { commit = git('rev-parse', 'HEAD'); } catch (error) { fail(`${checkout} is not a git checkout of T3 Code: ${error.message}`); }
const changed = git('status', '--porcelain', '--', ...sources);
if (changed) fail(`${checkout} has uncommitted changes in the files copied, so commit ${commit} would not describe them:\n${changed}`);
const header = (comment, file) => `${comment} Copied by sync-t3.mjs from T3 Code (MIT, see LICENSE), ${file} at ${commit.slice(0, 12)}. Do not edit: run sync-t3.mjs.`;
const files = {};

// T3 marks its appearance with classes on <html>, which a selector inside a shadow root cannot
// see; :host-context() can. Each variant must have exactly the form below, or the rewrite would
// quietly leave T3's selector in place and the variant would never match.
const variants = [
  ['@custom-variant dark (&:is(.dark, .dark *));', '@custom-variant dark (:host-context(.dark) &);'],
  ['@custom-variant light (&:not(.dark, .dark *));', '@custom-variant light (:host-context(html:not(.dark)) &);'],
  ['@custom-variant wco (&:is(.wco, .wco *));', '@custom-variant wco (:host-context(.wco) &);'],
];
const css = await read('apps/web/src/index.css');
const lines = css.split('\n');
for (const [from] of variants) {
  if (!lines.includes(from)) fail(`apps/web/src/index.css no longer has the line \`${from}\`: T3's form of the variant changed, so update its :host-context rewrite in sync-t3.mjs`);
}
// Every top-level @custom-variant, @theme and @utility, as written; a block ends at the first `}`
// in column 0. Tailwind emits a utility only where a class uses it.
// A plain `@theme` holds the variables T3 puts on its root and changes at runtime (Settings ->
// Appearance sets --font-sans and --font-mono on <html>). Declared again on the shadow root they
// would hide T3's, so ours is `@theme reference`: utilities read the variable inherited from T3.
const theme = [];
let open = false;
for (const line of lines) {
  if (/^@(theme( inline)?|custom-variant [\w-]+|utility [\w-]+) /.test(line)) open = true;
  if (!open) continue;
  theme.push(variants.find(([from]) => from === line)?.[1] ?? (line === '@theme {' ? '@theme reference {' : line));
  if (line === '}' || /^@custom-variant [^{]*;$/.test(line)) open = false;
}
if (open) fail('apps/web/src/index.css: a @theme, @custom-variant or @utility block has no closing `}` in column 0');
// T3's element defaults from its base layers: without them a bare `border` takes the text colour,
// not T3's border colour, and a lucide icon is drawn heavier than T3 draws it. The rest of those
// layers styles T3's own page and is left out.
const base = [
  '  * {\n    @apply border-border outline-ring/50;\n  }\n',
  '  *,\n  ::before,\n  ::after {\n    outline-offset: -2px;\n  }\n',
  '  :where(svg.lucide[stroke-width="2"]) {\n    stroke-width: 1.75;\n  }\n',
];
for (const rule of base) {
  if (!css.includes(rule)) fail(`apps/web/src/index.css no longer has, in its base layer:\n${rule}update the base rules copied in sync-t3.mjs`);
}
theme.push('', '@layer base {', ...base.map(rule => rule.trimEnd()), '}');
files['theme.css'] = `${header('/*', 'apps/web/src/index.css')} */\n${theme.join('\n')}\n`;
files['index.css'] = `${header('/*', 'apps/web/src/index.css')} The component check builds it (vite.check.config.ts). */\n${css}`;

// T3's `cn` and the tailwind-merge it configures, without the rest of lib/utils.
const utils = await read('apps/web/src/lib/utils.ts');
const twMerge = utils.match(/^const twMerge = extendTailwindMerge\(\{\n[\s\S]*?\n\}\);$/m)?.[0];
const cn = utils.match(/^export function cn\(\.\.\.inputs: CxOptions\) \{\n[\s\S]*?\n\}$/m)?.[0];
if (!twMerge || !cn) fail('apps/web/src/lib/utils.ts no longer has `const twMerge = extendTailwindMerge({...});` and `export function cn(...inputs: CxOptions) {...}` in column 0: update sync-t3.mjs');
files['cn.ts'] = `${header('//', 'apps/web/src/lib/utils.ts (twMerge and cn)')}\nimport { type CxOptions, cx } from "class-variance-authority";\nimport { extendTailwindMerge } from "tailwind-merge";\n\n${twMerge}\n\n${cn}\n`;

// Imports from T3's app become ours; any other import of the app stops the copy.
const imports = [
  [/(["'])(?:~\/|(?:\.\.\/)+)lib\/utils\1/g, '"../cn"'],
  [/(["'])~\/lib\/windowControlsOverlay\1/g, '"../../react/windowControlsOverlay"'],
];
// Base UI portals a popup (a tooltip, a select's list, a preview card) to <body>, outside the
// shadow root its styles live in; the mount gives each portal a container inside it
// (src/react/mount.tsx, usePortalContainer).
const portal = primitive => [
  [`    <${primitive}.Portal>\n`, `    <${primitive}.Portal container={usePortalContainer()}>\n`],
  ['import { cn } from "../cn";\n', 'import { cn } from "../cn";\nimport { usePortalContainer } from "../../react/portal";\n'],
];
const rewrites = {
  tooltip: portal('TooltipPrimitive'),
  select: portal('SelectPrimitive'),
  // A record link's card is placed at an ID in T3's conversation, a Range and not an element
  // React draws, so the popup passes Base UI's anchor to its positioner, as T3's SelectPopup does.
  'preview-card': [
    ...portal('PreviewCardPrimitive'),
    ['  sideOffset = 6,\n  ...props\n', '  sideOffset = 6,\n  anchor,\n  ...props\n'],
    ['  sideOffset?: PreviewCardPrimitive.Positioner.Props["sideOffset"];\n}) {', '  sideOffset?: PreviewCardPrimitive.Positioner.Props["sideOffset"];\n  anchor?: PreviewCardPrimitive.Positioner.Props["anchor"];\n}) {'],
    ['        align={align}\n', '        align={align}\n        anchor={anchor}\n'],
  ],
};
for (const name of components) {
  const file = `apps/web/src/components/ui/${name}.tsx`;
  let source = await read(file);
  for (const [pattern, replacement] of imports) source = source.replace(pattern, replacement);
  for (const [from, to] of rewrites[name] ?? []) {
    if (source.split(from).length !== 2) fail(`${file} no longer has exactly one \`${from.trim()}\`: update its rewrite in sync-t3.mjs`);
    source = source.replace(from, to);
  }
  const app = [...source.matchAll(/from\s+["']((?:~|\.)[^"']*)["']/g)].map(match => match[1]).filter(specifier => !['../cn', '../../react/windowControlsOverlay', '../../react/portal'].includes(specifier));
  if (app.length) fail(`${file} imports ${app.join(', ')} from T3's app: map it to the extension's own in sync-t3.mjs, or copy that module too`);
  files[`ui/${name}.tsx`] = `${header('//', file)}\n${source}`;
}
files.LICENSE = await read('LICENSE');
const date = git('show', '-s', '--format=%cs', commit);
files.SOURCE = `T3 Code ${commit}, committed ${date}, copied from ${git('remote', 'get-url', 'origin').replace(/^git@github\.com:/, 'https://github.com/').replace(/\.git$/, '')}\nby sync-t3.mjs. Every other file here is generated from it; do not edit them.\n`;

await fs.rm(out, { recursive: true, force: true });
await fs.mkdir(path.join(out, 'ui'), { recursive: true });
for (const [name, text] of Object.entries(files)) await fs.writeFile(path.join(out, name), text);
console.log(`Copied from T3 Code ${commit.slice(0, 12)} (${date}) into ${path.relative(process.cwd(), out) || '.'}: ${Object.keys(files).join(', ')}`);
