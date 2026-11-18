// Building the DOM the extension adds among T3's own nodes: the Desk tab, record links, the sidebar
// rows and the layer. Pages of the extension's own are React (src/react/mount.tsx).

type Attributes = Record<string, string | number | boolean> & { text?: string; class?: string; html?: string };

/** An element with attributes and children. `text` is its text, `class` its class name, `html` its markup. */
export function element<K extends keyof HTMLElementTagNameMap>(tag: K, attributes: Attributes = {}, ...children: (Node | string)[]): HTMLElementTagNameMap[K] {
  const node = document.createElement(tag);
  for (const [name, value] of Object.entries(attributes)) {
    if (name === "text") node.textContent = String(value);
    else if (name === "class") node.className = String(value);
    // Only for the constant icons below; anything from the Desk or T3 goes in as text.
    else if (name === "html") node.innerHTML = String(value);
    else node.setAttribute(name, String(value));
  }
  node.append(...children);
  return node;
}

// Lucide icons, as T3 draws its own panel controls.
export const icon = (paths: string): string =>
  `<svg xmlns="http://www.w3.org/2000/svg" width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" aria-hidden="true">${paths}</svg>`;
export const icons = {
  close: icon('<path d="M18 6 6 18"/><path d="m6 6 12 12"/>'),
  refresh: icon('<path d="M21 12a9 9 0 1 1-9-9c2.52 0 4.93 1 6.74 2.74L21 8"/><path d="M21 3v5h-5"/>'),
  external: icon('<path d="M15 3h6v6"/><path d="M10 14 21 3"/><path d="M18 13v6a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2V8a2 2 0 0 1 2-2h6"/>'),
  settings: icon(
    '<path d="M12.22 2h-.44a2 2 0 0 0-2 2v.18a2 2 0 0 1-1 1.73l-.43.25a2 2 0 0 1-2 0l-.15-.08a2 2 0 0 0-2.73.73l-.22.38a2 2 0 0 0 .73 2.73l.15.1a2 2 0 0 1 1 1.72v.51a2 2 0 0 1-1 1.74l-.15.09a2 2 0 0 0-.73 2.73l.22.38a2 2 0 0 0 2.73.73l.15-.08a2 2 0 0 1 2 0l.43.25a2 2 0 0 1 1 1.73V20a2 2 0 0 0 2 2h.44a2 2 0 0 0 2-2v-.18a2 2 0 0 1 1-1.73l.43-.25a2 2 0 0 1 2 0l.15.08a2 2 0 0 0 2.73-.73l.22-.39a2 2 0 0 0-.73-2.73l-.15-.08a2 2 0 0 1-1-1.74v-.5a2 2 0 0 1 1-1.74l.15-.09a2 2 0 0 0 .73-2.73l-.22-.38a2 2 0 0 0-2.73-.73l-.15.08a2 2 0 0 1-2 0l-.43-.25a2 2 0 0 1-1-1.73V4a2 2 0 0 0-2-2z"/><circle cx="12" cy="12" r="3"/>',
  ),
  desk: icon('<rect width="18" height="18" x="3" y="3" rx="2"/><path d="M3 9h18"/><path d="M9 21V9"/>'),
  back: icon('<path d="m15 18-6-6 6-6"/>'),
};
