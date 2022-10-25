import { FC, useRef, useEffect, useState } from "react";
import { useStore } from "@nanostores/react";
import { errorLogs, validLogs } from "../stores/engine";

export interface Props {
  id?: string;
  emptyId?: string;
  logs?: Set<string | unknown>;
  scrolling?: boolean;
}

const LogItem: FC = ({ children }) => <li>{children}</li>;

const scrollStyles = {
  padding: "0.4rem",
  background: "transparent",
  color: "#fff",
  textAlign: "center",
};

// scroll to bottom hook
const useScrolling = ({ divRef, list, scrolling, atom }) => {
  useEffect(() => {
    const listener = atom.listen((v) => {
      if (scrolling && v.length > list.length) {
        divRef?.current?.scrollIntoView();
      }
    });
    return () => {
      listener();
    };
  }, [list, scrolling]);
};

// scroll to bottom of component
function ScrollButton({ empty, divRef, scrollEnabled, onToggleScrolling }) {
  return (
    <li
      className={"empty-log"}
      style={{
        display: !empty ? "list-item" : "none",
      }}
      ref={divRef}
    >
      <button onClick={onToggleScrolling} style={scrollStyles}>
        {scrollEnabled ? "Stop Scrolling" : "Stick to Bottom"}
      </button>
    </li>
  );
}

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
  const [scrollEnabled, setScrolling] = useState<boolean>(scrolling);

  useScrolling({
    divRef,
    list,
    scrolling: scrollEnabled,
    atom: errorLogs,
  });

  useEffect(() => {
    setScrolling(scrolling);
  }, [scrolling]);

  const onToggleScrolling = () => setScrolling((v) => !v);

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
        <ScrollButton
          empty={empty}
          scrollEnabled={scrollEnabled}
          divRef={divRef}
          onToggleScrolling={onToggleScrolling}
        />
      </ul>
    </div>
  );
}
// unused log valid panel single
export function LogValid({ id, emptyId, scrolling }: Props) {
  const list = useStore(validLogs);
  const divRef = useRef(null);
  const [scrollEnabled, setScrolling] = useState<boolean>(scrolling);

  useScrolling({
    divRef,
    list,
    scrolling: scrollEnabled,
    atom: validLogs,
  });

  // todo: control scroll state from parent
  useEffect(() => {
    setScrolling(scrolling);
  }, [scrolling]);

  const onToggleScrolling = () => setScrolling((v) => !v);

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
        <ScrollButton
          empty={empty}
          scrollEnabled={scrollEnabled}
          divRef={divRef}
          onToggleScrolling={onToggleScrolling}
        />
      </ul>
    </div>
  );
}
