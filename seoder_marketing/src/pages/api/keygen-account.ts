const id = import.meta.env.KEYGEN_ACCOUNT_ID;
const token = import.meta.env.KEYGEN_API_TOKEN;

export async function post({ request }) {
  const jsonData = await request.json();

  const response = await fetch(
    `https://api.keygen.sh/v1/accounts/${id}/users`,
    {
      method: "POST",
      headers: {
        "Content-Type": "application/vnd.api+json",
        Accept: "application/vnd.api+json",
        Authorization: `Bearer ${token}`,
      },
      body: JSON.stringify({
        data: { type: "users", attributes: jsonData?.data },
      }),
    }
  );

  const res = await response.json();

  return new Response(JSON.stringify(res), {
    status: 200,
    headers: {
      "Content-Type": "application/json",
    },
  });
}
