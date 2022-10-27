import { defineConfig } from "astro/config";
import react from "@astrojs/react";

// https://astro.build/config
export default defineConfig({
  integrations: [react()],
  srcDir: "./www",
  outDir: "../out",
  server: {
    port: 3001,
    host: true,
  },
});
