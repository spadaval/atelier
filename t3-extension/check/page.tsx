// The check's page script (test-components.mjs): the probe tree once in the page itself, styled by
// T3's stylesheet alone as T3 draws it, and once through mountReact in a shadow root, as the
// extension draws it.
import { createRoot } from "react-dom/client";
import { mountReact } from "../src/react/mount";
import { type HostApi, useDeskRead } from "../src/react/desk";
import { PortalContainer } from "../src/react/portal";
import { Probe } from "./Probe";

const errors: string[] = [];
(globalThis as { probeErrors?: string[] }).probeErrors = errors;
addEventListener("error", (event) => errors.push(String(event.error?.stack ?? event.message)));

const reference = document.body.appendChild(document.createElement("div"));
reference.id = "reference";
// T3 portals to <body>, as Base UI does by default.
createRoot(reference).render(
  <PortalContainer value={document.body}>
    <Probe />
  </PortalContainer>,
);

const host = document.body.appendChild(document.createElement("div"));
host.id = "extension";
const api = {
  request: ((action: string) => Promise.reject(new Error(`${action}: the check has no injector`))) as HostApi["request"],
  onCleanup: () => {},
  reportError: (error: unknown) => errors.push(String(error)),
  signal: new AbortController().signal,
};
mountReact(host, api).render(<Probe />);

// useDeskRead against a stand-in injector: /api/motion answers with a count of reads, then fails
// from the third on; /api/inbox always fails. test-components.mjs reads what each shows.
const reads: Record<string, number> = {};
const readApi = {
  ...api,
  request: ((_action: string, payload?: unknown) => {
    const { path } = payload as { desk: string; path: string };
    reads[path] = (reads[path] ?? 0) + 1;
    if (path === "/api/inbox" || reads[path] > 2) return Promise.reject(new Error(`GET ${path}: HTTP 503: down`));
    return Promise.resolve({ reads: reads[path] });
  }) as HostApi["request"],
};
function ShowRead({ path }: { path: string }) {
  const read = useDeskRead<{ reads: number }>("http://desk.invalid", path, 50);
  return <output data-path={path} data-loading={String(read.loading)} data-reads={read.data?.reads ?? "none"} data-error={read.error?.message ?? ""} />;
}
const readHost = document.body.appendChild(document.createElement("div"));
readHost.id = "desk-read";
mountReact(readHost, readApi).render(
  <>
    <ShowRead path="/api/motion" />
    <ShowRead path="/api/inbox" />
  </>,
);
