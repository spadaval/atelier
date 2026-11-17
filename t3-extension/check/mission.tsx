// The page script of test-mission.mjs: draws the mission page of `?scenario=` (check/mission-fixtures.ts)
// through mountMissionLayer, as the layer does, at `?width=` pixels, with a stand-in injector that
// answers the fixture's reads, and records what the page asks to open in `missionCalls`.
import { mountMissionLayer } from "../src/mission/mount";
import { NOW, REPO, scenarios } from "./mission-fixtures";

const params = new URLSearchParams(location.search);
const chosen = scenarios[params.get("scenario") ?? ""];
const calls: string[][] = [];
const reads: string[] = [];
const errors: string[] = [];
Object.assign(globalThis, { missionCalls: calls, missionReads: reads, missionErrors: errors });
addEventListener("error", (event) => errors.push(String(event.error?.stack ?? event.message)));

const host = document.body.appendChild(document.createElement("div"));
host.id = "mission";
host.style.cssText = `width: ${Number(params.get("width") ?? 1000)}px; height: 900px; outline: 1px solid var(--border)`;
const api = {
  request: (action: string, payload?: unknown) => {
    const { path } = payload as { desk: string; path: string };
    reads.push(path);
    if (action !== "desk-get" || !chosen || !(path in chosen.answers)) return Promise.reject(new Error(`GET ${path}: the fixture has no answer`));
    return Promise.resolve(structuredClone(chosen.answers[path]));
  },
  onCleanup: () => {},
  reportError: (error: unknown) => errors.push(String(error)),
  signal: new AbortController().signal,
};
mountMissionLayer(host, api).show({
  desk: "http://desk.invalid",
  repo: REPO,
  id: chosen?.id ?? "atelier-none",
  now: () => NOW,
  onClose: () => calls.push(["close"]),
  openThread: (identity) => calls.push(["thread", identity]),
  openItem: (id) => calls.push(["item", id]),
  openUrl: (url) => calls.push(["url", url]),
});
