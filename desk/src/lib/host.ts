/**
 * The host contract: how the embedded Desk talks to the page that frames it,
 * over `postMessage`. `docs/architecture/desk.md`, "The host contract", is
 * its definition. The Desk talks only to its parent window, and only when
 * that window's origin is one the server lists (`GET /api/embed`); with no
 * listed parent it sends nothing and reads nothing, and the Desk works as it
 * does unframed. What the host sends is data: a theme is applied only where
 * it parses, and no message makes the Desk write, act or navigate.
 */
import { useSyncExternalStore } from "react";
import type { EmbedResponse } from "../api/EmbedResponse";
import { EMBED_BASE, embedded } from "./embed";
import { apply, merge, type HostTheme } from "./theme";

/** A message the Desk sends its host. */
export type DeskMessage =
  | { type: "desk:ready"; version: 1 }
  | { type: "desk:navigated"; path: string; title: string }
  | { type: "desk:open-thread"; thread: string };

/** Where the contract stands: the listed origins, and the parent's once it is known to be one of them. */
const state: { origins: string[]; parent: string | null } = { origins: [], parent: null };
const listeners = new Set<() => void>();
let theme: HostTheme = { colours: {} };
let lastReport = "";

function changed() {
  for (const listener of listeners) listener();
}

/** Whether the Desk has a listed parent to talk to. */
export function connected(): boolean {
  return state.parent !== null;
}

/** Whether the Desk has a listed parent, for a component that should change when it gains one. */
export function useConnected(): boolean {
  return useSyncExternalStore(
    (listener) => {
      listeners.add(listener);
      return () => listeners.delete(listener);
    },
    connected,
    () => false,
  );
}

function send(message: DeskMessage) {
  if (state.parent === null) return;
  window.parent.postMessage(message, state.parent);
}

/** The parent window's origin, where the browser tells it. */
function parentOrigin(): string | null {
  const ancestors = location.ancestorOrigins;
  if (ancestors && ancestors.length > 0) return ancestors[0];
  try {
    return document.referrer ? new URL(document.referrer).origin : null;
  } catch {
    return null;
  }
}

/** Becomes connected to `origin`, and tells the host the Desk is ready and where it is. */
function connect(origin: string) {
  state.parent = origin;
  send({ type: "desk:ready", version: 1 });
  lastReport = "";
  report();
  changed();
}

function receive(event: MessageEvent) {
  if (event.source !== window.parent || !state.origins.includes(event.origin)) return;
  if (state.parent === null) connect(event.origin);
  else if (event.origin !== state.parent) return;
  const data: unknown = event.data;
  if (!data || typeof data !== "object" || Array.isArray(data)) return;
  const message = data as Record<string, unknown>;
  // A type the Desk does not know is ignored, so a host can send newer
  // messages to an older Desk.
  if (message.type === "host:theme") {
    theme = merge(theme, message);
    apply(theme);
  }
}

/**
 * Tells the host the path and title of the page shown, when either changed
 * since it was last told. The path is the Desk's own route, without the
 * embedded base.
 */
export function report() {
  if (state.parent === null) return;
  const path = (location.pathname.slice(EMBED_BASE.length) || "/") + location.search;
  const title = document.title;
  const key = `${path}\n${title}`;
  if (key === lastReport) return;
  lastReport = key;
  send({ type: "desk:navigated", path, title });
}

/** Asks the host to open the thread whose identity is `thread`. */
export function openThread(thread: string) {
  send({ type: "desk:open-thread", thread });
}

/**
 * Starts the contract when the Desk is embedded in a frame: learns the listed
 * origins from the server, and connects to the parent if its origin is one.
 * A parent whose origin the browser does not tell is connected once it sends
 * a message from a listed origin.
 */
export async function startHost() {
  if (!embedded || window.parent === window) return;
  let origins: string[] = [];
  try {
    const response = await fetch("/api/embed", { headers: { accept: "application/json" } });
    if (response.ok) origins = ((await response.json()) as EmbedResponse).origins ?? [];
  } catch {
    // Without the list the Desk talks to no one.
  }
  state.origins = origins.filter((o) => typeof o === "string" && o !== "*" && o !== "null");
  if (state.origins.length === 0) return;
  window.addEventListener("message", receive);
  new MutationObserver(report).observe(document.head, { subtree: true, childList: true, characterData: true });
  const parent = parentOrigin();
  if (parent !== null && state.origins.includes(parent)) connect(parent);
}
