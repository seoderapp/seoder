const VERSION = "0.0.2"; // todo: env
const APP = "Seoder_";

const LINUX = "_linux.app.tar.gz";
const WINDOWS = "_x64_en-US.msi";
const MAC = "_universal.dmg";
const MAC_M1 = "_aarch64.dmg";

const BASE = import.meta.env.DEV
  ? "http://localhost:3000"
  : "https://seoder.io";

const notes = import.meta.env.RELEASE_NOTES || "";
const pub_date = import.meta.env.PUBLISHED || new Date().toISOString();
const signature = import.meta.env.SIGNATURE || "";

export async function get({ params }) {
  const { target, version } = params;

  let schema = null;

  if (version !== VERSION) {
    let plat = "";

    switch (target) {
      case "darwin": {
        plat = MAC;
        break;
      }
      case "darwin-aarch64": {
        plat = MAC_M1;
        break;
      }
      case "linux-x86_64": {
        plat = LINUX;
        break;
      }
      case "windows-x86_64": {
        plat = WINDOWS;
        break;
      }
      default:
        plat = MAC;
        break;
    }

    schema = {
      url: `${BASE}/releases/${APP}${VERSION}${plat}`,
      version: VERSION,
      notes,
      pub_date: pub_date,
      signature,
    };
  }

  return new Response(schema ? JSON.stringify(schema) : null, {
    status: 200,
    headers: {
      "Content-Type": "application/json",
    },
  });
}
