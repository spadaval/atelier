// T3 Code's `cn`, without the rest of its `lib/utils`.
import { type CxOptions, cx } from "class-variance-authority";
import { extendTailwindMerge } from "tailwind-merge";

const twMerge = extendTailwindMerge({ extend: { theme: { text: ["2xs", "3xs", "4xs", "5xs"] } } });
export const cn = (...inputs: CxOptions) => twMerge(cx(inputs));
