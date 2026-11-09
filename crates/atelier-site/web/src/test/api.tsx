// Renders the app against atelier-zpmb's fixture JSON: each file holds a
// route, the status the server answered and its body.
import { render } from "@testing-library/react";
import { MemoryRouter } from "react-router";
import { vi } from "vitest";
import { App } from "../App";

type Fixture = { route: string; status: number; body: unknown };

const files = import.meta.glob<Fixture>("../fixtures/**/*.json", { eager: true, import: "default" });

/** Every fixture response, by route. */
export const fixtures: Map<string, Fixture> = new Map(
  Object.values(files).map((fixture) => [fixture.route, fixture]),
);

/** A fixture's body, deep-copied so a test may change it. */
export function body<T>(route: string): T {
  const fixture = fixtures.get(route);
  if (!fixture) throw new Error(`no fixture for ${route}`);
  return structuredClone(fixture.body) as T;
}

/**
 * Answers `fetch` from the fixtures, with `overrides` replacing the body of
 * a route; any other route is a 404 like the server's.
 */
export function serveFixtures(overrides: Record<string, unknown> = {}) {
  const fetch = vi.fn(async (input: RequestInfo | URL) => {
    const path = typeof input === "string" ? input : input instanceof URL ? input.pathname : input.url;
    const decoded = decodeURIComponent(path);
    if (decoded in overrides) return Response.json(overrides[decoded], { status: 200 });
    const fixture = fixtures.get(decoded);
    if (!fixture) return Response.json({ error: `no such route ${decoded}` }, { status: 404 });
    return Response.json(fixture.body, { status: fixture.status });
  });
  vi.stubGlobal("fetch", fetch);
  return fetch;
}

export function renderAt(path: string) {
  return render(
    <MemoryRouter initialEntries={[path]}>
      <App />
    </MemoryRouter>,
  );
}
