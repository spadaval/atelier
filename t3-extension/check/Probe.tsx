// Every copied component in each of its variants and sizes, for test-components.mjs, which draws
// this tree twice and compares the two. Each top-level element has a data-probe naming it.
import { Badge } from "../src/t3/ui/badge";
import { Button } from "../src/t3/ui/button";
import { CollapsibleSectionHeader, SectionHeaderStatus } from "../src/t3/ui/collapsible-section-header";
import { Kbd, KbdGroup } from "../src/t3/ui/kbd";
import { PreviewCard, PreviewCardPopup, PreviewCardTrigger } from "../src/t3/ui/preview-card";
import { Select, SelectItem, SelectPopup, SelectTrigger, SelectValue } from "../src/t3/ui/select";
import { Switch } from "../src/t3/ui/switch";
import { Tooltip, TooltipPopup, TooltipProvider, TooltipTrigger } from "../src/t3/ui/tooltip";

const badgeVariants = ["default", "destructive", "error", "info", "outline", "secondary", "success", "warning", "label"] as const;
const badgeSizes = ["control", "default", "lg", "sm"] as const;
const buttonVariants = ["default", "destructive", "destructive-outline", "ghost", "ghost-muted", "ghost-destructive", "glass", "link", "media-close", "media-navigation", "outline", "overlay", "secondary", "warning-outline"] as const;
const buttonSizes = ["compact", "default", "icon", "icon-lg", "icon-micro", "icon-tiny", "icon-sm", "icon-sm-pill", "icon-xl", "icon-xs", "lg", "micro", "multiline", "sm", "sm-pill", "sm-multiline", "xl", "xs"] as const;
const tones = ["muted", "emphasized", "accent"] as const;
const tooltipVariants = ["default", "glass", "code"] as const;
const selectSizes = ["compact", "default", "lg", "sm", "xs"] as const;
const selectItems = [
  { value: "automatic", label: "Automatic: atelier" },
  { value: "", label: "Not linked" },
];

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
        {[false, true].map((checked) =>
          (["default", "sm"] as const).map((size) => <Switch key={`${checked}-${size}`} checked={checked} size={size} data-probe={`switch-${size}-${checked ? "on" : "off"}`} />),
        )}
        <Switch disabled data-probe="switch-disabled" />
        {selectSizes.map((size) => (
          <Select key={size} items={selectItems} value="automatic">
            <SelectTrigger size={size} data-probe={`select-trigger-${size}`}>
              <SelectValue />
            </SelectTrigger>
          </Select>
        ))}
        <Select items={selectItems} value="automatic" disabled>
          <SelectTrigger data-probe="select-trigger-disabled">
            <SelectValue />
          </SelectTrigger>
        </Select>
        <Tooltip>
          <TooltipTrigger render={<span />} data-probe="tooltip-hover-trigger">
            hover
          </TooltipTrigger>
          <TooltipPopup data-probe="tooltip-hover">Opened by hovering</TooltipPopup>
        </Tooltip>
      </div>
      {/* Open popups, in a row of their own, so none covers the tooltip the check hovers. */}
      <div className="relative flex items-start gap-40 p-4 pt-10">
        {/* An open list: its popup, with one item selected and one not. Not modal, so the page
            stays hoverable for the tooltip check. */}
        <Select items={selectItems} value="automatic" open modal={false}>
          <SelectTrigger data-probe="select-trigger-open">
            <SelectValue />
          </SelectTrigger>
          <SelectPopup data-probe="select-popup">
            {selectItems.map((item) => (
              <SelectItem key={item.value} value={item.value}>
                {item.label}
              </SelectItem>
            ))}
          </SelectPopup>
        </Select>
        <PreviewCard open>
          <PreviewCardTrigger render={<span />}>card</PreviewCardTrigger>
          <PreviewCardPopup data-probe="preview-card" className="w-60 p-3">
            <strong>atelier-toee</strong>
            <p>A record&apos;s card</p>
          </PreviewCardPopup>
        </PreviewCard>
      </div>
    </TooltipProvider>
  );
}
