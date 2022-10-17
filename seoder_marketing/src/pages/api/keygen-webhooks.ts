// import { transport } from "../../utils/mailer";
import { stripe } from "../../utils/stripe";

const productToken =
  typeof process === "undefined"
    ? import.meta.env.KEYGEN_PRODUCT_TOKEN
    : process.env.KEYGEN_PRODUCT_TOKEN;

const accountId =
  typeof process === "undefined"
    ? import.meta.env.KEYGEN_ACCOUNT_ID
    : process.env.KEYGEN_ACCOUNT_ID;

export async function post({ request }) {
  const jsonData = await request.json();
  const { data } = jsonData ?? { data: null };
  const keygenEventId = data?.id;

  let statusCode = 200;

  const keygenWebhook = await fetch(
    `https://api.keygen.sh/v1/accounts/${accountId}/webhook-events/${keygenEventId}`,
    {
      method: "GET",
      headers: {
        Authorization: `Bearer ${productToken}`,
        Accept: "application/vnd.api+json",
      },
    }
  );

  const { data: keygenEvent, errors } = await keygenWebhook.json();

  if (errors) {
    return new Response(null, {
      status: 200,
    });
  }

  switch (keygenEvent.attributes.event) {
    // 1. Respond to user creation events within your Keygen account. Here, we'll create
    //    a new Stripe customer account for new Keygen users.
    case "user.created":
      const { data: keygenUser } = JSON.parse(keygenEvent.attributes.payload);

      // Make sure our Keygen user has a Stripe token, or else we can't charge them later on..
      if (!keygenUser.attributes.metadata.stripeToken) {
        throw new Error(
          `User ${keygenUser.id} does not have a Stripe token attached to their user account!`
        );
      }

      // 2. Create a Stripe customer, making sure we use our Stripe token as their payment
      //    method of choice.
      const stripeCustomer = await stripe.customers.create({
        description: `Customer for Keygen user ${keygenUser.attributes.email}`,
        email: keygenUser.attributes.email,
        // Source is a Stripe token obtained with Stripe.js during user creation and
        // temporarily stored in the user's metadata attribute.
        source: keygenUser.attributes.metadata.stripeToken,
        // Store the user's Keygen ID within the Stripe customer so that we can lookup
        // a Stripe customer's Keygen account.
        metadata: { keygenUserId: keygenUser.id },
      });

      // 3. Add the user's Stripe customer ID to the user's metadata attribute so that
      //    we can lookup their Stripe customer account when needed.
      const update = await fetch(
        `https://api.keygen.sh/v1/accounts/${accountId}/users/${keygenUser.id}`,
        {
          method: "PATCH",
          headers: {
            Authorization: `Bearer ${productToken}`,
            "Content-Type": "application/vnd.api+json",
            Accept: "application/vnd.api+json",
          },
          body: JSON.stringify({
            data: {
              type: "users",
              attributes: {
                metadata: { stripeCustomerId: stripeCustomer.id },
              },
            },
          }),
        }
      );

      const { data, errors } = await update.json();

      if (errors) {
        throw new Error(errors.map((e) => e.detail).toString());
      }

      // All is good! Stripe customer was successfully created for the new Keygen
      // user. Let Keygen know the event was received successfully.
      statusCode = 200;
      break;
    default:
      // For events we don't care about, let Keygen know all is good.
      statusCode = 200;
  }

  return new Response(JSON.stringify({ core: 123, jsonData }), {
    status: statusCode,
    headers: {
      "Content-Type": "application/json",
    },
  });
}
