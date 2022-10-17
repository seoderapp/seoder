import { transport } from "../../utils/mailer";

export async function get({ params }) {
  let info = await transport.sendMail({
    from: "support@seoder.io", // sender address
    to: "jeff@a11ywatch.com, hello@gilbertb.com", // list of receivers
    subject: "Seoder Astro SSR License✔", // Subject line
    text: "License key test", // plain text body
    html: "<b>12132423232</b>", // html body
  });

  return new Response(JSON.stringify({ core: 123, params }), {
    status: 200,
    headers: {
      "Content-Type": "application/json",
    },
  });
}
