const id = import.meta.env.KEYGEN_ACCOUNT_ID;

export async function post({ request }) {
  const jsonData = await request.json();

  let statusCode = 200;

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
    status: statusCode,
    headers: {
      "Content-Type": "application/json",
    },
  });
}
