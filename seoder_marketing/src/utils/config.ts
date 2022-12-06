export const id = import.meta.env.KEYGEN_ACCOUNT_ID;
export const productToken = import.meta.env.KEYGEN_PRODUCT_TOKEN;
export const token = import.meta.env.KEYGEN_API_TOKEN;
export const hunterioToken = import.meta.env.HUNTER_IO;

export const headers = {
  "Content-Type": "application/vnd.api+json",
  Accept: "application/vnd.api+json",
  Authorization: `Bearer ${token}`,
};
