import nodemailer from "nodemailer";

export const transport = nodemailer.createTransport({
  host: "smtp.gmail.com",
  port: 465,
  secure: true,
  auth: {
    type: "OAuth2",
    user: "support@seoder.io",
    serviceClient: import.meta.env.EMAIL_CLIENT_ID,
    privateKey: import.meta.env.EMAIL_CLIENT_KEY.replace(/\\n/gm, "\n"),
  },
});
