import { StrictMode } from "react";
import { createRoot } from "react-dom/client";
import { BrowserRouter } from "react-router";
import { App } from "./App";
import "./styles.css";

let saved: string | null = null;
try {
  saved = localStorage.getItem("desk-theme");
} catch {
  // Storage can be blocked; the system's choice stands.
}
if (saved === "light" || (!saved && matchMedia("(prefers-color-scheme: light)").matches)) {
  document.documentElement.dataset.theme = "light";
}

createRoot(document.getElementById("root")!).render(
  <StrictMode>
    <BrowserRouter>
      <App />
    </BrowserRouter>
  </StrictMode>,
);
