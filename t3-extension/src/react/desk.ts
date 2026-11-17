// What a React tree reaches the injector through: the injected script's api (t3-custom.mjs,
// buildBundle), given by mountReact. useDeskRead reads a Desk path through it.
import { createContext, useContext, useEffect, useState } from "react";

export type HostApi = {
  request(action: string, payload?: unknown): Promise<unknown>;
  onCleanup(callback: () => void): void;
  reportError(error: unknown): void;
  signal: AbortSignal;
};

export const DeskApi = createContext<HostApi | null>(null);

export function useHostApi(): HostApi {
  const api = useContext(DeskApi);
  if (!api) throw new Error("useHostApi: the tree is not drawn by mountReact (src/react/mount.tsx)");
  return api;
}

export type DeskRead<T> = { data: T | undefined; error: Error | null; loading: boolean; readAt: Date | null };

/**
 * Reads `path` from the Desk at `desk` through the injector's `desk-get`, which admits only the
 * reads its gate lists (t3-custom.mjs, deskReadUrl), then again every `intervalMs` while the page
 * is visible, and at once when it becomes visible again. A failed read is `error`, with the last
 * data kept beside it: a Desk that cannot be read never looks like one with nothing in it.
 */
export function useDeskRead<T>(desk: string, path: string, intervalMs = 15000): DeskRead<T> {
  const api = useHostApi();
  const [state, setState] = useState<DeskRead<T>>({ data: undefined, error: null, loading: true, readAt: null });
  useEffect(() => {
    let live = true;
    let timer: ReturnType<typeof setTimeout> | undefined;
    // One read at a time: becoming visible during a read must not start a second polling chain.
    let reading = false;
    setState({ data: undefined, error: null, loading: true, readAt: null });
    const read = async () => {
      clearTimeout(timer);
      if (reading) return;
      reading = true;
      try {
        const data = (await api.request("desk-get", { desk, path })) as T;
        if (live) setState({ data, error: null, loading: false, readAt: new Date() });
      } catch (error) {
        if (live) setState((last) => ({ ...last, error: error instanceof Error ? error : new Error(String(error)), loading: false }));
      }
      reading = false;
      if (live && document.visibilityState === "visible") timer = setTimeout(read, intervalMs);
    };
    const onVisibility = () => {
      if (document.visibilityState === "visible") void read();
      else clearTimeout(timer);
    };
    document.addEventListener("visibilitychange", onVisibility, { signal: api.signal });
    void read();
    return () => {
      live = false;
      clearTimeout(timer);
      document.removeEventListener("visibilitychange", onVisibility);
    };
  }, [api, desk, path, intervalMs]);
  return state;
}
