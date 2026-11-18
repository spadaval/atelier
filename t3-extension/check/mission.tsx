// The page script of test-mission.mjs: draws the mission page of `?scenario=` (check/mission-fixtures.ts)
// through mountNativeLayer, as the layer does, at `?width=` pixels, with a stand-in injector that
// answers the fixture's reads (failing one on request), and records what the page asks to open in `missionCalls`.
import { mountNativeLayer } from "../src/layer/native";
import { MissionPage } from "../src/mission/MissionPage";
import type { HostApi } from "../src/react/desk";
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
  request: ((action: string, payload?: unknown) => {
    const { path } = payload as { desk: string; path: string };
    reads.push(path);
    // `?fail=<path>&after=<n>`: that path answers its first n reads, then fails.
    if (path === params.get("fail") && reads.filter((read) => read === path).length > Number(params.get("after") ?? 0))
      return Promise.reject(new Error(`GET ${path}: HTTP 502: the fixture failed it`));
    if (action !== "desk-get" || !chosen || !(path in chosen.answers)) return Promise.reject(new Error(`GET ${path}: the fixture has no answer`));
    // An answer `{ sequence: [...] }` gives its first body to the first read, its second to the next, and its last ever after.
    const answer = chosen.answers[path] as { sequence?: unknown[] };
    const count = reads.filter((read) => read === path).length;
    return Promise.resolve(structuredClone(answer?.sequence ? answer.sequence[Math.min(count, answer.sequence.length) - 1] : answer));
  }) as HostApi["request"],
  onCleanup: () => {},
  reportError: (error: unknown) => errors.push(String(error)),
  signal: new AbortController().signal,
};
mountNativeLayer(host, api).show(
  <MissionPage
    desk="http://desk.invalid"
    repo={REPO}
    id={chosen?.id ?? "atelier-none"}
    now={() => NOW}
    onClose={() => calls.push(["close"])}
    openThread={(identity) => calls.push(["thread", identity])}
    openItem={(id) => calls.push(["item", id])}
    openUrl={(url) => calls.push(["url", url])}
  />,
);
