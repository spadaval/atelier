import { describe, expect, it } from "vitest";
import { resolveLink, slug, splitFrontMatter } from "./links";

const design = ".atelier/missions/atelier-z4cr/design.md";

describe("resolveLink", () => {
  it("resolves a link that climbs out of a mission's directory to the repository's file", () => {
    expect(resolveLink(design, "../../../docs/adr/0032-the-desk-is-the-humans-product.md")).toEqual({
      kind: "doc",
      path: "docs/adr/0032-the-desk-is-the-humans-product.md",
      hash: "",
    });
  });

  it("resolves a sibling document and keeps its anchor", () => {
    expect(resolveLink(design, "notes.md#open-questions")).toEqual({ kind: "doc", path: ".atelier/missions/atelier-z4cr/notes.md", hash: "open-questions" });
    expect(resolveLink(design, "./notes.md")).toEqual({ kind: "doc", path: ".atelier/missions/atelier-z4cr/notes.md", hash: "" });
  });

  it("reads a leading slash from the repository's root", () => {
    expect(resolveLink(design, "/docs/product/desk.md")).toEqual({ kind: "doc", path: "docs/product/desk.md", hash: "" });
  });

  it("decodes escaped characters in a path", () => {
    expect(resolveLink("docs/a.md", "b%20c.md")).toEqual({ kind: "doc", path: "docs/b c.md", hash: "" });
  });

  it("names a file that is not Markdown without opening it", () => {
    expect(resolveLink(design, "../../../crates/atelier-desk/src/api.rs")).toEqual({ kind: "file", path: "crates/atelier-desk/src/api.rs" });
  });

  it("refuses a path that leaves the repository", () => {
    expect(resolveLink("docs/a.md", "../../outside.md")).toEqual({ kind: "outside" });
  });

  it("leaves web links and anchors alone", () => {
    expect(resolveLink(design, "https://example.com/x.md")).toEqual({ kind: "external", href: "https://example.com/x.md" });
    expect(resolveLink(design, "mailto:x@example.com")).toEqual({ kind: "external", href: "mailto:x@example.com" });
    expect(resolveLink(design, "#scope")).toEqual({ kind: "anchor", hash: "scope" });
  });
});

describe("splitFrontMatter", () => {
  it("reads the converted design document's front matter and leaves its body", () => {
    const text = '---\nstatus: draft\nlast_changed: "2026-11-11: amended"\n---\n\n# Sizing\n\nText.\n';
    expect(splitFrontMatter(text)).toEqual({ fields: [["status", "draft"], ["last_changed", "2026-11-11: amended"]], body: "\n# Sizing\n\nText.\n" });
  });

  it("leaves a file without front matter whole", () => {
    expect(splitFrontMatter("# Notes\n---\nx: y\n---\n")).toEqual({ fields: [], body: "# Notes\n---\nx: y\n---\n" });
  });
});

describe("slug", () => {
  it("makes GitHub's heading anchors", () => {
    expect(slug("Risks and Unknowns")).toBe("risks-and-unknowns");
    expect(slug("The `atelier log` (M5)")).toBe("the-atelier-log-m5");
  });
});
