import { defineConfig } from "astro/config";
import react from "@astrojs/react";
import vercel from "@astrojs/vercel/serverless";

// https://astro.build/config
export default defineConfig({
  output: "server",
  adapter: vercel(),
  integrations: [react()],
  vite: {
    define: {
      // keygen
      "process.env.KEYGEN_ACCOUNT_ID": JSON.stringify(
        process.env.KEYGEN_ACCOUNT_ID
      ),
      "process.env.KEYGEN_PRODUCT_TOKEN": JSON.stringify(
        process.env.KEYGEN_PRODUCT_TOKEN
      ),
      "process.env.KEYGEN_POLICY_ID": JSON.stringify(
        process.env.KEYGEN_POLICY_ID
      ),
      // stripe
      "process.env.STRIPE_SECRET_KEY": JSON.stringify(
        process.env.STRIPE_SECRET_KEY
      ),
      "process.env.STRIPE_PUBLISHABLE_KEY": JSON.stringify(
        process.env.STRIPE_PUBLISHABLE_KEY
      ),
      "process.env.STRIPE_PLAN_ID": JSON.stringify(process.env.STRIPE_PLAN_ID),
      // email
      "process.env.EMAIL_CLIENT_KEY": JSON.stringify(
        process.env.EMAIL_CLIENT_KEY
      ),
      "process.env.EMAIL_CLIENT_ID": JSON.stringify(
        process.env.EMAIL_CLIENT_ID
      ),
    },
  },
});
