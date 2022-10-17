import Stripe from "stripe";

export const stripe = new Stripe(
  import.meta.env.PUBLIC_STRIPE_PUBLISHABLE_KEY,
  null
);
