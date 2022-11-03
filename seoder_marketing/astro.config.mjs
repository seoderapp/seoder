import { defineConfig } from "astro/config";
import react from "@astrojs/react";
import vercel from "@astrojs/vercel/serverless";
import robotsTxt from "astro-robots-txt";
import sitemap from "@astrojs/sitemap";
import image from "@astrojs/image";

const site =
  typeof process !== "undefined"
    ? `https://${process.env.VERCEL_URL}`
    : "https://seoder.com";

// https://astro.build/config
export default defineConfig({
  site,
  output: "server",
  adapter: vercel(),
  integrations: [
    react(),
    robotsTxt(),
    sitemap({
      customPages: [
        "/",
        "/payments",
        "/faq",
        "/privacy-policy",
        "/terms-of-service",
      ],
    }),
    image({
      serviceEntryPoint: "@astrojs/image/sharp",
    }),
  ],
});
