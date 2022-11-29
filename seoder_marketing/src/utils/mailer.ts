import nodemailer from "nodemailer";

const cid = import.meta.env.EMAIL_CLIENT_ID;
const ckey = import.meta.env.EMAIL_CLIENT_KEY;

export const transport = () => {
  return nodemailer.createTransport({
    host: "smtp.gmail.com",
    port: 465,
    secure: true,
    auth: {
      type: "OAuth2",
      user: "support@seoder.com",
      serviceClient: cid,
      privateKey: ckey.replace(/\\n/gm, "\n"),
    },
  });  
}