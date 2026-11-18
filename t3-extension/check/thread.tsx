// The page script of test-thread.mjs: draws This thread for `?scenario=` (check/thread-fixtures.ts)
// through mountThisThread, as the panel does, at `?width=` pixels, with a stand-in injector that
// answers the fixture's reads (failing `?fail=<path>` on request), and records what the page asks
// to open in `threadCalls`.
import type { HostApi } from "../src/react/desk";
import { mountThisThread } from "../src/thread/mount";
import { missing, NOW, REPO, scenarios } from "./thread-fixtures";

const params = new URLSearchParams(location.search);
const chosen = scenarios[params.get("scenario") ?? ""];
const calls: unknown[][] = [];
const reads: string[] = [];
const errors: string[] = [];
Object.assign(globalThis, { threadCalls: calls, threadReads: reads, threadErrors: errors });
addEventListener("error", (event) => errors.push(String(event.error?.stack ?? event.message)));

const host = document.body.appendChild(document.createElement("div"));
host.id = "thread";
host.style.cssText = `width: ${Number(params.get("width") ?? 520)}px; height: 900px; outline: 1px solid var(--border)`;
const api = {
  request: ((action: string, payload?: unknown) => {
    const { path } = payload as { desk: string; path: string };
    reads.push(path);
    if (path === params.get("fail")) return Promise.reject(Object.assign(new Error(`GET ${path}: HTTP 502: the fixture failed it`), { status: 502 }));
    if (missing.has(path)) return Promise.reject(Object.assign(new Error(`GET ${path}: HTTP 404: no record`), { status: 404 }));
    if (action !== "desk-get" || !chosen || !(path in chosen.answers)) return Promise.reject(new Error(`GET ${path}: the fixture has no answer`));
    return Promise.resolve(structuredClone(chosen.answers[path]));
  }) as HostApi["request"],
  onCleanup: () => {},
  reportError: (error: unknown) => errors.push(String(error)),
  signal: new AbortController().signal,
};
mountThisThread(host, api).show({
  desk: "http://desk.invalid",
  repository: REPO,
  thread: chosen?.thread ?? "thread:none",
  children: chosen?.children ?? [],
  mentioned: chosen?.mentioned ?? [],
  now: () => NOW,
  openItem: (repo, id) => calls.push(["item", repo, id]),
  openMission: (repo, id, proposed) => calls.push(["mission", repo, id, proposed]),
  openInbox: () => calls.push(["inbox"]),
  openThread: (identity) => calls.push(["thread", identity]),
  openUrl: (url) => calls.push(["url", url]),
});
