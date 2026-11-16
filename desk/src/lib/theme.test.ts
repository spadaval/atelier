import { describe, expect, it } from "vitest";
import { colour, fontFamily, isLight, merge, tokens, type Supports } from "./theme";

// Stands in for CSS.supports: hex colours, rgb(), and the named colours below parse.
const supports: Supports = (property, value) =>
  property === "font-family" || /^#[0-9a-f]{3,8}$/i.test(value) || /^rgba?\(/.test(value) || /^(red|white|black|inherit|currentcolor)$/i.test(value);

describe("colour", () => {
  it("keeps a value that parses as a colour", () => {
    expect(colour("#101010", supports)).toBe("#101010");
    expect(colour(" rgb(1, 2, 3) ", supports)).toBe("rgb(1, 2, 3)");
    expect(colour("red", supports)).toBe("red");
  });
  it("drops what does not parse, or names something other than a colour", () => {
    for (const bad of ["not a colour", "", "#12", "inherit", "currentColor", "var(--bg)", "rgb(var(--x))", "red; color: blue", "url(x)", 3, null, {}])
      expect(colour(bad, supports), String(bad)).toBeNull();
  });
});

describe("fontFamily", () => {
  it("keeps a family list and drops anything that could reach outside it", () => {
    expect(fontFamily('"Inter", system-ui, sans-serif', supports)).toBe('"Inter", system-ui, sans-serif');
    expect(fontFamily("Inter; color: red", supports)).toBeNull();
    expect(fontFamily("var(--x)", supports)).toBeNull();
    expect(fontFamily(12, supports)).toBeNull();
  });
});

describe("merge", () => {
  it("merges each message over the last, dropping a value that does not parse", () => {
    const first = merge({ colours: {} }, { type: "host:theme", colors: { background: "#ffffff", foreground: "#111111" }, font: "Inter" }, supports);
    expect(first).toEqual({ colours: { background: "#ffffff", foreground: "#111111" }, font: "Inter" });
    const second = merge(first, { colors: { background: "#000000", foreground: "nonsense", shadow: "#123456" }, scheme: "dark" }, supports);
    expect(second).toEqual({ colours: { background: "#000000", foreground: "#111111" }, font: "Inter", scheme: "dark" });
    expect(merge(second, { colors: "red", scheme: "blue" }, supports)).toEqual(second);
    // A new background without a scheme drops the scheme sent with the last one.
    expect(merge(second, { colors: { background: "#fafafa" } }, supports)).toEqual({ colours: { background: "#fafafa", foreground: "#111111" }, font: "Inter" });
  });
});

describe("tokens", () => {
  it("maps the host's colours onto the Desk's tokens and derives the rest", () => {
    const set = tokens({ colours: { background: "#fff", foreground: "#111" }, font: "Inter" });
    expect(set["--bg"]).toBe("#fff");
    expect(set["--ink"]).toBe("#111");
    expect(set["--sans"]).toBe("Inter");
    for (const derived of ["--canvas", "--panel", "--raised", "--hover", "--line", "--line-strong", "--ink-2", "--ink-3"]) expect(set[derived], derived).toMatch(/var\(--/);
    expect(set["--you"]).toBeUndefined();
  });
  it("uses a colour the host sent over the one it would derive", () => {
    const set = tokens({ colours: { background: "#fff", border: "#ccc", muted: "#777", surface: "#eee", accent: "#06f" } });
    expect([set["--line"], set["--ink-3"], set["--panel"], set["--you"]]).toEqual(["#ccc", "#777", "#eee", "#06f"]);
  });
  it("sets nothing for a theme with nothing in it", () => {
    expect(tokens({ colours: {} })).toEqual({});
  });
});

describe("isLight", () => {
  it("tells light from dark by luminance", () => {
    expect(isLight("rgb(255, 255, 255)")).toBe(true);
    expect(isLight("rgb(15, 16, 14)")).toBe(false);
    expect(isLight("oklch(0.5 0.1 200)")).toBeNull();
  });
});
