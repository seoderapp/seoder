import Stripe from "stripe";

const key = import.meta.env.PUBLIC_STRIPE_PUBLISHABLE_KEY;

const stripe = new Stripe(key, {
    apiVersion: "2022-08-01"
});

export { stripe }