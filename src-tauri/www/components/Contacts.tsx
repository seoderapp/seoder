import { useStore } from "@nanostores/react";
import { useEffect, useState } from "react";
import { contactsModalData } from "../stores/app";

interface EData {
  emails?: any[];
}

// get contacts from server
const getContacts = (title: string) => {
  if (title) {
    const url = "http://localhost:7050/prospect";
    const request = new XMLHttpRequest();

    return new Promise((resolve) => {
      request.open("POST", url, true);
      request.onload = function () {
        resolve(request.response);
      };
      request.onerror = function () {};

      const form = new FormData();

      const blob = new Blob([title.trim()], {
        type: "text/plain",
      });

      const file = new File([blob], title, {
        type: "text/plain",
      });

      form.append("title", file);

      request.send(form);
    });
  }
};

export const ProspectFinder = ({ title }: { title: string }) => {
  const [results, setResults] = useState<EData>();

  useEffect(() => {
    getContacts(title).then((res: any) => {
      try {
        if (res) {
          if (typeof res === "string") {
            const j = JSON.parse(res);

            setResults(j?.data);
          } else {
            setResults(res?.data);
          }
        } else {
          setResults({ emails: [] });
        }
      } catch (e) {
        console.error(e);
      }
    });
  }, []);

  if (results) {
    if (results.emails?.length) {
      return (
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
          {results.emails
            ?.filter((item) => item.confidence >= 50)
            ?.map((contact) => {
              return (
                <li
                  key={contact}
                  style={{
                    padding: "0.3rem",
                    border: "1px solid #ccc",
                    borderRadius: 4,
                  }}
                >
                  <div
                    style={{
                      textDecoration: "none",
                      display: "block",
                      width: "100%",
                      padding: "0.3rem 0.5rem",
                    }}
                  >
                    <div
                      style={{
                        display: "flex",
                        flexDirection: "column",
                        gap: "0.4rem",
                      }}
                    >
                      {contact.first_name ? (
                        <div>
                          {contact.first_name} {contact.last_name}
                        </div>
                      ) : (
                        <div>N/A</div>
                      )}
                      <div>
                        Email:{" "}
                        {contact.value ? (
                          <a href={contact.value}>{contact.value}</a>
                        ) : (
                          "N/A"
                        )}
                      </div>
                      <div>Department: {contact.department ?? "N/A"}</div>
                      <div>Position: {contact.position ?? "N/A"}</div>
                      <div>
                        Twitter:{" "}
                        {contact.twitter ? (
                          <a
                            rel="noopener"
                            target="_blank"
                            href={contact.twitter}
                          >
                            {contact.twitter}
                          </a>
                        ) : (
                          "N/A"
                        )}
                      </div>
                      <div>
                        LinkedIn:{" "}
                        {contact.linkedin ? (
                          <a
                            rel="noopener"
                            target="_blank"
                            href={contact.linkedin}
                          >
                            {contact.linkedin}
                          </a>
                        ) : (
                          "N/A"
                        )}
                      </div>
                      <div>Phone: {contact.phone_number ?? "N/A"}</div>
                    </div>
                  </div>
                </li>
              );
            })}
        </ul>
      );
    } else {
      return <div>No leads found</div>;
    }
  }
  return <div>Loading...</div>;
};

export const Contacts = () => {
  const $selected = useStore(contactsModalData);
  const [personal, setPersonal] = useState<boolean>(false);

  const contacts = $selected?.contacts ?? [];
  const title = $selected?.domain ?? "Contact Modal";

  const onClickCompany = () => {
    setPersonal(false);
  };

  const onClickPersonal = () => {
    setPersonal(true);
  };

  return (
    <div>
      <div className="row">
        <button
          onClick={onClickCompany}
          className={`tab${!personal ? " tab-active" : ""}`}
        >
          Company
        </button>
        <button
          onClick={onClickPersonal}
          className={`tab${!personal ? "" : " tab-active"}`}
        >
          Prospects
        </button>
      </div>
      <div style={{ padding: "1rem", overflow: "hidden" }}>
        <div style={{ fontSize: "2rem", fontWeight: "bold" }}>{title}</div>
        {!personal ? (
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
        ) : (
          <ProspectFinder title={title} />
        )}
      </div>
    </div>
  );
};
