import { describe, expect, it } from "vitest";
import { isEmbedded } from "./embed";

describe("isEmbedded", () => {
  it("takes /embed and the paths under it", () => {
    expect(isEmbedded("/embed")).toBe(true);
    expect(isEmbedded("/embed/")).toBe(true);
    expect(isEmbedded("/embed/r/atelier/item/atelier-1")).toBe(true);
  });
  it("leaves every other path alone", () => {
    expect(isEmbedded("/")).toBe(false);
    expect(isEmbedded("/inbox")).toBe(false);
    expect(isEmbedded("/embedded")).toBe(false);
    expect(isEmbedded("/r/embed")).toBe(false);
  });
});
