import { ChevronRight } from "lucide-react";
import { Fragment, useEffect, useState } from "react";
import { createPortal } from "react-dom";
import { Link } from "react-router";

export type Crumb = { label: string; to?: string };

/** The page's trail in the top bar, and its title in the tab. */
export function Crumbs({ trail }: { trail: Crumb[] }) {
  const [slot, setSlot] = useState<HTMLElement | null>(null);
  useEffect(() => setSlot(document.getElementById("crumbs")), []);
  const title = trail.at(-1)?.label ?? "Atelier";
  useEffect(() => {
    document.title = `${title} · Desk`;
  }, [title]);
  if (!slot) return null;
  return createPortal(
    trail.map((crumb, at) => (
      <Fragment key={at}>
        {at > 0 ? <ChevronRight size={13} aria-hidden style={{ flex: "none" }} /> : null}
        {crumb.to ? (
          <Link to={crumb.to} className="truncate">
            {crumb.label}
          </Link>
        ) : (
          <span className="here truncate">{crumb.label}</span>
        )}
      </Fragment>
    )),
    slot,
  );
}
