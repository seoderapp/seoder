import { defineConfig } from "astro/config";
import react from "@astrojs/react";
import vercel from "@astrojs/vercel/serverless";
import robotsTxt from "astro-robots-txt";
import sitemap from "@astrojs/sitemap";
import image from "@astrojs/image";

// https://astro.build/config
export default defineConfig({
  site: "https://seoder.io",
  output: "server",
  adapter: vercel({}),
  integrations: [
    react(),
    robotsTxt(),
    sitemap(),
    image({
      serviceEntryPoint: "@astrojs/image/sharp",
    }),
  ],
});
