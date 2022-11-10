import { defineConfig } from "astro/config";
import react from "@astrojs/react";
import vercel from "@astrojs/vercel/serverless";
import robotsTxt from "astro-robots-txt";
import sitemap from "@astrojs/sitemap";
import image from "@astrojs/image";
const site = process.env.DOMAIN || "https://seoder.com";

// https://astro.build/config
export default defineConfig({
  site,
  output: "server",
  adapter: vercel(),
  integrations: [
    react(),
    sitemap({
      customPages: [
        "/",
        "/about",
        "/payments",
        "/faq",
        "/privacy-policy",
        "/terms-of-service",
        "/seos-can-be-dangerous",
        "/compare/seoder-vs-zendesk-sell",
      ],
    }),
    robotsTxt({
      sitemap: true,
    }),
    image({
      serviceEntryPoint: "@astrojs/image/sharp",
    }),
  ],
});
