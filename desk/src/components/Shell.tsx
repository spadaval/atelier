import { Activity, Bot, FolderGit2, GitPullRequestArrow, Inbox, LayoutGrid, Menu, Moon, Scale, Sun } from "lucide-react";
import { useEffect, useRef, useState, type ReactNode } from "react";
import { Link, NavLink, useLocation, useNavigate } from "react-router";
import type { InboxResponse } from "../api/InboxResponse";
import type { ReposResponse } from "../api/ReposResponse";
import type { TiesResponse } from "../api/TiesResponse";
import { api, to } from "../lib/api";
import { embedded } from "../lib/embed";
import { place, serves, tied, type Place } from "../lib/follow";
import { report, useHostThread } from "../lib/host";
import { useData } from "../lib/api";

function Brand() {
  return (
    <Link to="/" className="brand">
      <svg width="20" height="20" viewBox="0 0 32 32" aria-hidden>
        <path d="M8 25 16 6l8 19M11.5 18h9" fill="none" stroke="currentColor" strokeWidth="3" strokeLinecap="round" strokeLinejoin="round" />
      </svg>
      Desk
    </Link>
  );
}

function ThemeToggle() {
  const [light, setLight] = useState(document.documentElement.dataset.theme === "light");
  return (
    <button
      type="button"
      className="icon-btn"
      title={light ? "Switch to dark" : "Switch to light"}
      aria-label={light ? "Switch to dark" : "Switch to light"}
      onClick={() => {
        const next = !light;
        setLight(next);
        if (next) document.documentElement.dataset.theme = "light";
        else delete document.documentElement.dataset.theme;
        try {
          localStorage.setItem("desk-theme", next ? "light" : "dark");
        } catch {
          // The choice lasts for this page only.
        }
      }}
    >
      {light ? <Moon size={16} /> : <Sun size={16} />}
    </button>
  );
}

/** How many things wait on the human in one repository of the inbox. */
export function inboxCount(repo: InboxResponse["repos"][number]): number {
  return repo.needs_human.reduce((n, g) => n + g.items.length, 0) + repo.proposed.length + repo.threads.length;
}

function Sidebar({ open }: { open: boolean }) {
  const repos = useData<ReposResponse>(api.repos());
  const inbox = useData<InboxResponse>(api.inbox());
  const waiting = inbox.data?.repos.reduce((n, r) => n + inboxCount(r), 0);
  return (
    <aside className={`side${open ? " open" : ""}`} aria-label="Pages">
      <Brand />
      <NavLink to="/" end className="nav">
        <LayoutGrid size={16} /> Repositories
      </NavLink>
      <NavLink to="/inbox" className="nav">
        <Inbox size={16} /> Inbox
        {waiting !== undefined ? <span className={`count${waiting > 0 ? " you" : ""}`}>{waiting}</span> : null}
      </NavLink>
      <NavLink to="/proposed" className="nav">
        <GitPullRequestArrow size={16} /> Proposed
      </NavLink>
      <NavLink to="/motion" className="nav">
        <Activity size={16} /> In motion
      </NavLink>
      <NavLink to="/disputed" className="nav">
        <Scale size={16} /> Disputed
      </NavLink>
      <NavLink to="/agents" className="nav">
        <Bot size={16} /> Agents
      </NavLink>
      <div className="side-head">Repositories</div>
      {(repos.data?.repos ?? []).map((repo) => (
        <NavLink key={repo.name} to={to.repo(repo.name)} className="nav">
          <FolderGit2 size={15} />
          <span className="truncate">{repo.name}</span>
          {repo.error ? <span className="count bad">error</span> : repo.proposed + repo.waiting > 0 ? <span className="count you">{repo.proposed + repo.waiting}</span> : null}
        </NavLink>
      ))}
      <div className="side-foot">
        <ThemeToggle />
        {repos.data ? (
          <span className="built" data-testid="built-from" title={`The Desk's server was built from commit ${repos.data.built_from}`}>
            Built from {repos.data.built_from.slice(0, 9)}
          </span>
        ) : null}
      </div>
    </aside>
  );
}

/** The inbox, reached from the top bar where the embedded Desk has no sidebar. */
function InboxLink() {
  const inbox = useData<InboxResponse>(api.inbox());
  const waiting = inbox.data?.repos.reduce((n, r) => n + inboxCount(r), 0);
  return (
    <NavLink to="/inbox" className="icon-btn inbox-btn" aria-label="Inbox" title="Inbox">
      <Inbox size={16} />
      {waiting ? <span className="count you">{waiting}</span> : null}
    </NavLink>
  );
}

/**
 * Opens the page for the thread the host shows, each time the host names
 * another thread, and returns that page with the line that names the
 * thread's part on it. The host naming the same thread again leaves alone
 * the page the human has since moved to.
 */
function useFollowThread(): Place | null {
  const host = useHostThread();
  const navigate = useNavigate();
  // The navigate function changes with the location; following a thread must
  // not run again when the human moves on, so the effect reads it here.
  const go = useRef(navigate);
  go.current = navigate;
  const [followed, setFollowed] = useState<Place | null>(null);
  useEffect(() => {
    if (host === null) return;
    // The line names the last thread's part; it goes until this thread's
    // ties are read, and stays gone if they cannot be.
    setFollowed(null);
    let stale = false;
    void (async () => {
      try {
        const read = async <T,>(path: string): Promise<T | null> => {
          const response = await fetch(path, { headers: { accept: "application/json" } });
          return response.ok ? ((await response.json()) as T) : null;
        };
        const body = await read<TiesResponse>(api.ties(host.thread));
        if (body === null || stale) return;
        // A thread with no current tie gets its project's page, when the host
        // names a repository the Desk serves; otherwise the inbox.
        let repository: string | null = null;
        if (tied(body.ties) === null && host.repository !== null) {
          const repos = await read<ReposResponse>(api.repos());
          if (repos !== null && serves(repos, host.repository)) repository = host.repository;
        }
        if (stale) return;
        const next = place(body.ties, repository);
        setFollowed(next);
        // Replacing, not pushing: the host chose this page, and the host's own
        // back button should not walk through the threads it showed.
        go.current(next.path, { replace: true });
      } catch {
        // Without the thread's ties the page stays where it is.
      }
    })();
    return () => {
      stale = true;
    };
  }, [host]);
  return followed;
}

/**
 * The embedded Desk draws content only: its host supplies the frame, so
 * there is no sidebar, brand, theme toggle or footer. The breadcrumb is the
 * way up, and the inbox is one link away from every page.
 */
function EmbeddedShell({ children }: { children: ReactNode }) {
  const location = useLocation();
  // Tells a listed host where the Desk is; the title is reported again as
  // the page sets it (lib/host.ts).
  useEffect(report, [location.pathname, location.search]);
  const followed = useFollowThread();
  return (
    <div className="shell embedded">
      <div className="main">
        <header className="top">
          <InboxLink />
          <nav id="crumbs" className="crumbs" aria-label="Where you are" />
        </header>
        {followed && followed.path === location.pathname ? <p className="thread-part">{followed.line}</p> : null}
        {children}
      </div>
    </div>
  );
}

export function Shell({ children }: { children: ReactNode }) {
  return embedded ? <EmbeddedShell>{children}</EmbeddedShell> : <FullShell>{children}</FullShell>;
}

function FullShell({ children }: { children: ReactNode }) {
  const [menu, setMenu] = useState(false);
  const location = useLocation();
  useEffect(() => setMenu(false), [location.pathname, location.search]);
  return (
    <div className="shell">
      {menu ? <div className="scrim" onClick={() => setMenu(false)} /> : null}
      <Sidebar open={menu} />
      <div className="main">
        <header className="top">
          <button type="button" className="icon-btn menu-btn" onClick={() => setMenu(true)} aria-label="Menu">
            <Menu size={18} />
          </button>
          <nav id="crumbs" className="crumbs" aria-label="Where you are" />
        </header>
        {children}
      </div>
    </div>
  );
}
