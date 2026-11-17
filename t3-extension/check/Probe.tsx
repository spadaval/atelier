// Every copied component in each of its variants and sizes, for test-components.mjs, which draws
// this tree twice and compares the two. Each top-level element has a data-probe naming it.
import { Badge } from "../src/t3/ui/badge";
import { Button } from "../src/t3/ui/button";
import { CollapsibleSectionHeader, SectionHeaderStatus } from "../src/t3/ui/collapsible-section-header";
import { Kbd, KbdGroup } from "../src/t3/ui/kbd";
import { Tooltip, TooltipPopup, TooltipProvider, TooltipTrigger } from "../src/t3/ui/tooltip";

const badgeVariants = ["default", "destructive", "error", "info", "outline", "secondary", "success", "warning", "label"] as const;
const badgeSizes = ["control", "default", "lg", "sm"] as const;
const buttonVariants = ["default", "destructive", "destructive-outline", "ghost", "ghost-muted", "ghost-destructive", "glass", "link", "media-close", "media-navigation", "outline", "overlay", "secondary", "warning-outline"] as const;
const buttonSizes = ["compact", "default", "icon", "icon-lg", "icon-micro", "icon-tiny", "icon-sm", "icon-sm-pill", "icon-xl", "icon-xs", "lg", "micro", "multiline", "sm", "sm-pill", "sm-multiline", "xl", "xs"] as const;
const tones = ["muted", "emphasized", "accent"] as const;
const tooltipVariants = ["default", "glass", "code"] as const;

/** The tree; `hover` adds a closed tooltip whose trigger the check hovers to open it. */
export function Probe() {
  return (
    <TooltipProvider delay={0}>
      <div className="relative flex flex-wrap items-start gap-2 p-4" style={{ ["--label" as string]: "oklch(0.6 0.2 30)" }}>
        {badgeVariants.map((variant) => (
          <Badge key={variant} variant={variant} data-probe={`badge-${variant}`}>
            {variant}
          </Badge>
        ))}
        {badgeSizes.map((size) => (
          <Badge key={size} size={size} data-probe={`badge-size-${size}`}>
            {size}
          </Badge>
        ))}
        <Badge render={<button type="button" />} data-probe="badge-button">
          as button
        </Badge>
        <Kbd data-probe="kbd">L</Kbd>
        <Kbd variant="outline" data-probe="kbd-outline">
          Esc
        </Kbd>
        <KbdGroup data-probe="kbd-group">
          <Kbd>Ctrl</Kbd>
          <Kbd>K</Kbd>
        </KbdGroup>
        {buttonVariants.map((variant) => (
          <Button key={variant} variant={variant} data-probe={`button-${variant}`}>
            {variant}
          </Button>
        ))}
        {buttonSizes.map((size) => (
          <Button key={size} size={size} variant="outline" data-probe={`button-size-${size}`}>
            {size.startsWith("icon") ? "+" : size}
          </Button>
        ))}
        <Button disabled data-probe="button-disabled">
          disabled
        </Button>
        {tones.map((tone) => (
          <div key={tone} className="w-60" data-probe={`section-header-${tone}`}>
            <CollapsibleSectionHeader expanded tone={tone}>
              {tone}
            </CollapsibleSectionHeader>
          </div>
        ))}
        <div className="w-60" data-probe="section-header-panel-folded">
          <CollapsibleSectionHeader expanded={false} variant="panel" accessory={<SectionHeaderStatus>2 failing</SectionHeaderStatus>}>
            panel
          </CollapsibleSectionHeader>
        </div>
        {tooltipVariants.map((variant) => (
          <Tooltip key={variant} open>
            <TooltipTrigger render={<span />} data-probe={`tooltip-trigger-${variant}`}>
              {variant}
            </TooltipTrigger>
            <TooltipPopup variant={variant} data-probe={`tooltip-${variant}`}>
              Open thread {variant}
            </TooltipPopup>
          </Tooltip>
        ))}
        <Tooltip>
          <TooltipTrigger render={<span />} data-probe="tooltip-hover-trigger">
            hover
          </TooltipTrigger>
          <TooltipPopup data-probe="tooltip-hover">Opened by hovering</TooltipPopup>
        </Tooltip>
      </div>
    </TooltipProvider>
  );
}
