/**
 * A host's theme, mapped onto the Desk's tokens (the custom properties at the
 * top of `styles.css`). What the host sends is data: a colour is kept only if
 * it parses as one, a font only if it parses as a family list, and anything
 * else is dropped, so its token keeps the value it had.
 */

/** The colours a host may send, by the name it sends them under. */
export const HOST_COLOURS = ["background", "foreground", "surface", "border", "muted", "accent"] as const;
export type HostColour = (typeof HOST_COLOURS)[number];

/** What the host has sent so far that parsed, merged across messages. */
export type HostTheme = { colours: Partial<Record<HostColour, string>>; font?: string; scheme?: "light" | "dark" };

/** Whether the browser parses `value` for `property`; injected so tests need no browser. */
export type Supports = (property: string, value: string) => boolean;

const browserSupports: Supports = (property, value) => typeof CSS !== "undefined" && CSS.supports(property, value);

/** Words that parse as a colour but name another value, not a colour of the host's. */
const NOT_A_COLOUR = /^(inherit|initial|unset|revert|revert-layer|currentcolor)$/i;
/** Functions that reach outside the value itself. */
const REACHES_OUT = /\b(var|env|attr|url|image|calc)\s*\(/i;

/** `value` as a colour, or `null` when it is not one. */
export function colour(value: unknown, supports: Supports = browserSupports): string | null {
  if (typeof value !== "string") return null;
  const v = value.trim();
  if (!v || v.length > 100 || !/^[#\w\s(),.%/+-]+$/.test(v)) return null;
  if (NOT_A_COLOUR.test(v) || REACHES_OUT.test(v)) return null;
  return supports("color", v) ? v : null;
}

/** `value` as a font family list, or `null` when it is not one. */
export function fontFamily(value: unknown, supports: Supports = browserSupports): string | null {
  if (typeof value !== "string") return null;
  const v = value.trim();
  if (!v || v.length > 200 || !/^[\w\s"',.-]+$/.test(v)) return null;
  if (NOT_A_COLOUR.test(v)) return null;
  return supports("font-family", v) ? v : null;
}

/**
 * `theme` with what parses in `message` merged over it. A field that does
 * not parse is dropped, and the value before it stands.
 */
export function merge(theme: HostTheme, message: Record<string, unknown>, supports: Supports = browserSupports): HostTheme {
  const next: HostTheme = { ...theme, colours: { ...theme.colours } };
  const sent = message.colors;
  if (sent && typeof sent === "object" && !Array.isArray(sent)) {
    for (const name of HOST_COLOURS) {
      const value = colour((sent as Record<string, unknown>)[name], supports);
      if (value) next.colours[name] = value;
    }
  }
  const font = fontFamily(message.font, supports);
  if (font) next.font = font;
  // A scheme holds for the background it came with: a new background sent
  // without one has its scheme read from the colour.
  if (message.scheme === "light" || message.scheme === "dark") next.scheme = message.scheme;
  else if (next.colours.background !== theme.colours.background) delete next.scheme;
  return next;
}

const mix = (a: string, share: number, b: string) => `color-mix(in oklab, ${a} ${share}%, ${b})`;

/**
 * The Desk's tokens for `theme`: those the host sent, and those derived from
 * them. A token neither sent nor derivable is absent, so the Desk's own value
 * stands. Derived tokens refer to `--bg` and `--ink`, so they follow whichever
 * of the two the host set and the Desk's own for the other.
 */
export function tokens(theme: HostTheme): Record<string, string> {
  const c = theme.colours;
  const out: Record<string, string> = {};
  if (c.background) {
    out["--bg"] = c.background;
    out["--canvas"] = mix("var(--bg)", 94, "#000");
  }
  if (c.foreground) out["--ink"] = c.foreground;
  if (c.background || c.foreground) {
    out["--panel"] = c.surface ?? mix("var(--bg)", 97, "var(--ink)");
    out["--raised"] = c.surface ?? mix("var(--bg)", 94, "var(--ink)");
    out["--paper"] = c.surface ?? mix("var(--bg)", 98, "var(--ink)");
    out["--hover"] = mix("var(--bg)", 90, "var(--ink)");
    out["--line"] = c.border ?? mix("var(--bg)", 87, "var(--ink)");
    out["--line-strong"] = mix("var(--line)", 75, "var(--ink)");
    out["--grid"] = "var(--line)";
    out["--ink-2"] = mix("var(--ink)", 75, "var(--bg)");
    out["--ink-3"] = c.muted ?? mix("var(--ink)", 58, "var(--bg)");
  } else {
    if (c.surface) Object.assign(out, { "--panel": c.surface, "--raised": c.surface, "--paper": c.surface });
    if (c.border) out["--line"] = c.border;
    if (c.muted) out["--ink-3"] = c.muted;
  }
  if (c.accent) {
    out["--you"] = c.accent;
    out["--you-text"] = c.accent;
    out["--you-wash"] = "color-mix(in srgb, var(--you) 12%, transparent)";
  }
  if (theme.font) out["--sans"] = theme.font;
  return out;
}

/** Whether a colour the browser computed as `rgb(r, g, b)` is light; `null` when it cannot tell. */
export function isLight(computed: string): boolean | null {
  const m = /^rgba?\(\s*([\d.]+)[\s,]+([\d.]+)[\s,]+([\d.]+)/.exec(computed);
  if (!m) return null;
  const [r, g, b] = [m[1], m[2], m[3]].map((x) => {
    const s = Number(x) / 255;
    return s <= 0.04045 ? s / 12.92 : ((s + 0.055) / 1.055) ** 2.4;
  });
  return 0.2126 * r + 0.7152 * g + 0.0722 * b > 0.18;
}

/** The scheme for `theme`: the one sent, else the one its background implies. */
function scheme(theme: HostTheme): "light" | "dark" | null {
  if (theme.scheme) return theme.scheme;
  if (!theme.colours.background) return null;
  const probe = document.createElement("span");
  probe.style.color = theme.colours.background;
  document.body.append(probe);
  const light = isLight(getComputedStyle(probe).color);
  probe.remove();
  return light === null ? null : light ? "light" : "dark";
}

/** The tokens set by the last theme applied, to clear those a later one drops. */
let applied: string[] = [];

/** Puts `theme` on the page: its tokens, and the light or dark set of the Desk's own colours to go with them. */
export function apply(theme: HostTheme) {
  const root = document.documentElement;
  const set = tokens(theme);
  for (const name of applied) if (!(name in set)) root.style.removeProperty(name);
  for (const [name, value] of Object.entries(set)) root.style.setProperty(name, value);
  applied = Object.keys(set);
  const s = scheme(theme);
  if (s === "light") root.dataset.theme = "light";
  else if (s === "dark") delete root.dataset.theme;
}
