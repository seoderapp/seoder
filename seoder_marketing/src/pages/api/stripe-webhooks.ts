import { transport } from "../../utils/mailer";
import { stripe } from "../../utils/stripe";

const stripePlanId =
  typeof process === "undefined"
    ? import.meta.env.STRIPE_PLAN_ID
    : process.env.STRIPE_PLAN_ID;

const keygenProductToken =
  typeof process === "undefined"
    ? import.meta.env.KEYGEN_PRODUCT_TOKEN
    : process.env.KEYGEN_PRODUCT_TOKEN;

const keygenAccountId =
  typeof process === "undefined"
    ? import.meta.env.KEYGEN_ACCOUNT_ID
    : process.env.KEYGEN_ACCOUNT_ID;

const keygenPolicyId =
  typeof process === "undefined"
    ? import.meta.env.KEYGEN_POLICY_ID
    : process.env.KEYGEN_POLICY_ID;

export async function post({ request }) {
  const jsonData = await request.json();
  const { stripeEvent } = jsonData ?? { stripeEvent: null };

  let statusCode = 200;

  switch (stripeEvent.type) {
    // todo: re-send subscription new licenses
    // case "invoice.payment_succeeded"

    // 4. Respond to customer creation events within your Stripe account. Here, we'll
    //    create a new Stripe subscription for the customer as well as a Keygen license
    //    for the Keygen user that belongs to the Stripe customer.
    case "customer.created":
      const { object: stripeCustomer } = stripeEvent.data;

      // Make sure our Stripe customer has a Keygen user ID, or else we can't work with it.
      if (!stripeCustomer.metadata.keygenUserId) {
        throw new Error(
          `Customer ${stripeCustomer.id} does not have a Keygen user ID attached to their customer account!`
        );
      }

      // 5. Create a subscription for the new Stripe customer. This will charge the
      //    Stripe customer. (You may or may not want to also check if the customer
      //    already has an existing subscription.)
      const stripeSubscription = await stripe.subscriptions.create(
        {
          customer: stripeCustomer.id,
          items: [{ price: stripePlanId }],
        },
        {
          // Use an idempotency key so that we don't charge a customer more than one
          // time regardless of how many times this webhook is retried.
          // See: https://stripe.com/docs/api/node#idempotent_requests
          idempotency_key: stripeCustomer.metadata.keygenUserId,
        }
      );

      // 6. Create a license for the new Stripe customer after we create a subscription
      //    for them. We're pulling the Keygen user's ID from the Stripe customer's
      //    metadata attribute (we stored it there earler).
      const keygenLicense = await fetch(
        `https://api.keygen.sh/v1/accounts/${keygenAccountId}/licenses`,
        {
          method: "POST",
          headers: {
            Authorization: `Bearer ${keygenProductToken}`,
            "Content-Type": "application/vnd.api+json",
            Accept: "application/vnd.api+json",
          },
          body: JSON.stringify({
            data: {
              type: "licenses",
              attributes: {
                metadata: { stripeSubscriptionId: stripeSubscription.id },
              },
              relationships: {
                policy: {
                  data: {
                    type: "policies",
                    id: keygenPolicyId,
                  },
                },
                user: {
                  data: {
                    type: "users",
                    id: stripeCustomer.metadata.keygenUserId,
                  },
                },
              },
            },
          }),
        }
      );

      const { data, errors } = await keygenLicense.json();

      if (errors) {
        statusCode = 500;

        // If you receive an error here, then you may want to handle the fact the customer
        // may have been charged for a license that they didn't receive e.g. easiest way
        // would be to create it manually, or refund their subscription charge.
        throw new Error(errors.map((e) => e.detail).toString());
      }

      // All is good! License was successfully created for the new Stripe customer's
      // Keygen user account. Next up would be for us to email the license key to
      // our user's email using `stripeCustomer.email` or something similar.
      // Let Stripe know the event was received successfully.
      if (data && data.attributes.status === "ACTIVE") {
        await transport.sendMail({
          from: "support@seoder.io", // sender address
          to: stripeCustomer.email, // list of receivers
          subject: "Seoder Astro SSR License✔", // Subject line
          text: "License key test", // plain text body
          html: `
          <div>
            <h1>View License for Seoder<h1>
            <h2>Key<h2/>
            <b>${data.attributes.key}</b>
            <h3>Number of Machines ${data.attributes.maxMachines}<h3/>
          </div>`, // html body
        });
      }

      statusCode = 200;
      break;
    default:
      // todo: re-send new key
      // For events we don't care about, let Stripe know all is good.
      statusCode = 200;
  }

  return new Response(JSON.stringify({ message: "Success" }), {
    status: statusCode,
    headers: {
      "Content-Type": "application/json",
    },
  });
}
