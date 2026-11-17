// Copied by sync-t3.mjs from T3 Code (MIT, see LICENSE), apps/web/src/lib/utils.ts (twMerge and cn) at 532a7aaf424d. Do not edit: run sync-t3.mjs.
import { type CxOptions, cx } from "class-variance-authority";
import { extendTailwindMerge } from "tailwind-merge";

const twMerge = extendTailwindMerge({
  extend: {
    theme: { text: ["2xs", "3xs", "4xs", "5xs"] },
    classGroups: { "bg-image": [{ bg: ["composer-seam-below", "composer-seam-above"] }] },
  },
});

export function cn(...inputs: CxOptions) {
  return twMerge(cx(inputs));
}
