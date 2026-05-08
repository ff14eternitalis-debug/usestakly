import { readFile } from "node:fs/promises";
import { dirname, join } from "node:path";
import { fileURLToPath } from "node:url";

import sharp from "sharp";

const root = join(dirname(fileURLToPath(import.meta.url)), "..");
const svgPath = join(root, "public", "og-card.svg");
const outPath = join(root, "public", "og-image.png");

const svg = await readFile(svgPath);
await sharp(svg, { density: 144 }).png({ compressionLevel: 9 }).toFile(outPath);
