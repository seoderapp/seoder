import { headers, id, productToken } from "../../utils/config";

export async function post({ request }) {
  let jsonData = {
    key: "",
    fingerprint: "",
    platform: "",
  };

  // todo: fix
  try {
    jsonData = await request.json();
  } catch (e) {
    console.error(e);
  }

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
