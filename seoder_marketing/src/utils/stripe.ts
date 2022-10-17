import Stripe from "stripe";

const key = import.meta.env.PUBLIC_STRIPE_PUBLISHABLE_KEY;

export const stripe = new Stripe(key, undefined);
