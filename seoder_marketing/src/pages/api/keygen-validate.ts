const id = import.meta.env.KEYGEN_ACCOUNT_ID;

export async function post({ request }) {
  const jsonData = await request.json();
  const key = jsonData?.key;

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

  const response = await fetch(
    `https://api.keygen.sh/v1/accounts/${id}/licenses/actions/validate-key`,
    {
      method: "POST",
      headers: {
        "Content-Type": "application/vnd.api+json",
        Accept: "application/vnd.api+json",
      },
      body: JSON.stringify({
        meta: { key },
      }),
    }
  );

  const { meta } = await response.json();

  const valid = meta?.valid;

  return new Response(JSON.stringify({ valid }), {
    status: valid ? 200 : 401,
    headers: {
      "Content-Type": "application/json",
    },
  });
}
