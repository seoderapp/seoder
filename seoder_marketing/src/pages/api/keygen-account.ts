export async function post({ request }) {
  const jsonData = await request.json();

  let statusCode = 200;

  const response = await fetch(
    `https://api.keygen.sh/v1/accounts/${
      import.meta.env.KEYGEN_ACCOUNT_ID
    }/users`,
    {
      method: "POST",
      headers: {
        "Content-Type": "application/vnd.api+json",
        Accept: "application/vnd.api+json",
      },
      body: JSON.stringify({
        data: { type: "users", attributes: jsonData?.data },
      }),
    }
  );

  const res = await response.json();

  return new Response(JSON.stringify(res), {
    status: statusCode,
    headers: {
      "Content-Type": "application/json",
    },
  });
}
