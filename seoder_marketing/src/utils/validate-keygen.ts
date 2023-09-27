import { headers, id, productToken } from "./config";

export const validateKey = async (jsonData) => {
  const { key, fingerprint, platform } = jsonData ?? {};

  // todo rate limit
  if (!key) {
    return new Response(
      JSON.stringify({ valid: false, message: "Missing license key" }),
      {
        status: 400,
        headers: {
          "Content-Type": "application/json",
        },
      }
    );
  }

  if (!fingerprint) {
    return new Response(
      JSON.stringify({ valid: false, message: "Missing fingerprint body" }),
      {
        status: 400,
        headers: {
          "Content-Type": "application/json",
        },
      }
    );
  }

  const metaData = {
    scope: {
      product: productToken,
      fingerprint,
    },
    key,
  };

  const response = await fetch(
    `https://api.keygen.sh/v1/accounts/${id}/licenses/actions/validate-key`,
    {
      method: "POST",
      headers,
      body: JSON.stringify({
        meta: metaData,
      }),
    }
  );

  const data = await response.json();
  const { meta } = data;

  return meta?.valid;
};
