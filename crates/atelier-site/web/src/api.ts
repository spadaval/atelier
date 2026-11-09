import { useEffect, useState } from "react";

/** What a request to the API came back with. */
export type Loaded<T> =
  | { state: "loading" }
  | { state: "ok"; body: T }
  | { state: "error"; status: number; error: string };

/** `GET path` as JSON; an error status gives the body's `error`. */
export async function getJson<T>(path: string): Promise<Loaded<T>> {
  let response: Response;
  try {
    response = await fetch(path, { headers: { accept: "application/json" } });
  } catch (error) {
    return { state: "error", status: 0, error: `cannot reach the server: ${String(error)}` };
  }
  let body: unknown;
  try {
    body = await response.json();
  } catch {
    return { state: "error", status: response.status, error: `${path} answered ${response.status} without JSON` };
  }
  if (!response.ok) {
    const error =
      body && typeof body === "object" && "error" in body ? String((body as { error: unknown }).error) : `${response.status}`;
    return { state: "error", status: response.status, error };
  }
  return { state: "ok", body: body as T };
}

/** The response to `GET path`, fetched again whenever `path` changes. */
export function useApi<T>(path: string): Loaded<T> {
  const [loaded, setLoaded] = useState<{ path: string; value: Loaded<T> }>({
    path,
    value: { state: "loading" },
  });
  useEffect(() => {
    let current = true;
    void getJson<T>(path).then((value) => {
      if (current) setLoaded({ path, value });
    });
    return () => {
      current = false;
    };
  }, [path]);
  return loaded.path === path ? loaded.value : { state: "loading" };
}
