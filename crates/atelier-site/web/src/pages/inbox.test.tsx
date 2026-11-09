import { cleanup, screen, within } from "@testing-library/react";
import { afterEach, describe, expect, it } from "vitest";
import { renderAt, serveFixtures } from "../test/api";

afterEach(() => cleanup());

/** The section under the `h2` named `title`. */
function section(title: string): HTMLElement {
  return screen.getByRole("heading", { level: 2, name: title }).closest("section")!;
}

/** The list row in `container` holding the item `id`. */
function row(container: HTMLElement, id: string): HTMLElement {
  return within(container).getByText(id, { selector: ".item > code" }).closest("li")!;
}

describe("the inbox", () => {
  it("lists each repository's needs-human items with their item and mission pages", async () => {
    serveFixtures();
    renderAt("/inbox");
    await screen.findByRole("heading", { level: 1, name: "Inbox" });
    expect(screen.getByText("4 entries wait on you.")).toBeInTheDocument();
    expect(
      screen.getAllByRole("heading", { level: 2 }).map((heading) => heading.textContent),
    ).toEqual(["alpha", "beta"]);

    const alpha = section("alpha");
    expect(within(alpha).getByRole("link", { name: "alpha" })).toHaveAttribute("href", "/alpha");
    // A decision in draft alpha-m1's scope links its brief.
    const decision = row(alpha, "alpha-a2");
    expect(within(decision).getByRole("link", { name: /Choose the server/ })).toHaveAttribute(
      "href",
      "/alpha/item/alpha-a2",
    );
    expect(
      within(decision).getByRole("link", { name: "Brief: Read plans on the base branch" }),
    ).toHaveAttribute("href", "/alpha/mission/alpha-m1");
    // A loose end blocked by done mission alpha-m2 links its debrief.
    const looseEnd = row(alpha, "alpha-l2");
    expect(within(looseEnd).getByRole("link", { name: "Debrief: Ship the loader" })).toHaveAttribute(
      "href",
      "/alpha/mission/alpha-m2",
    );
    // A draft mission with a planning approval, linking its brief.
    expect(within(alpha).getByRole("heading", { level: 3, name: "Draft Missions to Approve" })).toBeInTheDocument();
    expect(row(alpha, "alpha-m1").querySelector("a")).toHaveAttribute("href", "/alpha/mission/alpha-m1");

    // A validation related only to a superseded mission links only its item.
    const validation = row(section("beta"), "beta-v1");
    expect(within(validation).getAllByRole("link").map((link) => link.getAttribute("href"))).toEqual([
      "/beta/item/beta-v1",
    ]);
  });

  it("is linked, with its count, from the repository list and every board", async () => {
    serveFixtures();
    for (const [path, title] of [
      ["/", "Repositories"],
      ["/alpha", "Board: alpha"],
      ["/beta", "Board: beta"],
    ]) {
      renderAt(path);
      await screen.findByRole("heading", { level: 1, name: title });
      expect(await screen.findByRole("link", { name: "Inbox (4)" })).toHaveAttribute("href", "/inbox");
      cleanup();
    }
  });

  it("says when nothing waits", async () => {
    serveFixtures({ "/api/inbox": { count: 0, repos: [] } });
    renderAt("/inbox");
    expect(await screen.findByText("Nothing waits on you.")).toBeInTheDocument();
  });
});
