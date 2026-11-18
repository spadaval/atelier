// The page script of test-home.mjs: draws the project home of `?scenario=` (check/home-fixtures.ts)
// through mountNativeLayer, as the layer does, at `?width=` pixels, with a stand-in injector that
// answers the fixture's reads (failing one path on `?fail=`), and records what the page asks to
// open in `homeCalls`.
import { ProjectHome } from "../src/home/ProjectHome";
import { mountNativeLayer } from "../src/layer/native";
import type { HostApi } from "../src/react/desk";
import { NOW, REPO, scenarios } from "./home-fixtures";

const params = new URLSearchParams(location.search);
const answers = scenarios[params.get("scenario") ?? ""] ?? {};
const calls: unknown[][] = [];
const reads: string[] = [];
const errors: string[] = [];
Object.assign(globalThis, { homeCalls: calls, homeReads: reads, homeErrors: errors });
addEventListener("error", (event) => errors.push(String(event.error?.stack ?? event.message)));

const host = document.body.appendChild(document.createElement("div"));
host.id = "home";
host.style.cssText = `width: ${Number(params.get("width") ?? 1000)}px; height: 1100px; outline: 1px solid var(--border)`;
const api = {
  request: ((action: string, payload?: unknown) => {
    const { path } = payload as { desk: string; path: string };
    reads.push(path);
    if (path === params.get("fail")) return Promise.reject(new Error(`GET ${path}: HTTP 502: the fixture failed it`));
    if (action !== "desk-get" || !(path in answers)) return Promise.reject(new Error(`GET ${path}: the fixture has no answer`));
    return Promise.resolve(structuredClone(answers[path]));
  }) as HostApi["request"],
  onCleanup: () => {},
  reportError: (error: unknown) => errors.push(String(error)),
  signal: new AbortController().signal,
};
mountNativeLayer(host, api).show(
  <ProjectHome
    desk="http://desk.invalid"
    repo={REPO}
    now={() => NOW}
    onClose={() => calls.push(["close"])}
    openMission={(id, proposed) => calls.push(["mission", id, proposed])}
    openItem={(id) => calls.push(["item", id])}
  />,
);
