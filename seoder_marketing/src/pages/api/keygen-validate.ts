const id = import.meta.env.KEYGEN_ACCOUNT_ID;

export async function post({ request }) {
  const jsonData = await request.json();

  const key = jsonData?.key;

  const response = await fetch(
    `https://api.keygen.sh/v1/accounts/${id}/licenses/actions/validate-key`,
    {
      method: "POST",
      headers: {
        "Content-Type": "application/vnd.api+json",
        Accept: "application/vnd.api+json",
      },
      body: JSON.stringify({
        data: { key: key },
      }),
    }
  );

  const { meta } = await response.json();

  return new Response(JSON.stringify({ valid: meta?.valid }), {
    status: meta?.valid ? 200 : 401,
    headers: {
      "Content-Type": "application/json",
    },
  });
}
