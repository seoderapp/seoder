import nodemailer from "nodemailer";
import { stripe } from "../../utils/stripe";

const cid = import.meta.env.EMAIL_CLIENT_ID;
const ckey = import.meta.env.EMAIL_CLIENT_KEY;

const stripePlanId = import.meta.env.STRIPE_PLAN_ID;

// const keygenProductToken = import.meta.env.KEYGEN_PRODUCT_TOKEN;
const keygenAccountId = import.meta.env.KEYGEN_ACCOUNT_ID;
const keygenPolicyId = import.meta.env.KEYGEN_POLICY_ID;
const token = import.meta.env.KEYGEN_API_TOKEN;

const transportor = nodemailer.createTransport({
  host: "smtp.gmail.com",
  port: 465,
  secure: true,
  auth: {
    type: "OAuth2",
    user: "support@seoder.com",
    serviceClient: cid,
    privateKey: ckey.replace(/\\n/gm, "\n"),
  },
});

export async function post({ request }) {
  const stripeEvent = await request?.json();
  const { object: stripeCustomer } = stripeEvent?.data ?? {};

  let statusCode = 200;
  
  switch (stripeEvent.type) {
    case "invoice.payment_succeeded": {
      // Make sure our Stripe customer has a Keygen user ID, or else we can't work with it.
      if (!stripeCustomer.metadata.keygenUserId) {
        throw new Error(
          `Customer ${stripeCustomer.id} does not have a Keygen user ID attached to their customer account!`
        );
      }

      // if new payment renew the key
      if(stripeCustomer.billing_reason !== 'subscription_create') {
        const keygenLicense = await fetch(
          `https://api.keygen.sh/v1/accounts/${keygenAccountId}/licenses`,
          {
            method: "POST",
            headers: {
              Authorization: `Bearer ${token}`,
              "Content-Type": "application/vnd.api+json",
              Accept: "application/vnd.api+json",
            },
            body: JSON.stringify({
              data: {
                type: "licenses",
                attributes: {
                  metadata: {
                    stripeSubscriptionId:
                      stripeCustomer?.subscription?.id ?? stripePlanId,
                  },
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
  
        const key = data?.attributes?.key;
  
        if (key) {
          // renew a valid
          const keygenRenewResponse = await fetch(
            `https://api.keygen.sh/v1/accounts/${keygenAccountId}/licenses/${key}/actions/renew`,
            {
              method: "GET",
              headers: {
                Authorization: `Bearer ${token}`,
                Accept: "application/vnd.api+json",
              },
            }
          );
  
          const renewable = await keygenRenewResponse.json();
  
          if (renewable?.errors) {
            console.error(renewable.errors);
          }
        } else {
          console.error(errors);
        }
      }
    }

    case "customer.created": {
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
          items: [{ plan: stripePlanId }],
          trial_period_days: 7
        },
        {
          // Use an idempotency key so that we don't charge a customer more than once
          idempotencyKey: stripeCustomer.metadata.keygenUserId,
        }
      );

      // 6. Create a license for the new Stripe customer after we create a subscription
      //    for them. We're pulling the Keygen user's ID from the Stripe customer's
      const keygenLicense = await fetch(
        `https://api.keygen.sh/v1/accounts/${keygenAccountId}/licenses`,
        {
          method: "POST",
          headers: {
            Authorization: `Bearer ${token}`,
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

      const { data, errors } = await keygenLicense?.json();
      
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
        await transportor.sendMail({
          from: "support@seoder.com",
          to: stripeCustomer.email,
          subject: "Seoder One Year License✔",
          text: "License key for the application.",
          html: `
          <div>
            <h1>Thank you for purchasing Seoder<h1>
            <h2>License Key<h2/>
            <b style="font-size: 2rem;">${data.attributes.key}</b>
            <h3>Number of Machines ${data?.attributes?.maxMachines || 1}<h3/>
            <div style="padding-bottom: 1.5rem;">
              <p>Please do not share the key. Only one machine is allowed per license.</p>
            </div>
            <div style="padding-bottom: 3rem;">
              <a href="https://seoder.com">Seoder</a>
            </div>
          </div>`, // html body
        });
      }

      statusCode = 200;
      break;
    }

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
