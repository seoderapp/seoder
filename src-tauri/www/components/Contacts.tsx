import { useStore } from "@nanostores/react";
import { contactsModalData } from "../stores/app";

export const Contacts = () => {
  const $selected = useStore(contactsModalData);

  const contacts = $selected?.contacts ?? [];
  const title = $selected?.domain ?? "Contact Modal";

  return (
    <div style={{ padding: "1rem", overflow: "hidden" }}>
      <div style={{ fontSize: "2rem", fontWeight: "bold" }}>{title}</div>
      <ul
        style={{
          gap: "0.5rem",
          listStyle: "none",
          margin: 0,
          padding: "1rem 0",
          maxWidth: "60vw",
          maxHeight: "50vh",
          overflowY: "auto",
          overflowX: "hidden",
        }}
      >
        {contacts?.map((contact) => {
          const url = new URL(contact);

          return (
            <li
              key={contact}
              style={{
                padding: "0.3rem",
                border: "1px solid #ccc",
                borderRadius: 4,
              }}
            >
              <a
                href={contact}
                style={{
                  textDecoration: "none",
                  display: "block",
                  width: "100%",
                }}
              >
                {url?.pathname || contact}
              </a>
            </li>
          );
        })}
      </ul>
    </div>
  );
};
