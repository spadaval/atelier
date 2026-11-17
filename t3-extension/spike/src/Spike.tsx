import { Tooltip } from "@base-ui/react/tooltip";
import { Badge } from "./ui/badge";
import { Kbd } from "./ui/kbd";

const variants = ["default", "secondary", "outline", "success", "warning", "info", "error"] as const;

/** T3's own Badge and Kbd, and a Base UI tooltip whose popup is portalled into `portal`. */
export function Spike({ portal }: { portal?: HTMLElement | null }) {
  return (
    <div className="flex flex-wrap items-center gap-2 bg-background p-4 font-sans text-foreground text-sm">
      {variants.map((variant) => (
        <Badge key={variant} variant={variant} data-probe={`badge-${variant}`}>
          {variant}
        </Badge>
      ))}
      <Kbd data-probe="kbd">L</Kbd>
      <Kbd variant="outline" data-probe="kbd-outline">Esc</Kbd>
      <span className="text-muted-foreground text-xs" data-probe="muted">14 of 26 merged</span>
      <Tooltip.Provider delay={0}>
        <Tooltip.Root>
          <Tooltip.Trigger className="rounded-md border border-border px-2 py-1 text-xs shadow-xs" data-probe="trigger">
            Hover
          </Tooltip.Trigger>
          <Tooltip.Portal container={portal}>
            <Tooltip.Positioner sideOffset={6}>
              <Tooltip.Popup className="rounded-md border border-border bg-popover px-2 py-1 text-popover-foreground text-xs shadow-md" data-probe="tooltip">
                Open thread
              </Tooltip.Popup>
            </Tooltip.Positioner>
          </Tooltip.Portal>
        </Tooltip.Root>
      </Tooltip.Provider>
    </div>
  );
}
