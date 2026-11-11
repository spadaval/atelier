import { ExternalLink, Info } from "lucide-react";
import { useState } from "react";
import { Link } from "react-router";
import type { Agent } from "../api/Agent";
import type { Hold } from "../api/Hold";
import type { LiveThread } from "../api/LiveThread";
import type { T3State } from "../api/T3State";
import { to } from "../lib/api";
import { roleWords, t3Note, threadStatus } from "../lib/words";
import { Card } from "./ui";

/**
 * A thread identity, whole. It can hold `/`, `:` and `%`, so it is never
 * split or put in a link; a long one is cut short on screen only, and the
 * whole value is in the text and its title.
 */
export function Identity({ id }: { id: string }) {
  return (
    <span className="ident mono" title={id}>
      {id}
    </span>
  );
}

/** A thread's title, linking to it in T3 Code when T3 Code gave a web address. */
export function ThreadTitle({ thread }: { thread: { title: string; link: string | null } }) {
  if (!thread.link || !/^https?:\/\//i.test(thread.link)) return <span className="truncate">{thread.title}</span>;
  return (
    <a href={thread.link} target="_blank" rel="noreferrer" className="t-link" title={`${thread.title}, in T3 Code`}>
      <span className="truncate">{thread.title}</span>
      <ExternalLink size={11} aria-hidden />
    </a>
  );
}

function Pulse({ status }: { status: string | null }) {
  return <span className={`pulse ${status ?? "none"}`} aria-hidden />;
}

/** Why the threads shown may be missing or old, when they are. */
export function T3Notice({ t3 }: { t3: T3State }) {
  const note = t3Note(t3);
  if (!note) return null;
  return (
    <p className="t3-note" data-testid="t3-note">
      <Info size={13} aria-hidden />
      <span>
        {note.text}
        {note.error ? <span className="t3-why">{note.error}</span> : null}
      </span>
    </p>
  );
}

function AgentRow({ repo, agent, known }: { repo: string; agent: Agent; known: boolean }) {
  const thread = agent.thread;
  return (
    <div className="agent" data-testid="agent">
      <Pulse status={thread?.status ?? null} />
      <span className="grow">
        <span className="t">{thread ? <ThreadTitle thread={thread} /> : <Identity id={agent.identity} />}</span>
        <span className="s">
          <span>{roleWords[agent.role]}</span>
          {agent.item ? (
            <Link to={to.item(repo, agent.item)} className="mono">
              {agent.item}
            </Link>
          ) : null}
          {thread ? (
            <>
              <span className={thread.status === "waiting" ? "you-text" : undefined}>{threadStatus(thread.status)}</span>
              <span className="mono" title={thread.provider}>
                {thread.model}
              </span>
            </>
          ) : known ? (
            <span>not running in T3 Code</span>
          ) : null}
        </span>
      </span>
    </div>
  );
}

/** How many idle identities a card shows before folding the rest. */
const SHOWN_IDLE = 4;

/**
 * Who works on a mission or an item, from the identities its records name:
 * live threads first, linking to T3 Code, then identities T3 Code does not
 * list as running.
 */
export function AgentsCard({ repo, agents, t3, none }: { repo: string; agents: Agent[]; t3: T3State; none: string }) {
  const [all, setAll] = useState(false);
  const note = t3Note(t3);
  const known = note === null || note.known;
  const live = agents.filter((a) => a.thread !== null);
  const idle = agents.filter((a) => a.thread === null);
  const shown = all ? idle : idle.slice(0, SHOWN_IDLE);
  return (
    <Card title="Agents" tally={agents.length} flush>
      <T3Notice t3={t3} />
      {agents.length === 0 ? <p className="card-empty">{none}</p> : null}
      {live.map((a, at) => (
        <AgentRow key={`live${at}`} repo={repo} agent={a} known={known} />
      ))}
      {shown.map((a, at) => (
        <AgentRow key={`idle${at}`} repo={repo} agent={a} known={known} />
      ))}
      {idle.length > shown.length ? (
        <button type="button" className="fold" onClick={() => setAll(true)}>
          Show {idle.length - shown.length} more {known ? "not running in T3 Code" : "named in the records"}
        </button>
      ) : null}
    </Card>
  );
}

function HoldLink({ hold, showRepo }: { hold: Hold; showRepo: boolean }) {
  const href = hold.kind === "item" ? to.item(hold.repo, hold.id) : to.mission(hold.repo, hold.id);
  return (
    <span className="hold">
      {roleWords[hold.role]} {hold.kind === "mission" ? "of mission" : "on"}{" "}
      <Link to={href} className="mono">
        {hold.id}
      </Link>
      {showRepo ? <span className="repo-mark">{hold.repo}</span> : null}
    </span>
  );
}

/** One of T3 Code's live threads, with what it holds in the records. */
export function LiveThreadRow({ thread, holds, showRepo = false }: { thread: LiveThread; holds: Hold[]; showRepo?: boolean }) {
  return (
    <div className="agent" data-testid="thread">
      <Pulse status={thread.status} />
      <span className="grow">
        <span className="t">
          <ThreadTitle thread={thread} />
          {thread.status === "waiting" ? <span className="tag you">Waiting on a person</span> : null}
        </span>
        <span className="s">
          {thread.status === "waiting" ? null : <span>{threadStatus(thread.status)}</span>}
          <span className="mono" title={thread.provider}>
            {thread.model}
          </span>
          {thread.parent ? <span>Delegated</span> : null}
          {thread.pull_requests.map((pr) =>
            /^https:\/\//i.test(pr.url) ? (
              <a key={pr.url} href={pr.url} target="_blank" rel="noreferrer" className="mono">
                #{pr.number}
                {pr.state ? ` ${pr.state.toLowerCase()}` : ""}
              </a>
            ) : null,
          )}
        </span>
        {holds.length > 0 ? (
          <span className="holds">
            {holds.map((h) => (
              <HoldLink key={`${h.repo}/${h.kind}/${h.id}/${h.role}`} hold={h} showRepo={showRepo} />
            ))}
          </span>
        ) : null}
      </span>
    </div>
  );
}
