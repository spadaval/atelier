import { useEffect, useSyncExternalStore } from "react";
import type { ApiError } from "../api/ApiError";

export type Loaded<T> = { data?: T; error?: string; status?: number; loading: boolean };

type Entry = { value: Loaded<unknown>; at: number; inflight: boolean; listeners: Set<() => void> };
const store = new Map<string, Entry>();

function entry(path: string): Entry {
  let e = store.get(path);
  if (!e) store.set(path, (e = { value: { loading: true }, at: 0, inflight: false, listeners: new Set() }));
  return e;
}

async function refresh(path: string) {
  const e = entry(path);
  if (e.inflight) return;
  e.inflight = true;
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
  e.inflight = false;
  e.at = Date.now();
  // Keep the old object when nothing changed, so a poll repaints nothing.
  if (JSON.stringify(next) !== JSON.stringify(e.value)) {
    e.value = next;
    for (const listener of e.listeners) listener();
  }
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
    if (Date.now() - entry(path).at > 2_000) void refresh(path);
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
  missions: (repo: string) => `/api/repos/${seg(repo)}/missions`,
  mission: (repo: string, id: string) => `/api/repos/${seg(repo)}/missions/${seg(id)}`,
  item: (repo: string, id: string) => `/api/repos/${seg(repo)}/items/${seg(id)}`,
  doc: (repo: string, path: string) => `/api/repos/${seg(repo)}/doc?path=${seg(path)}`,
};

/** The app's own routes. */
export const to = {
  repo: (repo: string) => `/r/${seg(repo)}`,
  mission: (repo: string, id: string) => `/r/${seg(repo)}/mission/${seg(id)}`,
  item: (repo: string, id: string) => `/r/${seg(repo)}/item/${seg(id)}`,
  doc: (repo: string, mission: string, path: string) => `/r/${seg(repo)}/mission/${seg(mission)}?tab=docs&doc=${seg(path)}`,
};
