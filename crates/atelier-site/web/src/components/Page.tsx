import { useEffect, type ReactNode } from "react";
import { Link } from "react-router";
import type { Loaded } from "../api";

export type Crumb = { label: string; to?: string };

/** A page: the trail back to the repository list, a title and its body. */
export function Page({ crumbs, title, children }: { crumbs: Crumb[]; title: string; children?: ReactNode }) {
  useEffect(() => {
    document.title = `${title} · Atelier`;
  }, [title]);
  return (
    <>
      <nav className="crumbs" aria-label="Breadcrumbs">
        <Link to="/">Atelier</Link>
        {crumbs.map((crumb, at) => (
          <span key={at}>
            {" / "}
            {crumb.to ? <Link to={crumb.to}>{crumb.label}</Link> : crumb.label}
          </span>
        ))}
      </nav>
      <main>
        <h1>{title}</h1>
        {children}
      </main>
    </>
  );
}

/** A titled section, left out when it has nothing to show. */
export function Section({ title, children, show = true }: { title: string; children: ReactNode; show?: boolean }) {
  if (!show) return null;
  return (
    <section>
      <h2>{title}</h2>
      {children}
    </section>
  );
}

/** Labelled facts, such as a review's metadata. */
export function Facts({ facts }: { facts: [string, ReactNode][] }) {
  if (facts.length === 0) return null;
  return (
    <dl className="facts">
      {facts.map(([label, value]) => (
        <div key={label}>
          <dt>{label}</dt>
          <dd>{value}</dd>
        </div>
      ))}
    </dl>
  );
}

/** An item as the CLI lists it: id, type, status, title, then any cues. */
export function ItemLine({
  id,
  issueType,
  status,
  title,
  cues = [],
}: {
  id: string;
  issueType?: string;
  status?: string;
  title?: string;
  cues?: ReactNode[];
}) {
  return (
    <span className="item">
      <code>{id}</code>
      {issueType && issueType !== "mission" ? <span className="tag">{issueType}</span> : null}
      {status ? <span className={`status status-${status}`}>{status}</span> : null}
      {title ? <span className="title">{title}</span> : null}
      {cues.map((cue, at) => (
        <span className="cue" key={at}>
          {cue}
        </span>
      ))}
    </span>
  );
}

/** Shows the loading state or the error of `loaded`, else `children`. */
export function Loading<T>({
  loaded,
  crumbs,
  children,
}: {
  loaded: Loaded<T>;
  crumbs: Crumb[];
  children: (body: T) => ReactNode;
}) {
  if (loaded.state === "loading") {
    return (
      <Page crumbs={crumbs} title="Loading…">
        <p className="muted">Loading…</p>
      </Page>
    );
  }
  if (loaded.state === "error") {
    return <ErrorPage crumbs={crumbs} status={loaded.status} error={loaded.error} />;
  }
  return <>{children(loaded.body)}</>;
}

export function ErrorPage({ crumbs, status, error }: { crumbs: Crumb[]; status: number; error: string }) {
  const title = status === 404 ? "Not found" : "Cannot show this page";
  return (
    <Page crumbs={crumbs} title={title}>
      <p className="error" role="alert">
        {error}
      </p>
      <p>
        <Link to="/">Back to every repository</Link>
      </p>
    </Page>
  );
}
