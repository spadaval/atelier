import { useEffect, useState, useSyncExternalStore } from "react";
import type { ApiError } from "../api/ApiError";
import type { ThreadInfo } from "../api/ThreadInfo";

export type Loaded<T> = { data?: T; error?: string; status?: number; loading: boolean };

/** One path's answer, the read under way, and the read promised to follow it. */
type Entry = { value: Loaded<unknown>; at: number; inflight: Promise<void> | null; next: Promise<void> | null; listeners: Set<() => void> };
const store = new Map<string, Entry>();

function entry(path: string): Entry {
  let e = store.get(path);
  if (!e) store.set(path, (e = { value: { loading: true }, at: 0, inflight: null, next: null, listeners: new Set() }));
  return e;
}

/**
 * Reads `path` again, and resolves once a read that started after this call
 * has its answer. A read already under way may have started before the
 * change this call is for, such as a commit or an action, so its answer is not
 * enough: one more read follows it, shared by every caller that asks while
 * it waits.
 */
export function refresh(path: string): Promise<void> {
  const e = entry(path);
  if (!e.inflight) {
    e.inflight = fetchInto(path, e).finally(() => (e.inflight = null));
    return e.inflight;
  }
  e.next ??= e.inflight.then(() => {
    e.next = null;
    return refresh(path);
  });
  return e.next;
}

async function fetchInto(path: string, e: Entry) {
  let next: Loaded<unknown>;
  try {
    const response = await fetch(path, { headers: { accept: "application/json" } });
    const body: unknown = await response.json().catch(() => null);
    next = response.ok
      ? { data: body, loading: false }
      : { data: e.value.data, error: (body as ApiError | null)?.error ?? `${path} answered ${response.status}`, status: response.status, loading: false };
  } catch (error) {
    next = { data: e.value.data, error: `Cannot reach the Desk's server: ${String(error)}`, status: 0, loading: false };
  }
  e.at = Date.now();
  // Keep the old object when nothing changed, so a poll repaints nothing.
  if (JSON.stringify(next) !== JSON.stringify(e.value)) {
    e.value = next;
    for (const listener of e.listeners) listener();
  }
}

/**
 * Reads every page the app has open again, after an action changed the
 * records, and resolves once each has an answer read after the action.
 * Pages not on screen are dropped, to be read when next opened.
 */
export async function reloadAll(): Promise<void> {
  const shown: Promise<void>[] = [];
  for (const [path, e] of store) {
    if (e.listeners.size === 0) {
      store.delete(path);
      continue;
    }
    shown.push(refresh(path));
  }
  await Promise.all(shown);
}

const IDLE: Loaded<unknown> = { loading: false };

/** `GET path`, kept fresh: shown from cache at once, fetched again every `interval` ms while the page is visible. */
export function useData<T>(path: string | null, interval = 30_000): Loaded<T> {
  const value = useSyncExternalStore(
    (listener) => {
      if (!path) return () => undefined;
      const e = entry(path);
      e.listeners.add(listener);
      return () => e.listeners.delete(listener);
    },
    () => (path ? entry(path).value : IDLE),
  );
  useEffect(() => {
    if (!path) return;
    // On opening, a read under way is new enough; a poll asks for a read after it.
    const e = entry(path);
    if (!e.inflight && Date.now() - e.at > 2_000) void refresh(path);
    const tick = () => {
      if (document.visibilityState === "visible") void refresh(path);
    };
    const timer = setInterval(tick, interval);
    document.addEventListener("visibilitychange", tick);
    return () => {
      clearInterval(timer);
      document.removeEventListener("visibilitychange", tick);
    };
  }, [path, interval]);
  return value as Loaded<T>;
}

const seg = encodeURIComponent;

/** The API's paths, in one place. */
export const api = {
  repos: () => "/api/repos",
  inbox: () => "/api/inbox",
  threads: () => "/api/threads",
  proposed: () => "/api/proposed",
  motion: () => "/api/motion",
  disputed: () => "/api/disputed",
  missions: (repo: string) => `/api/repos/${seg(repo)}/missions`,
  mission: (repo: string, id: string) => `/api/repos/${seg(repo)}/missions/${seg(id)}`,
  item: (repo: string, id: string) => `/api/repos/${seg(repo)}/items/${seg(id)}`,
  history: (repo: string, opts: { days?: number; ids?: string[] } = {}) =>
    `/api/history?repo=${seg(repo)}${opts.days ? `&days=${opts.days}` : ""}${opts.ids ? `&ids=${seg(opts.ids.join(","))}` : ""}`,
  checks: (repo: string, pr: number) => `/api/checks?repo=${seg(repo)}&pr=${pr}`,
  roadmap: (repo: string) => `/api/roadmap?repo=${seg(repo)}`,
  thread: (id: string, full = false) => `/api/thread?id=${seg(id)}${full ? "&full=1" : ""}`,
  record: (id: string) => `/api/records/${seg(id)}`,
  ties: (thread: string) => `/api/ties?thread=${seg(thread)}`,
  doc: (repo: string, path: string, at?: string | null) => `/api/repos/${seg(repo)}/doc?path=${seg(path)}${at ? `&at=${seg(at)}` : ""}`,
};

/**
 * Several paths read together, and again every `interval` ms while the page
 * is visible. Each answer is the body, or null where its read failed, so one
 * failed read leaves the rest drawn.
 */
export function useAll<T>(paths: string[], interval = 30_000): { data: (T | null)[]; loading: boolean } {
  const key = paths.join("\n");
  const [value, setValue] = useState<{ key: string; data: (T | null)[] }>({ key: "", data: [] });
  useEffect(() => {
    let stale = false;
    const list = key === "" ? [] : key.split("\n");
    const read = async () => {
      const data = await Promise.all(
        list.map(async (path) => {
          try {
            const response = await fetch(path, { headers: { accept: "application/json" } });
            return response.ok ? ((await response.json()) as T) : null;
          } catch {
            return null;
          }
        }),
      );
      if (!stale) setValue((old) => (old.key === key && JSON.stringify(old.data) === JSON.stringify(data) ? old : { key, data }));
    };
    void read();
    const timer = setInterval(() => {
      if (document.visibilityState === "visible") void read();
    }, interval);
    return () => {
      stale = true;
      clearInterval(timer);
    };
  }, [key, interval]);
  return value.key === key ? { data: value.data, loading: false } : { data: paths.map(() => null), loading: paths.length > 0 };
}

/**
 * Each identity's thread as T3 Code knows it, live or ended, read with
 * `GET /api/thread`; with `full`, its latest run and line, read more often.
 * An identity T3 Code does not know is left out.
 */
export function useThreads(ids: string[], full = false): Map<string, ThreadInfo> {
  const unique = [...new Set(ids)];
  const read = useAll<ThreadInfo>(
    unique.map((id) => api.thread(id, full)),
    full ? 15_000 : 60_000,
  );
  const map = new Map<string, ThreadInfo>();
  read.data.forEach((info, at) => info && map.set(unique[at], info));
  return map;
}

/** The app's own routes. */
export const to = {
  repo: (repo: string) => `/r/${seg(repo)}`,
  home: (repo: string) => `/home/${seg(repo)}`,
  control: (repo: string, id: string) => `/mission/${seg(repo)}/${seg(id)}`,
  agents: (repo?: string) => (repo === undefined ? "/agents" : `/r/${seg(repo)}/agents`),
  proposed: () => "/proposed",
  motion: () => "/motion",
  disputed: () => "/disputed",
  mission: (repo: string, id: string) => `/r/${seg(repo)}/mission/${seg(id)}`,
  item: (repo: string, id: string) => `/r/${seg(repo)}/item/${seg(id)}`,
  doc: (repo: string, mission: string, path: string) => `/r/${seg(repo)}/mission/${seg(mission)}?tab=docs&doc=${seg(path)}`,
};
