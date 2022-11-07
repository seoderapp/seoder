import { hunterioToken } from "../../utils/config";
import { validateKey } from "../../utils/validate-keygen";

// todo: try to improve og images or add default images per route
export async function post({ request }) {
  console.log("sp");
  let jsonData = {
    key: "",
    fingerprint: "",
    platform: "",
    title: "",
  };

  // todo: fix
  try {
    jsonData = await request.json();
  } catch (e) {
    console.error(e);
  }

  const valid = await validateKey(jsonData);

  if (valid) {
    const data = await fetch(
      `https://api.hunter.io/v2/domain-search?domain=${jsonData?.title}&api_key=${hunterioToken}`
    );
    const json = await data.json();

    return new Response(JSON.stringify(json), {
      headers: {
        "content-type": "application/json",
      },
    });
  } else {
    return new Response(JSON.stringify({ message: "error", data: null }), {
      headers: {
        "content-type": "application/json",
      },
    });
  }
}
