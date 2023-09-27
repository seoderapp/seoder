import { defineConfig } from "astro/config";
import react from "@astrojs/react";
import vercel from "@astrojs/vercel/serverless";
import robotsTxt from "astro-robots-txt";
import sitemap from "@astrojs/sitemap";
import image from "@astrojs/image";

const site = process.env.PUBLIC_VERCEL_URL ? `https://${process.env.PUBLIC_VERCEL_URL}` : process.env.DOMAIN || "https://seoder.com";

// https://astro.build/config
export default defineConfig({
  site: process.env.NODE_ENV === "development" ? "http://localhost:3000": site,
  output: "server",
  adapter: vercel(),
  integrations: [
    react(),
    sitemap({
      customPages: [
        `${site}/`,
        `${site}/about`,
        `${site}/payments`,
        `${site}/faq`,
        `${site}/privacy-policy`,
        `${site}/terms-of-service`,
        `${site}/seos-can-be-dangerous`,
        `${site}/compare/seoder-vs-zendesk-sell`,
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
