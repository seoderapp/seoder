import { FC, useRef, useEffect, useState } from "react";
import { useStore } from "@nanostores/react";
import { errorLogs, validLogs } from "../stores/engine";

export interface Props {
  id?: string;
  emptyId?: string;
  logs?: Set<string | unknown>;
  scrolling?: boolean;
}

const LogItem: FC = ({ children }) => {
  return <li>{children}</li>;
};

const scrollStyles = {
  padding: "0.4rem",
  background: "transparent",
  color: "#fff",
  textAlign: "center",
};

// log panel
export function Log({ id, emptyId, logs }: Props) {
  const list = Array.from(logs?.keys() ?? []);

  return (
    <div className="log">
      {!list.length ? (
        <div id={emptyId} className={"block gutter-t"}>
          Ready to run
        </div>
      ) : null}
      <ul id={id} role={"list"}>
        {list.map((item: string) => (
          <LogItem key={item}>{item}</LogItem>
        ))}
      </ul>
    </div>
  );
}

// unused log errors single
export function LogErrors({ id, emptyId, scrolling }: Props) {
  const list = useStore(errorLogs);
  const divRef = useRef(null);
  const [scroll, setScrolling] = useState<boolean>(scrolling);

  useEffect(() => {
    setScrolling(scrolling);
  }, [scrolling]);

  useEffect(() => {
    const listener = errorLogs.listen((v) => {
      if (scroll && v.length > list.length) {
        divRef?.current?.scrollIntoView({
          behavior: "smooth",
          block: "end",
          inline: "nearest",
        });
      }
    });
    return () => {
      listener();
    };
  }, [list, scroll]);

  const onToggleScrolling = () => {
    setScrolling((v) => !v);
  };

  const empty = !list.length;

  return (
    <div className="log">
      {empty ? (
        <div id={emptyId} className={"block gutter-t"}>
          Ready to run
        </div>
      ) : null}
      <ul id={id} role={"list"}>
        {list.map((item: string) => (
          <LogItem key={item}>{item}</LogItem>
        ))}
        <li
          style={{
            padding: 0,
            textAlign: "center",
            display: !empty ? "list-item" : "none",
          }}
          ref={divRef}
        >
          <button onClick={onToggleScrolling} style={scrollStyles}>
            {scroll ? "Stop Scrolling" : "Stick to Bottom"}
          </button>
        </li>
      </ul>
    </div>
  );
}

// unused log valid panel single
export function LogValid({ id, emptyId, scrolling }: Props) {
  const list = useStore(validLogs);
  const divRef = useRef(null);
  const [scroll, setScrolling] = useState<boolean>(scrolling);

  // todo: control scroll state from parent
  useEffect(() => {
    setScrolling(scrolling);
  }, [scrolling]);

  useEffect(() => {
    const listener = validLogs.listen((v) => {
      if (scroll && v.length > list.length) {
        divRef?.current?.scrollIntoView({
          behavior: "smooth",
          block: "end",
          inline: "nearest",
        });
      }
    });
    return () => {
      listener();
    };
  }, [list, scroll]);

  const onToggleScrolling = () => {
    setScrolling((v) => !v);
  };

  const empty = !list.length;

  return (
    <div className="log">
      {empty ? (
        <div id={emptyId} className={"block gutter-t"}>
          Ready to run
        </div>
      ) : null}
      <ul id={id} role={"list"}>
        {list.map((item: string) => (
          <LogItem key={item}>{item}</LogItem>
        ))}
        <li
          style={{
            padding: 0,
            textAlign: "center",
            display: !empty ? "list-item" : "none",
          }}
          ref={divRef}
        >
          <button onClick={onToggleScrolling} style={scrollStyles}>
            {scroll ? "Stop Scrolling" : "Stick to Bottom"}
          </button>
        </li>
      </ul>
    </div>
  );
}
