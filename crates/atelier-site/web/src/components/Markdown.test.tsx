import { cleanup, render, screen } from "@testing-library/react";
import { afterEach, describe, expect, it } from "vitest";
import { Markdown, allowedUrl } from "./Markdown";

afterEach(() => cleanup());

const strategy = `# Plan

<script>window.injected = 1</script>

<div class="raw"><b>bold HTML</b></div>

Inline <span class="raw">span</span> HTML.

- [relative](../adr/0030.md)
- [fragment](#plan)
- [web](https://example.com/a)
- [plain web](http://example.com/b)
- [mail](mailto:someone@example.com)
- [script](javascript:alert(1))
- [spaced script](java&#9;script:alert(1))
- [upper script](JAVASCRIPT:alert(1))
- [data](data:text/html;base64,PHNjcmlwdD4=)
- [vb](vbscript:msgbox(1))
- [file](file:///etc/passwd)

![relative image](img/a.png)
![web image](https://example.com/a.png)
![data image](data:image/png;base64,AAAA)
![script image](javascript:alert(1))
`;

describe("Markdown", () => {
  it("skips raw HTML", () => {
    const { container } = render(<Markdown text={strategy} />);
    expect(container.querySelector("script")).toBeNull();
    expect(container.querySelector(".raw")).toBeNull();
    expect(container.querySelector("b")).toBeNull();
    expect(container.textContent).not.toContain("injected");
    expect((window as { injected?: number }).injected).toBeUndefined();
  });

  it("keeps relative, fragment, http, https and mailto links", () => {
    render(<Markdown text={strategy} />);
    for (const [name, href] of [
      ["relative", "../adr/0030.md"],
      ["fragment", "#plan"],
      ["web", "https://example.com/a"],
      ["plain web", "http://example.com/b"],
      ["mail", "mailto:someone@example.com"],
    ]) {
      expect(screen.getByRole("link", { name })).toHaveAttribute("href", href);
    }
  });

  it("renders any other scheme as plain text", () => {
    const { container } = render(<Markdown text={strategy} />);
    for (const name of ["script", "spaced script", "upper script", "data", "vb", "file"]) {
      expect(screen.queryByRole("link", { name })).toBeNull();
      expect(screen.getByText(name).tagName).toBe("SPAN");
    }
    const hrefs = Array.from(container.querySelectorAll("a")).map((a) => a.getAttribute("href"));
    expect(hrefs).toHaveLength(5);
  });

  it("keeps relative and web images and shows others as their alt text", () => {
    const { container } = render(<Markdown text={strategy} />);
    const srcs = Array.from(container.querySelectorAll("img")).map((img) => img.getAttribute("src"));
    expect(srcs).toEqual(["img/a.png", "https://example.com/a.png"]);
    expect(screen.getByText("data image").tagName).toBe("SPAN");
    expect(screen.getByText("script image").tagName).toBe("SPAN");
  });
});

describe("allowedUrl", () => {
  it("reads the scheme as a browser would", () => {
    expect(allowedUrl("docs/a.md")).toBe(true);
    expect(allowedUrl("/abs/path")).toBe(true);
    expect(allowedUrl("?q=1")).toBe(true);
    expect(allowedUrl("//example.com")).toBe(true);
    expect(allowedUrl(" javascript:alert(1)")).toBe(false);
    expect(allowedUrl("java\nscript:alert(1)")).toBe(false);
    expect(allowedUrl(":nothing")).toBe(false);
  });
});
