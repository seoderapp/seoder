import { ImageResponse } from "@vercel/og";

export const config = {
  runtime: "experimental-edge",
};

export async function get({ request }) {
  try {
    const { searchParams } = new URL(request.url);

    const hasTitle = searchParams.has("title");
    const title = hasTitle
      ? searchParams.get("title")?.slice(0, 100)
      : "Seoder Marketing Tool";

    return new ImageResponse(
      (
        <div
          style={{
            backgroundColor: "black",
            backgroundSize: "150px 150px",
            height: "100%",
            width: "100%",
            display: "flex",
            textAlign: "center",
            alignItems: "center",
            justifyContent: "center",
            flexDirection: "column",
            flexWrap: "nowrap",
          }}
        >
          <div
            style={{
              display: "flex",
              alignItems: "center",
              justifyContent: "center",
              justifyItems: "center",
            }}
          >
            <img
              alt="Seoder"
              height={200}
              src="data:image/svg+xml,%3Csvg width='30' height='30' viewBox='0 0 30 30' fill='none' xmlns='http://www.w3.org/2000/svg'%3E%3Cpath fill-rule='evenodd' clip-rule='evenodd' d='M21.0176 1.30456C16.9402 0.624228 12.7788 0.624228 8.70179 1.30456C6.81543 1.61609 5.07411 2.51168 3.72244 3.86522C2.37045 5.21875 1.47643 6.9628 1.16531 8.85175C0.485912 12.9344 0.485912 17.1019 1.16531 21.1845C1.47641 23.0735 2.37045 24.8172 3.72244 26.1711C5.07411 27.5246 6.81543 28.4202 8.70179 28.7314C10.7382 29.0587 12.7977 29.2176 14.8597 29.2064C16.9229 29.2058 18.9825 29.0352 21.0176 28.6961C22.9039 28.3846 24.6453 27.489 25.9969 26.1354C27.3489 24.7819 28.2429 23.0379 28.5541 21.1489C28.8859 19.1165 29.0504 17.0601 29.046 15.0004C29.0454 12.9344 28.875 10.872 28.5364 8.83405C28.2232 6.95096 27.3301 5.21278 25.9817 3.86297C24.6338 2.51284 22.898 1.61855 21.0175 1.30478L21.0176 1.30456ZM14.0701 13.2243H17.0759V13.2244C17.9659 13.2263 18.7938 13.6811 19.2734 14.4318C19.753 15.1824 19.8189 16.1256 19.4478 16.9354V16.9357L17.3376 21.5126C17.1508 21.9558 16.7924 22.3044 16.3443 22.4782C15.8961 22.6519 15.397 22.6361 14.9607 22.4348C14.5243 22.2332 14.1885 21.8629 14.0299 21.4086C13.8712 20.9547 13.9034 20.4555 14.1189 20.0253L15.6175 16.7757H12.6117C11.7204 16.7745 10.8911 16.3193 10.4111 15.5674C9.93119 14.8152 9.86656 13.8704 10.2398 13.06L12.35 8.48745C12.5368 8.04428 12.8952 7.6956 13.3433 7.52188C13.7912 7.34816 14.2906 7.36395 14.7269 7.56523C15.1633 7.76683 15.4991 8.13716 15.6577 8.59144C15.8164 9.04537 15.7842 9.54458 15.5687 9.97471L14.0701 13.2243Z' fill='%232F2768'/%3E%3C/svg%3E%0A"
              style={{ margin: "0 30px" }}
              width={232}
            />
          </div>
          <div
            style={{
              fontSize: 60,
              fontStyle: "normal",
              letterSpacing: "-0.025em",
              color: "white",
              marginTop: 30,
              padding: "0 120px",
              lineHeight: 1.4,
              whiteSpace: "pre-wrap",
            }}
          >
            {title}
          </div>
        </div>
      ),
      {
        width: 1200,
        height: 630,
      }
    );
  } catch (e) {
    return new Response(`Failed to generate the image`, {
      status: 500,
    });
  }
}
