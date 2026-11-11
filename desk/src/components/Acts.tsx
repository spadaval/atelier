import { CircleCheck, MessagesSquare, TriangleAlert } from "lucide-react";
import { useId, useState, type ReactNode } from "react";
import type { ActResponse } from "../api/ActResponse";
import type { MissionPage } from "../api/MissionPage";
import type { Outcome } from "../api/Outcome";
import type { SessionResponse } from "../api/SessionResponse";
import { act, decideRequest, readyRequest, sessionRequest, startSession, succeeded, type Sent } from "../lib/acts";
import { reloadAll } from "../lib/api";
import { shortCommit } from "../lib/words";
import { ThreadTitle } from "./Agents";

/** A refusal as the server gave it: its text, the rules it names and its next step. */
function Refusal({ title, text, rules = [], next = null }: { title: string; text: string; rules?: string[]; next?: string | null }) {
  return (
    <div className="act-result bad" role="alert" data-testid="refusal">
      <TriangleAlert size={15} aria-hidden />
      <div className="grow">
        <b>{title}</b>
        <p className="act-text">{text}</p>
        {rules.length > 0 ? (
          <ul className="act-rules">
            {rules.map((rule, at) => (
              <li key={at}>{rule}</li>
            ))}
          </ul>
        ) : null}
        {next ? (
          <p className="act-next">
            <span className="muted">Next: </span>
            {next}
          </p>
        ) : null}
      </div>
    </div>
  );
}

function Failed({ error }: { error: string }) {
  return (
    <div className="act-result bad" role="alert" data-testid="failure">
      <TriangleAlert size={15} aria-hidden />
      <p className="grow">It did not run. {error}</p>
    </div>
  );
}

/** What an act answered: done, refused, or no answer at all. */
function ActOutcome({ sent, refusedTitle, done }: { sent: Sent<ActResponse>; refusedTitle: string; done: (text: string, commit: string | null) => ReactNode }) {
  if (sent.state === "failed") return <Failed error={sent.error} />;
  if (sent.state !== "answered") return null;
  const r = sent.response;
  if (r.result === "refused") return <Refusal title={refusedTitle} text={r.text} rules={r.rules} next={r.next} />;
  return (
    <div className="act-result ok" role="status" data-testid="done">
      <CircleCheck size={15} aria-hidden />
      <div className="grow">{done(r.text, r.commit)}</div>
    </div>
  );
}

/** Asks once before an act runs. */
function Confirm({ label, children, yes, onYes, onNo }: { label: string; children: ReactNode; yes: string; onYes: () => void; onNo: () => void }) {
  return (
    <div className="confirm" role="group" aria-label={label}>
      <p>{children}</p>
      <div className="row">
        <button type="button" className="btn primary" autoFocus onClick={onYes}>
          {yes}
        </button>
        <button type="button" className="btn ghost" onClick={onNo}>
          Not yet
        </button>
      </div>
    </div>
  );
}

/**
 * Say go on a draft mission: `ready` with the mission's ID. Shown while the
 * mission is a draft; the button only when the server says go would be
 * accepted. After it is done every page is read again, so this page shows
 * the mission ready, and this keeps the act's own words beneath.
 */
export function SayGo({ repo, page }: { repo: string; page: MissionPage }) {
  const [sent, setSent] = useState<Sent<ActResponse>>({ state: "idle" });
  const m = page.mission;
  const draft = m.status === "draft";
  if (!draft && sent.state !== "answered") return null;
  const go = async () => {
    setSent({ state: "sending" });
    const result = await act(readyRequest(repo, m.id));
    if (succeeded(result)) await reloadAll();
    setSent(result);
  };
  const ask = sent.state === "idle" || (sent.state !== "confirming" && sent.state !== "sending" && !succeeded(sent));
  return (
    <div className={draft ? "go-note" : "go-done"} data-testid="say-go">
      {draft ? (
        <p>
          <b>This mission is a draft.</b> Agents start its items only after you say go.
          {!page.can_ready ? " Its plan cannot be made ready yet, so there is no go to give." : null}
        </p>
      ) : null}
      {draft && page.can_ready && ask ? (
        <div className="row act-bar">
          <button type="button" className="btn primary" onClick={() => setSent({ state: "confirming" })}>
            Say go
          </button>
        </div>
      ) : null}
      {sent.state === "confirming" ? (
        <Confirm label="Confirm saying go" yes="Yes, say go" onYes={() => void go()} onNo={() => setSent({ state: "idle" })}>
          Say go on this mission? Your go is committed to the repository, and agents may then start its ready items.
        </Confirm>
      ) : null}
      {sent.state === "sending" ? (
        <p className="muted" role="status">
          Saying go…
        </p>
      ) : null}
      <ActOutcome
        sent={sent}
        refusedTitle="Your go was refused."
        done={(text, commit) => (
          <>
            <b>You said go{commit ? `, committed as ${shortCommit(commit)}` : ""}.</b>
            <p className="act-text">{text}</p>
          </>
        )}
      />
    </div>
  );
}

/**
 * The answer to an item's oldest open question, with the settlement of the
 * finding it is about when it names one: `decide`. After it is done every
 * page is read again, `onAnswered` runs with the act's text and the answer
 * sent, and the form
 * closes empty: the page shows the answer as pending, and the next question
 * of the item, which may be shown in the same place, starts with none.
 */
export function AnswerForm({ repo, item, finding, onAnswered }: { repo: string; item: string; finding: string | null; onAnswered?: (text: string, answer: string) => void }) {
  const [open, setOpen] = useState(false);
  const [answer, setAnswer] = useState("");
  const [settle, setSettle] = useState<Outcome | null>(null);
  const [sent, setSent] = useState<Sent<ActResponse>>({ state: "idle" });
  const id = useId();
  const sending = sent.state === "sending";
  if (!open) {
    return (
      <div className="act-bar">
        <button type="button" className="btn sm" onClick={() => setOpen(true)} aria-label={`Answer the question on ${item}`}>
          Answer
        </button>
      </div>
    );
  }
  const submit = async () => {
    setSent({ state: "sending" });
    const result = await act(decideRequest(repo, item, answer.trim(), finding ? settle : null));
    if (succeeded(result)) {
      await reloadAll();
      if (result.state === "answered" && result.response.result === "done") onAnswered?.(result.response.text, answer.trim());
      setOpen(false);
      setAnswer("");
      setSettle(null);
      setSent({ state: "idle" });
      return;
    }
    setSent(result);
  };
  const settlements: [Outcome | null, string][] = [
    [null, "Leave it to the implementer"],
    ["defer", "Defer it"],
    ["no_action", "No action needed"],
  ];
  return (
    <form
      className="answer-form"
      aria-label={`Answer on ${item}`}
      onSubmit={(event) => {
        event.preventDefault();
        if (answer.trim() !== "" && !sending) void submit();
      }}
    >
      <label htmlFor={`${id}-answer`}>Your answer, in one line</label>
      <input id={`${id}-answer`} className="field" type="text" autoFocus value={answer} disabled={sending} onChange={(event) => setAnswer(event.target.value)} />
      {finding ? (
        <fieldset disabled={sending}>
          <legend>
            What becomes of finding <span className="mono">{finding}</span>
          </legend>
          {settlements.map(([value, words]) => (
            <label key={value ?? "none"} className="choice">
              <input type="radio" name={`${id}-settle`} checked={settle === value} onChange={() => setSettle(value)} /> {words}
            </label>
          ))}
        </fieldset>
      ) : null}
      <div className="row">
        <button type="submit" className="btn primary" disabled={answer.trim() === "" || sending}>
          {sending ? "Sending…" : "Send the answer"}
        </button>
        <button type="button" className="btn ghost" disabled={sending} onClick={() => setOpen(false)}>
          Cancel
        </button>
      </div>
      <ActOutcome sent={sent} refusedTitle="Your answer was refused." done={(text) => <p className="act-text">{text}</p>} />
    </form>
  );
}

/** Starts a planning session for a mission as a new T3 Code thread, then links to it. */
export function PlanningSession({ repo, mission }: { repo: string; mission: string }) {
  const [sent, setSent] = useState<Sent<SessionResponse>>({ state: "idle" });
  const start = async () => {
    setSent({ state: "sending" });
    setSent(await startSession(sessionRequest(repo, mission)));
  };
  const response = sent.state === "answered" ? sent.response : null;
  return (
    <div className="session" data-testid="session">
      {sent.state === "idle" || sent.state === "failed" || response?.result === "refused" ? (
        <button type="button" className="btn" onClick={() => setSent({ state: "confirming" })}>
          <MessagesSquare size={14} aria-hidden /> Start a planning session
        </button>
      ) : null}
      {sent.state === "confirming" ? (
        <Confirm label="Confirm starting a planning session" yes="Yes, start it" onYes={() => void start()} onNo={() => setSent({ state: "idle" })}>
          Start a new thread in T3 Code to plan this mission with you? It changes no record.
        </Confirm>
      ) : null}
      {sent.state === "sending" ? (
        <p className="muted" role="status">
          Starting a thread in T3 Code…
        </p>
      ) : null}
      {sent.state === "failed" ? <Failed error={sent.error} /> : null}
      {response?.result === "refused" ? <Refusal title="T3 Code did not start a session." text={response.text} /> : null}
      {response?.result === "started" ? (
        <div className="act-result ok" role="status" data-testid="started">
          <CircleCheck size={15} aria-hidden />
          <div className="grow">
            <b>A planning session started in T3 Code.</b>{" "}
            {response.link ? (
              <ThreadTitle thread={{ title: "Open the thread", link: response.link }} />
            ) : (
              <span>T3 Code gave no address for it; find the thread named for this mission in T3 Code.</span>
            )}
          </div>
        </div>
      ) : null}
    </div>
  );
}
