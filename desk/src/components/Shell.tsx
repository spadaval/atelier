import { Activity, Bot, FolderGit2, GitPullRequestArrow, Inbox, LayoutGrid, Menu, Moon, Scale, Sun } from "lucide-react";
import { useEffect, useState, type ReactNode } from "react";
import { Link, NavLink, useLocation } from "react-router";
import type { InboxResponse } from "../api/InboxResponse";
import type { ReposResponse } from "../api/ReposResponse";
import { api, to } from "../lib/api";
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

export function Shell({ children }: { children: ReactNode }) {
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
