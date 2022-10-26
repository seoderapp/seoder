const id = import.meta.env.KEYGEN_ACCOUNT_ID;
const productToken = import.meta.env.KEYGEN_PRODUCT_TOKEN;
const token = import.meta.env.KEYGEN_API_TOKEN;

const headers = {
  "Content-Type": "application/vnd.api+json",
  Accept: "application/vnd.api+json",
  Authorization: `Bearer ${token}`,
};

export async function post({ request }) {
  const jsonData = await request.json();
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
  let valid = meta?.valid;

  // assign finger print to machine
  if (!valid && meta && meta.code === "NO_MACHINES") {
    const activeResponse = await fetch(
      `https://api.keygen.sh/v1/accounts/${id}/machines`,
      {
        method: "POST",
        headers,
        body: JSON.stringify({
          data: {
            type: "machines",
            attributes: {
              fingerprint: fingerprint,
              platform: platform,
            },
            relationships: {
              license: {
                data: {
                  type: "licenses",
                  id: data?.data?.id,
                },
              },
            },
          },
        }),
      }
    );

    const ds = await activeResponse.json();

    if (ds && ds?.data?.attributes?.fingerprint === fingerprint) {
      valid = true;
    }
  }

  let status = valid ? 200 : 401;

  return new Response(JSON.stringify({ valid }), {
    status,
    headers: {
      "Content-Type": "application/json",
    },
  });
}
