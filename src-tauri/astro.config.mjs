import { defineConfig } from "astro/config";
import react from "@astrojs/react";

// https://astro.build/config
import partytown from "@astrojs/partytown";

// https://astro.build/config
export default defineConfig({
  integrations: [react(), partytown()],
  srcDir: "./www",
  outDir: "../out",
  server: {
    port: 3001,
    host: true
  }
});